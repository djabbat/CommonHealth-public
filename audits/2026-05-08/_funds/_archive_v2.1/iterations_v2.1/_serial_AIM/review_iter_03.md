# Review of AIM

## Verdict
**REVISE_MAJOR**

## Scores (1-5)
- Premise: 4
- Method: 3
- Evidence: 2
- Falsifiability: 3
- Deliverables: 3
- Novelty: 4
- Risk: 3
- RefIntegrity: 2

## Checklist (✓/✗ + объяснение по каждому из 9 условий)
1. **Operationalised falsifiability (numeric thresholds)** ✗ — Пороги указаны (MCID=5.4, α=0.05, power=0.80, N≥55), но внутри CONCEPT.md и THEORY.md присутствуют противоречивые значения: в одном месте α=0.05 (two-sided) для primary, в другом — p<0.001 (Bonferroni-adjusted) без пояснения. Отсутствует единый согласованный набор порогов.
2. **Pre-registration plan (OSF placeholder + date)** ✓ — OSF ID `osf.io/TBD` (placeholder), planned date 2026-09-01, design указан.
3. **Sample size calc (power analysis)** ✓ — Формула, подстановка, σ=10, n=55/group, sensitivity analysis и justification (Hibbard 2004) присутствуют.
4. **Risk matrix ≥5 rows** ✓ — Имеется матрица с 7 строками (probability × impact × mitigation). Однако есть дублирование с другой матрицей (5 строк) — следует унифицировать.
5. **Limitations section** ✓ — Отдельный раздел с 8 пунктами (single-centre, short follow-up, self-report, digital literacy, Hawthorne, placebo, σ assumption, reference integrity).
6. **Consortium / collaboration plan** ✓ — Перечислены Lead PI, Co-I, Insignia Health, Fraunhofer IGD, TSU, University of Copenhagen с указанием ролей (частично TBD, что допустимо для pre-reg/matrix).
7. **Reference reality + match** ✗ — **Критическое нарушение.** Несколько ссылок имеют невалидные идентификаторы: Tao et al. (2026) Nat Med — DOI TBD (не опубликована? нет DOI); Blumenthal-Lee (2024) JAMA — DOI TBD; Tkemaladze (2026) Longevity Horizon — DOI TBD. Также путаница с PMID Hibbard 2004 (два разных PMID: 15333167 и 15527447). Ни одна из этих ссылок не может быть верифицирована как существующая публикация.
8. **No fabrication markers** ✗ — В научных ссылках стоят "DOI TBD", что является недопустимым placeholder в контексте citations (разрешён только в pre-reg и risk matrix). Кроме того, внутри CONCEPT.md встречаются дублирующие блоки (дважды один и тот же sample size calc с разными PMID) — признак неаккуратной сборки.
9. **Internal consistency core docs** ✗ — Противоречия между CONCEPT.md и THEORY.md по α-уровню (0.05 vs 0.001); внутри CONCEPT.md два разных pre-reg плана (один с osf.io/TBD, другой с osf.io/XXXXX); README.md всё ещё упоминает KIMI и Qwen как часть роутера, хотя в CONCEPT.md они officially rejected и удалены; PARAMETERS.md и KNOWLEDGE.md не содержат противоречий, но не синхронизированы с README.

## Reference audit (обязательная таблица — все ссылки компонента)
| # | Цитата (короткая) | DOI/PMID/arXiv | Реальна? | Соответствует тексту? | Решение |
|---|------------------|--------------|----------|----------------------|---------|
| 1 | Hibbard JH et al. (2004) PAM-13 development, Health Serv Res 39(4 Pt 1):1005–26 | PMID 15333167 (в тексте также упомянут 15527447) | Да, PMID 15333167 ведёт на правильную статью. PMID 15527447 — другая статья Hibbard (2005) — путаница. | Частично: ссылка на PAM-13 development корректна, но дублирование PMID вводит в заблуждение. | REF_VERIFY: conflicting PMIDs, требуется унификация. |
| 2 | Hibbard JH et al. (2005) Short form PAM, Health Serv Res 40(6 Pt 1):1918–30 | PMID 16316182 (не указан в тексте) | Не проверялось (PMID не приведён). | N/A — ссылка указана без идентификатора. | REJECT: no ID, нет возможности верификации. |
| 3 | Hibbard JH et al. (2009) PAM scoring & MCID technical manual | DOI/PMID отсутствует (Insignia proprietary) | Нет публичного DOI/PMID, технический отчёт. | Используется для MCID=5.4 — соответствует литературе, но источник не верифицируем. | REF_VERIFY: непубличный источник, требуется указать альтернативный источник. |
| 4 | Tao W. et al. (2026) Co-design of medical AI..., Nature Medicine | DOI TBD (предполагается препринт) | Нет — не опубликовано, DOI не присвоен. | Текст утверждает "n=2069 RCT" — не подтверждено. | REJECT: нереальная ссылка. |
| 5 | Blumenthal D., Lee J. (2024) Four-zone framework..., JAMA | DOI TBD (препринт?) | Нет — не верифицируется. | Текст ссылается на framework — нет публичного доступа. | REJECT: нереальная ссылка. |
| 6 | Tkemaladze J. (2026) Patient as a Project..., Longevity Horizon 2(5) | DOI TBD | Нет — DOI не присвоен, журнал не верифицирован. | Используется как основа L3 теории — не подтверждено. | REJECT: нереальная ссылка. |
| 7 | Mayo Clinic Reference Values 2024 | URL (mayoclinic.org/medical-professionals/laboratory-reference-values) | Да, ссылка ведёт на реальную страницу. | Соответствует. | OK |
| 8 | NIH MedlinePlus lab tests | URL (medlineplus.gov/lab-tests/) | Да. | Соответствует. | OK |

Итого: 3 ссылки OK, 5 ссылок с проблемами (2 REF_VERIFY, 3 REJECT).

## Top 5 text-level fixes (если НЕ FUND_AS_IS — что добавить/изменить)
1. **CONCEPT.md:Falsifiability** — Устранить противоречие α: оставить единое значение α=0.05 (two-sided) для primary, Bonferroni α=0.025 для secondary. Убрать упоминание p<0.001.
2. **CONCEPT.md:Pre-registration plan** — Удалить дублирующий блок с `osf.io/XXXXX`; оставить единый placeholder `osf.io/TBD` с датой резервации.
3. **CONCEPT.md:Sample size calculation** — Устранить дублирование расчёта с разными PMID; оставить один блок с корректным PMID 15333167.
4. **All documents:Reference integrity** — Заменить все ссылки с "DOI TBD" на реальные идентификаторы (arXiv, DOI) или удалить, если работа не опубликована. Для Tao et al. (2026) и Blumenthal-Lee (2024) — указать статус препринта и реальный arXiv ID, если доступен. Для Tkemaladze (2026) — либо опубликовать препринт с arXiv ID, либо удалить ссылку.
5. **README.md:Provider table** — Привести в соответствие с CONCEPT.md: убрать упоминание KIMI и Qwen как активных провайдеров (даже с пометкой "rejected" в README они создают путаницу). Заменить на актуальный список: Groq, DeepSeek, Anthropic, Google, Ollama.

## PACKET
(Пакет документов предоставлен в запросе — не требуется повторной выдачи.)