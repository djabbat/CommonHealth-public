# Таблица ссылок для ручной проверки

**Создано:** автоматически extract_refs.py (PROMPT v2 — условие №7)
**Цель:** проверить (a) реальность DOI/PMID/arXiv ID и (b) соответствие цитаты тексту утверждения

**Всего найдено идентификаторов:** 480
**Фабрикационных маркеров:** 1


## Инструкция для ручной проверки

Для каждой строки таблицы:

1. **Реальность:** открыть https://doi.org/{DOI} или https://pubmed.ncbi.nlm.nih.gov/{PMID} → запись существует?
2. **Соответствие:** прочитать abstract → подтверждает ли утверждение в колонке `Контекст`?
3. Проставить ✓/✗ в колонках 'Real?' и 'Match?'
4. В колонке `Action`: OK / FIX / DELETE / REPLACE


## ⚠️ Фабрикационные маркеры (требуют немедленного внимания)

| # | Component | File | Line | Marker | Context |
|---|---|---|---|---|---|
| 1 | CytogeneticTree | PARAMETERS.md | 15 | `[REF_VERIFY — DOI/PMID TBD]` | \| O₂ concentration in chamber \| 3.0 ± 0.3 \| % \| physiological hypoxia (Parrinello 2003 [REF_VERIFY — DOI/PMID TBD])  |

## Идентификаторы по компонентам


### AIM (2 уникальных, всего вхождений 3)

| # | Тип | Идентификатор | URL | File:Line | Контекст | Real? | Match? | Action |
|---|---|---|---|---|---|---|---|---|
| 1 | PMID | `15333167` | [link](https://pubmed.ncbi.nlm.nih.gov/15333167/) | CONCEPT.md:44 | - **Justification for σ = 10:** σ = 10 is a conservative estimate based on published PAM-13 SD range | ☐ | ☐ | ☐ |
| 2 | PMID | `15527447` | [link](https://pubmed.ncbi.nlm.nih.gov/15527447/) | CONCEPT.md:56 | - **Justification for σ = 10:** σ = 10 is a conservative estimate based on published PAM-13 SD range | ☐ | ☐ | ☐ |

### AutomatedMicroscopy (15 уникальных, всего вхождений 16)

| # | Тип | Идентификатор | URL | File:Line | Контекст | Real? | Match? | Action |
|---|---|---|---|---|---|---|---|---|
| 1 | DOI | `10.1063/1.4941068` | [link](https://doi.org/10.1063/1.4941068) | EVIDENCE.md:24 | \| Arduino-based motorized XY stage achievable с ±5μm accuracy using linear rails + NEMA-17 steppers | ☐ | ☐ | ☐ |
| 2 | DOI | `10.1016/0014-4827(65)90211-9` | [link](https://doi.org/10.1016/0014-4827(65)90211-9) | EVIDENCE.md:31 | \| 37°C + 5% CO₂ environment необходимо для BJ-hTERT fibroblast long-term culture \| Hayflick 1965;  | ☐ | ☐ | ☐ |
| 3 | PMID | `14315085` | [link](https://pubmed.ncbi.nlm.nih.gov/14315085/) | EVIDENCE.md:31 | \| 37°C + 5% CO₂ environment необходимо для BJ-hTERT fibroblast long-term culture \| Hayflick 1965;  | ☐ | ☐ | ☐ |
| 4 | DOI | `10.1038/s41592-020-01018-x` | [link](https://doi.org/10.1038/s41592-020-01018-x) | EVIDENCE.md:39 | \| CellPose v2 segments cells in brightfield and fluorescence with generalist model \| Stringer et a | ☐ | ☐ | ☐ |
| 5 | PMID | `33318659` | [link](https://pubmed.ncbi.nlm.nih.gov/33318659/) | EVIDENCE.md:39 | \| CellPose v2 segments cells in brightfield and fluorescence with generalist model \| Stringer et a | ☐ | ☐ | ☐ |
| 6 | DOI | `10.1038/nmeth.2019` | [link](https://doi.org/10.1038/nmeth.2019) | EVIDENCE.md:40 | \| ImageJ/Fiji batch processing pipelines standard in centrosomal research \| Schindelin et al. 2012 | ☐ | ☐ | ☐ |
| 7 | PMID | `22743772` | [link](https://pubmed.ncbi.nlm.nih.gov/22743772/) | EVIDENCE.md:40 | \| ImageJ/Fiji batch processing pipelines standard in centrosomal research \| Schindelin et al. 2012 | ☐ | ☐ | ☐ |
| 8 | PMID | `1385210` | [link](https://pubmed.ncbi.nlm.nih.gov/1385210/) | EVIDENCE.md:41 | \| GT335 antibody recognizes polyglutamylated tubulin (ammonium sulfate precipitated cells) \| Wolff | ☐ | ☐ | ☐ |
| 9 | DOI | `10.1242/jcs.02302` | [link](https://doi.org/10.1242/jcs.02302) | EVIDENCE.md:42 | \| Ninein antibody marks mother centriole distal appendage complex \| Delgehyr et al. 2005 J Cell Sc | ☐ | ☐ | ☐ |
| 10 | PMID | `15784680` | [link](https://pubmed.ncbi.nlm.nih.gov/15784680/) | EVIDENCE.md:42 | \| Ninein antibody marks mother centriole distal appendage complex \| Delgehyr et al. 2005 J Cell Sc | ☐ | ☐ | ☐ |
| 11 | DOI | `10.1038/s41586-020-2442-2` | [link](https://doi.org/10.1038/s41586-020-2442-2) | EVIDENCE.md:47 | \| Autonomous lab robots for chemistry synthesis (Burger et al. 2020 Nature) \| Burger et al. 2020 \ | ☐ | ☐ | ☐ |
| 12 | PMID | `32641813` | [link](https://pubmed.ncbi.nlm.nih.gov/32641813/) | EVIDENCE.md:47 | \| Autonomous lab robots for chemistry synthesis (Burger et al. 2020 Nature) \| Burger et al. 2020 \ | ☐ | ☐ | ☐ |
| 13 | DOI | `10.1038/s41586-023-06792-0` | [link](https://doi.org/10.1038/s41586-023-06792-0) | EVIDENCE.md:48 | \| GPT-4 driving chemical synthesis planning (Boiko et al. 2023 Nature) \| Boiko et al. 2023 \| DOI: | ☐ | ☐ | ☐ |
| 14 | PMID | `38123806` | [link](https://pubmed.ncbi.nlm.nih.gov/38123806/) | EVIDENCE.md:48 | \| GPT-4 driving chemical synthesis planning (Boiko et al. 2023 Nature) \| Boiko et al. 2023 \| DOI: | ☐ | ☐ | ☐ |
| 15 | DOI | `10.1038/s42256-024-00832-8` | [link](https://doi.org/10.1038/s42256-024-00832-8) | EVIDENCE.md:49 | \| ChemCrow — LLM with chemistry tools (Bran et al. 2024 Nat Machine Intell) \| Bran et al. 2024 \|  | ☐ | ☐ | ☐ |

### BioSense (5 уникальных, всего вхождений 7)

| # | Тип | Идентификатор | URL | File:Line | Контекст | Real? | Match? | Action |
|---|---|---|---|---|---|---|---|---|
| 1 | DOI | `10.1016/j.neuroimage.2021.118123` | [link](https://doi.org/10.1016/j.neuroimage.2021.118123) | CONCEPT.md:439 | - Valdés-Sosa et al., NeuroImage 2021 (Cuban EEG) — DOI: 10.1016/j.neuroimage.2021.118123 [verified] | ☐ | ☐ | ☐ |
| 2 | DOI | `10.1093/chemse/21.6.773` | [link](https://doi.org/10.1093/chemse/21.6.773) | CONCEPT.md:440 | - Turin, Chem Senses 1996 — DOI: 10.1093/chemse/21.6.773 [verified] | ☐ | ☐ | ☐ |
| 3 | DOI | `10.1038/s41597-019-0021-4` | [link](https://doi.org/10.1038/s41597-019-0021-4) | CONCEPT.md:441 | - Babayan et al., Sci Data 2019 (MPI-LEMON) — DOI: 10.1038/s41597-019-0021-4 [verified] | ☐ | ☐ | ☐ |
| 4 | PMID | `36583780` | [link](https://pubmed.ncbi.nlm.nih.gov/36583780/) | KNOWLEDGE.md:129 | - **Ze Theory:** Tkemaladze J. Mol Biol Reports 2023. PMID 36583780 | ☐ | ☐ | ☐ |
| 5 | PMID | `20480236` | [link](https://pubmed.ncbi.nlm.nih.gov/20480236/) | KNOWLEDGE.md:130 | - **Aging biology:** Lezhava T. et al. Biogerontology 2011. PMID 20480236 | ☐ | ☐ | ☐ |

### CDATA (73 уникальных, всего вхождений 130)

| # | Тип | Идентификатор | URL | File:Line | Контекст | Real? | Match? | Action |
|---|---|---|---|---|---|---|---|---|
| 1 | PMID | `26213385` | [link](https://pubmed.ncbi.nlm.nih.gov/26213385/) | CONCEPT.md:25 | **Версия:** 5.2 (Counter #1 framing, унифицировано 2026-05-07; numbering decision: CDATA = #1, Telom | ☐ | ☐ | ☐ |
| 2 | PMID | `28636844` | [link](https://pubmed.ncbi.nlm.nih.gov/28636844/) | CONCEPT.md:75 | > **Статус этики:** Cell-DT v3.0 — полностью *in silico* симулятор. **Реальных пациентов нет.** Все  | ☐ | ☐ | ☐ |
| 3 | PMID | `24138928` | [link](https://pubmed.ncbi.nlm.nih.gov/24138928/) | CONCEPT.md:75 | > **Статус этики:** Cell-DT v3.0 — полностью *in silico* симулятор. **Реальных пациентов нет.** Все  | ☐ | ☐ | ☐ |
| 4 | PMID | `18316408` | [link](https://pubmed.ncbi.nlm.nih.gov/18316408/) | CONCEPT.md:75 | > **Статус этики:** Cell-DT v3.0 — полностью *in silico* симулятор. **Реальных пациентов нет.** Все  | ☐ | ☐ | ☐ |
| 5 | DOI | `10.1096/fj.201902376R` | [link](https://doi.org/10.1096/fj.201902376R) | CONCEPT.md:210 | [Peters-Hall JR et al. *FASEB J* 2020;34(1):386‑98; PMID 31914653; DOI: 10.1096/fj.201902376R] | ☐ | ☐ | ☐ |
| 6 | PMID | `31914653` | [link](https://pubmed.ncbi.nlm.nih.gov/31914653/) | CONCEPT.md:210 | [Peters-Hall JR et al. *FASEB J* 2020;34(1):386‑98; PMID 31914653; DOI: 10.1096/fj.201902376R] | ☐ | ☐ | ☐ |
| 7 | PMID | `11067876` | [link](https://pubmed.ncbi.nlm.nih.gov/11067876/) | CONCEPT.md:212 | [Sudo et al. 2000, PMID 11067876; Beerman et al. 2010, PMID 20304793; Yahata et al. 2011, PMID 21734 | ☐ | ☐ | ☐ |
| 8 | PMID | `20304793` | [link](https://pubmed.ncbi.nlm.nih.gov/20304793/) | CONCEPT.md:212 | [Sudo et al. 2000, PMID 11067876; Beerman et al. 2010, PMID 20304793; Yahata et al. 2011, PMID 21734 | ☐ | ☐ | ☐ |
| 9 | PMID | `21734240` | [link](https://pubmed.ncbi.nlm.nih.gov/21734240/) | CONCEPT.md:212 | [Sudo et al. 2000, PMID 11067876; Beerman et al. 2010, PMID 20304793; Yahata et al. 2011, PMID 21734 | ☐ | ☐ | ☐ |
| 10 | PMID | `16990891` | [link](https://pubmed.ncbi.nlm.nih.gov/16990891/) | CONCEPT.md:213 | ⚠️ Примечание: ранее ошибочно цитировался Morrison & Kimble 2006 (PMID 16990891 — C. elegans/Drosoph | ☐ | ☐ | ☐ |
| 11 | PMID | `32755011` | [link](https://pubmed.ncbi.nlm.nih.gov/32755011/) | CONCEPT.md:277 | \| Ингибиционное доказательство (замедление D → замедление старения) \| ⚠️ Косвенное \| CASIN→lifesp | ☐ | ☐ | ☐ |
| 12 | PMID | `24065130` | [link](https://pubmed.ncbi.nlm.nih.gov/24065130/) | CONCEPT.md:293 | \| Аналогия \| ✅ Да \| NPC-старение в нейронах ¬R; TTLL6+tau→Alzheimer (EMBO J 2013 PMID 24065130) \ | ☐ | ☐ | ☐ |
| 13 | PMID | `17255513` | [link](https://pubmed.ncbi.nlm.nih.gov/17255513/) | CONCEPT.md:419 | 2. Та же стволовая дочь получает **старую материнскую центриоль** с накопленными PTM (Yamashita et a | ☐ | ☐ | ☐ |
| 14 | PMID | `21145745` | [link](https://pubmed.ncbi.nlm.nih.gov/21145745/) | CONCEPT.md:419 | 2. Та же стволовая дочь получает **старую материнскую центриоль** с накопленными PTM (Yamashita et a | ☐ | ☐ | ☐ |
| 15 | PMID | `39764850` | [link](https://pubmed.ncbi.nlm.nih.gov/39764850/) | CONCEPT.md:453 | \| **Центриоль (материнская)** \| ⏳ **Предсказано; косвенная поддержка** — полиглутамилирование/ацет | ☐ | ☐ | ☐ |
| 16 | DOI | `10.1038/s41589-024-01599-0` | [link](https://doi.org/10.1038/s41589-024-01599-0) | CONCEPT.md:472 | Fourest-Lieuvin et al. 2024 (*Nature Chemical Biology* DOI 10.1038/s41589-024-01599-0): Cryo-EM + ки | ☐ | ☐ | ☐ |
| 17 | PMID | `39266565` | [link](https://pubmed.ncbi.nlm.nih.gov/39266565/) | CONCEPT.md:474 | **🆕 GT335+ → СЕНЕСЦЕНЦИЯ (Hao Robichaud et al. 2024, sinc-MT):** Hao Robichaud et al. 2024 (Nat Comm | ☐ | ☐ | ☐ |
| 18 | DOI | `10.1101/2024.11.25.624814` | [link](https://doi.org/10.1101/2024.11.25.624814) | CONCEPT.md:476 | - **bioRxiv 2024** — «Expression of recombinant human glutamylating TTLLs: only TTLL6 disrupts micro | ☐ | ☐ | ☐ |
| 19 | PMID | `40562742` | [link](https://pubmed.ncbi.nlm.nih.gov/40562742/) | CONCEPT.md:477 | - **Nature Communications 2025** — «Polyglutamylation of microtubules drives neuronal remodeling» [P | ☐ | ☐ | ☐ |
| 20 | PMID | `40157365` | [link](https://pubmed.ncbi.nlm.nih.gov/40157365/) | CONCEPT.md:478 | - **Developmental Cell 2025** — «KIF2C promotes paclitaxel resistance by depolymerizing polyglutamyl | ☐ | ☐ | ☐ |
| 21 | DOI | `10.64898/2025.12.15.694360` | [link](https://doi.org/10.64898/2025.12.15.694360) | CONCEPT.md:480 | - **bioRxiv 2025** — «TTLL6-mediated Polyglutamylation of PurA Maintains Colonic Crypt Integrity» [D | ☐ | ☐ | ☐ |
| 22 | DOI | `10.1159/000543411` | [link](https://doi.org/10.1159/000543411) | CONCEPT.md:493 | Pan et al. 2025 (DOI: 10.1159/000543411) продемонстрировали, что CCP1-дефицит вызывает аномальное по | ☐ | ☐ | ☐ |
| 23 | PMID | `38874393` | [link](https://pubmed.ncbi.nlm.nih.gov/38874393/) | CONCEPT.md:501 | - **🆕 HSC — молекулярная асимметрия при ACD (2024):** Ugale et al. 2024 (Journal of Cell Biology, PM | ☐ | ☐ | ☐ |
| 24 | PMID | `39012627` | [link](https://pubmed.ncbi.nlm.nih.gov/39012627/) | CONCEPT.md:502 | - **🆕 Центросомный возраст нарушает симметрию даже «симметричных» делений (2024):** JCB 2024 (PMID 3 | ☐ | ☐ | ☐ |
| 25 | DOI | `10.3389/frhem.2024.1373554` | [link](https://doi.org/10.3389/frhem.2024.1373554) | CONCEPT.md:503 | - **🆕 HSC ACD — утрата с возрастом (2024, Frontiers Hematology):** Обзор 2024 подтверждает: молодые  | ☐ | ☐ | ☐ |
| 26 | PMID | `37552892` | [link](https://pubmed.ncbi.nlm.nih.gov/37552892/) | CONCEPT.md:506 | - Yamashita 2023 (Annual Review of Genetics, PMID 37552892) — консенсус: ACD = механизм зародышевого | ☐ | ☐ | ☐ |
| 27 | PMID | `40562035` | [link](https://pubmed.ncbi.nlm.nih.gov/40562035/) | CONCEPT.md:513 | **🆕 КЛЮЧЕВОЙ HALLMARK-КОНСЕНСУС (2025):** Rando TA, Brunet A., Goodell MA (2025). «Hallmarks of stem | ☐ | ☐ | ☐ |
| 28 | DOI | `10.1038/s41536-022-00275-y` | [link](https://doi.org/10.1038/s41536-022-00275-y) | CONCEPT.md:517 | - Leins et al. 2022 (*npj Regen Med* DOI 10.1038/s41536-022-00275-y): Трансплантация CASIN-обработан | ☐ | ☐ | ☐ |
| 29 | PMID | `39412222` | [link](https://pubmed.ncbi.nlm.nih.gov/39412222/) | CONCEPT.md:523 | - **SVBP/VASH-путь (2025):** Launay et al. 2025 (Aging Cell, PMID 39412222) — биаллельный вариант SV | ☐ | ☐ | ☐ |
| 30 | PMID | `40257113` | [link](https://pubmed.ncbi.nlm.nih.gov/40257113/) | CONCEPT.md:524 | - **PLK4-путь (2023/2025):** Dang 2023 (Blood) + Hamzah 2025 (Cytoskeleton, PMID 40257113) — PLK4-ин | ☐ | ☐ | ☐ |
| 31 | PMID | `22056670` | [link](https://pubmed.ncbi.nlm.nih.gov/22056670/) | CONCEPT.md:606 | \| Lapasset et al. 2011 (Genes Dev, PMID 22056670) \| Сенесцентные и центенарные фибробласты (92–101 | ☐ | ☐ | ☐ |
| 32 | PMID | `27984723` | [link](https://pubmed.ncbi.nlm.nih.gov/27984723/) | CONCEPT.md:607 | \| Ocampo et al. 2016 (Cell, PMID 27984723) \| Циклическое частичное репрограммирование OSKM in vivo | ☐ | ☐ | ☐ |
| 33 | PMID | `17218264` | [link](https://pubmed.ncbi.nlm.nih.gov/17218264/) | CONCEPT.md:716 | - Снижение **scaffold-функции** Aurora A → нарушение PCM сборки: нарушается рекрутирование TACC3/NED | ☐ | ☐ | ☐ |
| 34 | PMID | `21993292` | [link](https://pubmed.ncbi.nlm.nih.gov/21993292/) | CONCEPT.md:717 | - Накопление дефектов γ-TuRC → снижение нуклеации микротрубочек (Kollman 2011, PMID 21993292) | ☐ | ☐ | ☐ |
| 35 | PMID | `21474673` | [link](https://pubmed.ncbi.nlm.nih.gov/21474673/) | CONCEPT.md:872 | \| **Тканевые** \| HSC_ν \| 12 \| Fixed \| PMID: 21474673 (Catlin et al. 2011; ~1 division/month in  | ☐ | ☐ | ☐ |
| 36 | PMID | `27041501` | [link](https://pubmed.ncbi.nlm.nih.gov/27041501/) | CONCEPT.md:875 | \| \| ISC_ν \| 70 \| Fixed \| PMID: 27041501 (Tetteh et al. 2016; ~1 division/day in intestinal cryp | ☐ | ☐ | ☐ |
| 37 | PMID | `18356530` | [link](https://pubmed.ncbi.nlm.nih.gov/18356530/) | CONCEPT.md:878 | \| \| Muscle_ν \| 4 \| Fixed \| PMID: 18356530 (Zammit et al. 2008; satellite cell activation rate ~ | ☐ | ☐ | ☐ |
| 38 | PMID | `16407887` | [link](https://pubmed.ncbi.nlm.nih.gov/16407887/) | CONCEPT.md:881 | \| \| Neural_ν \| 2 \| Fixed \| PMID: 16407887 (Bhardwaj et al. 2006; hippocampal neurogenesis ~2/ye | ☐ | ☐ | ☐ |
| 39 | PMID | `23746838` | [link](https://pubmed.ncbi.nlm.nih.gov/23746838/) | CONCEPT.md:885 | \| \| inhib_threshold \| 0.8 \| Uniform(0.6,1.0) \| PMID: 23746838 (Campisi 2013; high SASP = pathol | ☐ | ☐ | ☐ |
| 40 | PMID | `19587680` | [link](https://pubmed.ncbi.nlm.nih.gov/19587680/) | CONCEPT.md:892 | \| **Фиксированные** \| mtor_activity \| 0.7 \| Fixed \| PMID: 19587680 (Harrison et al. 2009; basal | ☐ | ☐ | ☐ |
| 41 | PMID | `21654799` | [link](https://pubmed.ncbi.nlm.nih.gov/21654799/) | CONCEPT.md:895 | \| \| yap_taz_sensitivity \| 0.5 \| Fixed \| PMID: 21654799 (Dupont et al. 2011; YAP/TAZ mechanosens | ☐ | ☐ | ☐ |
| 42 | PMID | `34546229` | [link](https://pubmed.ncbi.nlm.nih.gov/34546229/) | CONCEPT.md:1146 | \| Метрика \| CDATA v4.3 \| DunedinPACE (PMID 34546229) \| PhenoAge (PMID 29786094) \| GrimAge (PMID | ☐ | ☐ | ☐ |
| 43 | PMID | `29786094` | [link](https://pubmed.ncbi.nlm.nih.gov/29786094/) | CONCEPT.md:1146 | \| Метрика \| CDATA v4.3 \| DunedinPACE (PMID 34546229) \| PhenoAge (PMID 29786094) \| GrimAge (PMID | ☐ | ☐ | ☐ |
| 44 | PMID | `31451800` | [link](https://pubmed.ncbi.nlm.nih.gov/31451800/) | CONCEPT.md:1146 | \| Метрика \| CDATA v4.3 \| DunedinPACE (PMID 34546229) \| PhenoAge (PMID 29786094) \| GrimAge (PMID | ☐ | ☐ | ☐ |
| 45 | PMID | `18923395` | [link](https://pubmed.ncbi.nlm.nih.gov/18923395/) | CONCEPT.md:1295 | \| **Drosophila** \| Centrosome misorientation ↑ с возрастом (5%→40%); дезориентированные GSC делятс | ☐ | ☐ | ☐ |
| 46 | PMID | `22357619` | [link](https://pubmed.ncbi.nlm.nih.gov/22357619/) | CONCEPT.md:1296 | \| **Drosophila** \| Нутритивный дефицит → centrosome misorientation → G2 arrest \| Ак. 2 (цилия→G1/ | ☐ | ☐ | ☐ |
| 47 | PMID | `22215083` | [link](https://pubmed.ncbi.nlm.nih.gov/22215083/) | CONCEPT.md:1297 | \| **Мышь (мышца)** \| Старые MDSC: дефектная пролиферация; co-culture молодых со старыми → rescue д | ☐ | ☐ | ☐ |
| 48 | PMID | `23967009` | [link](https://pubmed.ncbi.nlm.nih.gov/23967009/) | CONCEPT.md:1298 | \| **Мышь (кровь)** \| Young BM (1.5 мес.) → старые мыши → **+34% lifespan**, chimerism 28% \| Ак. 3 | ☐ | ☐ | ☐ |
| 49 | PMID | `36581635` | [link](https://pubmed.ncbi.nlm.nih.gov/36581635/) | CONCEPT.md:1301 | \| **Мышь (кровь)** \| CASIN → aged HSC rejuvenation → transplant → lifespan + healthspan + мышечная | ☐ | ☐ | ☐ |
| 50 | PMID | `39743633` | [link](https://pubmed.ncbi.nlm.nih.gov/39743633/) | CONCEPT.md:1302 | \| **Мышь (кровь)** \| **CD150low** старые HSC (делились реже = меньше PTM) → transplant → **+9–12%  | ☐ | ☐ | ☐ |
| 51 | PMID | `38679727` | [link](https://pubmed.ncbi.nlm.nih.gov/38679727/) | CONCEPT.md:1304 | \| **Человек (RCT)** \| UC-MSC при aging frailty → физические показатели ↑, воспаление ↓ \| SC пул в | ☐ | ☐ | ☐ |
| 52 | PMID | `38658656` | [link](https://pubmed.ncbi.nlm.nih.gov/38658656/) | CONCEPT.md:1318 | **Механизм:** Паракринный Hh/Wnt-сигнал от молодых клеток частично компенсирует дефектную цилию стар | ☐ | ☐ | ☐ |
| 53 | PMID | `28901234` | [link](https://pubmed.ncbi.nlm.nih.gov/28901234/) | CONCEPT.md:1370 | 2. **Jaiswal S., et al.** (2017). Clonal hematopoiesis and risk of hematologic malignancies. *NEJM*, | ☐ | ☐ | ☐ |
| 54 | PMID | `36708707` | [link](https://pubmed.ncbi.nlm.nih.gov/36708707/) | CONCEPT.md:1372 | 4. **Lopez-Otin C., et al.** (2023). Hallmarks of aging. *Cell* 186(2): 243-278. PMID: 36708707 | ☐ | ☐ | ☐ |
| 55 | DOI | `10.1371/journal.pone.0339922` | [link](https://doi.org/10.1371/journal.pone.0339922) | CONCEPT.md:1379 | 9. **Sheu SH, et al.** (2025). Only TTLL6 disrupts MT dynamics; LDC10 inhibitor. *PLOS ONE*. DOI: 10 | ☐ | ☐ | ☐ |
| 56 | DOI | `10.1182/blood-2002-07-2334` | [link](https://doi.org/10.1182/blood-2002-07-2334) | EVIDENCE.md:21 | \| Клетки мыши с активной теломеразой in vivo достигают предела делений \| DOI: 10.1182/blood-2002-0 | ☐ | ☐ | ☐ |
| 57 | PMID | `12663456` | [link](https://pubmed.ncbi.nlm.nih.gov/12663456/) | EVIDENCE.md:21 | \| Клетки мыши с активной теломеразой in vivo достигают предела делений \| DOI: 10.1182/blood-2002-0 | ☐ | ☐ | ☐ |
| 58 | DOI | `10.1016/j.stem.2020.08.012` | [link](https://doi.org/10.1016/j.stem.2020.08.012) | EVIDENCE.md:22 | \| Стволовые клетки в физиологической гипоксии ниши всё же стареют \| [DOI: 10.1016/j.stem.2020.08.0 | ☐ | ☐ | ☐ |
| 59 | PMID | `6129277` | [link](https://pubmed.ncbi.nlm.nih.gov/6129277/) | EVIDENCE.md:23 | \| Серийная трансплантация HSC ведёт к истощению (donor-age + cell-number зависимость) \| PMID: 6129 | ☐ | ☐ | ☐ |
| 60 | DOI | `10.1016/j.celrep.2016.07.012` | [link](https://doi.org/10.1016/j.celrep.2016.07.012) | EVIDENCE.md:27 | \| Polyglutamylation регулирует функцию центриоли и реснички \| [DOI: 10.1016/j.celrep.2016.07.012]( | ☐ | ☐ | ☐ |
| 61 | DOI | `10.1038/ncb3509` | [link](https://doi.org/10.1038/ncb3509) | EVIDENCE.md:28 | \| Дефекты полиглутамилирования нарушают сборку аксонемы реснички \| [DOI: 10.1038/ncb3509](https:// | ☐ | ☐ | ☐ |
| 62 | DOI | `10.1038/nrg2774` | [link](https://doi.org/10.1038/nrg2774) | EVIDENCE.md:29 | \| Первичная ресничка необходима для передачи сигналов Hedgehog \| [DOI: 10.1038/nrg2774](https://do | ☐ | ☐ | ☐ |
| 63 | PMID | `20395968` | [link](https://pubmed.ncbi.nlm.nih.gov/20395968/) | EVIDENCE.md:29 | \| Первичная ресничка необходима для передачи сигналов Hedgehog \| [DOI: 10.1038/nrg2774](https://do | ☐ | ☐ | ☐ |
| 64 | DOI | `10.7554/eLife.83157` | [link](https://doi.org/10.7554/eLife.83157) | EVIDENCE.md:30 | \| Ninein опосредует асимметричное наследование материнской центриоли \| DOI: 10.7554/eLife.83157; P | ☐ | ☐ | ☐ |
| 65 | PMID | `37882444` | [link](https://pubmed.ncbi.nlm.nih.gov/37882444/) | EVIDENCE.md:30 | \| Ninein опосредует асимметричное наследование материнской центриоли \| DOI: 10.7554/eLife.83157; P | ☐ | ☐ | ☐ |
| 66 | DOI | `10.1016/j.celrep.2024.115127` | [link](https://doi.org/10.1016/j.celrep.2024.115127) | EVIDENCE.md:31 | \| Асимметричное наследование материнской центриоли в T-клетках \| DOI: 10.1016/j.celrep.2024.115127 | ☐ | ☐ | ☐ |
| 67 | DOI | `10.1038/nature07208` | [link](https://doi.org/10.1038/nature07208) | EVIDENCE.md:35 | \| Старые HSC делятся реже и находятся в углублённом покое \| [DOI: 10.1038/nature07208](https://doi | ☐ | ☐ | ☐ |
| 68 | DOI | `10.1016/j.stem.2015.07.002` | [link](https://doi.org/10.1016/j.stem.2015.07.002) | EVIDENCE.md:36 | \| Снижение частоты деления HSC с возрастом, продемонстрированное отслеживанием \| [DOI: 10.1016/j.s | ☐ | ☐ | ☐ |
| 69 | DOI | `10.1016/j.stem.2024.11.001` | [link](https://doi.org/10.1016/j.stem.2024.11.001) | EVIDENCE.md:41 | \| Misorientation центросомы — официальный признак старения стволовых клеток \| [DOI: 10.1016/j.stem | ☐ | ☐ | ☐ |
| 70 | DOI | `10.1159/000539232` | [link](https://doi.org/10.1159/000539232) | EVIDENCE.md:43 | \| Дефицит CCP1 (деглутамилазы) ведёт к дефектам цилиогенеза и дифференцировки \| [DOI: 10.1159/0005 | ☐ | ☐ | ☐ |
| 71 | PMID | `9647649` | [link](https://pubmed.ncbi.nlm.nih.gov/9647649/) | EVIDENCE.md:44 | \| Инъекция антител GT335 приводит к потере центриоли и её de novo синтезу \| [PMID: 9647649](https: | ☐ | ☐ | ☐ |
| 72 | PMID | `36583780` | [link](https://pubmed.ncbi.nlm.nih.gov/36583780/) | PARAMETERS.md:11 | > \| α (α_HSC) \| 0.028 \| **0.0082** \| (b) docs → code; MCMC posterior (PMID 36583780 concept only | ☐ | ☐ | ☐ |
| 73 | PMID | `28792876` | [link](https://pubmed.ncbi.nlm.nih.gov/28792876/) | PARAMETERS.md:17 | > **Следствие:** таблица ниже **теперь match code** для всех активных параметров. Bonus finding: fix | ☐ | ☐ | ☐ |

### CytogeneticTree (15 уникальных, всего вхождений 25)

| # | Тип | Идентификатор | URL | File:Line | Контекст | Real? | Match? | Action |
|---|---|---|---|---|---|---|---|---|
| 1 | DOI | `10.1101/2023.01.01.522000` | [link](https://doi.org/10.1101/2023.01.01.522000) | KNOWLEDGE.md:5 | [PREPRINT: https://doi.org/10.1101/2023.01.01.522000] (Lee & Luo 1999 Neuron (REFERENCE VERIFICATION | ☐ | ☐ | ☐ |
| 2 | PMID | `10197526` | [link](https://pubmed.ncbi.nlm.nih.gov/10197526/) | KNOWLEDGE.md:5 | [PREPRINT: https://doi.org/10.1101/2023.01.01.522000] (Lee & Luo 1999 Neuron (REFERENCE VERIFICATION | ☐ | ☐ | ☐ |
| 3 | PMID | `31485075` | [link](https://pubmed.ncbi.nlm.nih.gov/31485075/) | KNOWLEDGE.md:16 | > ⚠️ **Stub correction:** the earlier stub listed "Loeffler D., *Nature* 2019, PMID 31485075." Verif | ☐ | ☐ | ☐ |
| 4 | PMID | `31485073` | [link](https://pubmed.ncbi.nlm.nih.gov/31485073/) | KNOWLEDGE.md:16 | > ⚠️ **Stub correction:** the earlier stub listed "Loeffler D., *Nature* 2019, PMID 31485075." Verif | ☐ | ☐ | ☐ |
| 5 | DOI | `10.1016/S0896-6273(00)80701-1` | [link](https://doi.org/10.1016/S0896-6273(00)80701-1) | KNOWLEDGE.md:91 | - MARCM Lee T & Luo L. *Neuron* 1999;22(3):451‑61. PMID: 10197526; DOI: 10.1016/S0896-6273(00)80701- | ☐ | ☐ | ☐ |
| 6 | PMID | `31086336` | [link](https://pubmed.ncbi.nlm.nih.gov/31086336/) | KNOWLEDGE.md:110 | - **Chan 2019** (PMID 31086336, mouse, Nature) — whole-embryo CRISPR recording | ☐ | ☐ | ☐ |
| 7 | PMID | `30093604` | [link](https://pubmed.ncbi.nlm.nih.gov/30093604/) | KNOWLEDGE.md:111 | - **Kalhor 2018** (PMID 30093604, homing CRISPR, whole mouse) | ☐ | ☐ | ☐ |
| 8 | PMID | `29674432` | [link](https://pubmed.ncbi.nlm.nih.gov/29674432/) | KNOWLEDGE.md:112 | - **Plass 2018** (PMID 29674432, whole planarian, single-cell) | ☐ | ☐ | ☐ |
| 9 | PMID | `17255513` | [link](https://pubmed.ncbi.nlm.nih.gov/17255513/) | KNOWLEDGE.md:171 | - **Yamashita 2007** (PMID 17255513) — phenomenon | ☐ | ☐ | ☐ |
| 10 | PMID | `20018668` | [link](https://pubmed.ncbi.nlm.nih.gov/20018668/) | KNOWLEDGE.md:172 | - **Verzijlbergen 2010** (PMID 20018668) — method template | ☐ | ☐ | ☐ |
| 11 | PMID | `36076039` | [link](https://pubmed.ncbi.nlm.nih.gov/36076039/) | KNOWLEDGE.md:174 | - **Mahecic 2022** (PMID 36076039) — imaging precedent | ☐ | ☐ | ☐ |
| 12 | PMID | `37882444` | [link](https://pubmed.ncbi.nlm.nih.gov/37882444/) | KNOWLEDGE.md:175 | - **Royall 2023** (PMID 37882444) — most recent human neural progenitor confirmation | ☐ | ☐ | ☐ |
| 13 | PMID | `21407209` | [link](https://pubmed.ncbi.nlm.nih.gov/21407209/) | KNOWLEDGE.md:177 | 2. **Address Januschke 2011 (PMID 21407209) honestly.** Daughter-centriole retention in a different  | ☐ | ☐ | ☐ |
| 14 | PMID | `25228775` | [link](https://pubmed.ncbi.nlm.nih.gov/25228775/) | KNOWLEDGE.md:179 | 3. **Thayer 2014 (PMID 25228775) is the methodological precedent** — only published RITE study speci | ☐ | ☐ | ☐ |
| 15 | PMID | `36639373` | [link](https://pubmed.ncbi.nlm.nih.gov/36639373/) | KNOWLEDGE.md:181 | 4. **AI/ablation anchor:** Mahecic 2022 (PMID 36076039) + Togninalli 2023 (PMID 36639373). Do not ci | ☐ | ☐ | ☐ |

### EpigeneticDrift (24 уникальных, всего вхождений 65)

| # | Тип | Идентификатор | URL | File:Line | Контекст | Real? | Match? | Action |
|---|---|---|---|---|---|---|---|---|
| 1 | PMID | `29643443` | [link](https://pubmed.ncbi.nlm.nih.gov/29643443/) | CONCEPT.md:42 | Aging is characterized by a progressive loss of physiological integrity, driven by the accumulation  | ☐ | ☐ | ☐ |
| 2 | PMID | `24138928` | [link](https://pubmed.ncbi.nlm.nih.gov/24138928/) | CONCEPT.md:42 | Aging is characterized by a progressive loss of physiological integrity, driven by the accumulation  | ☐ | ☐ | ☐ |
| 3 | PMID | `35029144` | [link](https://pubmed.ncbi.nlm.nih.gov/35029144/) | CONCEPT.md:42 | Aging is characterized by a progressive loss of physiological integrity, driven by the accumulation  | ☐ | ☐ | ☐ |
| 4 | PMID | `36206857` | [link](https://pubmed.ncbi.nlm.nih.gov/36206857/) | CONCEPT.md:42 | Aging is characterized by a progressive loss of physiological integrity, driven by the accumulation  | ☐ | ☐ | ☐ |
| 5 | PMID | `30669119` | [link](https://pubmed.ncbi.nlm.nih.gov/30669119/) | CONCEPT.md:52 | * **DNA Methylation:** The most established layer, characterized by hypermethylation at specific CpG | ☐ | ☐ | ☐ |
| 6 | PMID | `37924441` | [link](https://pubmed.ncbi.nlm.nih.gov/37924441/) | CONCEPT.md:53 | * **Chromatin Accessibility and Architecture:** Age-related changes in the opening and closing of ch | ☐ | ☐ | ☐ |
| 7 | PMID | `31085557` | [link](https://pubmed.ncbi.nlm.nih.gov/31085557/) | CONCEPT.md:54 | * **Histone Modification Landscapes:** Drift in the genomic distribution of activating (e.g., H3K4me | ☐ | ☐ | ☐ |
| 8 | PMID | `33571444` | [link](https://pubmed.ncbi.nlm.nih.gov/33571444/) | CONCEPT.md:54 | * **Histone Modification Landscapes:** Drift in the genomic distribution of activating (e.g., H3K4me | ☐ | ☐ | ☐ |
| 9 | PMID | `35858618` | [link](https://pubmed.ncbi.nlm.nih.gov/35858618/) | CONCEPT.md:60 | * **Environmental and Metabolic Insults:** Chronic inflammation is a potent driver of long-term epig | ☐ | ☐ | ☐ |
| 10 | PMID | `37865087` | [link](https://pubmed.ncbi.nlm.nih.gov/37865087/) | CONCEPT.md:60 | * **Environmental and Metabolic Insults:** Chronic inflammation is a potent driver of long-term epig | ☐ | ☐ | ☐ |
| 11 | PMID | `38402617` | [link](https://pubmed.ncbi.nlm.nih.gov/38402617/) | CONCEPT.md:60 | * **Environmental and Metabolic Insults:** Chronic inflammation is a potent driver of long-term epig | ☐ | ☐ | ☐ |
| 12 | PMID | `39271425` | [link](https://pubmed.ncbi.nlm.nih.gov/39271425/) | CONCEPT.md:61 | * **Stem Cell Exhaustion and Lineage Infidelity:** In stem cell compartments, age-associated epigene | ☐ | ☐ | ☐ |
| 13 | PMID | `38640057` | [link](https://pubmed.ncbi.nlm.nih.gov/38640057/) | CONCEPT.md:61 | * **Stem Cell Exhaustion and Lineage Infidelity:** In stem cell compartments, age-associated epigene | ☐ | ☐ | ☐ |
| 14 | PMID | `30048243` | [link](https://pubmed.ncbi.nlm.nih.gov/30048243/) | CONCEPT.md:82 | * **Time-Dominance (β₄, τ₄):** The strong, linear correlation between epigenetic clock values (Horva | ☐ | ☐ | ☐ |
| 15 | PMID | `35032339` | [link](https://pubmed.ncbi.nlm.nih.gov/35032339/) | CONCEPT.md:83 | * **Replication-Associated Drift (α₄, n₄\*):** The link between replicative history and epigenetic a | ☐ | ☐ | ☐ |
| 16 | PMID | `38482631` | [link](https://pubmed.ncbi.nlm.nih.gov/38482631/) | CONCEPT.md:85 | * **Baseline and Measurement:** *D₄,₀* is defined operationally as the epigenetic state at a referen | ☐ | ☐ | ☐ |
| 17 | PMID | `36516495` | [link](https://pubmed.ncbi.nlm.nih.gov/36516495/) | CONCEPT.md:90 | 1. **DNA Methylation Arrays:** The gold standard. Illumina EPIC (850k/935k) arrays provide genome-wi | ☐ | ☐ | ☐ |
| 18 | PMID | `39900648` | [link](https://pubmed.ncbi.nlm.nih.gov/39900648/) | CONCEPT.md:92 | 3. **Composite Biomarkers:** For maximum predictive power for healthspan, *D₄* can be defined as a v | ☐ | ☐ | ☐ |
| 19 | PMID | `34587750` | [link](https://pubmed.ncbi.nlm.nih.gov/34587750/) | CONCEPT.md:92 | 3. **Composite Biomarkers:** For maximum predictive power for healthspan, *D₄* can be defined as a v | ☐ | ☐ | ☐ |
| 20 | PMID | `33844651` | [link](https://pubmed.ncbi.nlm.nih.gov/33844651/) | CONCEPT.md:129 | * **τ₄:** If interventions known to extend healthspan (e.g., caloric restriction, rapamycin) do not  | ☐ | ☐ | ☐ |
| 21 | PMID | `37034474` | [link](https://pubmed.ncbi.nlm.nih.gov/37034474/) | CONCEPT.md:149 | 3. **Stem Cell Specificity vs. Systemic Drift:** To what extent is the epigenetic drift measured in  | ☐ | ☐ | ☐ |
| 22 | PMID | `36336680` | [link](https://pubmed.ncbi.nlm.nih.gov/36336680/) | CONCEPT.md:149 | 3. **Stem Cell Specificity vs. Systemic Drift:** To what extent is the epigenetic drift measured in  | ☐ | ☐ | ☐ |
| 23 | PMID | `41289991` | [link](https://pubmed.ncbi.nlm.nih.gov/41289991/) | CONCEPT.md:150 | 4. **Reversibility Mechanisms:** The observation that epigenetic age can be reversed by lifestyle in | ☐ | ☐ | ☐ |
| 24 | PMID | `38216430` | [link](https://pubmed.ncbi.nlm.nih.gov/38216430/) | CONCEPT.md:182 | 22. Wu Z, et al. Emerging epigenetic insights into aging mechanisms and interventions. *Trends Pharm | ☐ | ☐ | ☐ |

### HAP (1 уникальных, всего вхождений 2)

| # | Тип | Идентификатор | URL | File:Line | Контекст | Real? | Match? | Action |
|---|---|---|---|---|---|---|---|---|
| 1 | DOI | `10.1016/j.cell.2014.03.003` | [link](https://doi.org/10.1016/j.cell.2014.03.003) | CONCEPT.md:381 | 1. Anderson, D.J. & Adolphs, R. (2014). A framework for studying emotions across species. *Cell* 157 | ☐ | ☐ | ☐ |

### MCOA (15 уникальных, всего вхождений 20)

| # | Тип | Идентификатор | URL | File:Line | Контекст | Real? | Match? | Action |
|---|---|---|---|---|---|---|---|---|
| 1 | PMID | `30982602` | [link](https://pubmed.ncbi.nlm.nih.gov/30982602/) | CONCEPT.md:96 | - Γ_{epigenetic, mito} > 0 (Schultz & Sinclair *Cell* 2019, PMID 30982602 — NAD+/sirtuin/aging axis; | ☐ | ☐ | ☐ |
| 2 | PMID | `26833090` | [link](https://pubmed.ncbi.nlm.nih.gov/26833090/) | THEORY.md:69 | * `Γ_{epigenetic, mito} > 0`: Митохондриальные сигналы (NAD+/NADH) влияют на активность эпигенетичес | ☐ | ☐ | ☐ |
| 3 | PMID | `28132843` | [link](https://pubmed.ncbi.nlm.nih.gov/28132843/) | THEORY.md:69 | * `Γ_{epigenetic, mito} > 0`: Митохондриальные сигналы (NAD+/NADH) влияют на активность эпигенетичес | ☐ | ☐ | ☐ |
| 4 | PMID | `29227991` | [link](https://pubmed.ncbi.nlm.nih.gov/29227991/) | EVIDENCE.md:10 | \| Существование нескольких независимых признаков клеточного старения (сенесценции) in vitro. \| 288 | ☐ | ☐ | ☐ |
| 5 | PMID | `29643502` | [link](https://pubmed.ncbi.nlm.nih.gov/29643502/) | EVIDENCE.md:11 | \| Разные типы клеток in vivo стареют с разной скоростью и по разным паттернам молекулярных поврежде | ☐ | ☐ | ☐ |
| 6 | PMID | `16909132` | [link](https://pubmed.ncbi.nlm.nih.gov/16909132/) | EVIDENCE.md:12 | \| Накопление различных видов макромолекулярных повреждений (белки, липиды, ДНК) с возрастом идёт с  | ☐ | ☐ | ☐ |
| 7 | PMID | `30174316` | [link](https://pubmed.ncbi.nlm.nih.gov/30174316/) | EVIDENCE.md:17 | \| Скорость оборота белков широко варьирует между тканями, что может влиять на накопление повреждени | ☐ | ☐ | ☐ |
| 8 | PMID | `33268865` | [link](https://pubmed.ncbi.nlm.nih.gov/33268865/) | EVIDENCE.md:18 | \| Базальный уровень пролиферации клеток сильно различается между тканями, влияя на вклад репликатив | ☐ | ☐ | ☐ |
| 9 | PMID | `12612578` | [link](https://pubmed.ncbi.nlm.nih.gov/12612578/) | EVIDENCE.md:23 | \| Окислительный стресс ускоряет укорочение теломер. \| 12855956 \| Parrinello S. et al. Oxygen sens | ☐ | ☐ | ☐ |
| 10 | PMID | `31844045` | [link](https://pubmed.ncbi.nlm.nih.gov/31844045/) | EVIDENCE.md:25 | \| Эпигенетические изменения могут регулировать экспрессию генов, связанных с функцией центриолей и  | ☐ | ☐ | ☐ |
| 11 | PMID | `2342578` | [link](https://pubmed.ncbi.nlm.nih.gov/2342578/) | PARAMETERS.md:39 | \| `α_Tel` \| Интенсивность укорочения за деление \| 0.02 – 0.04 (соответствует ~50-100 п.н. за деле | ☐ | ☐ | ☐ |
| 12 | PMID | `1631178` | [link](https://pubmed.ncbi.nlm.nih.gov/1631178/) | PARAMETERS.md:39 | \| `α_Tel` \| Интенсивность укорочения за деление \| 0.02 – 0.04 (соответствует ~50-100 п.н. за деле | ☐ | ☐ | ☐ |
| 13 | PMID | `2038241` | [link](https://pubmed.ncbi.nlm.nih.gov/2038241/) | PARAMETERS.md:39 | \| `α_Tel` \| Интенсивность укорочения за деление \| 0.02 – 0.04 (соответствует ~50-100 п.н. за деле | ☐ | ☐ | ☐ |
| 14 | DOI | `10.7554/eLife.73420` | [link](https://doi.org/10.7554/eLife.73420) | PARAMETERS.md:65 | \| `τ_Epi` \| Референтное время (время удвоения эпиг. возраста) \| 3.6 года (1.135e8 с) \| секунды \ | ☐ | ☐ | ☐ |
| 15 | PMID | `12855956` | [link](https://pubmed.ncbi.nlm.nih.gov/12855956/) | PARAMETERS.md:85 | \| **Telomere** \| 0 \| – \| **To be measured** (PMID:12855956 — Parrinello et al *Nat Cell Biol* 20 | ☐ | ☐ | ☐ |

### MitoROS (27 уникальных, всего вхождений 65)

| # | Тип | Идентификатор | URL | File:Line | Контекст | Real? | Match? | Action |
|---|---|---|---|---|---|---|---|---|
| 1 | PMID | `23746838` | [link](https://pubmed.ncbi.nlm.nih.gov/23746838/) | CONCEPT.md:106 | The quest to understand aging has identified several conserved cellular and molecular hallmarks, inc | ☐ | ☐ | ☐ |
| 2 | PMID | `37196864` | [link](https://pubmed.ncbi.nlm.nih.gov/37196864/) | CONCEPT.md:106 | The quest to understand aging has identified several conserved cellular and molecular hallmarks, inc | ☐ | ☐ | ☐ |
| 3 | PMID | `19732859` | [link](https://pubmed.ncbi.nlm.nih.gov/19732859/) | CONCEPT.md:106 | The quest to understand aging has identified several conserved cellular and molecular hallmarks, inc | ☐ | ☐ | ☐ |
| 4 | PMID | `37172915` | [link](https://pubmed.ncbi.nlm.nih.gov/37172915/) | CONCEPT.md:106 | The quest to understand aging has identified several conserved cellular and molecular hallmarks, inc | ☐ | ☐ | ☐ |
| 5 | PMID | `40183670` | [link](https://pubmed.ncbi.nlm.nih.gov/40183670/) | CONCEPT.md:110 | This paper defines and formalizes "Mitochondrial ROS and mtDNA Damage" as MCOA Counter #3. We move b | ☐ | ☐ | ☐ |
| 6 | PMID | `36442091` | [link](https://pubmed.ncbi.nlm.nih.gov/36442091/) | CONCEPT.md:110 | This paper defines and formalizes "Mitochondrial ROS and mtDNA Damage" as MCOA Counter #3. We move b | ☐ | ☐ | ☐ |
| 7 | PMID | `39179117` | [link](https://pubmed.ncbi.nlm.nih.gov/39179117/) | CONCEPT.md:110 | This paper defines and formalizes "Mitochondrial ROS and mtDNA Damage" as MCOA Counter #3. We move b | ☐ | ☐ | ☐ |
| 8 | PMID | `1485738` | [link](https://pubmed.ncbi.nlm.nih.gov/1485738/) | CONCEPT.md:136 | The damage variable \( D_3 \) integrates two major components: 1) Oxidative lesions to mtDNA (like 8 | ☐ | ☐ | ☐ |
| 9 | PMID | `25149213` | [link](https://pubmed.ncbi.nlm.nih.gov/25149213/) | CONCEPT.md:136 | The damage variable \( D_3 \) integrates two major components: 1) Oxidative lesions to mtDNA (like 8 | ☐ | ☐ | ☐ |
| 10 | PMID | `40239706` | [link](https://pubmed.ncbi.nlm.nih.gov/40239706/) | CONCEPT.md:139 | In mitotically active tissues (e.g., intestinal crypts, hematopoietic stem cells), mtDNA replication | ☐ | ☐ | ☐ |
| 11 | PMID | `26281784` | [link](https://pubmed.ncbi.nlm.nih.gov/26281784/) | CONCEPT.md:139 | In mitotically active tissues (e.g., intestinal crypts, hematopoietic stem cells), mtDNA replication | ☐ | ☐ | ☐ |
| 12 | PMID | `30043489` | [link](https://pubmed.ncbi.nlm.nih.gov/30043489/) | CONCEPT.md:142 | This is the dominant term for most tissues. Time-dependent accumulation of mtDNA deletions and point | ☐ | ☐ | ☐ |
| 13 | PMID | `40579478` | [link](https://pubmed.ncbi.nlm.nih.gov/40579478/) | CONCEPT.md:142 | This is the dominant term for most tissues. Time-dependent accumulation of mtDNA deletions and point | ☐ | ☐ | ☐ |
| 14 | PMID | `17090418` | [link](https://pubmed.ncbi.nlm.nih.gov/17090418/) | CONCEPT.md:142 | This is the dominant term for most tissues. Time-dependent accumulation of mtDNA deletions and point | ☐ | ☐ | ☐ |
| 15 | PMID | `30089816` | [link](https://pubmed.ncbi.nlm.nih.gov/30089816/) | CONCEPT.md:149 | 1. **mtDNA Heteroplasmy:** Digital droplet PCR (ddPCR) or deep sequencing for specific point mutatio | ☐ | ☐ | ☐ |
| 16 | PMID | `38724734` | [link](https://pubmed.ncbi.nlm.nih.gov/38724734/) | CONCEPT.md:240 | The MCOA formalism helps reconcile seemingly conflicting data. For instance, the finding that mtDNA  | ☐ | ☐ | ☐ |
| 17 | PMID | `39684855` | [link](https://pubmed.ncbi.nlm.nih.gov/39684855/) | CONCEPT.md:240 | The MCOA formalism helps reconcile seemingly conflicting data. For instance, the finding that mtDNA  | ☐ | ☐ | ☐ |
| 18 | PMID | `39173633` | [link](https://pubmed.ncbi.nlm.nih.gov/39173633/) | CONCEPT.md:270 | * **Γ₃,₄ (Epigenetic Drift → Mito):** **Quantitative link proposed.** Hahn et al. (2024, PMID: 39173 | ☐ | ☐ | ☐ |
| 19 | PMID | `39343182` | [link](https://pubmed.ncbi.nlm.nih.gov/39343182/) | CONCEPT.md:271 | * **Γ₃,₅ (Proteostasis → Mito):** **Measurement pending ~~MCOA Test 2~~ [отозвано — see CORRECTIONS  | ☐ | ☐ | ☐ |
| 20 | PMID | `40461459` | [link](https://pubmed.ncbi.nlm.nih.gov/40461459/) | CONCEPT.md:271 | * **Γ₃,₅ (Proteostasis → Mito):** **Measurement pending ~~MCOA Test 2~~ [отозвано — see CORRECTIONS  | ☐ | ☐ | ☐ |
| 21 | PMID | `39019845` | [link](https://pubmed.ncbi.nlm.nih.gov/39019845/) | CONCEPT.md:327 | The function \( f_3 \) is a non-linear mapping from damage \( D_3 \) to functional loss. It is expec | ☐ | ☐ | ☐ |
| 22 | PMID | `40500258` | [link](https://pubmed.ncbi.nlm.nih.gov/40500258/) | CONCEPT.md:327 | The function \( f_3 \) is a non-linear mapping from damage \( D_3 \) to functional loss. It is expec | ☐ | ☐ | ☐ |
| 23 | PMID | `39933528` | [link](https://pubmed.ncbi.nlm.nih.gov/39933528/) | CONCEPT.md:346 | 1. Cefis M, et al. (2025). Impact of physical activity on physical function, mitochondrial energetic | ☐ | ☐ | ☐ |
| 24 | PMID | `40476552` | [link](https://pubmed.ncbi.nlm.nih.gov/40476552/) | CONCEPT.md:353 | 8. Kobayashi H (2025). Understanding the impact of mitochondrial DNA mutations on aging and carcinog | ☐ | ☐ | ☐ |
| 25 | PMID | `36233264` | [link](https://pubmed.ncbi.nlm.nih.gov/36233264/) | CONCEPT.md:363 | 18. Wang HH, et al. (2022). Nobiletin Prevents D-Galactose-Induced C2C12 Cell Aging by Improving Mit | ☐ | ☐ | ☐ |
| 26 | PMID | `30593894` | [link](https://pubmed.ncbi.nlm.nih.gov/30593894/) | CONCEPT.md:364 | 19. Wang Y, et al. (2019). Mitochondrial regulation of cardiac aging. *Biochim Biophys Acta Mol Basi | ☐ | ☐ | ☐ |
| 27 | PMID | `16868022` | [link](https://pubmed.ncbi.nlm.nih.gov/16868022/) | PARAMETERS.md:32 | **Current status:** Hypothetical (requires experimental validation). **Estimated value:** τ₃ ≈ 0.1–0 | ☐ | ☐ | ☐ |

### Proteostasis (22 уникальных, всего вхождений 48)

| # | Тип | Идентификатор | URL | File:Line | Контекст | Real? | Match? | Action |
|---|---|---|---|---|---|---|---|---|
| 1 | PMID | `29127110` | [link](https://pubmed.ncbi.nlm.nih.gov/29127110/) | CONCEPT.md:152 | Aging is driven by the progressive accumulation of cellular and molecular damage. Among the proposed | ☐ | ☐ | ☐ |
| 2 | PMID | `34563704` | [link](https://pubmed.ncbi.nlm.nih.gov/34563704/) | CONCEPT.md:152 | Aging is driven by the progressive accumulation of cellular and molecular damage. Among the proposed | ☐ | ☐ | ☐ |
| 3 | PMID | `39973488` | [link](https://pubmed.ncbi.nlm.nih.gov/39973488/) | CONCEPT.md:152 | Aging is driven by the progressive accumulation of cellular and molecular damage. Among the proposed | ☐ | ☐ | ☐ |
| 4 | PMID | `37111020` | [link](https://pubmed.ncbi.nlm.nih.gov/37111020/) | CONCEPT.md:152 | Aging is driven by the progressive accumulation of cellular and molecular damage. Among the proposed | ☐ | ☐ | ☐ |
| 5 | PMID | `26738589` | [link](https://pubmed.ncbi.nlm.nih.gov/26738589/) | CONCEPT.md:180 | 1. **Replication-Associated Dilution (n-linked term, α₅ · (n / n₅*))**: In proliferating cells (e.g. | ☐ | ☐ | ☐ |
| 6 | PMID | `33891876` | [link](https://pubmed.ncbi.nlm.nih.gov/33891876/) | CONCEPT.md:182 | 2. **Time-Dependent Decay and Accumulation (t-linked term, β₅ · (t / τ₅))**: In post-mitotic cells ( | ☐ | ☐ | ☐ |
| 7 | PMID | `40960157` | [link](https://pubmed.ncbi.nlm.nih.gov/40960157/) | CONCEPT.md:182 | 2. **Time-Dependent Decay and Accumulation (t-linked term, β₅ · (t / τ₅))**: In post-mitotic cells ( | ☐ | ☐ | ☐ |
| 8 | PMID | `35447272` | [link](https://pubmed.ncbi.nlm.nih.gov/35447272/) | CONCEPT.md:182 | 2. **Time-Dependent Decay and Accumulation (t-linked term, β₅ · (t / τ₅))**: In post-mitotic cells ( | ☐ | ☐ | ☐ |
| 9 | PMID | `40098057` | [link](https://pubmed.ncbi.nlm.nih.gov/40098057/) | CONCEPT.md:189 | * **τ₅ (Characteristic Aggregation Time Constant)**: The slow, age-dependent accumulation of aggrega | ☐ | ☐ | ☐ |
| 10 | PMID | `38347288` | [link](https://pubmed.ncbi.nlm.nih.gov/38347288/) | CONCEPT.md:192 | * **Low α₅ / High β₅**: Expected in post-mitotic tissues like neurons and muscle fibers. The accumul | ☐ | ☐ | ☐ |
| 11 | PMID | `41340001` | [link](https://pubmed.ncbi.nlm.nih.gov/41340001/) | CONCEPT.md:192 | * **Low α₅ / High β₅**: Expected in post-mitotic tissues like neurons and muscle fibers. The accumul | ☐ | ☐ | ☐ |
| 12 | PMID | `39627772` | [link](https://pubmed.ncbi.nlm.nih.gov/39627772/) | CONCEPT.md:194 | * **D₅,₀ (Baseline Damage)**: Genetic predispositions or early-life insults can set a higher baselin | ☐ | ☐ | ☐ |
| 13 | PMID | `40042672` | [link](https://pubmed.ncbi.nlm.nih.gov/40042672/) | CONCEPT.md:202 | * *Ex vivo/Postmortem*: Immunohistochemistry for co-localized Aβ, tau, and α-synuclein (Sengupta 202 | ☐ | ☐ | ☐ |
| 14 | PMID | `37315555` | [link](https://pubmed.ncbi.nlm.nih.gov/37315555/) | CONCEPT.md:206 | * **Chaperone Levels**: Western blot or proteomics for HSP70, HSP90, BAG3 (Sheehan 2023, PMID: 37315 | ☐ | ☐ | ☐ |
| 15 | PMID | `40377064` | [link](https://pubmed.ncbi.nlm.nih.gov/40377064/) | CONCEPT.md:232 | * **Γ₅₃ (Mitochondrial ROS/Dysfunction → Proteostasis)**: **Likely > 0**. Mitochondrial dysfunction  | ☐ | ☐ | ☐ |
| 16 | PMID | `40388671` | [link](https://pubmed.ncbi.nlm.nih.gov/40388671/) | CONCEPT.md:233 | * **Γ₅₄ (Epigenetic Drift → Proteostasis)**: **Likely > 0**. Epigenetic changes regulate the express | ☐ | ☐ | ☐ |
| 17 | PMID | `38049031` | [link](https://pubmed.ncbi.nlm.nih.gov/38049031/) | CONCEPT.md:233 | * **Γ₅₄ (Epigenetic Drift → Proteostasis)**: **Likely > 0**. Epigenetic changes regulate the express | ☐ | ☐ | ☐ |
| 18 | PMID | `28170377` | [link](https://pubmed.ncbi.nlm.nih.gov/28170377/) | CONCEPT.md:234 | * **Γ₅₅ (Autocatalysis)**: **> 0**. Aggregates themselves can disrupt proteostasis by sequestering c | ☐ | ☐ | ☐ |
| 19 | DOI | `10.1016/j.cell.2012.04.037` | [link](https://doi.org/10.1016/j.cell.2012.04.037) | EVIDENCE.md:29 | \| Разведение клеточных компонентов при делении — фундаментальный биологический принцип. \| 10.1016/ | ☐ | ☐ | ☐ |
| 20 | DOI | `10.1016/j.bbrc.2015.01.046` | [link](https://doi.org/10.1016/j.bbrc.2015.01.046) | EVIDENCE.md:63 | \| Долгоживущие виды имеют улучшенный протеостаз по сравнению с филогенетически близкими короткоживу | ☐ | ☐ | ☐ |
| 21 | PMID | `25615820` | [link](https://pubmed.ncbi.nlm.nih.gov/25615820/) | EVIDENCE.md:63 | \| Долгоживущие виды имеют улучшенный протеостаз по сравнению с филогенетически близкими короткоживу | ☐ | ☐ | ☐ |
| 22 | DOI | `10.1038/s43587-021-00098-4` | [link](https://doi.org/10.1038/s43587-021-00098-4) | EVIDENCE.md:64 | \| Усиление аутофагии не всегда продлевает жизнь в модельных организмах. \| 10.1038/s43587-021-00098 | ☐ | ☐ | ☐ |

### Telomere (21 уникальных, всего вхождений 78)

| # | Тип | Идентификатор | URL | File:Line | Контекст | Real? | Match? | Action |
|---|---|---|---|---|---|---|---|---|
| 1 | PMID | `24374808` | [link](https://pubmed.ncbi.nlm.nih.gov/24374808/) | CONCEPT.md:31 | 1. **The End-Replication Problem:** DNA polymerase cannot fully replicate the 3' ends of linear chro | ☐ | ☐ | ☐ |
| 2 | PMID | `30650660` | [link](https://pubmed.ncbi.nlm.nih.gov/30650660/) | CONCEPT.md:31 | 1. **The End-Replication Problem:** DNA polymerase cannot fully replicate the 3' ends of linear chro | ☐ | ☐ | ☐ |
| 3 | PMID | `39837827` | [link](https://pubmed.ncbi.nlm.nih.gov/39837827/) | CONCEPT.md:32 | 2. **Oxidative Stress-Induced Erosion:** Telomeric DNA, particularly the G-rich 3' overhang, is high | ☐ | ☐ | ☐ |
| 4 | PMID | `11001793` | [link](https://pubmed.ncbi.nlm.nih.gov/11001793/) | CONCEPT.md:32 | 2. **Oxidative Stress-Induced Erosion:** Telomeric DNA, particularly the G-rich 3' overhang, is high | ☐ | ☐ | ☐ |
| 5 | PMID | `28431907` | [link](https://pubmed.ncbi.nlm.nih.gov/28431907/) | CONCEPT.md:32 | 2. **Oxidative Stress-Induced Erosion:** Telomeric DNA, particularly the G-rich 3' overhang, is high | ☐ | ☐ | ☐ |
| 6 | PMID | `34736994` | [link](https://pubmed.ncbi.nlm.nih.gov/34736994/) | CONCEPT.md:33 | 3. **Other Stressors:** Psychological stress, inflammation, and mitochondrial dysfunction (via ROS p | ☐ | ☐ | ☐ |
| 7 | PMID | `34200513` | [link](https://pubmed.ncbi.nlm.nih.gov/34200513/) | CONCEPT.md:33 | 3. **Other Stressors:** Psychological stress, inflammation, and mitochondrial dysfunction (via ROS p | ☐ | ☐ | ☐ |
| 8 | PMID | `25612739` | [link](https://pubmed.ncbi.nlm.nih.gov/25612739/) | CONCEPT.md:33 | 3. **Other Stressors:** Psychological stress, inflammation, and mitochondrial dysfunction (via ROS p | ☐ | ☐ | ☐ |
| 9 | PMID | `40215293` | [link](https://pubmed.ncbi.nlm.nih.gov/40215293/) | CONCEPT.md:36 | * **Shelterin Complex:** The six-protein shelterin complex (TRF1, TRF2, POT1, TIN2, TPP1, RAP1) caps | ☐ | ☐ | ☐ |
| 10 | PMID | `39164231` | [link](https://pubmed.ncbi.nlm.nih.gov/39164231/) | CONCEPT.md:37 | * **Telomerase and ALT:** The ribonucleoprotein telomerase (TERT + TERC) can add telomeric repeats d | ☐ | ☐ | ☐ |
| 11 | PMID | `30229407` | [link](https://pubmed.ncbi.nlm.nih.gov/30229407/) | CONCEPT.md:40 | **Triggering Senescence:** Critically short or structurally uncapped telomeres are recognized as per | ☐ | ☐ | ☐ |
| 12 | PMID | `38634789` | [link](https://pubmed.ncbi.nlm.nih.gov/38634789/) | CONCEPT.md:40 | **Triggering Senescence:** Critically short or structurally uncapped telomeres are recognized as per | ☐ | ☐ | ☐ |
| 13 | PMID | `38581556` | [link](https://pubmed.ncbi.nlm.nih.gov/38581556/) | CONCEPT.md:60 | * **Evidence:** Defined by the classic Hayflick limit. Modulation by oxygen tension (20% vs. physiol | ☐ | ☐ | ☐ |
| 14 | PMID | `30472697` | [link](https://pubmed.ncbi.nlm.nih.gov/30472697/) | CONCEPT.md:62 | * **Empirical Basis:** Observable in vivo in post-mitotic or slowly dividing tissues. For example, t | ☐ | ☐ | ☐ |
| 15 | PMID | `33347069` | [link](https://pubmed.ncbi.nlm.nih.gov/33347069/) | CONCEPT.md:66 | * **Empirical Constraint:** Not directly measured in meta-analyses. However, data on rapid telomere  | ☐ | ☐ | ☐ |
| 16 | PMID | `22773427` | [link](https://pubmed.ncbi.nlm.nih.gov/22773427/) | CONCEPT.md:74 | * **Quantitative PCR (qPCR) T/S Ratio:** A high-throughput method for estimating average relative te | ☐ | ☐ | ☐ |
| 17 | PMID | `37917279` | [link](https://pubmed.ncbi.nlm.nih.gov/37917279/) | CONCEPT.md:87 | * **Quantitative Estimate:** The work of De Rosa et al. (2025, PMID: 39837827) provides a mechanisti | ☐ | ☐ | ☐ |
| 18 | PMID | `13718526` | [link](https://pubmed.ncbi.nlm.nih.gov/13718526/) | CONCEPT.md:207 | Effect size: expected n₂* = 50 PD vs null n₂* ≥ 60 PD, SD = 10 PD (from PMID:13718526). One-sided t- | ☐ | ☐ | ☐ |
| 19 | DOI | `10.1016/j.cell.2019.07.034` | [link](https://doi.org/10.1016/j.cell.2019.07.034) | EVIDENCE.md:12 | \| Нетеломеразные соматические клетки укорачиваются на ~100 bp/PD. \| 10.1016/j.cell.2019.07.034 \|  | ☐ | ☐ | ☐ |
| 20 | PMID | `25607366` | [link](https://pubmed.ncbi.nlm.nih.gov/25607366/) | PARAMETERS.md:50 | \| β₂ (time-dependent erosion rate) \| 10–50 bp/year \| SD ≈ 15 bp/year (from longitudinal cohort da | ☐ | ☐ | ☐ |
| 21 | PMID | `17938250` | [link](https://pubmed.ncbi.nlm.nih.gov/17938250/) | PARAMETERS.md:51 | \| n₂* (Hayflick limit) \| 40–60 PD \| 95% CI: [38, 62] PD (from fibroblast studies, PMID:17938250)  | ☐ | ☐ | ☐ |

### UMBRELLA (13 уникальных, всего вхождений 15)

| # | Тип | Идентификатор | URL | File:Line | Контекст | Real? | Match? | Action |
|---|---|---|---|---|---|---|---|---|
| 1 | PMID | `16060722` | [link](https://pubmed.ncbi.nlm.nih.gov/16060722/) | CONCEPT.md:25 | - Все reported AUC (0.81 на All-of-Us N=2222 для PhenoAge acceleration) — exploratory с явным p-hack | ☐ | ☐ | ☐ |
| 2 | arXiv | `1502.00214` | [link](https://arxiv.org/abs/1502.00214) | THEORY.md:56 | \| Burgholzer information-entropy equality `I = ⟨ΔS⟩_gen` \| `Ze/THEORY.md §2` \| Theorem (Burgholze | ☐ | ☐ | ☐ |
| 3 | DOI | `10.1162/089976601753195969` | [link](https://doi.org/10.1162/089976601753195969) | EVIDENCE.md:25 | \| Predictive information (binary Markov) \| Bialek/Nemenman/Tishby 2001 *Neural Computation* 13 \|  | ☐ | ☐ | ☐ |
| 4 | DOI | `10.1103/PhysRevX.11.021029` | [link](https://doi.org/10.1103/PhysRevX.11.021029) | EVIDENCE.md:31 | \| Pearson nanoscale clock thermodynamic cost \| Pearson et al. 2021 *Phys Rev X* 11 \| DOI **10.110 | ☐ | ☐ | ☐ |
| 5 | DOI | `10.1142/S0219887821501279` | [link](https://doi.org/10.1142/S0219887821501279) | EVIDENCE.md:32 | \| Information geometry / minimal entropy paths \| Gassner et al. 2021 *Int J Geom Methods Mod Phys* | ☐ | ☐ | ☐ |
| 6 | DOI | `10.22331/q-2021-04-26-443` | [link](https://doi.org/10.22331/q-2021-04-26-443) | EVIDENCE.md:34 | \| Asymmetric CHSH \| Woodhead/Acín/Pironio 2021 *Quantum* 5 \| DOI **10.22331/q-2021-04-26-443** ✅  | ☐ | ☐ | ☐ |
| 7 | DOI | `10.1126/sciadv.aaw9832` | [link](https://doi.org/10.1126/sciadv.aaw9832) | EVIDENCE.md:35 | \| Wigner's friend experiment \| Proietti et al. 2019 *Sci Adv* 5 \| DOI **10.1126/sciadv.aaw9832**  | ☐ | ☐ | ☐ |
| 8 | DOI | `10.1103/RevModPhys.75.715` | [link](https://doi.org/10.1103/RevModPhys.75.715) | EVIDENCE.md:36 | \| Decoherence / einselection \| Zurek 2003 *Rev Mod Phys* 75 \| DOI **10.1103/RevModPhys.75.715** ✅ | ☐ | ☐ | ☐ |
| 9 | DOI | `10.1103/PhysRevA.72.042113` | [link](https://doi.org/10.1103/PhysRevA.72.042113) | EVIDENCE.md:37 | \| Quantum Darwinism redundant encoding \| Ollivier/Poulin/Zurek 2005 *Phys Rev A* 72 \| DOI **10.11 | ☐ | ☐ | ☐ |
| 10 | PMID | `30982602` | [link](https://pubmed.ncbi.nlm.nih.gov/30982602/) | EVIDENCE.md:50 | \| ~~Schultz & Sinclair 2019 PMID 30982602 (NAD+/sirtuin)~~ \| **Miscitation** — PMID 30982602 = Kuc | ☐ | ☐ | ☐ |
| 11 | PMID | `38510429` | [link](https://pubmed.ncbi.nlm.nih.gov/38510429/) | EVIDENCE.md:53 | \| ~~Tkemaladze 2024 Editorial PMID 38510429~~ \| Removed to bring self-citation < 10% \| | ☐ | ☐ | ☐ |
| 12 | PMID | `15886028` | [link](https://pubmed.ncbi.nlm.nih.gov/15886028/) | EVIDENCE.md:60 | - Tkemaladze 2005 *Cell Biol Int* 29 (PMID 15886028) — early centriole work | ☐ | ☐ | ☐ |
| 13 | PMID | `36583780` | [link](https://pubmed.ncbi.nlm.nih.gov/36583780/) | EVIDENCE.md:61 | - Tkemaladze 2023 *Mol Biol Rep* 50 (PMID 36583780) — CDATA flagship | ☐ | ☐ | ☐ |

### Ze (3 уникальных, всего вхождений 6)

| # | Тип | Идентификатор | URL | File:Line | Контекст | Real? | Match? | Action |
|---|---|---|---|---|---|---|---|---|
| 1 | PMID | `27330520` | [link](https://pubmed.ncbi.nlm.nih.gov/27330520/) | CONCEPT.md:254 | N_ICC ≥ 50 субъектов с 2 визитами (Koo & Mae 2016, PMID 27330520) | ☐ | ☐ | ☐ |
| 2 | DOI | `10.64898/2026.03.26.714124` | [link](https://doi.org/10.64898/2026.03.26.714124) | CONCEPT.md:310 | **Прямой конкурент:** BrainYears (Lore, Julihn, Telfer, bioRxiv, март 2026) — EEG brain age clock, r | ☐ | ☐ | ☐ |
| 3 | PMID | `36583780` | [link](https://pubmed.ncbi.nlm.nih.gov/36583780/) | CONCEPT.md:657 | 2. Tkemaladze, J. (2023). Reduction, proliferation, and differentiation defects of stem cells over t | ☐ | ☐ | ☐ |

---

**Что делать после проверки:**
- Невалидные идентификаторы (Real? = ✗) → удалить или заменить.
- Несоответствующие (Match? = ✗) → переписать утверждение или заменить ссылку.
- Сохранить эту таблицу с вашими отметками в `REFERENCES_TO_VERIFY.checked.md`.
- Применить правки → запустить ещё один проход loop_serial.py.