# Ze Theory

Entropic-Geometric Theory of Everything (Tkemaladze).

**Канон:** `Ze Theory.pdf` (англ.) + `Ze Теория.pdf` (рус.) — 24 главы + 4 приложения.
**Источник истины проекта:** `CONCEPT.md`.

## Структура

```
Ze/
├── Ze Theory.pdf · Ze Теория.pdf   ← канон
├── CONCEPT.md                       ← мост книга ↔ код
├── README · CLAUDE · TODO · PARAMETERS · MAP · MEMORY · LINKS · KNOWLEDGE · UPGRADE
├── Cargo.toml                       ← Rust workspace
├── simulator/                       ← crate ze_simulator (3 модуля + CLI)
├── backend/                         ← crate ze_backend (axum REST)
├── frontend/                        ← Phoenix LiveView
└── _archive/articles_2026-04-23/    ← старые THEORY/EVIDENCE/PARAMETERS/README
```

## Быстрый старт

```sh
cargo build --release
cargo test --release -p ze_simulator
./target/release/ze_sim impedance --scenario novelty --horizon 50
./target/release/ze_sim chsh --h 0.5
./target/release/ze_sim autowaves --steps 2000
./target/release/ze_backend                    # 127.0.0.1:4001
cd frontend && mix deps.get && mix phx.server  # 127.0.0.1:4000
```

## Что покрывает код (из 24 глав книги)

✅ гл. 2-5 (impedance ODE) · гл. 7 (CHSH) · гл. 8.4 (quantum damping) · гл. 13/17 (autowaves)
❌ гл. 9-11 (GR/cosmology/quantum gravity) · гл. 15 (EEG correlates) · гл. 19-21 (lab experiments)

Полная карта в `CONCEPT.md §6`.

Private only.
