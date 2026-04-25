# Ze Theory — Entropic-Geometric Theory of Everything

**Статус:** концепция и прототип реализации, **2026-04-23** (переработка на основе `~/Desktop/5.md`).

Физико-математический TOE-уровневой теории Дж. Ткемаладзе. Симуляторы покрывают три количественных блока: импеданс-ODE, CHSH-деформацию и cheating-autowaves.

## Документы

- `CONCEPT.md` — конспект Ze Theory.
- `THEORY.md` — формальные уравнения, привязанные к главам 5.md.
- `PARAMETERS.md` — значения параметров и сценарии.
- `EVIDENCE.md` — первоисточники и их роли.
- `CLAUDE.md` — идентичность подпроекта.

## Стек

- **Симуляторы:** Rust crate `ze_simulator` — CLI `ze_sim` с режимами `impedance | chsh | autowaves`, JSON-выход.
- **Backend:** Rust crate `ze_backend` (axum) — REST `/api/impedance`, `/api/chsh`, `/api/autowaves`, `/api/scenarios`. Слушает `127.0.0.1:4001`.
- **Frontend:** Phoenix LiveView — три вкладки (по одной на модуль симулятора), графики через Chart.js. Слушает `127.0.0.1:4000`.

## Структура

```
Ze/
├── CONCEPT.md · THEORY.md · PARAMETERS.md · EVIDENCE.md · CLAUDE.md · README.md
├── Cargo.toml                       # rust workspace
├── simulator/                       # crate: 3 симулятора + CLI
│   ├── Cargo.toml
│   ├── src/lib.rs                   # модули impedance · chsh · autowaves
│   └── src/bin/ze_sim.rs
├── backend/                         # crate: axum REST
│   ├── Cargo.toml
│   └── src/main.rs
└── frontend/                        # Phoenix LiveView
    ├── mix.exs · config/ · lib/ · assets/
```

## Быстрый старт

```sh
# Предусловия: Rust 1.77+, Elixir 1.17+, Phoenix 1.8+.
cd Ze
cargo build --release
cargo test --release -p ze_simulator          # прогон F-тестов

# CLI:
./target/release/ze_sim impedance --scenario novelty --horizon 50
./target/release/ze_sim chsh --h 0.5
./target/release/ze_sim autowaves --steps 2000

# Сервер:
./target/release/ze_backend                   # 127.0.0.1:4001

# Frontend (в другом терминале):
cd frontend
mix deps.get && mix phx.server                # 127.0.0.1:4000
```

## Предсказания (см. THEORY §3–§4, PARAMETERS §2)

- CHSH: сдвиг `S_Ze − S_QM = δ·1.7478`; при параметрах по умолчанию ≈ 0.085 (42σ при 10⁹ совпадений).
- Autowaves: в окрестности `I_crit` возникают осцилляции между режимами learning / cheating.
- Impedance: meditation-сценарий даёт монотонно убывающий `𝓘(τ)`, positive `𝒞(τ)`, сходящийся `Φ_Ze`.

## Лицензия и статус

Исследовательский прототип. Некоторые предсказания (черные дыры, время внутри горизонта) — гипотетические экстраполяции без экспериментального подтверждения. Квантовый damping-оператор доказан только для специального случая — см. `THEORY.md §5` и `CONCEPT.md §7`.
