# LongevityCommon — Единый Design Concept

> Канон стиля, дизайна и расположения для всех страниц всех `*.longevity.ge` (кроме Annals/`/rescience/` и Longevity Horizon/`/longhoriz/`).
> Применять при добавлении новой страницы или субдомена. **Не дублировать стили локально** — всё тянется через `https://longevity.ge/eco-inject.js?v=NN`.

---

## 1. Вертикальное расположение (одно и то же для всех)

```
┌──────────────────────────────────────────────────┐
│ .eco-bar-injected                                │ ← sticky cross-site nav
│   (LongevityCommon | Home | MCOA | … | Source ☾) │
├──────────────────────────────────────────────────┤
│ .hero / .lc-sub-hero                             │ ← indigo gradient banner
│   - .hero-pill (зелёная точка + tag)             │
│   - .hero-title (clamp 2.5–3.75rem)              │
│   - .hero-subtitle                               │
│   - .hero-stats > .s > .k+.v (опц.)              │
│   - .hero-cta > .btn.btn-primary + .btn-ghost    │
├──────────────────────────────────────────────────┤
│ <header class="header"> own-header               │ ← brand + nav + lang
│   - .brand (logo)                                │
│   - <nav> in-site links                          │
│   - <form class="lang-switcher"> справа          │
├──────────────────────────────────────────────────┤
│ <main class="container"> page content            │ ← max-width 1100px
│   - .section-title + .section-lead               │
│   - .grid > .card.link > .role + h3 + p          │
├──────────────────────────────────────────────────┤
│ <footer> native                                  │ ← никаких extra-баров
└──────────────────────────────────────────────────┘
```

## 2. Цветовая палитра

| Token | Light | Dark |
|---|---|---|
| `--c-bg` | `#f8fafc` | `#0f1117` |
| `--c-card` | `#ffffff` | `#15171f` |
| `--c-border` | `#e2e8f0` | `#2a2f40` |
| `--c-accent` | `#4f46e5` | `#88a8ff` |
| `--c-text` | `#0f172a` | `#d8dce4` |
| `--c-text-soft` | `#475569` | `#c8ccd5` |
| `--c-text-muted` | `#64748b` | `#a0a8b5` |

**Hero gradient (одинаков в обеих темах — это бренд):**
```css
linear-gradient(135deg, #1e1b4b 0%, #312e81 35%, #4338ca 75%, #6366f1 100%)
```
Текст внутри hero — **всегда `#fff`**.

**Donate banner (зелёный):**
```css
linear-gradient(135deg, #064e3b 0%, #047857 50%, #10b981 100%)
```

**Тёмная тема** активируется через `<html data-theme="dark">`. Cookie `lc_theme` на `.longevity.ge` синхронизирует все субдомены.

## 3. Шрифты

- Body: **Inter** (Google Fonts, грузит eco-inject.js).
- Code: **JetBrains Mono**.
- Container max-width: **1100px** на ALL inner-обёртках (`.eco-inner-i`, `.container`, `.header-inner`, `.aim-subnav-inner`, `.page-hero-inner`, `.footer-inner`, `.hero-inner`, `.lc-sub-hero-inner`).

## 4. Канонические классы — использовать ровно эти

| Уровень | Класс | Назначение |
|---|---|---|
| Top | `.eco-bar-injected` > `.eco-inner-i` | Cross-site nav |
| Top | `.eco-brand-i` + `.eco-nav-i a` | Brand + 12 nav links |
| Top | `.theme-toggle-i` | ☾/☀ кнопка |
| Hero | `.hero` или `.lc-sub-hero` | Indigo banner |
| Hero | `.hero-pill / .hero-title / .hero-subtitle` | |
| Hero | `.hero-stats > .s > .k+.v` | Stat tiles (полупрозрачные) |
| Hero | `.btn.btn-primary` (white pill) | Primary CTA |
| Hero | `.btn.btn-ghost` (transparent + border) | Secondary CTA |
| Header | `<header class="header">` или `.site-header` или `.lc-own-header` | Own-header под hero |
| Header | `.header-inner` или `.container` или `.aim-subnav-inner` | Inner wrapper |
| Header | `.brand` + `<nav>` + `<form class="lang-switcher">` | Logo + nav + lang |
| Content | `.container` | Wrapper 1100px |
| Content | `.section-title + .section-lead` | Section header |
| Content | `.grid > .card.link > .role + h3 + p + .badges` | Card grid |
| Badges | `.badge.green / .blue / .purple / .gray / .warn / .red` | |

## 5. Контракт на JS-инжекторы (`eco-inject.js`)

| Функция | Вызывается | Работа |
|---|---|---|
| `injectSubHero()` | ze, biosense, fclc, mcoa, cdata, hive | Вставляет indigo `.lc-sub-hero` после eco-bar (текст из `SUB_HERO[host]`). |
| `injectOwnHeader()` | longevity.ge, mcoa, cdata | Создаёт `<header class="header lc-own-header">` если нативного нет (контент из `INJECTED_NAV[host]`). |
| `relocateOwnHeader()` | ze, biosense, fclc, hive (skip AIM) | Двигает существующий `<header>` сразу под hero. |
| `addLangToOwnHeader()` | все кроме AIM | Добавляет `<form class="lang-switcher"><select>` в первый `<header>` (любой не-eco-bar). |
| `dedupeHiveHero()` | hive | Скрывает native `<section class="hero">` (display:none). |
| `forceHeroBranding()` | home, AIM | Inline `color:#fff !important` на каждом потомке `.hero`. |
| `MutationObserver + setTimeout reapply` | все | Перевыполняет инжекторы если LiveView morphdom стёр. |

## 6. CSP-контракт для Phoenix-приложений

Каждое `*.longevity.ge` Phoenix-приложение должно разрешить:

```
default-src 'self' https://longevity.ge;
script-src  'self' https://longevity.ge;
style-src   'self' 'unsafe-inline' https://fonts.googleapis.com https://fonts.gstatic.com;
font-src    'self' https://fonts.googleapis.com https://fonts.gstatic.com;
img-src     'self' data: https://longevity.ge;
connect-src 'self' wss: https://longevity.ge;
```

В dev — добавить `'unsafe-inline' 'unsafe-eval'` в `script-src`.

## 7. Annals exclusion

`/rescience/` и `/longhoriz/` — early return в eco-inject.js. PKP theme не трогается. **Не добавлять** эти paths в инжекторы.

## 8. Deploy-чеклист (после изменения eco-inject.js)

1. `scp deploy/web-shared/eco-inject.js server:/home/jaba/web/ngo/eco-inject.js`
2. `sed -i s/v=N/v=N+1/ /etc/nginx/snippets/eco-inject.conf`
3. `sed -i s/v=*/v=N+1/ /home/jaba/web/ngo/index.html /home/jaba/web/longevity/index.html /var/www/longevitycommon-landing/index.html`
4. `systemctl reload nginx`
5. Bump `<script src="...?v=N+1">` в Phoenix `root.html.heex` для каждого Phoenix subdomain (currently AIM)
6. `mix release --overwrite` + redeploy Phoenix releases
7. `git push origin main` (private monorepo)
8. Sync to public — branch `public-main` from `private/main`, merge `main`, strip core .md, push `private public-main:main`

## 9. Запрещено

- ❌ `.lc-lang-bar` standalone bottom strip — депрекейт с v=54.
- ❌ `html[data-theme="dark"] section { ... }` — перекрашивает branded `.hero` / `.donate`.
- ❌ Локальные `<style>` overrides на `.eco-bar-injected`, `.header`, `.lang-switcher` — eco-inject уже задаёт через `!important`.
- ❌ Per-app загрузка Google Fonts через `<link>` — eco-inject грузит один раз.
- ❌ Hero gradient в per-subdomain CSS без `!important` — будет перебит dark cascade.
- ❌ Docker (системное правило проекта).

---

**История версий eco-inject.js, существенные изменения:**
- v=39 — sub-hero injection per host
- v=42 — `forceHeroBranding` + dedupeHiveHero
- v=44 — `.page-hero` (team/grants)
- v=46 — header inner widths унифицированы 1100px
- v=47 — own-header стиль одинаковый
- v=49 — order: header под hero
- v=50 — `injectOwnHeader` для home/mcoa/cdata
- v=54 — lang switcher в own-header (bottom-bar убран)
- v=55 — расширенный header selector (Tailwind FCLC)
- v=56+ — MutationObserver + reapply для LiveView reconnect
