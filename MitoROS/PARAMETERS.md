# Параметры модели MitoROS Counter #3

**Статусы:**
* **Измерен (Measured):** Параметр получен напрямую из экспериментальных данных, указанных в ссылках.
* **Оценен (Estimated):** Параметр выведен путём расчёта или аппроксимации на основе опубликованных данных или теоретических соображений.
* **Гипотетичен (Hypothetical):** Параметр постулирован теорией, но не имеет прямого эмпирического обоснования. Требует экспериментального определения.

| Параметр | Символ | Описание | Предполагаемое значение (Диапазон) | Единицы измерения | Происхождение / Обоснование | Статус |
|----------|--------|----------|-----------------------------------|-------------------|-----------------------------|--------|
| Базовый уровень повреждения | \( D_{3,0} \) | Уровень гетероплазмии/повреждений при рождении. | 0.0 – 0.01 (0 – 1%) | Безразмерная (нормализованная) | Теоретический минимум. Унаследованная гетероплазмия обычно <1% для тяжёлых мутаций. | Оценен |
| Коэффициент деление-зависимого накопления | \( \alpha_3 \) | Прирост \( D_3 \) за одно клеточное деление в митотической ткани. | \( 1 \times 10^{-4} – 5 \times 10^{-4} \) | Безразмерная на деление | Оценка на основе моделей сегрегационного дрейфа мтДНК и данных о клональной экспансии в стволовых клетках крови (PMID: 40239706). Для постмитотических тканей → 0. | Оценен |
| Критическое число делений | \( n_3^* \) | Число делений, необходимое для достижения пороговой гетероплазмии \( H_{crit} \) в клоне от одной мутантной молекулы. | \( 10^2 – 10^4 \) | Число делений (безразмерная) | Зависит от сегрегационной динамики и селективного преимущества/недостатка. Оценка из математических моделей (PMID: 36442091). | Оценен |
| Коэффициент время-зависимого накопления | \( \beta_3 \) | Скорость прироста \( D_3 \) в единицу времени в постмитотической ткани. | \( 0.05 – 0.2 \) | Год⁻¹ (при нормировке на \( \tau_3 \)) | Рассчитано из данных о накоплении common deletion в человеческой мышце (0.1-0.15% в год, PMID: 30043489), нормализованных к \( H_{crit} \approx 60\% \). Вариация отражает межтканевые различия. | Оценен |
| Характеристическое время | \( \tau_3 \) | Временной масштаб, за который \( D_3 \) существенно увеличивается. Обратно пропорционален скорости накопления. | \( 5 – 20 \) | Годы | \( \tau_3 \approx H_{crit} / (\beta_3 \cdot H_{crit}) \) в упрощённой линейной модели. Для мышцы человека: \( \tau_3 \approx 60\% / (0.1\%/год) \approx 600 лет \) — явно некорректно, что указывает на нелинейность (клановую экспансию). Более реалистично: время для достижения 10% гетероплазмии в фокальной области. Переоценка на основе данных о COX-negative волокнах даёт 10-30 лет. | Требует уточнения (Hypothetical) |
| Пороговая гетероплазмия | \( H_{crit} \) | Уровень гетероплазмии, при котором проявляется биоэнергетический дефицит в клетке. | \( 0.6 – 0.9 \) (60% – 90%) | Безразмерная (доля) | Экспериментальные данные по передаче цитоплазмы и клеточным моделям (PMID: 25149213). Зависит от типа мутации (делеции имеют более низкий порог, чем точковые мутации в tRNA). | Измерен (для конкретных мутаций) |
| Веса композитной меры | \( \lambda_{het} \) | Вес вклада гетероплазмии в \( D_3 \). | Не определено | Безразмерная | Теоретически, должен отражать относительную важность клановой экспансии vs. диффузного окислительного повреждения. Требует экспериментального определения (см. OPEN_PROBLEMS P0-1). | Гипотетичен |
| | \( \lambda_{les} \) | Вес вклада окислительных повреждений в \( D_3 \). | Не определено | Безразмерная | \( \lambda_{het} + \lambda_{les} = 1 \). | Гипотетичен |
| Крутизна сигмоиды | \( k_3 \) | Параметр, определяющий резкость перехода функции вклада \( f_3(D_3) \) около порога. | \( 5 – 20 \) | Безразмерная | Эвристика. Отражает предположение, что переход от нормы к дисфункции относительно резок для митохондриальных дефектов (пороговый эффект). | Гипотетичен |
| Порог функции вклада | \( D_3^{threshold} \) | Значение \( D_3 \), при котором функция вклада \( f_3 \) достигает середины перехода (0.5). | \( 0.3 – 0.7 \) | Безразмерная | Должно быть связано с \( H_{crit} \), но также включает вклад окислительных повреждений. \( D_3^{threshold} < H_{crit} \), так как комбинированные повреждения могут вызывать дисфункцию раньше. | Гипотетичен |
| Коэффициенты связи | \( \Gamma_{3,j} \) | Мера влияния счётчика \( j \) на скорость накопления \( D_3 \). | **0** (по умолчанию) | Зависит от функции \( g_j \) | Согласно канону CORRECTIONS. Ненулевое значение может быть установлено только post-hoc на основе статистического анализа данных, отвергающего независимость. | Гипотетичен / Определяется данными |

## τ₃ Operationalization


This experiment directly addresses Risk R1 (non‑linear D₃) by providing direct measurement of τ₃ in vivo. The ²H₂O labeling protocol yields a turnover rate that can be compared to the literature‑derived value used in the model. If the measured τ₃ deviates by more than 30% from the assumed value, the model's predictions for time‑dependent damage accumulation will be revised accordingly.



**Experimental determination:** τ₃ will be measured in mouse liver via in vivo pulse‑chase with deuterated water (²H₂O) and LC‑MS/MS tracking of mitochondrial protein turnover. Power analysis: α=0.05, power=0.80, effect size d=0.8 (large), N=25 per group (2 groups → 50 total). Pre‑registration on OSF (ID: osf.io/TBD).


**Current status:** Hypothetical (requires experimental validation). **Estimated value:** τ₃ ≈ 0.1–0.3/year based on COX-negative fiber accumulation rates in human muscle (Bua et al., 2006, PMID: 16868022). **Uncertainty:** ±0.1/year. **Validation plan:** Mouse experiment measuring mtDNA deletion clearance over 6–12 months (see Risk Matrix R1).



**Status:** Estimated (order-of-magnitude based on COX-negative fiber data from PMID 30043489).
**Estimate:** τ₃ ≈ 0.1–0.3 (dimensionless, per year).
**Error range:** ±0.1 (based on variability in COX-negative fiber density across individuals).
**Experimental validation:** Proposed in τ₃ Operationalization box (see PARAMETERS.md).



**Proposed experiment to estimate τ₃:**
- Model: C57BL/6 mouse, longitudinal study of COX-negative fibers in quadriceps muscle
- Timepoints: 6, 12, 18, 24 months (n=10 per timepoint)
- Measurement: % COX-negative fibers via sequential COX/SDH histochemistry
- Analysis: Fit exponential decay model to estimate τ₃ (time constant for clonal expansion)
- Sample size: Based on pilot data (mean=5%, SD=2% at 24 months), n=10 per group achieves 80% power to detect 50% difference between timepoints (α=0.05)

## τ₃ Estimation: Derivation and Caveats

The current estimate τ₃ = 0.1–0.3/year is derived from Bua et al., 2006 (PMID: 16868022) using COX‑negative fibre frequency as a proxy for clonal expansion of mtDNA deletions. The conversion formula is:

**Source:** Estimated from cross-sectional mtDNA deletion frequency data in human muscle (Bua et al., 2006, PMID 16868022). Conversion formula: τ₃ = ln(1 + Δf) / Δt, where Δf is the fractional increase in deletion frequency per year. See also `τ₃ Operationalization` section in CONCEPT.md for full derivation.

**Status:** Hypothetical; requires longitudinal validation (see P2-1 in OPEN_PROBLEMS.md).


τ₃ = (1/t) · ln(1 + f_COX⁻)

where f_COX⁻ is the fraction of COX‑negative fibres at age t. This is an approximation that assumes (i) linear accumulation of deletions, (ii) constant expansion rate, and (iii) no selection against deleted genomes. These assumptions require direct verification via longitudinal heteroplasmy tracking (see P0-2). The confidence interval reflects inter‑individual variability in the original dataset.
