#!/usr/bin/env python3
"""
Единая точка входа для LLM-вызовов (мульти-провайдер).

Поддерживает (автовыбор бесплатного если доступно):
  - Groq (Llama 3.1 8B) — БЕСПЛАТНО
  - Gemini 2.5 Flash — БЕСПЛАТНО
  - Ollama (локально) — БЕСПЛАТНО
  - DeepSeek (chat + reasoner) — платный, дешёвый

Usage:
    from llm import ask
    reply = ask("Hello")                       # авто-выбор (бесплатный если есть)
    reply = ask("Hello", provider="groq")       # конкретный провайдер
    reply = ask("Hello", task="coding")         # по типу задачи
    reply = ask_reasoner("Complex problem")     # для сложных рассуждений

Старый API (только DeepSeek):
    reply = ask("Hello", model="deepseek-chat")  # работает как раньше

see providers.py для полной документации.
"""

import os
import sys
from pathlib import Path
from typing import Optional

_HERE = Path(__file__).resolve().parent
_PROVIDERS_PATH = _HERE.parent / "Services/tbpr/scripts/providers.py"

# Если providers.py не найден — используем прямой DeepSeek
_USE_MULTI = _PROVIDERS_PATH.is_file()

if _USE_MULTI:
    sys.path.insert(0, str(_PROVIDERS_PATH.parent))
    from providers import ask as _multi_ask
    from providers import ask_reasoner as _multi_reasoner
    from providers import list_providers, diagnose

    def ask(
        prompt: str,
        model: Optional[str] = None,
        provider: Optional[str] = None,
        task: Optional[str] = None,
        temperature: float = 0.2,
        max_tokens: int = 4096,
        timeout: int = 300,
    ) -> str:
        """
        Отправить prompt в LLM (автовыбор провайдера).

        Args:
            prompt: текст запроса
            model: конкретная модель (deepseek-chat, deepseek-reasoner, ...)
            provider: провайдер (deepseek, groq, gemini-free, ollama)
            task: тип задачи (quick, coding, reasoning, factcheck, draft, review, translate, local)
            temperature: 0.0-1.0
            max_tokens: макс. токенов
            timeout: таймаут

        Если provider не указан — автовыбор:
          - quick → Groq (free)
          - coding → DeepSeek
          - reasoning → DeepSeek-reasoner
          - иначе → бесплатный (Groq > Gemini > DeepSeek)
        """
        return _multi_ask(
            prompt,
            provider=provider,
            model=model,
            task=task,
            temperature=temperature,
            max_tokens=max_tokens,
            timeout=timeout,
        )

    def ask_reasoner(prompt: str, **kwargs) -> str:
        """Для сложных рассуждений (deepseek-reasoner или локальный deepseek-r1)."""
        return _multi_reasoner(prompt, **kwargs)

else:
    # Fallback на старый DeepSeek-only
    import requests

    _DOTENV_LOADED = False

    def _load_env():
        global _DOTENV_LOADED
        if _DOTENV_LOADED:
            return
        env_path = Path.home() / ".aim_env"
        if env_path.is_file():
            for line in env_path.read_text().splitlines():
                line = line.strip()
                if not line or line.startswith("#") or "=" not in line:
                    continue
                k, v = line.split("=", 1)
                os.environ.setdefault(k.strip(), v.strip())
        _DOTENV_LOADED = True

    def ask(
        prompt: str,
        model: str = "deepseek-chat",
        temperature: float = 0.2,
        max_tokens: int = 4096,
        timeout: int = 300,
        **kwargs,
    ) -> str:
        """Старый DeepSeek-only вызов."""
        _load_env()
        api_key = os.environ.get("DEEPSEEK_API_KEY") or os.environ.get("AIM_DEEPSEEK_KEY")
        if not api_key:
            raise RuntimeError("DEEPSEEK_API_KEY not set")
        payload = {
            "model": model,
            "messages": [{"role": "user", "content": prompt}],
            "temperature": temperature,
            "max_tokens": max_tokens,
        }
        resp = requests.post(
            "https://api.deepseek.com/v1/chat/completions",
            headers={"Authorization": f"Bearer {api_key}", "Content-Type": "application/json"},
            json=payload,
            timeout=timeout,
        )
        resp.raise_for_status()
        return resp.json()["choices"][0]["message"]["content"]

    def ask_reasoner(prompt: str, **kwargs) -> str:
        kwargs.setdefault("temperature", 0)
        return ask(prompt, model="deepseek-reasoner", **kwargs)

    def list_providers():
        return {"deepseek (fallback)": {"models": ["deepseek-chat", "deepseek-reasoner"]}}

    def diagnose():
        return {"deepseek (fallback)": {"available": True}}
