# CLAUDE.md — FCLC

## Контекст

**FCLC (Federated Clinical Learning Cooperative)** — privacy-preserving федеративное обучение для медицинского ИИ без передачи сырых пациентских данных.
Статус: v6.2 · Research Prototype · EIC Pathfinder candidate.
Расположение: `~/Desktop/CommonHealth/FCLC/`

## Авторитетный документ

`CONCEPT.md` v6.2 — единственный источник истины.
Ключевые компоненты: SecAgg+, Differential Privacy (ε≤1.0), LEVF, χ_Ze.

## Правила разработки

- **Ядро протокола, алгоритмы DP** → Rust
- **Координатор, API** → Python (FastAPI)
- **Тексты, гранты** → DeepSeek API
- **НИКОГДА** не писать "validated" для χ_Ze без ссылки на empirical test — peer review flagged это как misrepresentation
- **LEVF**: не заявлять EIC-ineligibility в публикациях — только в приватных документах

## Связь с экосистемой

- CommonHealth CLAUDE.md: `~/Desktop/CommonHealth/CLAUDE.md`
- Ze: `~/Desktop/CommonHealth/Ze/` — источник χ_Ze алгоритма
- BioSense: `~/Desktop/CommonHealth/BioSense/` — источник биомаркеров
- Git: djabbat/CommonHealth (монорепо)
