# AUDIT PACKET — srv_spellcheckerka (server)

Host: `server`  Path: `/home/jaba/web/spellcheckerka`  Date: 2026-05-08

## Size
```
55M	/home/jaba/web/spellcheckerka
```
## Tree
```
/home/jaba/web/spellcheckerka
/home/jaba/web/spellcheckerka/assets
/home/jaba/web/spellcheckerka/assets/css
/home/jaba/web/spellcheckerka/assets/package.json
/home/jaba/web/spellcheckerka/assets/js
/home/jaba/web/spellcheckerka/assets/package-lock.json
/home/jaba/web/spellcheckerka/assets/vendor
/home/jaba/web/spellcheckerka/assets/tailwind.config.js
/home/jaba/web/spellcheckerka/PARAMETERS.md
/home/jaba/web/spellcheckerka/docker
/home/jaba/web/spellcheckerka/docker/Dockerfile
/home/jaba/web/spellcheckerka/docker/docker-compose.yml
/home/jaba/web/spellcheckerka/docker/nginx.conf
/home/jaba/web/spellcheckerka/mix.exs
/home/jaba/web/spellcheckerka/config
/home/jaba/web/spellcheckerka/config/test.exs
/home/jaba/web/spellcheckerka/config/config.exs
/home/jaba/web/spellcheckerka/config/runtime.exs
/home/jaba/web/spellcheckerka/config/prod.exs
/home/jaba/web/spellcheckerka/config/dev.exs
/home/jaba/web/spellcheckerka/run.sh
/home/jaba/web/spellcheckerka/priv
/home/jaba/web/spellcheckerka/priv/static
/home/jaba/web/spellcheckerka/priv/gettext
/home/jaba/web/spellcheckerka/OPEN_PROBLEMS.md
/home/jaba/web/spellcheckerka/deploy.sh
/home/jaba/web/spellcheckerka/extension
/home/jaba/web/spellcheckerka/extension/manifest.json
/home/jaba/web/spellcheckerka/extension/background.js
/home/jaba/web/spellcheckerka/extension/popup.html
/home/jaba/web/spellcheckerka/extension/content_generic.js
/home/jaba/web/spellcheckerka/extension/icons
/home/jaba/web/spellcheckerka/extension/content_gdocs.js
/home/jaba/web/spellcheckerka/extension/popup.js
/home/jaba/web/spellcheckerka/extension/panel.css
/home/jaba/web/spellcheckerka/EVIDENCE.md
/home/jaba/web/spellcheckerka/README.md
/home/jaba/web/spellcheckerka/_archive
/home/jaba/web/spellcheckerka/_archive/core_pre_9file_2026-04-25
/home/jaba/web/spellcheckerka/DESIGN.md
/home/jaba/web/spellcheckerka/lib
/home/jaba/web/spellcheckerka/lib/spellcheckerka_web.ex
/home/jaba/web/spellcheckerka/lib/spellcheckerka
/home/jaba/web/spellcheckerka/lib/spellcheckerka.ex
/home/jaba/web/spellcheckerka/lib/spellcheckerka_web
/home/jaba/web/spellcheckerka/test
/home/jaba/web/spellcheckerka/test/test_helper.exs
/home/jaba/web/spellcheckerka/test/spellcheckerka
/home/jaba/web/spellcheckerka/test/support
/home/jaba/web/spellcheckerka/test/spellcheckerka_web
/home/jaba/web/spellcheckerka/CLAUDE.md
/home/jaba/web/spellcheckerka/THEORY.md
/home/jaba/web/spellcheckerka/CONCEPT_CODE_AUDIT_2026-04-21.md
/home/jaba/web/spellcheckerka/STATE.md
/home/jaba/web/spellcheckerka/mix.lock
/home/jaba/web/spellcheckerka/docs
/home/jaba/web/spellcheckerka/docs/PEER_REVIEW_SpellCheckerKa.md
/home/jaba/web/spellcheckerka/docs/REFERENCE_AUDIT_SpellCheckerKa.md
/home/jaba/web/spellcheckerka/docs/META_ANALYSIS_SpellCheckerKa.md
/home/jaba/web/spellcheckerka/CONCEPT.md

```
## Stack probe
```
---rust---
---elixir---
./mix.exs
---go---
---php---
---python---
./extension/icons/generate_icons.py
---node---
./assets/package.json

```
### `CLAUDE.md` (head 200 lines)
```
# CLAUDE.md — Operating Rules for Claude on SpellCheckerKa

Short. Authoritative. Points to other core docs rather than duplicating them.

---

## Startup protocol

On every new session:

1. **Read `STATE.md`** first — current status, P0–P5 queue, "What NOT to do", Decision Log, freeze dates.
2. Consult `CONCEPT.md` for canonical scope.
3. For code details pull from `DESIGN.md` (architecture), `THEORY.md` (algorithms), `PARAMETERS.md` (values).
4. For external references pull from `EVIDENCE.md`.
5. For known gaps and risks pull from `OPEN_PROBLEMS.md`.
6. Never assume — if the answer isn't in the 9 core files plus the `CONCEPT_CODE_AUDIT_2026-04-21.md` artifact, say so.

The 9 core docs: `CONCEPT`, `README`, `CLAUDE`, `THEORY`, `DESIGN`, `EVIDENCE`, `PARAMETERS`, `STATE`, `OPEN_PROBLEMS`. Legacy docs live only in `_archive/core_pre_9file_2026-04-25/`.

---

## Project identity at a glance

- **SpellCheckerKa** — Georgian spell checker (ქართული მართლწერის შემოწმება).
- **Domain:** `spellcheckerka.drjaba.com` (legacy: `schecker.ge`).
- **Stack:** Elixir 1.16 + Phoenix 1.7, Vanilla JS, ETS (no SQL), Manifest V3 extension.
- **Status:** Phase 1 (Web) + Phase 3 (Extension) complete; Phase 2 (Mobile) frozen until 2026-09-27.

For anything deeper see the pointed-to docs above.

---

## Non-negotiable rules (freeze-era, applies daily)

See STATE.md §"What NOT to do" for the full list. The most important items:

- **Never replace ETS with a SQL database.**
- **Never switch the editor to `contenteditable`** — the mirror overlay stays.
- **Never load `ge-<hash>.txt`** (the 993k variant). Canonical dict is `ge.txt`.
- **Never downgrade the extension to Manifest V2.**
- **Never ship grammar/auto-correct features during the freeze** (ends 2026-09-27). Allowed: critical bug fixes + admin tooling.
- **Never commit `CLAUDE.md`, `STATE.md`, `PARAMETERS.md` to the public repo.**
- **Never commit `user_words.txt`** — runtime state only.

Before violating any of the above, surface the trade-off explicitly and ask the user.

---

## Files that MUST stay at the root

Phoenix / Mix project structure requirements:
- `mix.exs` · `mix.lock` · `.formatter.exs` · `.gitignore`
- `lib/`, `assets/`, `priv/`, `config/`, `test/`, `docker/`, `extension/`, `deploy.sh`, `run.sh`

These are **not** to be moved during doc reorganisations.

---

## Build & run

```bash
bash run.sh dev           # local development
bash run.sh test          # mix test
bash run.sh docker-build  # build docker image
bash deploy.sh            # build → scp → docker compose on production
```

Full request-flow and deployment diagram: **DESIGN.md §§4–6**.

---

## Repo split (public vs private)

- **Private:** `djabbat/SpellCheckerKa` — full content (all 9 core files + audit + code).
- **Public:** `djabbat/SpellCheckerKa-public` — excludes `CLAUDE.md`, `STATE.md`, `PARAMETERS.md`. Ships: `README.md`, `CONCEPT.md`, `THEORY.md`, `DESIGN.md`, `EVIDENCE.md`, `OPEN_PROBLEMS.md`, code.

---

## DeepSeek rule

Route all non-code tasks through DeepSeek API (`~/.aim_env → DEEPSEEK_API_KEY`).

---

## Git rule

The repo is a git repo. When moving core files, prefer `git mv` so history is preserved. When blocked, fall back to plain `mv`; git's content-similarity rename detection recovers the link on the next commit.

---

## Where to find what (cheat sheet)

| You need… | Read… |
|---|---|
| Vision, scope, ecosystem role | `CONCEPT.md` |
| Algorithms, formal model, accuracy targets | `THEORY.md` |
| Repo tree, API contracts, deploy, performance | `DESIGN.md` |
| Numerical values (dict size, limits, prices, env vars) | `PARAMETERS.md` |
| External refs, competitors, licensing, ecosystem URLs | `EVIDENCE.md` |
| Current TODOs, milestones, Decision Log, constraints | `STATE.md` |
| Known gaps, risks, research questions | `OPEN_PROBLEMS.md` |
| Public quickstart | `README.md` |
| 2026-04-21 doc-vs-code audit | `CONCEPT_CODE_AUDIT_2026-04-21.md` (historical artifact) |

```
### `README.md` (head 200 lines)
```
# SpellCheckerKa — Georgian Spell Checker / ქართული მართლწერის შემოწმება

**English** · Real-time Georgian spell checker built with Elixir / Phoenix.
142,285 base words (+ user additions) · Levenshtein + Georgian letter confusion table (ს↔შ, კ↔ქ, რ↔ლ, and 22 more) · Morphological analysis (134 suffix rules, 7 noun cases, 13 verb preverbs) · `.docx` import/export · REST API · User dictionary · MIT License.

Production: **https://spellcheckerka.drjaba.com**

---

## Quickstart

```bash
# 1. dependencies
mix deps.get
cd assets && npm install && cd ..

# 2. build assets
mix assets.build

# 3. run
mix phx.server          # → http://localhost:4000
```

Or use the helper:

```bash
bash run.sh dev
bash run.sh test
bash run.sh docker-build
```

### Production

```bash
MIX_ENV=prod mix do deps.get, compile, assets.deploy
SECRET_KEY_BASE=<key> PHX_HOST=example.com mix phx.server
```

### Docker

```bash
docker compose -f docker/docker-compose.yml up
```

---

## REST API (3 endpoints)

| Method | Path | Body | Returns |
|---|---|---|---|
| POST | `/api/check` | `{text, lang}` or `{chunks: [...], lang}` | `{total_words, error_count, accuracy, errors[], typography[], stopwords[]}` |
| POST | `/api/dictionary/add` | `{word}` | `{ok: true, word}` |
| POST | `/api/dictionary/remove` | `{word}` | `{ok: true, word}` |

Rate limits: **30 req/min per IP** · **30 min/day of active checking per IP** (free tier). Errors: 400 · 402 (quota) · 429 (rate).

Example:

```bash
curl -X POST https://spellcheckerka.drjaba.com/api/check \
  -H "Content-Type: application/json" \
  -d '{"text": "ქართული ტექსტი", "lang": "ka"}'
```

Full contract (including the `X-Admin-Token` bypass, chunk mode, and planned endpoints): see `DESIGN.md`.

---

## Features

- **Real-time checking** with adaptive debounce (500 ms – 2.5 s).
- **Mirror overlay** — highlights errors directly over a `<textarea>` (cursor-safe, IME-safe, native undo/redo).
- **Hover menu** — top-5 suggestions per misspelling.
- **User dictionary** — add words, persisted to `user_words.txt`.
- **Typography + stopwords** — double spaces, quote style, hyphen/dash, 25 Georgian filler words.
- **`.docx` I/O** — fully client-side (fflate + DOMParser).
- **Multi-language backend** — Georgian (full model) + English / French / Spanish / Russian (plain Levenshtein).
- **8-language UI** — Georgian · English · French · Spanish · Russian · Arabic · Chinese · (+1).
- **Browser extension** — Manifest V3, keyboard shortcut `Ctrl+Shift+K` (`Cmd+Shift+K` on macOS).

---

## Project layout (brief)

```
lib/spellcheckerka/              core modules (dictionary, morphology, usage tracker, …)
lib/spellcheckerka_web/          Phoenix router, controllers, plugs
assets/                          Vanilla JS + Tailwind
priv/static/dictionaris/         ge.txt (142,285 words), user_words.txt, en/fr/es/ru
extension/                       Manifest V3 browser extension
docker/                          Dockerfile + compose + nginx.conf
```

Full tree and per-module responsibilities: see `DESIGN.md`.

---

## How it works (one-paragraph version)

A token `w` is valid iff it exists in the Georgian ETS dictionary or its morphology routine can reduce it to a known stem via the 134 suffix rules + 13 verb preverbs. Misspelled tokens trigger a Levenshtein search (max 4 edits) across dictionary buckets indexed by `{first_char, length}`; the confusion table expands the candidate pool across phonetically/visually similar letters (ს↔შ, კ↔ქ, ბ↔პ, and 22 more). Results are sorted by distance and truncated to the top 5. Formal definitions and accuracy predictions: `THEORY.md`.

---

## License

MIT — © 2025 2sco. Dictionary derived from Hunspell `ka_GE` (LGPL). Full licensing notes in `EVIDENCE.md §7`.

---

# ქართულად

**ႫႠႰႧႪႼႤႰႠ** არის ვებ-აპლიკაცია ქართული ტექსტების მართლწერის შემოწმებისთვის, აგებული Elixir / Phoenix-ზე. მხარდაჭერა 142,285 ძირითადი სიტყვის ლექსიკონით, მორფოლოგიური ანალიზით და .docx ფაილების იმპორტ/ექსპორტით.

## ფუნქციონალი

- **რეალურ დროში შემოწმება** — ადაპტური დებაუნსი 500 მწმ – 2.5 წმ.
- **Mirror-overlay** — შეცდომების მონიშვნა textarea-ზე (კურსორი და undo/redo ნატიური).
- **Hover-მენიუ** — top-5 შემოთავაზება.
- **ლექსიკონში დამატება** — `user_words.txt`-ში ინახება.
- **ტიპოგრაფიის შემოწმება** — ორმაგი სფასები, ბრჭყალები, ტირე.
- **სტოპ-სიტყვები** — 25 გამავსებელი სიტყვა.
- **.docx გახსნა/შენახვა** — მთლიანად კლიენტის მხარეს (fflate + DOMParser).
- **API** — REST, Rate limit 30 მოთხოვნა/წუთში.
- **Browser Extension** — Manifest V3, Ctrl+Shift+K.

ტექნიკური დეტალები: `DESIGN.md` · ალგორითმი: `THEORY.md` · პარამეტრები: `PARAMETERS.md`.

---

*Production domain: **https://spellcheckerka.drjaba.com***

```
### `CONCEPT.md` (head 200 lines)
```
# CONCEPT.md — SpellCheckerKa

> **Single source of truth for what this project IS.**
> Vision, scope, ecosystem position, and what is explicitly out of scope.
> Technical details live elsewhere — see §7.

---

## 1. Identity

- **Name:** SpellCheckerKa (SpellChecker + `ka` = ISO 639-1 code for Georgian).
- **One-line pitch:** multi-platform Georgian spell-checking ecosystem — Web + Mobile + Browser Extension + REST API.
- **Domain:** `spellcheckerka.drjaba.com` (legacy redirect: `schecker.ge`).
- **Stack:** Elixir/Phoenix backend · Vanilla JS frontend · ETS-only storage · Manifest V3 browser extension · React Native mobile (planned).
- **Licence:** MIT code / LGPL derived dictionary.

---

## 2. Vision

Georgian speakers write everywhere: in browsers, messengers, documents, mobile apps, social media. No existing tool provides serious Georgian spell checking across all of those contexts. SpellCheckerKa fills that gap with a single shared backend and three coordinated clients.

**Core claim.** The only product combining (a) full Georgian support (142,285 base words + morphology + confusion table), (b) Web + Mobile + Extension distribution, (c) open REST API, (d) a free tier.

---

## 3. Scope (three products, one backend)

| Product | Purpose | Status |
|---|---|---|
| **Web app** | Long-form editing, `.docx` import/export, statistics, user dictionary | Phase 1 complete |
| **Mobile (iOS + Android)** | Writing on the go — messengers, notes, SMS, camera OCR | Phase 2, post-freeze |
| **Browser extension** | Inline checking in any text field on any site | Phase 3 complete |
| **REST API** | Shared backend; ecosystem microservice | v1 live (3 endpoints) |

All three share one Phoenix backend and one lexicon. User dictionaries sync across clients (post-auth).

---

## 4. Target audience (priority segments)

1. **Writers, journalists, students** — long-form Georgian text, high volume.
2. **Social-media users** — short texts at scale, extension-led.
3. **Academic authors** — especially via the OJS / Longevity Horizon pipeline (see §6.4).
4. **Developers** — REST API consumers.
5. **Government and educational institutions** — B2G / B2B on-premise.

---

## 5. Competitive differentiators

Summarised here; full analysis in **EVIDENCE.md §2**.

- **Georgian-first, not Georgian-afterthought.** Morphology + confusion pairs, not just a word list.
- **Multi-platform from day one.** Grammarly has extension + web, not Georgian. MS Word has Georgian, not a browser. We have both.
- **Open REST API.** Free tier + paid tiers; explicit microservice role.
- **Client-side `.docx`.** Privacy-preserving; no text leaves the browser except for the check call.

---

## 6. Ecosystem position (drjaba.com umbrella)

SpellCheckerKa is a sibling of AIM, kSystem, DrJaba, monetaria, and the OJS longevity.ge journal. Full descriptions and integration points in **EVIDENCE.md §5**.

### 6.1. SpellCheckerKa ↔ AIM
AIM uses `/api/check` (with `X-Admin-Token`) to validate Georgian patient-facing content.

### 6.2. SpellCheckerKa ↔ kSystem
Bidirectional: kSystem's Georgian article generation is spell-checked pre-publication; its specialist vocabulary is a candidate for dictionary enrichment.

### 6.3. SpellCheckerKa ↔ DrJaba infrastructure
Hosted at `spellcheckerka.drjaba.com` on shared DrJaba production infra.

### 6.4. SpellCheckerKa ↔ OJS (Longevity Horizon)
Academic Georgian text quality for journal submissions. **The project is frozen until 2026-09-27 (OJS Scholar submission date).** Only bug fixes and admin tooling during freeze. See **STATE.md** for details.

### 6.5. SpellCheckerKa ↔ monetaria
Candidate billing integration for Premium / Corporate tiers.

---

## 7. Pointers into the rest of the core docs

Everything below is **not repeated here** — read the pointed-to doc for detail.

| Question | Doc |
|---|---|
| How do the algorithms work? (dictionary model, morphology, Levenshtein, ranking, falsifiable accuracy targets) | **THEORY.md** |
| What's the architecture and file layout? (full tree, API contracts, request flow, deploy plan, performance targets) | **DESIGN.md** |
| What numerical values are in effect? (dict size, thresholds, limits, pricing tiers, env vars) | **PARAMETERS.md** |
| What's the status today? (TODOs, milestones, decisions, constraints, startup checklist) | **STATE.md** |
| Where do external references and ecosystem links live? | **EVIDENCE.md** |
| What's known-broken, unimplemented, or risky? | **OPEN_PROBLEMS.md** |
| How should Claude operate on this repo? | **CLAUDE.md** |
| How does a user or contributor get started? | **README.md** (public-safe) |

---

## 8. Explicitly out of scope (v1)

Full discussion in **OPEN_PROBLEMS.md §3**. Summary:

- **Not a grammar checker.** Spell + typography + stopwords only. Grammar is a Phase 5 research direction.
- **Not a style editor.** No tone, readability, or clarity suggestions.
- **No auto-correct.** Substitutions are always user-initiated.
- **No voice input / Georgian ASR.**
- **No native iOS / Android keyboard in Phase 1 or Phase 3.** Phase 2 evaluates keyboard extensions; fallback UX is copy-check-paste.
- **No Microsoft Word plugin.** Revisit only on customer request.
- **No SQL database — ever.** ETS is the storage layer by architectural decision (see STATE.md §What NOT to do).

---

## 9. Monetisation posture

Freemium. Free tier = 30 min/day active checking per IP, 30 req/min. Premium and Corporate tiers documented with exact figures in **PARAMETERS.md §6**. Payment integration is P1 (tracked in STATE.md), not yet live.

---

## 10. Success definition

The project has succeeded when all of the following are true:

1. Georgian speakers reach for SpellCheckerKa before Word / Grammarly / LibreOffice when checking Georgian text.
2. The API is a stable microservice that sibling projects (AIM, kSystem, OJS) rely on without workarounds.
3. Premium MRR covers infrastructure and documented R&D costs.
4. THEORY.md's accuracy predictions (P1–P6) are measured against live corpora, not just asserted.

Quantitative targets (1-year, per marketing plan): 50,000 MAU · 20,000 extension installs · 30,000 mobile installs · $10,000+ MRR.

---

*© 2025–2026 SpellCheckerKa — built for the Georgian language.*

```
### `THEORY.md` (head 200 lines)
```
# THEORY.md — SpellCheckerKa

Formal definitions of the spell-checking model: dictionary structure, morphology rules, edit-distance metric, suggestion ranking, and falsifiable accuracy predictions.

For numerical values (table sizes, thresholds, limits), see **PARAMETERS.md**.
For linguistic sources and related work, see **EVIDENCE.md**.

---

## 1. Dictionary Structure

### 1.1. Lexical store
The canonical Georgian lexicon is a finite set `D` of surface word forms (no lemma compression, no paradigm expansion at storage time). It is represented as:

- An ETS set `:ge_dictionary` keyed by the exact string; membership test is O(1) average.
- An ETS companion index `:ge_dictionary_index` bucketed by `{first_char, length}` → `[words]`, used to restrict Levenshtein candidate search.

User-added words live in the same table (no separate `:user_dict` ETS). Persistence of user additions is an append-only file `user_words.txt`, loaded on boot.

### 1.2. Formal definition
For an input token `w`:

```
valid(w)  ≡  w ∈ D  ∨  morph_valid(w)
```

- `D` = canonical dictionary ∪ user-added words (see §1.1).
- `morph_valid/1` = morphological validity check (see §2).

A token is flagged as misspelled iff `¬valid(w)`.

### 1.3. Non-Georgian languages
For `lang ∈ {en, fr, es, ru}` the dictionary is a plain ETS set with no morphology and no confusion table. Membership is exact. False-positive rates on inflected forms are expected to be higher than for Georgian, where morphology is modelled explicitly.

---

## 2. Morphology

Georgian is an agglutinative language. A naive surface-form dictionary cannot cover the inflectional space without combinatorial blow-up, so validity is extended via a longest-suffix-match stripping procedure.

### 2.1. Procedure

```
morph_valid(w):
  for each (suffix, restores) in @suffixes sorted by |suffix| desc:
    if w ends with suffix:
      stem = w without suffix
      for end in restores ∪ @restore_endings ∪ { strip_preverb(stem, preverb) | preverb ∈ @preverbs }:
        if (stem + end) ∈ D:   return true
  return false
```

where:
- `@suffixes` = 134 `(suffix, restores)` tuples covering the 7 Georgian noun cases (nominative, ergative, dative, genitive, instrumental, adverbial, vocative) plus plural and possessive forms.
- `@preverbs` = 13 verbal preverbs: `გამო, გადმო, შემო, ჩამო, გადა, მომ, შე, გა, და, მი, მო, ამ, ჩა`.
- `@restore_endings` = `["", "ი", "ა", "ე"]` (global fallback endings tried after any suffix strip).

### 2.2. Correctness property (empirical, not proven)
For a word `w` that is a morphologically valid inflection of some base stem `s ∈ D`, the procedure returns `true`. Adversarial counter-examples exist (e.g. rare suppletive forms, reduplication patterns, some verb aspect chains) — these manifest as false positives (flagged as misspelled).

---

## 3. Edit-Distance Metric

### 3.1. Definition
`levenshtein_gs/2` is the standard Levenshtein distance with uniform cost 1.0 for insert, delete, and substitute. Transposition is not a primitive operation (treated as two substitutions).

```
lev(ε, s) = |s|
lev(s, ε) = |s|
lev(a·x, b·y) = if a == b: lev(x, y)
                else: 1 + min(lev(a·x, y),   // insert
                              lev(x, b·y),   // delete
                              lev(x, y))     // substitute
```

### 3.2. Threshold
A candidate `c` qualifies as a suggestion for misspelled `w` iff `lev(w, c) ≤ 4`. Candidates are drawn from the index bucket `{first_char(w), |w| ± 3}` and from buckets for every `c'` that is a confusable variant of `first_char(w)` (radius 2).

### 3.3. Confusion table
Phonetically/visually similar Georgian letter pairs expand the **candidate pool** (not the edit cost). The table is implemented as 25 clauses in `Dictionary.georgian_confusions/1` covering 24 letters: `ს შ ც ჩ ძ ზ ჯ კ ქ გ ყ პ ბ ფ ტ დ თ ხ ღ ვ ნ მ რ ლ`. Representative pairs: ს↔შ, კ↔ქ, ბ↔პ, გ↔ყ, დ↔ტ, ვ↔ბ, რ↔ლ, ნ↔მ, ძ↔ზ, ჯ↔ზ, ხ↔{ყ,ღ,ქ}, ღ↔{გ,ყ,ხ}, ფ↔{პ,ვ}.

> Design note. Earlier design docs described a 0.5-cost weighting for confusion substitutions inside the Levenshtein recurrence. That weighting is **not implemented**; see OPEN_PROBLEMS.md. The current implementation uses confusion pairs only for candidate-pool expansion.

---

## 4. Suggestion Ranking

For a flagged token `w`, candidate suggestions are produced by:

1. `fetch_candidates(first_char(w), |w|, radius=3)` → words whose `{first_char, length}` falls in the bucket range.
2. For each confusable `c'` of `first_char(w)`, add `fetch_candidates(c', |w|, radius=2)`.
3. Compute `lev(w, c)` for each candidate `c`; reject if `> 4`.
4. Sort ascending by distance; ties broken by insertion order (the dictionary file's natural order).
5. Truncate to top `max_suggestions = 5`.

> Ranking note. Word-frequency weighting is described in aspirational design but **not implemented**. See OPEN_PROBLEMS.md.

---

## 5. Typography and Stopword Checks (non-spelling)

Applied after the spell-check pass, on the original text:

- **Typography:** double spaces, ASCII quotes where Georgian "„..."" is expected, hyphen-vs-dash misuse, Georgian semicolon conventions.
- **Stopwords:** detection of 25 Georgian filler words (style flags, not errors).

These are heuristic rule-based passes with no formal recall/precision target; they surface candidates for the author's review, not hard errors.

---

## 6. Falsifiable Accuracy Predictions

These predictions define what "the system works" means operationally. They are the primary levers for regression detection.

| Prediction | Target | Measured by |
|------------|--------|-------------|
| P1. Exact-match recall on in-lexicon words | ≥ 99.9% | ETS membership test on `D`; a miss implies data corruption |
| P2. Morphological recall on inflected forms of in-lexicon stems | ≥ 95% | Curated regression set of 1,000 inflected forms across the 7 noun cases + verbal paradigms |
| P3. Suggestion recall@5 for single-edit typos (1 insertion, deletion, or substitution) | ≥ 90% | Synthetic corpus: apply 1 random edit to each of 5,000 dictionary words; correct form must appear in the top-5 suggestions |
| P4. Suggestion recall@5 for confusion-pair typos (ს↔შ, კ↔ქ, etc.) | ≥ 85% | Synthetic corpus targeting the 25 confusion clauses |
| P5. Suggestion recall@5 for double-edit typos | ≥ 70% | Synthetic corpus with 2 random edits |
| P6. False-positive rate on valid Georgian prose | ≤ 5% | Curated paragraph set (news, academic, literary) measured by flagged-words ÷ total-words |
| P7. API p95 latency for typical text (≤ 1 KB) | ≤ 500 ms | Load test against production; see PARAMETERS.md §7 |
| P8. Dictionary load time on boot | ≤ 500 ms | Phoenix startup log |

A measured deviation on any of P1–P6 by more than 5 absolute percentage points is a regression and warrants rollback of the triggering change. P7–P8 are performance contracts; breaches trigger a performance investigation.

---

## 7. Pointer to Canon

Linguistic authority: **Hunspell `ka_GE`** (LGPL/GPL, distributed with LibreOffice/OpenOffice). `priv/static/dictionaris/ge.txt` is a 142,285-word excerpt.

Additional corpora considered for future ingestion: **Georgian National Corpus** (CC BY-SA 4.0), **Tbilisi State University (TSU) Georgian NLP group** outputs.

For the full reference list, URLs, and licensing notes, see **EVIDENCE.md**.

```
### `PARAMETERS.md` (head 200 lines)
```
# PARAMETERS.md — SpellCheckerKa Numerical Reference

All numeric values in effect, grouped by subsystem. For the reasons behind these numbers see **DESIGN.md** / **THEORY.md**; for active changes see **STATE.md**.

---

## 1. Dictionary & morphology

| Parameter | Value | Location | Notes |
|---|---|---|---|
| `georgian_dict_size` | **142,285 base words** (+ user additions) | `priv/static/dictionaris/ge.txt` | Hunspell `ka_GE` origin. The 993k digest variant `ge-<hash>.txt` is present but NOT LOADED — rejected 2026-04-21 for noise / typos. |
| `user_dict_file` | `user_words.txt` | `priv/static/dictionaris/` | Auto-created; persists across restarts; co-located in the same ETS table as the base dictionary. |
| `max_levenshtein_edit_distance` | 4 | `Dictionary.@suggestion_threshold` | Max edits for suggestion candidates. |
| `max_suggestions` | 5 | `Dictionary.suggestions/2` default | Top N returned per error. |
| `morphology_suffix_rules` | **134** | `Morphology.@suffixes` | Longest-match order. |
| `verb_preverb_count` | **13** | `Morphology.@preverbs` | `გამო, გადმო, შემო, ჩამო, გადა, მომ, შე, გა, და, მი, მო, ამ, ჩა` |
| `stem_try_endings` | `["", "ი", "ა", "ე"]` | `Morphology.@restore_endings` | Appended after suffix strip (plus per-rule restores). |
| `ets_table_name_georgian` | `:ge_dictionary` | `Dictionary` | ETS set, public, `read_concurrency: true`. |
| `ets_table_name_index` | `:ge_dictionary_index` | `Dictionary` | `{first_char, length}` → `[words]` buckets. |
| `candidate_bucket_radius` | 3 (first-char match); 2 (confusion-pair expansion) | `Dictionary` | Word-length tolerance when fetching candidates. |

### 1.1. Georgian letter confusion table (as implemented in code)

25 clauses in `Dictionary.georgian_confusions/1` covering **24 letters**: `ს შ ც ჩ ძ ზ ჯ კ ქ გ ყ პ ბ ფ ტ დ თ ხ ღ ვ ნ მ რ ლ`.

Representative pairs (phonetic grouping):

| Group | Pairs |
|---|---|
| Sibilants / palatals | ს↔შ · ც↔ჩ · ძ↔ჯ · ძ↔ზ · ჯ↔ზ |
| Velars | კ↔ქ · გ↔ყ · ხ↔ქ · ხ↔ყ · ხ↔ღ · ღ↔გ · ღ↔ყ · ღ↔ხ |
| Dentals | დ↔ტ · თ↔დ · თ↔ტ |
| Labials | ბ↔პ · ვ↔ბ · ფ↔პ · ფ↔ვ |
| Liquids / nasals | რ↔ლ · ნ↔მ |

> Purpose: **candidate-pool expansion only.** The confusion table is *not* used to weight Levenshtein cost (cost stays uniform at 1.0). See THEORY.md §3 and OPEN_PROBLEMS.md §1.4.

---

## 2. Rate limiting & quotas

| Parameter | Value | Location | Notes |
|---|---|---|---|
| `rate_limit_requests_per_minute` | 30 | `RateLimiter` plug | Free tier, per IP. |
| `rate_limit_window_ms` | 60,000 ms | `RateLimiter` | Rolling 1-minute window. |
| `rate_limit_authenticated` | 60 req/min | (planned) | Authenticated tier. |
| `rate_limit_premium` | 300 req/min | (planned) | Premium tier. |
| `rate_limit_corporate` | 1,000+ req/min | (planned) | Corporate tier. |
| `daily_free_quota_seconds` | 1,800 s (30 min/day) | `UsageTracker` | Per IP, resets at midnight. |
| `rate_limiter_cleanup_interval_ms` | 300,000 ms (5 min) | `RateLimiterCleaner` | GenServer sweep. |
| `admin_bypass_header` | `X-Admin-Token` | `UsageTracker` | Bypasses daily quota; for AIM / kSystem / internal tools. |

---

## 3. Text-processing limits

| Parameter | Value | Location | Notes |
|---|---|---|---|
| `max_text_bytes` | 2,000,000 B (2 MB) | `SpellController` | Per request. |
| `max_word_count` | 100,000 words | `SpellController` | Per request. |
| `chunk_threshold_bytes` | 3,072 B (3 KB) | `assets/js/app.js` | Above this → paragraph chunking. |
| `chunk_split_strategy` | paragraph (newline) | `app.js` | One paragraph = one chunk. |
| `debounce_min_ms` | 500 ms | `app.js` | Minimum debounce. |
| `debounce_max_ms` | 2,500 ms | `app.js` | Maximum debounce (large texts). |
| `debounce_scale_threshold_words` | 500 words | `app.js` | Debounce scales above this size. |
| `error_pagination_size` | 50 | `app.js` | Errors per UI page. |

---

## 4. Browser extension

| Parameter | Value | Location | Notes |
|---|---|---|---|
| `manifest_version` | 3 | `extension/manifest.json` | Chrome Manifest V3. |
| `keyboard_shortcut` | `Ctrl+Shift+K` (`Cmd+Shift+K` on macOS) | `manifest.json` | Triggers spell check. |
| `host_permissions` | `<all_urls>` | `manifest.json` | All sites. |
| `gdocs_content_script` | `content_gdocs.js` | `manifest.json` | Scoped to `docs.google.com/*`. |
| `generic_content_script` | `content_generic.js` | `manifest.json` | All other URLs. |
| `background_worker` | `background.js` | `manifest.json` | Service Worker. |
| `extension_cache_ttl_ms` | 30,000 ms (30 s) | `background.js` | In-memory response cache. |
| `extension_debounce_ms` | 800 ms | `background.js` | Request debounce. |

---

## 5. HTTP error codes

| Code | Meaning |
|---|---|
| 200 | Success |
| 400 | Bad request (missing `text`/`lang`, text > 2 MB or > 100,000 words) |
| 402 | Daily free quota exhausted — Premium upgrade required |
| 429 | Rate limit exceeded (`Retry-After` header included) |

Full response shape: **DESIGN.md §3**.

---

## 6. Monetisation tiers (planned)

| Tier | Price (USD) | Price (₾) | Rate Limit | Daily Quota | Notes |
|---|---|---|---|---|---|
| Free | $0 | 0 | 30 req/min | 30 min/day | today |
| Premium (personal, monthly) | $4.99 | 14.99 | 60 req/min | unlimited | |
| Premium (personal, annual) | $49.99 | 149.99 | 60 req/min | unlimited | 17% discount |
| Premium (student, monthly) | $2.99 | 8.99 | 60 req/min | unlimited | student ID required |
| Corporate (≤ 10 users) | $99 | 299 | 300 req/min | unlimited | B2B |
| API pay-as-you-go | $0.01 / 1,000 requests | — | per API key | — | developer tier |

Larger-scale text limits for Premium and Corporate: **50,000 chars / request** (Premium) and **unlimited** (Corporate). User-dictionary caps: 5,000 words (Premium), unlimited (Corporate).

Not yet implemented in code (see STATE.md P1: Stripe / Paddle integration).

---

## 7. Deployment parameters

| Parameter | Value | Notes |
|---|---|---|
| `production_host` | `spellcheckerka.drjaba.com` | legacy redirect: `schecker.ge` |
| `internal_port` | 4000 | Phoenix inside container |
| `external_port` | 4001 | Bound to `127.0.0.1`, behind nginx |
| `health_check_interval_s` | 30 s | `docker-compose.yml` |
| `docker_image_name` | `spellcheckerka:latest` | built by `deploy.sh` |
| `deploy_path` | `/opt/spellcheckerka/` | server-side |
| `database` | **None** | stateless; ETS + `user_words.txt` only |

---

## 8. Performance targets (contracts)

Duplicated from DESIGN.md §7 for ops reference; DESIGN.md is the source of truth.

| Metric | Target |
|---|---|
| Dictionary load on boot | ≤ 500 ms |
| Per-token ETS membership | < 1 ms |
| Levenshtein suggestion generation | < 50 ms |
| API p95 latency, text ≤ 1 KB | ≤ 500 ms |
| API p99 latency, text ≤ 1 KB | ≤ 1 s |
| Throughput per instance | ≥ 500 req/s |
| Memory footprint (ETS + BEAM) | ≤ 500 MB |
| Uptime | ≥ 99.9% |
| Web LCP | ≤ 2.5 s |

---

## 9. Languages

| Code | Language | Dictionary size | Morphology | Confusion table |
|---|---|---|---|---|
| `ka` | Georgian | **142,285** base words (+ user additions) | Full (134 rules) | ✓ 25 clauses / 24 letters |
| `en` | English | ~76,000 lines | Plain Levenshtein | — |
| `fr` | French | ~79,000 lines | Plain Levenshtein | — |
| `es` | Spanish | ~65,000 lines | Plain Levenshtein | — |
| `ru` | Russian | ~146,000 lines | Plain Levenshtein | — |

UI languages (frontend i18n): Georgian · English · French · Spanish · Russian · Arabic · Chinese (+ 1) — **8 total**.

---

## 10. Environment variables

| Variable | Used by | Notes |
|---|---|---|
| `SECRET_KEY_BASE` | Phoenix | Required for production. |
| `PHX_HOST` | Phoenix | Production hostname (e.g. `spellcheckerka.drjaba.com`). |
| `PORT` | Phoenix / Bandit | Defaults to 4000. |
| `MIX_ENV` | Mix | `dev` / `test` / `prod`. |
| `DEEPSEEK_API_KEY` | tooling | From `~/.aim_env`. Non-code tasks route through DeepSeek per CLAUDE.md. |
| `S3_*` | backup tooling | See `/home/jaba/CLAUDE.md` for the Hetzner S3 wrapper (out of this repo's scope). |

---

## 11. What is NOT configured

Things sometimes assumed but not actually parameterised in v1:

- **Word-frequency weighting** — no frequency table exists; suggestion ranking is distance-only. (See OPEN_PROBLEMS.md §1.3.)
- **Confusion-pair cost weighting** — Levenshtein cost is uniform 1.0; the 0.5-cost "phonetically similar" weighting described in older design notes is **not implemented**. (See THEORY.md §3.3 and OPEN_PROBLEMS.md §1.4.)
- **Word-level frequency cache** — no LRU, no hot-word shortcut.
- **Redis layer** — not used today. A latent option for horizontal scaling beyond 50k MAU. (See OPEN_PROBLEMS.md §4.5.)
- **WAF / Cloudflare rules** — not deployed. (See OPEN_PROBLEMS.md §4.7.)
- **OAuth / email auth** — no providers configured; `mailer.ex` is a Swoosh stub.
- **Payment gateway** — Stripe / Paddle keys not configured; monetisation tiers above are design, not runtime.
- **API keys** — no API-key management; all rate limiting is IP-scoped.
- **Feature flags** — none.
- **Analytics / telemetry export** — Phoenix telemetry in place, but no external sink configured.

```
### `STATE.md` (head 200 lines)
```
# STATE.md — SpellCheckerKa

**Volatile state. Replaces TODO.md + MEMORY.md + UPGRADE.md.**
Single place for current status, active work, milestones, decisions, and constraints. Read on startup.

---

## Current Status

- **Phase 1 (Web MVP):** complete
- **Phase 3 (Browser Extension, Manifest V3):** complete
- **Phase 2 (Mobile, React Native):** pending
- **Phase 4 (Monetization / Auth):** partial (rate limiting + quota in place; Stripe/Paddle not integrated)
- **Build:** `mix compile` OK, `mix test` passing
- **Deploy:** production domain `spellcheckerka.drjaba.com` (legacy redirect: `schecker.ge`)
- **Freeze:** project frozen until 2026-09-27 (OJS Scholar submission date of Longevity Horizon). Only critical bug fixes and admin tooling permitted during freeze. Phase 2 resumes after unfreeze.

---

## Active TODOs (P0–P5)

Priority legend: **P0** = ship blocker · **P1** = critical path · **P2** = high value · **P3** = medium · **P4** = low / research · **P5** = speculative.

| P | Task | Notes |
|---|------|-------|
| P0 | Deploy to `spellcheckerka.drjaba.com` (production launch) | run `bash deploy.sh` |
| P1 | Chrome Web Store submission (extension) | MV3 assets ready in `extension/` |
| P1 | CONCEPT.md §5.1: split 9-endpoint table into "Implemented" + "Planned" | only 3 endpoints live; see OPEN_PROBLEMS.md |
| P1 | Premium subscription integration (Stripe or Paddle) | webhook → grant token → tier-based rate limit |
| P2 | User authentication (email + OAuth) | prerequisite for account-based quota |
| P2 | Fix PARAMETERS.md Part 1 confusion table | documented 9 pairs ≠ 25 clauses / 24 letters in code; remove non-existent `ზ↔ჟ` row |
| P2 | README.md Georgian architecture tree: add `lang_dictionary.ex` + `usage_tracker.ex` | current list incomplete |
| P3 | Phase 2 scaffold: React Native project (post-freeze, after 2026-09-27) | iOS + Android |
| P3 | iOS keyboard extension | native module |
| P3 | Android keyboard extension | native module |
| P3 | Camera OCR (Tesseract.js + Georgian train data) | mobile only |
| P3 | Offline sync between devices | CRDT + WatermelonDB |
| P3 | Corporate tier: 300 req/min, API key management | B2B |
| P4 | Evaluate `priv/static/dictionaris/ge-<hash>.txt` (993k variant) | REJECTED 2026-04-21 (noisy); revisit only after cleaning pass |
| P4 | `mailer.ex` — either wire to registration flow or remove stub | currently 3-line Swoosh stub |
| P4 | Expand test coverage beyond current files | |
| P4 | ML spell correction (ByT5 / char seq2seq) | proposed 2026-03-29, not approved |
| P4 | Grammar checker layer (morphosyntactic) | proposed 2026-03-29, not approved |
| P4 | Google Docs Add-on (Apps Script, separate from MV3 content script) | proposed 2026-03-29, not approved |
| P5 | Analytics endpoints (aggregated usage stats) | |
| P5 | Auto-correct (false-positive analysis prerequisite) | |

---

## Milestones (completed)

### Phase 1 — Web MVP
- Georgian dictionary: 142,285 base words loaded into ETS
- Levenshtein algorithm + Georgian confusion table (25 clauses / 24 letters)
- Morphological analysis: 134 suffix rules, 7 noun cases, 13 verb preverbs
- Mirror overlay (textarea + invisible div, cursor-safe)
- Real-time spell check with adaptive debounce (500 ms – 2.5 s)
- `.docx` import/export (fflate ZIP + DOMParser XML, client-side)
- User dictionary (ETS + `user_words.txt` persistence)
- Rate limiting (30 req/min per IP, ETS rolling window)
- Daily quota (30 min/day free per IP, HTTP 402 on exhaustion)
- Typography checker (double spaces, quotes, hyphens, semicolons)
- Stopword detection (25 Georgian filler words)
- Multi-language backend (en/fr/es/ru dictionaries populated: 76k / 79k / 65k / 146k lines)
- UI i18n: 8 languages (Georgian + English, French, Spanish, Russian, Arabic, Chinese)
- Chunk processing (texts > 3 KB → paragraph chunks)
- Docker 2-stage build (Elixir 1.16 + Debian bookworm-slim)
- `deploy.sh` → production

### Phase 3 — Browser Extension (Manifest V3)
- `content_generic.js` (all websites, MutationObserver)
- `content_gdocs.js` (Google Docs)
- `background.js` (Service Worker, API client, 30s TTL cache, debounce)
- `popup.html` + `popup.js`
- Keyboard shortcut: Ctrl+Shift+K (Cmd+Shift+K on macOS)

### Documentation
- 2026-03-28 — CLAUDE.md, PARAMETERS.md, MAP.md (now archived), `run.sh` local launcher, `deploy.sh` updated for `docker/` subfolder
- 2026-04-21 — CONCEPT ↔ CODE audit (see `CONCEPT_CODE_AUDIT_2026-04-21.md`); dictionary claim reconciled 993k → 142k
- 2026-04-25 — migrated to 9-file core schema

---

## Decision Log (newest first)

### 2026-04-25 — migrated to 9-file core schema
Replaced legacy doc set (TODO + MEMORY + UPGRADE + LINKS + KNOWLEDGE + MAP) with the canonical 9-file core (CONCEPT, README, CLAUDE, THEORY, DESIGN, EVIDENCE, PARAMETERS, STATE, OPEN_PROBLEMS). Legacy files preserved in `_archive/core_pre_9file_2026-04-25/`.

### 2026-04-21 — Dictionary size canonicalised to 142,285 words
The 993,589-word digest variant (`ge-<hash>.txt`) was audited and REJECTED: 99.9% Georgian script but 582 concatenated-phrase artefacts plus sample typos (e.g. `პეიოდში` missing `ე`). All public-facing claims downgraded to 142,285. File retained for historical reference; do not load.

### 2026-03-26 — Rename: ScheckerGe → SpellCheckerKa
"Ka" = ISO 639-1 code for Georgian; internationally self-explanatory. Domain: `schecker.ge` → `spellcheckerka.drjaba.com`. Repos: `djabbat/SpellCheckerKa` (private) + `djabbat/SpellCheckerKa-public`.

### Storage: ETS over database
All dictionary data lives in ETS (Erlang Term Storage). O(1) lookup, in-process, zero serialisation overhead. Restart reloads from `ge.txt` + `user_words.txt`. Stateless architecture. **Immutable decision — do not move to PostgreSQL or any SQL store.**

### Freemium: 30 min/day free, HTTP 402 on exhaustion
`UsageTracker` GenServer tracks per-IP elapsed active-checking time in ETS; resets at midnight. `X-Admin-Token` header bypasses the daily quota (for AIM, kSystem, internal tools).

### Rate limiting: 30 req/min per IP, HTTP 429
Separate from the daily quota. Burst protection via `RateLimiter` Plug + ETS rolling window; `RateLimiterCleaner` GenServer sweeps every 5 min.

### Mirror overlay, not contenteditable
Highlighting via invisible mirror `<div>` over `<textarea>` — cursor-safe, IME-safe, native undo/redo. **Do not switch to contenteditable.**

### Client-side `.docx` processing
Import/export handled entirely in-browser via `fflate` (ZIP) + `DOMParser` (XML). No server-side file storage; privacy-preserving; fast.

### Browser extension: Manifest V3
MV2 deprecated by Google. Background script is a Service Worker (not a persistent page); `chrome.storage` replaces `localStorage`; stricter CSP; no remote code execution.

### Morphology: suffix-first, longest-match
134 suffix rules applied longest-first; Levenshtein fallback on resulting stem. Georgian's agglutinative nature makes pure dictionary lookup insufficient — morphological analysis is mandatory.

### Freeze until 2026-09-27
No new phases during freeze period. Reason: OJS journal (Longevity Horizon / Annals of Rejuvenation Science) must reach Google Scholar index before scaling the academic user base. Resume Phase 2 (Mobile) afterwards.

---

## What NOT to do

- **Do not switch from ETS to a SQL database.** ETS is the architectural core. O(1) in-process lookup is part of the performance budget.
- **Do not switch from the mirror-overlay editor to `contenteditable`.** Cursor/IME/undo behaviour breaks.
- **Do not load `ge-<hash>.txt` (993k words) at runtime.** Rejected 2026-04-21 — contains concatenated-phrase artefacts and typos. Canonical dictionary is `ge.txt` (142,285 words).
- **Do not remove `fflate`.** `.docx` import/export depends on it; no server-side file handling is permitted.
- **Do not downgrade Manifest V3 in the extension.** Chrome Web Store rejects MV2.
- **Do not bypass `RateLimiterCleaner`.** Without periodic sweeps, the rate-limit ETS grows unbounded on long-running servers.
- **Do not add grammar or auto-correct features while the project is frozen.** Allowed: critical bug fixes + admin tooling. Freeze ends 2026-09-27.
- **Do not commit CLAUDE.md / STATE.md / PARAMETERS.md to the public repo.** They belong to `djabbat/SpellCheckerKa` (private) only. Public repo is `djabbat/SpellCheckerKa-public`, which carries README.md + CONCEPT.md + THEORY.md + DESIGN.md + EVIDENCE.md + OPEN_PROBLEMS.md.
- **Do not commit `user_words.txt`.** Contains user-submitted content; treat as runtime state.

---

## Startup Checklist (for Claude on session start)

1. Read **STATE.md** (this file) first for current status, P0–P5 queue, and active constraints.
2. Consult **CONCEPT.md** for canonical scope/vision — it is the single source of truth for "what this project is".
3. For technical details: **DESIGN.md** (architecture, file tree, API), **THEORY.md** (algorithms, linguistic rules), **PARAMETERS.md** (numerical values).
4. For external references: **EVIDENCE.md** (linguistic sources, competitors, ecosystem links).
5. For known gaps: **OPEN_PROBLEMS.md**.
6. Respect the freeze (until 2026-09-27): critical bug fixes and admin tooling only.
7. Before changing any "What NOT to do" item above, surface the trade-off explicitly and get user approval.
8. Route non-code tasks through DeepSeek API (`~/.aim_env → DEEPSEEK_API_KEY`).

```
### `DESIGN.md` (head 200 lines)
```
# DESIGN.md — SpellCheckerKa Architecture

Runtime architecture, file tree, workflow, API contracts, deployment plan, and performance targets.

For the formal spell-checking model, see **THEORY.md**.
For numerical values, see **PARAMETERS.md**.
For vision, scope, and ecosystem links, see **CONCEPT.md**.

---

## 1. Repository Tree

```
spellcheckerka/
├── CONCEPT.md                              scope + vision (canonical)
├── README.md                               public-safe quickstart
├── CLAUDE.md                               operating rules for Claude
├── THEORY.md                               formal model
├── DESIGN.md                               this file
├── EVIDENCE.md                             external refs + ecosystem links
├── PARAMETERS.md                           numerical values
├── STATE.md                                volatile: TODOs, milestones, decisions
├── OPEN_PROBLEMS.md                        validation + implementation gaps
├── CONCEPT_CODE_AUDIT_2026-04-21.md        audit artifact (historical)
│
├── _archive/
│   └── core_pre_9file_2026-04-25/          legacy docs preserved
│
├── lib/
│   ├── spellcheckerka/
│   │   ├── application.ex                  OTP supervisor tree
│   │   ├── dictionary.ex                   ETS Georgian dict + Levenshtein + confusion-table candidate expansion + user-word persistence
│   │   ├── morphology.ex                   134 suffix rules, 13 preverbs, stem restoration
│   │   ├── lang_dictionary.ex              ETS dicts for en/fr/es/ru (plain Levenshtein)
│   │   ├── usage_tracker.ex                GenServer: 30 min/day free quota per IP
│   │   ├── rate_limiter_cleaner.ex         GenServer: sweeps rate-limit ETS every 5 min
│   │   └── mailer.ex                       Swoosh stub (3 lines, not wired — see STATE.md)
│   └── spellcheckerka_web/
│       ├── router.ex                       routes
│       ├── endpoint.ex                     Bandit HTTP endpoint
│       ├── gettext.ex                      i18n
│       ├── telemetry.ex
│       ├── controllers/
│       │   ├── page_controller.ex          GET /
│       │   └── spell_controller.ex         POST /api/check, /api/dictionary/add, /api/dictionary/remove
│       ├── plugs/
│       │   └── rate_limiter.ex             30 req/min per IP; emits 402/429 as appropriate
│       └── components/layouts/
│           ├── root.html.heex
│           └── app.html.heex
│
├── assets/
│   ├── js/app.js                           mirror overlay, debounce, .docx I/O, 8-language UI, chunk splitting
│   ├── css/app.css                         Tailwind + Georgian red accent
│   └── package.json                        fflate for .docx
│
├── priv/static/dictionaris/
│   ├── ge.txt                              142,285 Georgian words (canonical)
│   ├── ge-<hash>.txt                       993k digest variant — REJECTED 2026-04-21, not loaded
│   ├── user_words.txt                      auto-created; user additions
│   ├── en.txt                              ~76k lines
│   ├── fr.txt                              ~79k lines
│   ├── es.txt                              ~65k lines
│   └── ru.txt                              ~146k lines
│
├── extension/
│   ├── manifest.json                       Manifest V3
│   ├── background.js                       Service Worker: API client, 30 s TTL cache, debounce
│   ├── content_generic.js                  all websites: MutationObserver, DOM injection
│   ├── content_gdocs.js                    Google Docs DOM quirks
│   ├── popup.html
│   ├── popup.js
│   ├── panel.css
│   └── icons/
│
├── test/
│   ├── spellcheckerka/
│   │   ├── dictionary_test.exs
│   │   └── spell_core_test.exs
│   └── spellcheckerka_web/controllers/
│
├── docker/
│   ├── Dockerfile                          2-stage build (Elixir 1.16 builder → bookworm-slim runtime)
│   ├── docker-compose.yml                  127.0.0.1:4001 → 4000
│   └── nginx.conf                          reverse proxy
│
├── docs/                                   (auxiliary)
├── config/                                 (Phoenix configs)
├── mix.exs
├── mix.lock
├── deploy.sh                               build → scp → docker compose on production
├── run.sh                                  local dev launcher (dev | test | docker-build)
├── .formatter.exs
└── .gitignore
```

---

## 2. Workflow: corpus → dictionary → suggestions

```
Hunspell ka_GE corpus          user_words.txt (runtime)
       │                                │
       ▼                                ▼
ge.txt (142,285 lines)         append-only user additions
       │                                │
       └──────────┬─────────────────────┘
                  ▼
         Dictionary.start_link/0
                  │
                  ├─ ETS :ge_dictionary          exact-match set, read_concurrency: true
                  └─ ETS :ge_dictionary_index    {first_char, length} → [words] buckets
                  │
                  ▼
         SpellController.check/2
                  │
                  ├─ tokenize(text)      whitespace + Georgian punctuation
                  ├─ for each token w:
                  │    ├─ ETS.member?(w) → valid
                  │    └─ Morphology.valid?(w) → valid / else flag
                  ├─ for each flagged w:
                  │    ├─ fetch_candidates(first_char(w), |w|, radius=3)
                  │    ├─ ∪ fetch_candidates(confusable_first_char(w), |w|, radius=2)
                  │    ├─ compute Levenshtein; filter ≤ 4; sort asc
                  │    └─ take top 5
                  ├─ typography pass (double spaces, quotes, dashes)
                  ├─ stopword pass (25 filler words)
                  └─ JSON response
```

---

## 3. API Contracts

### 3.1. Implemented (v1.0)

| Method  | Path                       | Request body                                       | Success response                                               | Error codes    |
|---------|----------------------------|----------------------------------------------------|----------------------------------------------------------------|----------------|
| POST    | `/api/check`               | `{text, lang}` or `{chunks: [...], lang}`           | `{total_words, error_count, accuracy, errors[], typography[], stopwords[]}` | 400, 402, 429 |
| POST    | `/api/dictionary/add`      | `{word}`                                           | `{ok: true, word}`                                             | 400, 429       |
| POST    | `/api/dictionary/remove`   | `{word}`                                           | `{ok: true, word}`                                             | 400, 429       |
| OPTIONS | `/api/check`, `/api/dictionary/add` | — (CORS preflight)                         | CORS headers                                                   | —              |
| GET     | `/`                        | —                                                  | landing page                                                   | —              |
| GET     | `/upgrade`                 | —                                                  | premium upgrade page                                           | —              |

### 3.2. Error responses

- **400** — missing `text`/`lang`; text exceeds 2 MB or 100,000 words.
- **402 Payment Required** — daily 30 min free quota exhausted for this IP.
- **429 Too Many Requests** — burst rate limit exceeded (30 req/min per IP). `Retry-After` header included.

### 3.3. `POST /api/check` — response shape

```json
{
  "total_words": 120,
  "error_count": 3,
  "accuracy": 97.5,
  "errors": [
    {
      "word": "შეიყვანეთ",
      "count": 1,
      "suggestions": ["შეიყვანე", "შეიყვანება"],
      "base_form": "შეიყვან"
    }
  ],
  "typography": [
    { "type": "double_space", "position": { "start": 5, "end": 6 }, "suggestion": " " }
  ],
  "stopwords": [
    { "word": "რომ", "position": { "start": 0, "end": 3 } }
  ]
}
```

### 3.4. Batch via chunks
For texts above the 3 KB threshold, the frontend splits on paragraphs and posts `{chunks: [p1, p2, …], lang}`. The server processes chunks sequentially and returns a single merged result.

### 3.5. Planned (not implemented)
`POST /api/check/batch`, `GET /api/dictionary/list`, `POST /api/sync`, `GET /api/stats`, `POST /api/auth/register`, `POST /api/auth/login`.

### 3.6. Admin bypass
`X-Admin-Token: <token>` on `/api/check` bypasses the daily quota (`UsageTracker`). Used by AIM and kSystem for Georgian text QA on internal content.

---

## 4. Request Data Flow (Web)

```
BROWSER
   │  user types in textarea
   ▼
[app.js]
   │  debounce 500 ms–2.5 s (scales with word count; threshold 500 words)
   │  if |text| > 3 KB → split on paragraphs → chunks[]
   ▼
POST /api/check  {text | chunks, lang}
   │
PHOENIX (Bandit, port 4000 internal / 4001 external)
   │

```
### `EVIDENCE.md` (head 200 lines)
```
# EVIDENCE.md — SpellCheckerKa External References

External sources: linguistic references, Georgian-language resources, competitor landscape, related ecosystem projects.

For internal architecture, see **DESIGN.md**.
For the formal model and linguistic rules, see **THEORY.md**.

---

## 1. Linguistic & Lexical Sources

### 1.1. Dictionary base
- **Hunspell `ka_GE`** — LGPL / GPL. Source of the canonical 142,285-word lexicon (`priv/static/dictionaris/ge.txt`). Distributed with LibreOffice and OpenOffice.
- **Georgian National Corpus** — CC BY-SA 4.0. Candidate for future lexical enrichment (specialised/technical vocabulary).
- **Tbilisi State University (TSU) Georgian NLP group** — academic collaborators; source for a future Georgian Morphological Analyzer integration.
- **Community hand-additions** — MIT (user-contributed entries, tracked via `user_words.txt`).

### 1.2. Script
- **Mkhedruli (მხედრული)** — modern Georgian script, 11th century to present; 33 letters, unicameral, left-to-right, Unicode range U+10D0–U+10FF.
- Other historical scripts (Asomtavruli, Nuskhuri) are ecclesiastical and not supported.
- Near-perfect phoneme–grapheme correspondence with exceptions in aspirated vs. ejective pairs — this property motivates the confusion-pair approach in THEORY §3.3.

### 1.3. Morphology
- Georgian is polysynthetic + agglutinative.
- **7 noun cases:** nominative (-ი), ergative (-მა), dative (-ს), genitive (-ის), instrumental (-ით), adverbial (-ად), vocative (-ო).
- **Verb preverbs (implemented: 13):** გამო, გადმო, შემო, ჩამო, გადა, მომ, შე, გა, და, მი, მო, ამ, ჩა.
- Implication: raw word-list membership alone has unacceptably high false-positive rates → morphological stripping (longest-suffix match) is mandatory. See THEORY §2.

---

## 2. Competitor Landscape

| Competitor | Strengths | Weaknesses | SpellCheckerKa positioning |
|---|---|---|---|
| **Microsoft Word** | Bundled with Office; mature editor | Paid, heavy, no API, weak Georgian lexicon | Free, lightweight, open API, 142k + morphology |
| **Grammarly** | Excellent UX; browser extension | **No Georgian support** | Full Georgian support |
| **LibreOffice / OpenOffice** | Free, open source | Uses Hunspell `ka_GE` with no morphology layer on top; no API | Same base + morphology + API |
| **Google Docs** | Free, ubiquitous | Basic spell check only; no Georgian morphology | Extension + planned Google Docs Add-on |
| **LanguageTool** | Grammar rule engine | No Georgian language pack | Specialised for Georgian |
| **Yandex.Speller** | Solid API | No Georgian | Georgian first |
| **Reverso** | Context-aware corrections | No Georgian | Full ecosystem (Web + Mobile + Extension) |

**Key differentiator:** SpellCheckerKa is the only product combining (a) full Georgian support (142k base words + morphology + confusion table), (b) multi-platform distribution (Web + Mobile + Extension), (c) open REST API, (d) a free tier.

---

## 3. Related Work & Aspirational Sources

- **Character-level seq2seq / ByT5** for context-sensitive spell correction — research direction proposed 2026-03-29; not scheduled.
- **Morphosyntactic rule engine (LanguageTool-style)** — candidate framework for the future grammar-checker layer.
- **GeoBERT** — Georgian BERT variant, future option for context-aware suggestion ranking.
- **DAWG / GADDAG** — compact data structures for offline mobile dictionary (target: sub-10 MB with morphological expansion). Relevant for Phase 2.

---

## 4. URLs

### 4.1. Production
- Web app: **https://spellcheckerka.drjaba.com**
- Legacy domain (redirects): **https://schecker.ge**

### 4.2. Source repositories
- Private (full): **https://github.com/djabbat/SpellCheckerKa** — includes `CLAUDE.md`, `STATE.md`, `PARAMETERS.md`, etc.
- Public: **https://github.com/djabbat/SpellCheckerKa-public** — excludes `CLAUDE.md`, `STATE.md`, `PARAMETERS.md`; only public-safe core + code.

### 4.3. Distribution channels
- **Chrome Web Store** — submission pending (see STATE.md P1).
- **Firefox Add-ons / Safari / Edge** — Phase 4.
- **App Store / Play Store** — Phase 2 (post-freeze, after 2026-09-27).

### 4.4. Reference corpora & tooling (external)
- Hunspell project: https://hunspell.github.io/
- LibreOffice language packs: https://extensions.libreoffice.org/
- Georgian National Corpus: http://gnc.gov.ge/ (or successor host)
- Tesseract OCR (for Phase 2 Georgian OCR): https://github.com/tesseract-ocr/tesseract
- LanguageTool (reference for grammar-rule DSL): https://languagetool.org/

---

## 5. Internal Ecosystem Links

SpellCheckerKa is part of the **drjaba.com** umbrella. It interoperates with several sibling projects.

### 5.1. SpellCheckerKa ↔ AIM
- **Direction:** SpellCheckerKa serves AIM; AIM uses `/api/check` to validate Georgian patient notes, medication instructions, and reports before output.
- **Integration point:** AIM sends `X-Admin-Token` to bypass the free-tier daily quota.
- **Value:** Georgian text QA for patient-facing content; AIM acts as a guaranteed internal power user, validating production reliability.

### 5.2. SpellCheckerKa ↔ kSystem
- **Direction:** bidirectional.
- **Value for kSystem:** validate Georgian article output before publication.
- **Value for SpellCheckerKa:** kSystem's Georgian technical vocabulary is a candidate source for dictionary enrichment.

### 5.3. SpellCheckerKa ↔ DrJaba (domain infrastructure)
- Hosted under the DrJaba domain umbrella as `spellcheckerka.drjaba.com`.
- DNS and nginx managed alongside other DrJaba subdomains (`aim.drjaba.com`, `ksystem.drjaba.com`).
- Deploy target is the shared DrJaba production server.

### 5.4. SpellCheckerKa ↔ OJS (Longevity Horizon journal / longevity.ge)
- **Direction:** SpellCheckerKa is a quality tool for OJS academic text.
- **Use case:** Georgian-language manuscripts submitted to *Longevity Horizon / Annals of Rejuvenation Science* pass through SpellCheckerKa before editorial review.
- **Freeze connection:** SpellCheckerKa is frozen until **2026-09-27** (the OJS Google-Scholar submission date). After unfreeze, academic Georgian users become a primary growth channel via the journal.

### 5.5. SpellCheckerKa ↔ monetaria
- Payment tooling; candidate for routing Premium/Corporate subscription billing once Stripe/Paddle integration lands (STATE.md P1).

---

## 6. API as Microservice

Any ecosystem project can use SpellCheckerKa as a microservice:

```
POST https://spellcheckerka.drjaba.com/api/check
Content-Type: application/json
X-Admin-Token: <token from PARAMETERS.md admin_bypass_header>

{"text": "...", "lang": "ka"}
```

Response: `{total_words, error_count, accuracy, errors[], typography[], stopwords[]}`. Full contract in DESIGN.md §3.

---

## 7. Licensing

- **SpellCheckerKa code:** MIT (© 2025 2sco).
- **Dictionary (derived from Hunspell ka_GE):** LGPL — allows commercial use provided modifications are published. Compliance action: publish derived dictionary as MIT or LGPL; list sources in a forthcoming `CREDITS.md` (P3).
- **User additions (`user_words.txt`):** treated as MIT contributions from users.

---

## 8. Acknowledgements

- Hunspell project contributors (Georgian `ka_GE` maintainers).
- Georgian National Corpus team.
- TSU Georgian NLP group — academic expertise.
- Elixir / Phoenix community — runtime and framework.
- Georgian linguists and testers who validate rules.

```
### `OPEN_PROBLEMS.md` (head 200 lines)
```
# OPEN_PROBLEMS.md — SpellCheckerKa

Known gaps: validation, implementation, scope, and strategic risks. Companion to STATE.md (which owns actionable TODOs). This file captures problems that are **not** turn-the-crank work — they require investigation, a design call, or an external dependency.

---

## 1. Validation Gaps

### 1.1. Recall on rare / specialised vocabulary
The 142,285-word base is a general-purpose Hunspell `ka_GE` excerpt. Recall on specialised domains is unmeasured and likely poor:
- Medical terminology (relevant for AIM integration)
- Legal / notarial vocabulary
- Scientific terminology (relevant for OJS / Longevity Horizon)
- Proper nouns (Georgian and transliterated international)
- Neologisms, tech loanwords

**What's missing:** a labelled corpus per domain and a measured recall number against it. Until we have that, P2 and P4 in THEORY.md §6 are assertions of target, not measurements of ground truth.

### 1.2. Morphology coverage
The 134 suffix rules + 13 preverbs cover the 7 noun cases plus common verb paradigms. Uncovered or partially covered:
- Rare verb aspect chains
- Suppletive forms
- Reduplicated adjectives
- Compound words formed at runtime
- Clitics and enclitic pronouns

We do not have a labelled "morphologically valid forms" test set. P2's 95% target is unmeasured.

### 1.3. Suggestion ranking quality
Current ranking is `distance asc, tie-break by insertion order`. Aspirational design called for **word-frequency weighting** (promote commonly-used candidates over obscure ones). Not implemented — no frequency table is maintained. Until implemented, users hitting confusion-pair typos may see rare candidates above obvious ones.

### 1.4. Confusion-pair edit cost
THEORY §3.3 notes the design-time intent to apply cost 0.5 to confusion substitutions inside the Levenshtein recurrence. The production code uses uniform cost 1.0 and uses confusion pairs only for candidate-pool expansion. Expected effect of adopting 0.5-cost: higher recall on phonetically similar typos, at the risk of more false-positive suggestions for unrelated words whose first character happens to be confusable.

### 1.5. Non-Georgian languages
`en / fr / es / ru` dictionaries use plain Levenshtein with no morphology and no confusion table. On inflection-heavy languages (especially Russian), false positives on valid inflected forms are expected to be high. We accept this as out-of-scope for v1; Georgian is the focus.

---

## 2. Implementation Gaps

### 2.1. Mobile (Phase 2) — not started
React Native scaffold, iOS + Android keyboard extensions, camera OCR, offline dictionary (~15 MB compressed), offline sync (CRDT + WatermelonDB) — all pending. Blocked by project freeze until **2026-09-27**; full execution timeline per CONCEPT.md roadmap is 4 months once unblocked.

### 2.2. Authentication & accounts
- No user auth today (rate limits are IP-scoped).
- `mailer.ex` is a 3-line Swoosh stub.
- No OAuth (email / Google / Facebook) endpoints exist.
- Prerequisite for premium-tier account quotas, API keys, and user-dictionary sync across devices.

### 2.3. Payment integration
Stripe or Paddle → tier-token grant → rate-limit tier resolution — not wired. Blocker for Premium ($4.99/mo) and Corporate ($99/mo) monetisation.

### 2.4. Planned but unimplemented endpoints
Listed in `CONCEPT_CODE_AUDIT_2026-04-21.md`:
- `POST /api/check/batch` (batch of up to 100 texts)
- `GET /api/dictionary/list` (list user words)
- `POST /api/sync` (cross-device sync)
- `GET /api/stats` (usage stats)
- `POST /api/auth/register`, `POST /api/auth/login`

CONCEPT.md §5.1 still shows all 9 endpoints in a single table — needs a split into "Implemented" + "Planned" subtables. Tracked as P1 in STATE.md.

### 2.5. Doc ↔ code inconsistencies (residual)
- `PARAMETERS.md` confusion-table row for `ზ↔ჟ` does not correspond to any `georgian_confusions/1` clause in `dictionary.ex`. Either implement the clause or remove the row.
- `PARAMETERS.md` documents 8–9 confusion pairs, but the code has 25 clauses over 24 letters. Expand the table.
- `README.md` Georgian architecture tree omits `lang_dictionary.ex` and `usage_tracker.ex`.

### 2.6. Test coverage
Two module tests + three controller tests (`test/`). Depth and breadth are thin. No property-based tests, no regression corpus for the accuracy predictions in THEORY §6, no load tests checking the performance contracts in DESIGN §7.

### 2.7. 993k dictionary variant
`priv/static/dictionaris/ge-<hash>.txt` contains 993,589 lines. Rejected 2026-04-21 after audit: 99.9% pure Georgian script, but 582 concatenated-phrase artefacts and sampled typos (e.g. `პეიოდში` missing `ე`). Revisit only after a documented cleaning pass; until then, do not load (see STATE.md §What NOT to do).

---

## 3. Scope-Level Limitations

These are **deliberate** exclusions from v1. Re-opening them requires a scope decision.

### 3.1. Not a grammar checker
Spell-check + typography + stopwords only. Case-agreement, verb-subject agreement, postposition placement are out of scope. A grammar layer is a proposed Phase 5 research direction (see EVIDENCE.md §3).

### 3.2. Not a style editor
No readability scoring, no tone suggestions, no clarity metrics. If users want Grammarly-class style editing in Georgian, this is not the product.

### 3.3. No auto-correct
Substitutions are always user-initiated. False-positive analysis is a prerequisite for any silent/automatic correction — see STATE.md P5.

### 3.4. No voice input
Georgian ASR is out of scope for 2026.

### 3.5. No native keyboards (iOS / Android) in Phase 1 or Phase 3
Phase 2 will evaluate native keyboard extensions; fallback UX is copy → check → paste. iOS App Store review on custom keyboards is a known risk (see §4.1 below).

### 3.6. No Microsoft Word plugin
Low priority. Revisit only on corporate customer request.

---

## 4. Strategic Risks

### 4.1. Apple App Store rejection for keyboard extension
Custom iOS keyboards get heavy review scrutiny. Risk mitigation: follow Human Interface Guidelines strictly; prepare a video demo for reviewers showing unique Georgian-specific value. Phase 2 timing will absorb one potential resubmission cycle.

### 4.2. Government procurement access
On-premise licensing for Georgian ministries and state institutions is in the monetisation plan but depends on procurement processes we do not control. Requirements will include SLA 99.9%, audit logs, on-prem deployment, and likely localisation of privacy policy under Georgian data-protection law. No revenue visibility here yet.

### 4.3. Chrome Web Store review
Manifest V3 compliance is complete, but the `<all_urls>` host permission can slow review. Risk: delayed launch of extension distribution by weeks. Mitigation: submit early (STATE.md P1); prepare tighter host-permission fallback if required.

### 4.4. Hunspell `ka_GE` licensing drift
The base dictionary is LGPL-derived. If upstream Hunspell `ka_GE` maintainers change licensing, downstream compliance needs re-verification. Action: publish `CREDITS.md` with clear provenance; subscribe to upstream change notifications.

### 4.5. Concurrent-load performance on ETS
ETS with `read_concurrency: true` is fast for parallel reads, but a single BEAM node has a ceiling (the full dictionary + indexes occupy ~150–200 MB). If user growth outpaces single-node capacity, horizontal scaling requires either shared-nothing sharding (per-tenant) or a read-through Redis tier. Not needed at current MAU but is a latent risk beyond 50,000 MAU.

### 4.6. Project freeze window
All progress is paused until **2026-09-27** to prioritise OJS Scholar submission (Longevity Horizon). A 5-month freeze window creates three risks:
- Staff skill atrophy on this codebase.
- Competitor launch window (LanguageTool adding Georgian, or a Georgian startup releasing a competing extension).
- Documentation drift as CONCEPT-level decisions evolve without corresponding code work.

Mitigation: STATE.md is the single-truth anchor during the freeze; CLAUDE.md enforces "critical bug fixes + admin tooling only" discipline.

### 4.7. DDoS / API abuse
Public `/api/check` with a free tier is a natural target. Today: per-IP rate limit + daily quota. Missing: Cloudflare-scale WAF, anomaly detection, API-key-scoped throttling for the future developer tier. Not a today problem but becomes one as awareness grows.

### 4.8. Dictionary-quality drift via `user_words.txt`
Any caller of `/api/dictionary/add` writes to the server's `user_words.txt`. Abuse vector: a script adds adversarial strings to weaken future spell-check results. Today's mitigation: rate-limit on the endpoint. Not mitigated: content filtering. Phase 4 account-based quotas would let us attribute and roll back user-level pollution.

---

## 5. Research Questions (not actionable, but open)

- Would a char-level ByT5 fine-tune on Georgian text outperform the dictionary+Levenshtein baseline on held-out typos? How much data is needed?
- Can the 993k variant be repaired algorithmically (phrase decomposition + typo filter) to net out above 142k in quality-adjusted coverage?
- Is a per-word frequency weight worth the additional memory + load-time cost for a measurable top-1 suggestion accuracy lift?
- Does the Georgian NLP community have a shared benchmark we can adopt, or do we publish our own?

```
### `mix.exs` (head 200 lines)
```
defmodule SpellCheckerKa.MixProject do
  use Mix.Project

  def project do
    [
      app: :spellcheckerka,
      version: "0.1.0",
      elixir: "~> 1.14",
      elixirc_paths: elixirc_paths(Mix.env()),
      start_permanent: Mix.env() == :prod,
      aliases: aliases(),
      deps: deps()
    ]
  end

  # Configuration for the OTP application.
  #
  # Type `mix help compile.app` for more information.
  def application do
    [
      mod: {SpellCheckerKa.Application, []},
      extra_applications: [:logger, :runtime_tools]
    ]
  end

  # Specifies which paths to compile per environment.
  defp elixirc_paths(:test), do: ["lib", "test/support"]
  defp elixirc_paths(_), do: ["lib"]

  # Specifies your project dependencies.
  #
  # Type `mix help deps` for examples and options.
  defp deps do
    [
      {:phoenix, "~> 1.7.21"},
      {:phoenix_html, "~> 4.1"},
      {:phoenix_live_reload, "~> 1.2", only: :dev},
      {:phoenix_live_view, "~> 1.0"},
      {:floki, ">= 0.30.0", only: :test},
      {:phoenix_live_dashboard, "~> 0.8.3"},
      {:esbuild, "~> 0.8", runtime: Mix.env() == :dev},
      {:tailwind, "~> 0.2.0", runtime: Mix.env() == :dev},
      {:heroicons,
       github: "tailwindlabs/heroicons",
       tag: "v2.1.1",
       sparse: "optimized",
       app: false,
       compile: false,
       depth: 1},
      {:swoosh, "~> 1.5"},
      {:finch, "~> 0.13"},
      {:telemetry_metrics, "~> 1.0"},
      {:telemetry_poller, "~> 1.0"},
      {:gettext, "~> 0.26"},
      {:jason, "~> 1.2"},
      {:dns_cluster, "~> 0.1.1"},
      {:bandit, "~> 1.5"}
    ]
  end

  # Aliases are shortcuts or tasks specific to the current project.
  # For example, to install project dependencies and perform other setup tasks, run:
  #
  #     $ mix setup
  #
  # See the documentation for `Mix` for more info on aliases.
  defp aliases do
    [
      setup: ["deps.get", "assets.setup", "assets.build"],
      "assets.setup": ["tailwind.install --if-missing", "esbuild.install --if-missing"],
      "assets.build": ["tailwind spellcheckerka", "esbuild spellcheckerka"],
      "assets.deploy": [
        "tailwind spellcheckerka --minify",
        "esbuild spellcheckerka --minify",
        "phx.digest"
      ]
    ]
  end
end

```
## systemd snapshot
```
  UNIT                                                                                                      LOAD   ACTIVE SUB       DESCRIPTION
  sys-devices-pci0000:00-0000:00:02.0-0000:01:00.0-virtio1-net-eth0.device                                  loaded active plugged   Virtio 1.0 network device
  sys-devices-pci0000:00-0000:00:02.2-0000:03:00.0-virtio2-virtio\x2dports-vport2p1.device                  loaded active plugged   /sys/devices/pci0000:00/0000:00:02.2/0000:03:00.0/virtio2/virtio-ports/vport2p1
  sys-devices-pci0000:00-0000:00:02.5-0000:06:00.0-virtio5-host0-target0:0:0-0:0:0:0-block-sr0.device       loaded active plugged   QEMU_CD-ROM
  sys-devices-pci0000:00-0000:00:02.5-0000:06:00.0-virtio5-host0-target0:0:0-0:0:0:1-block-sda-sda1.device  loaded active plugged   QEMU_HARDDISK 1
  sys-devices-pci0000:00-0000:00:02.5-0000:06:00.0-virtio5-host0-target0:0:0-0:0:0:1-block-sda-sda14.device loaded active plugged   QEMU_HARDDISK 14
  sys-devices-pci0000:00-0000:00:02.5-0000:06:00.0-virtio5-host0-target0:0:0-0:0:0:1-block-sda-sda15.device loaded active plugged   QEMU_HARDDISK 15
  sys-devices-pci0000:00-0000:00:02.5-0000:06:00.0-virtio5-host0-target0:0:0-0:0:0:1-block-sda.device       loaded active plugged   QEMU_HARDDISK
  sys-devices-pci0000:00-0000:00:02.6-0000:07:00.0-virtio6-net-enp7s0.device                                loaded active plugged   Virtio 1.0 network device
  sys-devices-pci0000:00-0000:00:04.0-0000:00:04.0:0-0000:00:04.0:0.0-tty-ttyS0.device                      loaded active plugged   QEMU PCI 16550A Adapter (QEMU Virtual Machine)
  sys-devices-platform-ARMH0011:00-ARMH0011:00:0-ARMH0011:00:0.0-tty-ttyAMA0.device                         loaded active plugged   /sys/devices/platform/ARMH0011:00/ARMH0011:00:0/ARMH0011:00:0.0/tty/ttyAMA0
  sys-devices-platform-serial8250-serial8250:0-serial8250:0.1-tty-ttyS1.device                              loaded active plugged   /sys/devices/platform/serial8250/serial8250:0/serial8250:0.1/tty/ttyS1
  
```
## Code histogram
```
rs 0
ex 22
exs 14
heex 4
go 0
py 1
php 0
ts 0
tsx 0
js 7

```