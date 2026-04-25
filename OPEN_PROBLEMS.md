# OPEN_PROBLEMS — CommonHealth (Ecosystem)

**Назначение:** umbrella-level лимитации и блокеры. Подробности в OPEN_PROBLEMS.md каждого подпроекта.

## 1. EIC Pathfinder 2027 blockers

### 1.1 LoI gap
**Цель:** ≥2 signed EU-MS LoIs (DFKI, INRIA, ETH или эквивалент) к 2026-08.
**Risk:** cold-contact LoI turnaround 4-8 недель × 2 институтов + переговоры.
**Mitigation:** начать contact в мае 2026; параллельно несколько кандидатов.

### 1.2 PATE demo (FCLC)
ε_total=10 без PATE → Reviewer C REJECT 1.86/5.
**Цель:** ε≈0.63 path с working code + benchmark к 2026-12.
См. `FCLC/OPEN_PROBLEMS.md §1`.

### 1.3 CDATA ABL-2 Sobol paradox
S1 inverted vs central claim. Требует extended global sensitivity analysis.
См. `CDATA/OPEN_PROBLEMS.md`.

### 1.4 Fabricated PMIDs (HAP, Ontogenesis)
Halted после audit 2026-04-21. Recovery: rebuild EVIDENCE из верифицированных PubMed 2026-05 → 2026-09.

## 2. Architecture limitations

- **Subproject coupling:** через CONCEPT cross-references; нет formal interface validation
- **Health Score gone:** удалена формула с весами; UI должен использовать L_tissue напрямую — переделать
- **Federated layer не интегрирован** в server — REST endpoints stubs only
- **Ze·Guide AI** требует RAG над THEORY.md подпроектов — не имплементировано

## 3. Validation gaps

- Multi-site clinical pilot не запущен
- Биологический возраст dashboard валидация против Horvath/GrimAge — нет
- E2E test ecosystem (sensor → MCOA score → dashboard) — нет

## 4. Регулятивные

- DPIA templates готовы только для FCLC, не umbrella
- ISO 13485 / IEC 62304 — не цель в текущей итерации, но нужно для clinical-grade
- Грузия local законодательство — lawyer consultation pending

## 5. Стратегические риски

| Риск | Вероятность | Митигация |
|---|---|---|
| EIC 2027 also rejected | средняя | Параллельно: Marie Curie, Wellcome Leap, NIH HEAL, Longevity Impetus |
| Ключевой подпроект (CDATA) не валидируется | низкая | Counter #1 — один из 5; MCOA устойчива к удалению одного |
| FCLC compute cost prohibitive | средняя | University cluster partnerships |
| Privacy regulatory delays | высокая | Pre-built DPIA + lawyer review per site |
| Founder bottleneck (Tkemaladze) | средняя | EIC submission требует консорциума → распределённая ответственность |

## 6. Documentation gaps

- STRATEGY.md (5-track grant strategy) не существует
- REMINDER.md не существует
- Cross-subproject API documentation — fragmented
- Public-facing документация (для community contributors) — отсутствует

---

**Правило:** новый блокер EIC → §1; код/архитектура → §2; стратегический → §5.
