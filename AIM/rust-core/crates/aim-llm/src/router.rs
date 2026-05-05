use crate::providers::{Provider, ProviderId};
use aim_common::{ApiError, ApiResult};
use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Clone)]
pub struct RouterState {
    pub providers: Arc<Vec<Box<dyn Provider>>>,
    pub cache: crate::cache::PromptCache,
}

impl RouterState {
    pub fn from_env() -> Self {
        let providers = crate::providers::load_from_env();
        Self {
            providers: Arc::new(providers),
            cache: crate::cache::PromptCache::from_env(),
        }
    }

    pub fn provider_by_id(&self, id: ProviderId) -> Option<&dyn Provider> {
        self.providers.iter().find(|p| p.id() == id).map(|b| b.as_ref())
    }

    pub fn first_ready(&self) -> Option<&dyn Provider> {
        self.providers.iter().find(|p| p.is_ready()).map(|b| b.as_ref())
    }
}

#[derive(Serialize)]
pub struct ProviderInfo {
    pub id: ProviderId,
    pub ready: bool,
    pub default_model: &'static str,
}

pub async fn list_providers(State(state): State<RouterState>) -> Json<Vec<ProviderInfo>> {
    let infos = state.providers.iter().map(|p| ProviderInfo {
        id: p.id(),
        ready: p.is_ready(),
        default_model: p.default_model(),
    }).collect();
    Json(infos)
}

#[derive(Deserialize)]
pub struct ChatRequest {
    pub messages: Vec<ChatMessage>,
    /// Model identifier; we infer the provider from its prefix.
    pub model_hint: Option<String>,
    /// Force a specific provider; overrides model_hint inference.
    pub provider: Option<ProviderId>,
    /// Tier name (per CLAUDE.md): critical / deep / long / default / fast.
    pub tier: Option<String>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

#[derive(Serialize)]
pub struct ChatResponse {
    pub reply: String,
    pub provider: ProviderId,
    pub model: String,
    pub attempts: Vec<Attempt>,
}

#[derive(Serialize, Clone)]
pub struct Attempt {
    pub provider: ProviderId,
    pub model: String,
    pub ok: bool,
    pub error: Option<String>,
}

/// Map a model name (or hint) to its provider when possible.
pub fn provider_for_model(model: &str) -> Option<ProviderId> {
    let m = model.to_lowercase();
    if m.starts_with("deepseek") { return Some(ProviderId::DeepSeek); }
    if m.starts_with("claude")   { return Some(ProviderId::Anthropic); }
    if m.starts_with("gemini")   { return Some(ProviderId::Gemini); }
    if m.starts_with("llama")    { return Some(ProviderId::Groq); }
    if m.starts_with("qwen") || m.starts_with("phi") || m.contains(":") {
        // Ollama models often look "llama3.2" / "qwen2.5:7b"
        return Some(ProviderId::Ollama);
    }
    None
}

/// Tier → ordered chain of (provider, model) per CLAUDE.md.
pub fn tier_chain(tier: &str) -> Vec<(ProviderId, &'static str)> {
    match tier {
        "critical" => vec![
            (ProviderId::Anthropic, "claude-opus-4-7"),
            (ProviderId::Gemini,    "gemini-2.5-pro"),
            (ProviderId::DeepSeek,  "deepseek-reasoner"),
            (ProviderId::Ollama,    "deepseek-r1"),
        ],
        "deep" => vec![
            (ProviderId::DeepSeek,  "deepseek-reasoner"),
            (ProviderId::Anthropic, "claude-opus-4-7"),
            (ProviderId::Gemini,    "gemini-2.5-pro"),
            (ProviderId::Ollama,    "deepseek-r1"),
        ],
        "long" => vec![
            (ProviderId::DeepSeek,  "deepseek-chat"),
            (ProviderId::Gemini,    "gemini-2.5-pro"),
            (ProviderId::Ollama,    "qwen2.5:7b"),
        ],
        "fast" => vec![
            (ProviderId::Groq,      "llama-3.1-8b-instant"),
            (ProviderId::DeepSeek,  "deepseek-chat"),
            (ProviderId::Ollama,    "qwen2.5:3b"),
        ],
        // "default" or anything else
        _ => vec![
            (ProviderId::DeepSeek,  "deepseek-chat"),
            (ProviderId::Gemini,    "gemini-2.5-flash"),
            (ProviderId::Ollama,    "qwen2.5:7b"),
        ],
    }
}

pub async fn chat(
    State(state): State<RouterState>,
    Json(req): Json<ChatRequest>,
) -> ApiResult<Json<ChatResponse>> {
    if req.messages.is_empty() {
        return Err(ApiError::BadRequest("messages cannot be empty".into()));
    }

    // Build the chain. Priority:
    //  1. Explicit provider field.
    //  2. model_hint (try inferred provider first, then fall back to others).
    //  3. tier name.
    //  4. first ready provider with its default model.
    let chain: Vec<(ProviderId, String)> = if let Some(pid) = req.provider {
        let model = req.model_hint.unwrap_or_else(|| {
            state.provider_by_id(pid).map(|p| p.default_model().to_string()).unwrap_or_default()
        });
        vec![(pid, model)]
    } else if let Some(model) = &req.model_hint {
        let mut v = Vec::new();
        if let Some(inferred) = provider_for_model(model) {
            v.push((inferred, model.clone()));
        }
        // After explicit, allow tier-style fallback to default.
        for (p, m) in tier_chain(req.tier.as_deref().unwrap_or("default")) {
            if !v.iter().any(|(pp, _)| *pp == p) {
                v.push((p, m.into()));
            }
        }
        v
    } else if let Some(tier) = &req.tier {
        tier_chain(tier).into_iter().map(|(p, m)| (p, m.into())).collect()
    } else if let Some(p) = state.first_ready() {
        vec![(p.id(), p.default_model().to_string())]
    } else {
        return Err(ApiError::Upstream("no ready provider".into()));
    };

    let mut attempts = Vec::new();
    let max_retries_per_step: u32 = std::env::var("AIM_LLM_RETRIES").ok()
        .and_then(|s| s.parse().ok()).unwrap_or(2);

    for (pid, model) in &chain {
        let Some(provider) = state.provider_by_id(*pid) else {
            attempts.push(Attempt {
                provider: *pid, model: model.clone(),
                ok: false, error: Some("provider not loaded".into()),
            });
            continue;
        };
        if !provider.is_ready() {
            attempts.push(Attempt {
                provider: *pid, model: model.clone(),
                ok: false, error: Some("not ready (no API key?)".into()),
            });
            continue;
        }

        // Cache check (per provider+model combination).
        let cache_key = if state.cache.enabled() {
            let msgs_json = serde_json::to_string(&req.messages).unwrap_or_default();
            let composite = format!("{:?}/{model}", pid);
            Some(crate::cache::PromptCache::key(&composite, &msgs_json))
        } else { None };

        if let Some(key) = cache_key.as_deref() {
            if let Some(cached) = state.cache.get(key) {
                attempts.push(Attempt { provider: *pid, model: model.clone(), ok: true, error: Some("cache_hit".into()) });
                aim_common::req_inc("aim-llm", "/v1/chat", "cache_hit");
                return Ok(Json(ChatResponse {
                    reply: cached, provider: *pid, model: model.clone(), attempts,
                }));
            }
        }

        let mut last_err: Option<String> = None;
        for attempt in 0..=max_retries_per_step {
            let provider_label = format!("{:?}", pid).to_lowercase();
            match provider.complete(&req.messages, model).await {
                Ok(reply) => {
                    attempts.push(Attempt { provider: *pid, model: model.clone(), ok: true, error: None });
                    aim_common::upstream_inc("aim-llm", &provider_label, "ok");
                    aim_common::req_inc("aim-llm", "/v1/chat", "ok");
                    if let Some(key) = cache_key.as_deref() {
                        state.cache.put(key, key, &reply, model);
                    }
                    return Ok(Json(ChatResponse {
                        reply, provider: *pid, model: model.clone(), attempts,
                    }));
                }
                Err(e) => {
                    let msg = e.to_string();
                    if attempt < max_retries_per_step && is_transient(&msg) {
                        aim_common::upstream_inc("aim-llm", &provider_label, "retry");
                        let delay = std::time::Duration::from_millis(200 * (1 << attempt));
                        tokio::time::sleep(delay).await;
                        last_err = Some(msg);
                        continue;
                    }
                    aim_common::upstream_inc("aim-llm", &provider_label, "fail");
                    last_err = Some(msg);
                    break;
                }
            }
        }
        attempts.push(Attempt {
            provider: *pid, model: model.clone(),
            ok: false, error: last_err,
        });
    }

    let summary = attempts.iter()
        .map(|a| format!("{:?}/{}: {}", a.provider, a.model,
            a.error.clone().unwrap_or_else(|| "?".into())))
        .collect::<Vec<_>>()
        .join("; ");
    aim_common::req_inc("aim-llm", "/v1/chat", "all_failed");
    Err(ApiError::Upstream(format!("all providers failed: {summary}")))
}

fn is_transient(err: &str) -> bool {
    let e = err.to_lowercase();
    e.contains("timeout") || e.contains("timed out")
        || e.contains("503") || e.contains("502") || e.contains("504")
        || e.contains("connection") || e.contains("dns")
        || e.contains("rate limit") || e.contains("429")
}
