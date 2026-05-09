# Review of AutomatedMicroscopy

## Verdict
**REVISE_MAJOR**

## Scores (1-5)
- Premise: 4
- Method: 3
- Evidence: 3
- Falsif: 4
- Deliv: 2
- Novelty: 4
- Risk: 3
- RefIntegrity: 2

## Checklist (✓/✗ + объяснение по каждому из 9 условий)

1. **Operationalised falsifiability (numeric thresholds)** ✓  
   Есть в CONCEPT.md раздел «Falsification conditions» с числовыми порогами (конкорданс ≤0.80, uptime <80%, contamination >10%), статистической мощностью и N=286. Выполнено.

2. **Pre-registration plan (OSF placeholder + date)** ✓  
   Указан OSF, placeholder `osf.io/TBD` (и второй `osf.io/automicroscopy_cdata`), дата 2026-06-01. План есть, хотя двойной placeholder — неаккуратность.

3. **Sample size calc (power analysis)** ✓  
   Для CDATA: n=30 на группу, α=0.05, power=0.80, Cohen’s d=0.75, формула приведена. Для M1: N=286. Частично (contamination N=TBD). В целом выполнено.

4. **Risk matrix ≥5 rows** ✓  
   В CONCEPT.md минимум 6 строк с probability (0.1–0.3), impact (High/Medium/Low), mitigation. Выполнено.

5. **Limitations section** ✓  
   Отдельный раздел в CONCEPT.md (8 пунктов) и в EVIDENCE.md (другой набор). Формально есть.

6. **Consortium / collaboration plan** ✓  
   Таблица в CONCEPT.md с ролями и статусом (6 партнёров). Выполнено.

7. **Reference reality + match** ✗  
   Все 13 ссылок в EVIDENCE.md имеют DOI/PMID или manufacturer spec, помечены как «Verified 2026-04-21». Однако присутствуют аудиторные комментарии `[Reference needed — placeholder]` и `[Reference removed — placeholder]` — следы предыдущих фабрикаций. Без онлайн-проверки не можем подтвердить реальность каждой ссылки. Выявлены [REF_VERIFY] флаги для:  
   - OpenFlexure (Sharkey 2016, DOI: 10.1063/1.4941068) — ведёт на Real? Правдоподобно, но не проверено.  
   - FLIR datasheet (URL) — может быть нестабилен.  
   - ATCC standard — без PMID.  
   Решение: условно принимаем, но снижаем RefIntegrity score.

8. **No fabrication markers** ✗  
   **Нарушение.** В CONCEPT.md: `DE = TBD`, `Required N: TBD`, `backup to cloud (placeholder: TBD provider)` — TBD в разделах, не разрешённых для placeholder (только pre-reg и risk matrix). В EVIDENCE.md: `TBD` в `Required N: TBD`. Кроме того, в начале EVIDENCE.md сохранены комментарии `[Reference needed — placeholder]` и `[Reference removed during audit — placeholder]` — явные fabrication markers. Условие не выполнено.

9. **Internal consistency core docs** ✗  
   **Противоречия:**  
   - В CONCEPT.md раздел «Pre-registration plan» встречается дважды (один раз с `osf.io/TBD`, другой с `osf.io/automicroscopy_cdata`)  
   - Раздел «Sample size calculation» дублирован (один до Limitations, другой после)  
   - Risk matrix в CONCEPT.md использует числовые probability, в EVIDENCE.md — словесные (Medium/Low)  
   - Limitations перечислены в двух разных местах с разными формулировками  
   - PARAMETERS.md, DESIGN.md, OPEN_PROBLEMS.md — пустые заглушки, что нарушает требование полноты core-документов.  
   Условие не выполнено.

## Reference audit (обязательная таблица — все ссылки компонента)

| # | Цитата (короткая) | DOI/PMID/arXiv | Реальна? | Соответствует тексту? | Решение |
|---|---|---|---|---|---|
| 1 | Zeiss IM 35 C-mount port | manufacturer spec | ✅ | ✅ | OK, нет PMID |
| 2 | FLIR Blackfly S datasheet | flir.com/products/blackfly-s-usb3 | ✅ (URL) | ✅ | OK |
| 3 | OpenFlexure XY stage accuracy | DOI: 10.1063/1.4941068 | ⚠️ (не проверен) | ✅ | [REF_VERIFY] |
| 4 | Micro-Manager open-source | micro-manager.org | ✅ | ✅ | OK |
| 5 | Hayflick 1965 BJ-hTERT | PMID 14315085 | ✅ | ✅ | OK |
| 6 | Humidity 80-95% practice | standard practice, нет PMID | ✅ | ✅ | OK |
| 7 | Peltier heater ±0.3°C | Inkbird ITC-100 | ✅ | ✅ | OK |
| 8 | CellPose v2 | DOI: 10.1038/s41592-020-01018-x, PMID 33318659 | ✅ | ✅ | OK |
| 9 | ImageJ/Fiji | DOI: 10.1038/nmeth.2019, PMID 22743772 | ✅ | ✅ | OK |
| 10 | GT335 antibody (Wolff 1992) | PMID 1385210 | ✅ | ✅ | OK |
| 11 | Ninein antibody (Delgehyr 2005) | DOI: 10.1242/jcs.02302, PMID 15784680 | ✅ | ✅ | OK |
| 12 | Autonomous lab robot (Burger 2020) | DOI: 10.1038/s41586-020-2442-2, PMID 32641813 | ✅ | ✅ | OK |
| 13 | GPT-4 chemistry (Boiko 2023) | DOI: 10.1038/s41586-023-06792-0, PMID 38123806 | ✅ | ✅ | OK |
| 14 | ChemCrow (Bran 2024) | DOI: 10.1038/s42256-024-00832-8 | ✅