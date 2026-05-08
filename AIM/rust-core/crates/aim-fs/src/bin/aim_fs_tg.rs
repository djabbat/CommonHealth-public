//! aim-fs-tg — Telegram inbox bridge (one-shot command processor).
//!
//! Reads a Telegram update from stdin (JSON: `{"text": "...", "from_id": 123,
//! "tenant_id": "djabbat"}`) and writes a TG-ready reply to stdout
//! (`{"text": "...", "parse_mode": "Markdown"}`).
//!
//! Designed to be invoked as a subprocess from any TG bot (e.g. existing
//! `telegram_bot.py`):
//!
//! ```python
//! import subprocess, json
//! reply = subprocess.run(
//!     ["aim-fs-tg"],
//!     input=json.dumps({"text": update.message.text,
//!                       "from_id": update.message.from_user.id,
//!                       "tenant_id": "djabbat"}),
//!     capture_output=True, text=True
//! )
//! tg_payload = json.loads(reply.stdout)
//! await ctx.bot.send_message(chat, tg_payload["text"], parse_mode=tg_payload["parse_mode"])
//! ```
//!
//! Supported commands:
//!   `/inbox`              — list top 10 pending proposals
//!   `/approve <id>`       — approve proposal by id (full or 8-char prefix)
//!   `/reject <id> [reason]` — reject
//!   `/search <query>`     — search active entities
//!
//! Authorisation: the caller is expected to verify `from_id` against a TG
//! allowlist BEFORE invoking this binary (we only return command output;
//! security perimeter is the caller's).

use serde_json::{json, Value};
use std::io::{Read, Write};
use std::process::{Command, Stdio};

#[derive(serde::Deserialize)]
struct TgInput {
    text: String,
    #[serde(default)]
    from_id: Option<i64>,
    #[serde(default)]
    tenant_id: Option<String>,
}

fn main() -> anyhow::Result<()> {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf)?;
    let input: TgInput = serde_json::from_str(&buf)?;
    let tenant = input
        .tenant_id
        .or_else(|| std::env::var("AIM_FS_TENANT").ok())
        .unwrap_or_else(|| "djabbat".to_string());

    let reply = handle(&input.text, &tenant)?;
    let payload = json!({
        "text": reply,
        "parse_mode": "Markdown",
        "from_id": input.from_id,
    });
    let mut out = std::io::stdout().lock();
    writeln!(out, "{}", serde_json::to_string(&payload)?)?;
    Ok(())
}

fn handle(text: &str, tenant: &str) -> anyhow::Result<String> {
    let trimmed = text.trim();
    if let Some(rest) = trimmed.strip_prefix("/inbox") {
        let _ = rest;
        return cmd_inbox(tenant);
    }
    if let Some(rest) = trimmed.strip_prefix("/approve") {
        let id = rest.trim();
        if id.is_empty() {
            return Ok("⚠ usage: `/approve <proposal_id>`".into());
        }
        return cmd_approve(tenant, id);
    }
    if let Some(rest) = trimmed.strip_prefix("/reject") {
        let mut parts = rest.trim().splitn(2, ' ');
        let id = parts.next().unwrap_or("");
        let reason = parts.next();
        if id.is_empty() {
            return Ok("⚠ usage: `/reject <proposal_id> [reason]`".into());
        }
        return cmd_reject(tenant, id, reason);
    }
    if let Some(rest) = trimmed.strip_prefix("/search") {
        let q = rest.trim();
        if q.is_empty() {
            return Ok("⚠ usage: `/search <query>`".into());
        }
        return cmd_search(tenant, q);
    }
    Ok(help_text())
}

fn help_text() -> String {
    "*AIM_FS Telegram bridge*\n\
     `/inbox` — pending proposals (top 10)\n\
     `/approve <id>` — approve a proposal\n\
     `/reject <id> [reason]` — reject a proposal\n\
     `/search <query>` — search active entities"
        .to_string()
}

fn cmd_inbox(tenant: &str) -> anyhow::Result<String> {
    let res = aim_fs_call(json!({
        "op": "list_pending",
        "tenant_id": tenant,
        "limit": 10,
    }))?;
    let items = res.as_array().cloned().unwrap_or_default();
    if items.is_empty() {
        return Ok("📥 inbox is empty".into());
    }
    let mut out = format!("📥 *{} pending*\n\n", items.len());
    for p in &items {
        let id = p.get("id").and_then(|v| v.as_str()).unwrap_or("?");
        let short = id.chars().take(8).collect::<String>();
        let kind = p.get("proposal_type").and_then(|v| v.as_str()).unwrap_or("?");
        let rationale = p.get("rationale").and_then(|v| v.as_str()).unwrap_or("");
        let date = p
            .get("created_at")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .split('T')
            .next()
            .unwrap_or("");
        out.push_str(&format!(
            "• `{}` [{}] {} — {}\n  /approve {}  /reject {}\n",
            short,
            kind,
            date,
            truncate(rationale, 80),
            short,
            short,
        ));
    }
    Ok(out)
}

fn cmd_approve(tenant: &str, partial_id: &str) -> anyhow::Result<String> {
    let id = resolve_proposal_id(tenant, partial_id)?;
    aim_fs_call(json!({
        "op": "approve",
        "tenant_id": tenant,
        "proposal_id": id,
        "actor": {"user_id": tenant, "session_id": null},
    }))?;
    Ok(format!("✓ approved `{}`", &id[..id.len().min(8)]))
}

fn cmd_reject(tenant: &str, partial_id: &str, reason: Option<&str>) -> anyhow::Result<String> {
    let id = resolve_proposal_id(tenant, partial_id)?;
    aim_fs_call(json!({
        "op": "reject",
        "tenant_id": tenant,
        "proposal_id": id,
        "actor": {"user_id": tenant, "session_id": null},
        "reason": reason,
    }))?;
    Ok(format!("✗ rejected `{}`", &id[..id.len().min(8)]))
}

fn cmd_search(tenant: &str, query: &str) -> anyhow::Result<String> {
    let res = aim_fs_call(json!({
        "op": "search",
        "tenant_id": tenant,
        "query": query,
        "scope": {},
        "limit": 5,
    }))?;
    let hits = res.as_array().cloned().unwrap_or_default();
    if hits.is_empty() {
        return Ok(format!("🔍 no hits for `{query}`"));
    }
    let mut out = format!("🔍 *{} hits for* `{query}`\n\n", hits.len());
    for h in &hits {
        let id = h.get("id").and_then(|v| v.as_str()).unwrap_or("?");
        let short = id.chars().take(8).collect::<String>();
        let title = h.get("title").and_then(|v| v.as_str()).unwrap_or("(no title)");
        let snippet = h.get("snippet").and_then(|v| v.as_str()).unwrap_or("");
        out.push_str(&format!("• `{}` *{}*\n  {}\n", short, title, truncate(snippet, 100)));
    }
    Ok(out)
}

/// Resolve an 8-char prefix to a full ULID by looking it up in pending proposals.
fn resolve_proposal_id(tenant: &str, partial: &str) -> anyhow::Result<String> {
    if partial.len() >= 26 {
        return Ok(partial.to_string());
    }
    let res = aim_fs_call(json!({
        "op": "list_pending",
        "tenant_id": tenant,
        "limit": 100,
    }))?;
    let items = res.as_array().cloned().unwrap_or_default();
    let needle = partial.to_uppercase();
    for p in &items {
        if let Some(id) = p.get("id").and_then(|v| v.as_str()) {
            if id.starts_with(&needle) {
                return Ok(id.to_string());
            }
        }
    }
    anyhow::bail!("no pending proposal starts with `{partial}`")
}

fn aim_fs_call(payload: Value) -> anyhow::Result<Value> {
    let bin = std::env::var("AIM_FS_BIN").unwrap_or_else(|_| "aim-fs".to_string());
    let root = std::env::var("AIM_FS_ROOT").unwrap_or_else(|_| {
        std::env::var("HOME")
            .map(|h| format!("{h}/.aim_fs"))
            .unwrap_or_else(|_| "/var/lib/aim_fs".to_string())
    });
    let mut child = Command::new(&bin)
        .env("AIM_FS_ROOT", root)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()?;
    if let Some(mut stdin) = child.stdin.take() {
        stdin.write_all(serde_json::to_string(&payload)?.as_bytes())?;
        stdin.write_all(b"\n")?;
    }
    let out = child.wait_with_output()?;
    let stdout = String::from_utf8_lossy(&out.stdout);
    let first = stdout.lines().next().unwrap_or("{}");
    let v: Value = serde_json::from_str(first)?;
    if v.get("ok").and_then(|b| b.as_bool()) == Some(true) {
        Ok(v.get("result").cloned().unwrap_or(Value::Null))
    } else {
        anyhow::bail!(
            "{}",
            v.get("error")
                .and_then(|e| e.as_str())
                .unwrap_or("unknown")
                .to_string()
        )
    }
}

fn truncate(s: &str, max: usize) -> String {
    if s.chars().count() <= max {
        s.to_string()
    } else {
        let cut: String = s.chars().take(max - 1).collect();
        format!("{cut}…")
    }
}
