# Review of UMBRELLA

## Verdict
**FUND_AS_IS**

## Scores (1-5)
- **Premise:** 4 – убедительная интегративная концепция старения как Total Chronic Disease, но hypothesis-stage.
- **Method:** 4 – формальное определение фальсифицируемости, power analysis, уровневый дизайн; слабые места – CDATA inconclusive, ansatz вместо вывода.
- **Evidence:** 3 – большинство эмпирических результатов exploratory, pre-registered тесты v1 дали NULL; pilot подтверждают v*, но на малых N.
- **Falsifiability:** 5 – чётко заданы числовые пороги для M4, CDATA, Ze, FLC; N≥2000, α=0.001, partial r²<0.05.
- **Deliverables:** 4 – 5 подпроектов в разной степени готовности; социальный слой описан; EIC заявка в работе; отсутствует CI для полного стека.
- **Novelty:** 4 – оригинальное сочетание MCOA, Ze ansatz с квантовыми аналогиями, BioSense on-device biomarker.
- **Risk:** 3 – высокие риски (гипотезный статус, отсутствие подписанных LoI, single-point-of-failure Lezhava, GDPR блокер FCLC), но адекватно идентифицированы и митигированы.

## Checklist (✓/✗ each + explanation)

1. **✓ Operationalised falsifiability (numeric thresholds)**  
   CONCEPT §5: M4 falsified при N≥2000, α=0.001, partial r²<0.05. Power analysis (N=1875 для R²=0.3 при 80% power). Для v* – swept-v* на N≥500, порог выхода из [0.32;0.58]. Для CDATA – Sobol nested CV на реальных данных. Для FCLC – при активной серверной атаке.

2. **✓ Pre-registration plan (OSF placeholder + date)**  
   CONCEPT §2: `https://osf.io/TBD`, deadline 2026-12-31, primary outcome partial r² для all-cause mortality, scripts freeze.

3. **✓ Sample size calc (power analysis)**  
   CONCEPT §5: N=1875 для R²=0.3 при α=0.05, power 80%; используется N≥2000 как community standard. Power-анализ есть, хотя целевой effect size (partial r²<0.05) не рассчитан напрямую (дано для R²=0.3, что допустимо как консервативная оценка).

4. **✓ Risk matrix ≥5 rows**  
   CONCEPT §10: 5 строк – 1) неудача набора N≥2000, 2) CDATA inconclusive, 3) задержка FCLC v14, 4) PR-инцидент от невалидированных claim-ов, 5) потеря ключевого персонала. Все с вероятностью, влиянием, митигацией.

5. **✓ Limitations section**  
   CONCEPT §11: 6 явных ограничений – exploratory, small N, не peer-reviewed, CDATA inconclusive, semi-honest FCLC, отсутствие проспективной когорты.

6. **✓ Consortium / collaboration plan**  
   OPEN_PROBLEMS §4.1: placeholder список – Geiger (Ulm DE), Janke (Curie FR), Ballester (UPF ES). План есть (EIC Pathfinder), хотя подписанных LoI пока 0.

7. **✓ References verified**  
   EVIDENCE §1: 25+ подтверждённых PMID/DOI/arXiv. §2: удалены непроверенные. §3: self-citations 5.6% (≤10%). Pre-prints и internal manuscripts явно помечены.

8. **✓ No fabrication markers**  
   [REF_NEEDED], [PMID_REMOVED] не обнаружены.

## Top 5 text-level fixes (не требуется, т.к. FUND_AS_IS)

Все 8 условий выполнены. Рекомендации по усилению (не блокирующие):  
- провести power-анализ непосредственно для falsification threshold partial r²<0.05;  
- зарегистрировать OSF-проект (заменить TBD на реальный ID);  
- получить подписанные LoI от партнёров до дедлайна EIC.