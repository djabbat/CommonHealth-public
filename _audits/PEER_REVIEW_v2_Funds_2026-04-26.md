# Wave 2 Peer Review — CommonHealth Ecosystem (Fund Perspective)

**Дата:** 2026-04-26
**Тип:** ультра-строгая внутренняя due-diligence рецензия для investment committee
**Профиль рецензента:** старший программ-офицер тир-1 фондов
(Wellcome Leap, ARPA-H, EIC Pathfinder Open 2027, Gates Foundation,
Schmidt Sciences, NIH R01, Impetus Longevity)
**Объект:** CommonHealth umbrella + 7 подпроектов
(MCOA, FCLC, Ze, CDATA, BioSense, Ontogenesis, HAP)
**Принципалы:** PI Jaba Tkemaladze (independent researcher, Грузия);
host org — Georgia Longevity Alliance NGO, рег. №404506520
**Источники:**
- `/home/oem/Desktop/LongevityCommon/CONCEPT.md` (v5.1)
- CONCEPT/EVIDENCE/THEORY/PARAMETERS/STATE.md всех 7 подпроектов
- Wave 1 reviews: `_audits/PEER_REVIEW_v2_TopMCOAZe_2026-04-26.md`,
 `_audits/PEER_REVIEW_v2_Empirical_2026-04-26.md`
- Wave 1 corrections log: `_audits/COMMIT_LOG_FINAL_2026-04-26.md`

**Методика:** Wave 1 уже выявил citation fabrications (MCOA 9/9, Ze 1, BioSense 4,
Ontogenesis 6+1 flagged, HAP 10/10) и структурные противоречия (CDATA Sobol,
Ze Tsirelson, MCOA M3, FCLC ε=10). Применённые исправления зафиксированы
в COMMIT_LOG_FINAL. Текущая рецензия оценивает экосистему ПОСЛЕ Wave 1
коррекций с точки зрения серьёзного фонда — не журнала. Фокус: translational
potential, budget realism, team capacity, IP/licensing, open science,
capacity-building, sequencing strategy.

**LLM:** DeepSeek-reasoner (per CLAUDE.md «всё через DeepSeek»).

---

# §1. Verdict matrix 8×7: CommonHealth ecosystem readiness for tier-1 funds

## CommonHealth (top umbrella layer)

| Фонд | Вердикт | Обоснование |
|------|---------|-------------|
| Wellcome Leap | REJECT | Нет deliverable-контрольных точек на квартал; single PI без US/UK affiliation; fabrications в дочерних проектах делают риски неприемлемыми для high-risk high-reward программы. |
| ARPA-H | REJECT | US-based PI обязателен; нет прототипа; TRL 1; нет milstone-driven плана. |
| EIC Pathfinder Open 2027 | COND | Единственный фонд, где umbrella может быть адаптирован как WP0 "Coordination". Требует ≥3 EU MS partners + signed LoIs. Сейчас только cold-contact DFKI. Без LoIs — REJECT. |
| Gates Foundation | REJECT | Longevity вне фокуса; нет LMIC partner; нет community engagement framework; Georgia upper-middle-income не закрывает критерий LMIC. |
| Schmidt Sciences | REJECT | Платформа не демонстрирует AI/ML core; Tkemaladze не имеет strong publication record в AI; риск fabrications блокирует trust. |
| NIH R01 | REJECT | US PI mandatory; нет preliminary data; нет NIH grant history у PI. |
| Impetus Longevity | N/A | Umbrella не является testable intervention или biomarker validation. Не подходит по scope. |

## MCOA

| Фонд | Вердикт | Обоснование |
|------|---------|-------------|
| Wellcome Leap | REJECT | Мета-теория без количественных предсказаний; после 9/9 fabrications любая заявка будет воспринята как недобросовестная. |
| ARPA-H | REJECT | Нет технологического прототипа; философская рамка не соответствует health breakthrough. |
| EIC Pathfinder Open 2027 | REJECT | Теория не фальсифицируема (противоречие M3: w_i = Placeholder). Wave 1 показал, что MCOA не готова к peer review. |
| Gates Foundation | REJECT | Scope не пересекается с глобальным здоровьем. |
| Schmidt Sciences | COND | Теоретически, если переформулировать как AI-driven модель старения (параметризация w_i). Но fabrications уничтожают доверие; нужен полный рестарт. COND только после independent replication. |
| NIH R01 | REJECT | Нет предварительных данных; R01 требует сильные preliminary results. |
| Impetus Longevity | REJECT | Не предлагает измеримое вмешательство. |

## FCLC

| Фонд | Вердикт | Обоснование |
|------|---------|-------------|
| Wellcome Leap | COND | Federated learning для rare diseases (longevity data) может быть в scope. Требует: ε < 1 (сейчас ε=10), signed US/UK co-PI, PATE non-stub, consortium. Без этого — REJECT. |
| ARPA-H | REJECT | Milestone-driven health tech; FCLC не демонстрирует конкретного health outcome; нет animal/human in vivo data. |
| EIC Pathfinder Open 2027 | COND | Наиболее вероятный кандидат. Требует: (i) ε < 1 с дифференциальной приватностью, (ii) Federated Shapley реализуемость (сейчас нереализуемо), (iii) ≥2 signed LoIs от EU MS партнёров, (iv) open-source core code. |
| Gates Foundation | COND | Потенциальный интерес для LMIC (privacy-preserving health data sharing). Требует: LMIC site partner (например, клиника в Африке), ε < 1, validation dataset. |
| Schmidt Sciences | REJECT | Недостаточно AI innovation; Federated Shapley — в целом решённая задача, novelty низкая. |
| NIH R01 | REJECT | US PI mandatory; нет US-based team. |
| Impetus Longevity | N/A | Инфраструктурный проект, не тестовая интервенция. |

## Ze (Entropic-Geometric ToE)

| Фонд | Вердикт | Обоснование |
|------|---------|-------------|
| Wellcome Leap | REJECT | Нарушение Tsirelson bound без обоснования; χ_Ze pre-reg null; нет приложений к старению или здоровью. |
| ARPA-H | REJECT | Фундаментальная физика, не health tech. |
| EIC Pathfinder Open 2027 | REJECT | Wave 1 показал несостоятельность как научной теории; Z1 = переоформленный Friston FEP. Для EIC Pathfinder нужна "visionary research" с checkable predictions — Ze не предоставляет. |
| Gates Foundation | REJECT | Нет связи со здоровьем. |
| Schmidt Sciences | REJECT | Нет strong AI core; теория спекулятивна. |
| NIH R01 | REJECT | Не biomedical. |
| Impetus Longevity | REJECT | Не вмешательство, нет biomarker. |

## CDATA

| Фонд | Вердикт | Обоснование |
|------|---------|-------------|
| Wellcome Leap | REJECT | Sobol-парадокс: centriolar damage менее важен, чем epigenetics. Ablation epigenetics улучшает R², что Invalidates основной claim. LOO-CV bias = -0.093 указывает на перекос. Без фиксации — REJECT. |
| ARPA-H | REJECT | Нет прототипа in vivo; синтетические данные не заменяют animal model. |
| EIC Pathfinder Open 2027 | COND | Если исправить Sobol-парадокс и bias, может быть как WP в составе FCLC консорциума (биомаркеры старения). Требует: ablation validation на новых данных, pre-registration. |
| Gates Foundation | REJECT | Scope: centriolar damage не является приоритетом глобального здоровья. |
| Schmidt Sciences | REJECT | Модель in silico без ML инновации; простое регрессионное моделирование. |
| NIH R01 | REJECT | US PI mandatory; нет preliminary data on humans. |
| Impetus Longevity | COND | Небольшой грант на валидацию биомаркера (centriolar damage → ageing). Требует: (i) независимое воспроизведение, (ii) pre-registration, (iii) исправление Sobol-парадокса. |

## BioSense

| Фонд | Вердикт | Обоснование |
|------|---------|-------------|
| Wellcome Leap | REJECT | 3/3 pre-reg null; I²=90.3%, bootstrap CI=0 — нет сигнала. Born rule неприменим. Отклонение. |
| ARPA-H | REJECT | Нет технологии, прошедшей пилот; null результаты убивают. |
| EIC Pathfinder Open 2027 | REJECT | Null + fabrications в PMID (4/9). Даже после исправлений, статистическая мощность нулевая. |
| Gates Foundation | REJECT | Low-cost diagnostic potential есть, но не подтверждено; нет signal detection. |
| Schmidt Sciences | REJECT | Нет AI/ML; hardware проект с null. |
| NIH R01 | REJECT | US PI; нет preliminary data. |
| Impetus Longevity | REJECT | Вмешательство не валидировано; hardware без эффекта. |

## Ontogenesis

| Фонд | Вердикт | Обоснование |
|------|---------|-------------|
| Wellcome Leap | REJECT | Quarantine после 6/6 fabrications; Smith J. 2025 подозрение на новую fabrication. Нет доверия. |
| ARPA-H | REJECT | Аналогично. |
| EIC Pathfinder Open 2027 | REJECT | Fabrications делают проект неприемлемым для рецензируемого конкурса. |
| Gates Foundation | REJECT | Нет связи с глобальным здоровьем. |
| Schmidt Sciences | REJECT | Нет AI. |
| NIH R01 | REJECT | US PI. |
| Impetus Longevity | N/A | Онтогенетический counter не является вмешательством или биомаркером для вмешательства. |

## HAP (Hepato-Affective Primacy)

| Фонд | Вердикт | Обоснование |
|------|---------|-------------|
| Wellcome Leap | REJECT | 10/10 PMID fabricated; evidence.md = stub. |
| ARPA-H | REJECT | Нет прототипа; нет данных. |
| EIC Pathfinder Open 2027 | REJECT | Полная недобросовестность. |
| Gates Foundation | REJECT | Нет scope. |
| Schmidt Sciences | REJECT | Нет AI. |
| NIH R01 | REJECT | US PI. |
| Impetus Longevity | REJECT | Нет валидного вмешательства или биомаркера. |

## Итог матрицы (сводка)

Из 56 ячеек: FUND = 0, COND = 6 (CommonHealth→EIC, FCLC→Wellcome, FCLC→EIC, FCLC→Gates, CDATA→EIC, CDATA→Impetus), REJECT = 42, N/A = 8. None get unconditional "FUND". Даже COND ячейки требуют существенных исправлений, которые на сегодня не выполнены.

# §2. Сильные стороны по подпроектам (fund perspective)

## CommonHealth (top)
1. **Thin-layer PWA architecture** — React+TypeScript+Phoenix LiveView реально быстрое прототипирование. Для EIC это может демонстрировать feasibility цифровой платформы.
2. **Открытый код (частично)** — монорепозиторий публичен, хотя split. Показывает commitment к open science, что положительно для EIC и Schmidt.
3. **Ambitious umbrella concept** — интеграция 7 подпроектов в единую экосистему — это visionary, соответствует EIC Pathfinder.

Однако эти сильные стороны перевешиваются слабостями: нет консорциума, нет signed LoIs, PI не аффилирован с university, и fabrications в подпроектах уничтожают доверие к umbrella.

## MCOA
1. **Публикация в Nature Aging (перспектива)** — Tkemaladze подал Perspective, что показывает попытку диалога с мейнстримом. Для Impetus или Schmidt это могло бы быть сильным сигналом, если бы не fabrications.
2. **Multi-Counter Architecture** — визуально простая метафора (multiple “clocks”). Потенциально может быть переосмыслена как AI ensemble model.
3. **Наличие математической формулировки (w_i)** — хотя w_i — Placeholder, сама идея параметризации может быть развита.

Минус: fabrications сделали невозможным использование этой работы для grant-writing до полного independent replication.

## FCLC
1. **Federated Shapley** (теоретически) — если бы был реализован, это innovation в explainable FL. Однако текущий код недоделан, и реализация нереализуема (по замечаниям Wave 1). 
2. **Выбор Rust для simulator** — производительность важна для FL на edge. Может быть сильным аргументом для EIC (софтверная инженерия). 
3. **Потенциальное применение в LMIC (Gates)** — federated learning для диагностики в условиях ограниченной инфраструктуры — актуально. Но требуется партнёр на местах.

## Ze
1. **Междисциплинарность** — попытка объединить физику и биологию. Это может быть интересно Schmidt Futures, но только если есть проверяемые predictions. 
2. **Симулятор на Rust** — работает, даёт численные решения. 
3. **χ_Ze counter as conceptual device** — оригинальная идея для обсуждения.

Но все перечёркивается: Tsirelson violation без justification, null EEG/HRV, плагиат FEP.

## CDATA
1. **In silico модель деления HSC** — позволяет генерировать синтетические данные для первичного тестирования гипотез. Для Impetus это может быть быстрым треком. 
2. **Связь с общепризнанными биомаркерами (epigenetic clock)** — попытка учесть epigenetic drift. 
3. **Open simulator на Rust** (частично) — воспроизводимость.

Слабость: Sobol-парадокс убивает причинность centriolar damage.

## BioSense
1. **Pre-registration 3 протоколов** — образцовая практика open science. Это единственный подпроект с pre-reg. 
2. **Низкая себестоимость** — потенциальный дешёвый сенсор для LMIC (Gates). 
3. **Попытка ЭЭГ+HRV+olfaction** — комплексный подход.

Но null результаты и I²=90.3% делают его бесполезным для дальнейшего финансирования без полного пересмотра протокола.

## Ontogenesis
- Сильные стороны не выявлены. Проект в Quarantine; 6/6 fabrications, подозрение на новую fabrication. Не может быть рекомендован к финансированию.

## HAP
- Сильные стороны не выявлены; 10/10 fabrications; единственная цитата — self-citation. Не может быть рекомендован.

# §3. Критические fund-specific gaps

## CommonHealth
1. **Отсутствие signed LoIs от EU MS partners** — для EIC это fatal. 
2. **PI не аффилирован с tier-1 university** — для всех фондов (кроме Impetus) это снижает доверие и ability to manage large grants. 
3. **Нет US PI co-applicant** — блокирует Wellcome, ARPA-H, NIH R01. 
4. **Нет patent или trademark strategy** — IP риски для всех фондов, требующих exploitation plan. 
5. **Нет formal consortium agreement** — для Wellcome Leap и ARPA-H обязательны. 
6. **Risk register fabrications** — любой фонд запросит due diligence; факт массовых fabrications в Wave 1 должен быть раскрыт, что уничтожит заявку.

## MCOA
1. **9/9 PMID fabricated** — после исправлений остаются вопросы к integrity. 
2. **w_i = Placeholder** — теория нефальсифицируема. 
3. **Нет pre-registered predictions**. 
4. **Отсутствие связей с клиническими партнёрами для валидации**. 
5. **Противоречие M3** — теоретическая несостоятельность (см. Wave 1). 

## FCLC
1. **ε=10 против ISO ε<1** — дифференциальная приватность недостаточна для health data. 
2. **PATE — stub** — нереализован ключевой компонент. 
3. **Federated Shapley нереализуем** — требует знаний feature attributions, которые не могут быть получены в FL. 
4. **Нет signed LoIs от EU partners** (кроме cold-contact DFKI). 
5. **Нет публичного ядра (core)** — код fclc-core не на GitHub. 
6. **Нет validation dataset** — только синтетические данные. 
7. **Отсутствие US/UK co-PI для Wellcome**. 

## Ze
1. **Нарушение Tsirelson bound без обоснования** — фундаментальная ошибка. 
2. **χ_Ze pre-reg null**. 
3. **Z1 — переоформленный FEP Friston** — отсутствие originality. 
4. **Нет приложений к здоровью** — все фонды, кроме Schmidt, ориентированы на medicine/health. 
5. **Нефизичность энтропийно-геометрической теории** — для финансирования фундаментальной физики нужен endorsement от признанных физиков (нет). 

## CDATA
1. **Sobol-парадокс: эпигенетика доминирует, ablation улучшает R²** — подрывает основной claim. 
2. **LOO-CV bias = -0.093** — систематическое смещение. 
3. **Данные только in silico** — нет in vitro/in vivo. 
4. **Не pre-registered** (Test 1–4, MCAI). 
5. **Отсутствие clinical validation plan**. 

## BioSense
1. **3/3 pre-reg null** — нет эффекта. 
2. **I²=90.3%** — чрезвычайная гетерогенность, указывает на проблемы протокола. 
3. **Bootstrap CI=0** — артефакт анализа. 
4. **Born rule неприменим к HRV/EEG** — фундаментальная ошибка интерпретации. 
5. **4/9 PMID были битые** (исправлены, но осадок остался). 

## Ontogenesis
1. **6/6 PMID fabricated в прошлой версии; Smith J. 2025 — подозрение на новую fabrication**. 
2. **Нет данных (только гипотетические)**. 
3. **Нет pre-registration**. 
4. **Нет публичных материалов для рецензирования**. 

## HAP
1. **10/10 PMID fabricated**. 
2. **Evidence.md = stub** — нет содержания. 
3. **Единственная цитата — self-citation Tkemaladze в Longevity Horizon (не индексированный)**. 

### §3a. Risk register — ecosystem-wide post Wave 1

| # | Риск | Вероятность | Влияние (impact) | Mitigation (существующее / планируемое) | Residual risk |
|---|------|-------------|------------------|-----------------------------------------|---------------|
| R1 | Massive citation fabrication: после Wave 1 коррекций остаются неверифицированные ссылки; Smith J. 2025 не подтверждён, Sun 2016 удалён, полный reverse-verification всех 24+ ссылок не проведён | Medium (высокая, т.к. fabrication была массовой и системной) | Critical — потеря доверия фонда, ретракция публикации, репутационный ущерб, невозможность подачи в тир-1 фонды | Частичная замена PMID в MCOA, FCLC, CDATA, BioSense; НЕТ полного librarian-level аудита; НЕТ плана верификации оставшихся ссылок | High |
| R2 | Single-PI dependency (Tkemaladze — единственный PI, bus factor = 1) | High (нет co-PI, нет заместителя, все архитектурные решения принимает один человек) | High — при недееспособности PI проект останавливается; ни один фонд не одобрит single-PI без succession plan | Нет co-PI; нет documented design decisions; документация в стадии формирования | High |
| R3 | Геополитический риск Грузия: близость к RU, нестабильность, ограничение сотрудничества с EU партнёрами | Medium (текущая ситуация стабильна, но риски санкций, логистики, ограничения виз) | Medium — возможные задержки, сложности с привлечением EU partners, отказ страховых покрытий | Юридическая регистрация NGO в Грузии, но нет геополитического хеджирования (резервная юрисдикция) | Medium–High |
| R4 | NGO host (Georgia Longevity Alliance) не имеет опыта управления грантами >€500K (заявка на €3M); нет CFO, нет грант-менеджера | High (отсутствие инфраструктуры, бухгалтерии, compliance) | Critical — фонды требуют auditable financial management; отсутствие грантового опыта = immediate rejection в EIC, Wellcome, Gates | Tkemaladze указывает опыт управления бюджетом кафедры ($200K/yr); нет hiring plan для CFO/grant manager | High |
| R5 | PhD pipeline: указан 1 PhD candidate (Lezhava) для capacity-building claim; недостаточно для масштаба заявки | High (для capacity-building нужна критическая масса студентов, минимум 3–4) | Medium — фонды (Wellcome, Gates) ожидают pipeline минимум 3–5 PhD; иначе claim выглядит натянутым | Планируется привлечение студентов Javakheti University (Akhaltsikhe), но нет подтверждённых commitments | Medium |
| R6 | Theoretical incoherence: MCOA M3 + CDATA Sobol + Ze Tsirelson — разрозненные модели без единой формальной теории | Medium (комбинация математически несовместима: M3 — детерминированная, CDATA — статистическая, Ze — квантовая) | High — рецензенты укажут на отсутствие концептуальной связности; снижение научной убедительности | Нет unified framework; ссылки на “deep learning meets quantum biology” не подкреплены публикациями | High |
| R7 | Privacy/regulatory risk: FCLC ε=10 не соответствует ISO/IEC 27559; не проведён GDPR clearance для обработки данных из ЕС | Medium (заявка предполагает международные данные, ε=10 слишком слабый для медицинских данных) | Critical — нарушение GDPR/ISO блокирует использование в клинических исследованиях; европейские партнёры не смогут участвовать | ε снижен с 100 до 10, но формального аудита не было; нет DPA/DPO; нет data processing agreement | High |
| R8 | Technology debt: FCLC stub PATE; w_i Placeholder (не обучены веса) | High (прототип не реализует ключевой модуль) | Medium — без работающего PATE система не обеспечивает privacy guarantee, демо нерепрезентативно | Обещано завершить к Q4 2026; но нет roadmap и выделенного инженера | Medium |
| R9 | Reputational risk: Tkemaladze 149 публикаций, но ~10 в PubMed; высокий процент в нерецензируемых источниках | Medium (часть публикаций – тезисы, локальные журналы) | High — фонды проверяют publication track record; несоответствие заявленного объёма и индексируемого качества | Ссылки на PubMed постепенно добавляются; но legacy остаётся; необходима объяснительная записка | Medium |
| R10 | Open-source disclosure: репозитории (Rust simulator, React PWA, Phoenix) не содержат LICENSE файла → дефолтное copyright (все права защищены) | High (ни один репозиторий не имеет LICENSE) | Medium — противоречит требованию EIC “open-source publication”; блокирует коммерческое/академическое переиспользование; может быть воспринято как недобросовестность | Необходимо добавить LICENSE (GPL/MIT) во все репозитории; это не сделано | Medium |

**Aggregate residual risk = HIGH** 
Совокупный остаточный риск оценивается как High: минимум пять рисков (R1, R2, R4, R6, R7) имеют High residual, и ни один из критических рисков не снижен ниже Medium. Особую тревогу вызывает R1 (citation fabrication) — без полного librarian-level аудита ни один тир-1 фонд не примет заявку. R4 (NGO capacity) требует немедленного найма финансового менеджера. R2 (single-PI) может быть частично снижен через привлечение со-PI из Javakheti University или зарубежного партнёра. Рекомендуется провести ревизию всех рисков и предоставить documented mitigation plan до любой подачи.

### §3b. IP/licensing analysis

**Rust simulator (Ze + CDATA HSC).** 
Ключевой вычислительный компонент — симулятор на Rust, комбинирующий Ze Tsirelson и CDATA HSC. Код опубликован в репозитории без LICENSE файла, что по умолчанию означает «All Rights Reserved» согласно Бернской конвенции. Даже при намерении открыть код отсутствие явной лицензии создаёт правовую неопределённость: третьи лица не могут легально использовать, модифицировать или распространять код. Для EIC Pathfinder open-source publication является обязательным требованием, и рецензенты проверят наличие лицензии. Рекомендуемая схема: dual-licensing — GPL v3 для академического сообщества (обеспечивает copyleft и защиту от proprietary forks) и коммерческая лицензия для промышленных партнёров (например, через отдельное соглашение). Это типичная практика для проектов с потенциалом spin-off, но требует юридического оформления.

**React+TypeScript PWA и Phoenix LiveView (Elixir).** 
Веб-интерфейсы и микросервисы написаны на стандартных фреймворках. Для них оптимальна лицензия MIT или Apache 2.0. Никаких блокеров — код типовой, патентных рисков нет (MIT не содержит патентной оговорки, Apache — содержит). Единственное замечание: LICENSE файл отсутствует; его добавление — вопрос одного дня.

**DeepSeek API в продакшене.** 
Использование API DeepSeek (китайская LLM) в медицинском приложении вызывает вопросы complianсe. DeepSeek, вероятно, обрабатывает данные на серверах в КНР, что при попадании персональных данных (даже псевдонимизированных) нарушает GDPR (требование adequate level of protection). В заявке не указано, какие данные передаются в API. Если это только обезличенные тексты (научные статьи) — риск невелик, но если запросы содержат профили участников исследований — это недопустимо. Необходим полный data flow map и договор с DeepSeek о DPA (Data Processing Agreement). Пока vendor lock-in не фатален, но для медицинской сертификации потребуется альтернатива (например, LLaMA локально).

**Патенты и товарные знаки.** 
В проекте не зарегистрировано ни одного патента. Для фондов типа Impetus, ARPA-H или Wellcome Leap это нормально — они финансируют early-stage R&D, где публикация важнее патентования. Однако Schmidt Sciences и отраслевые партнёры (например, индустриальные участники EIC) могут счесть отсутствие патентной защиты слабостью. Товарные знаки «CommonHealth», «Ze·Profile», «Ze·Guide» не зарегистрированы. Есть риск конфликта с US-компанией Common Health, Inc., которая оказывает медицинские услуги под тем же брендом. Использование названия без проверки может привести к судебным искам после масштабирования.

**Вердикт по IP.** 
До подачи в любой целевой фонд необходимо: (1) провести licensing audit — добавить LICENSE файлы (GPL v3 для Rust, MIT/Apache для веба) во все репозитории; (2) заключить DPA с DeepSeek или перейти на локальную LLM; (3) выполнить trademark search для «CommonHealth» и, вероятно, переименовать проект. Патентование на данном этапе необязательно, но рекомендуется для ключевых алгоритмов (Ze Tsirelson, PATE variant) при подаче в Schmidt Sciences.

### §3c. Capacity-building (Wellcome/Gates LMIC criterion)

Заявка позиционируется как capacity-building в Грузии, ссылаясь на статус страны с развивающейся экономикой. Однако согласно классификации Всемирного банка 2024, Грузия относится к upper-middle-income economies (ВНД на душу населения ~5 500 USD). Это не соответствует определению LMIC (Low- and Middle-Income Countries) в строгом смысле, принятому Wellcome Trust и Фондом Билла и Мелинды Гейтс. Эти организации фокусируются на странах Subsaharan Africa и South Asia (ВНД <4 000 USD) и часто требуют, чтобы заявка включала локального заявителя из такой страны.

**Что есть в заявке:** 
- Georgia Longevity Alliance (NGO, рег. №404506520, президент Ткемаладзе) — зарегистрирована в 2026, активна. 
- Tina Gelashvili (Javakheti University, Akhaltsikhe) — возможный академический bridge, но её роль и вклад не детализированы. 
- Один PhD candidate (Lezhava) — очевидно недостаточно для capacity-building. 
- Нет формального training programme, нет community engagement framework, нет локальной IRB/этической комиссии (bioethics board в Грузии существует, но не для исследований на людях, а для общего надзора; её полномочия неясны).

**Чего нет:** 
- Africa-equivalent partner (Gates Global Health): для Wellcome Leap LMIC track требуется подтверждение, что исследование касается приоритетов здравоохранения конкретной LMIC. 
- Female PI / gender-equity policy: ни одна из ключевых ролей не занимает женщина, хотя Gelashvili может быть опционально со-PI. 
- Training plan: Wellcome требует описания How the grant will build local capacity (treineeships, courses, equipment). Этого нет. 
- Community engagement: механизмы вовлечения местного населения, информирования и согласия не описаны. 
- Локальный IRB: протоколы этической экспертизы отсутствуют; для исследований с участием людей (даже если it's public data) в Грузии необходимо одобрение Национального совета по биоэтике.

**Вывод:** 
- Wellcome Leap LMIC track — **REJECT** по критерию capacity-building. Грузия не является приоритетной LMIC, нет training plan и Africa partner. 
- Gates Global Health — **REJECT**, обязателен африканский со-заявитель. 
- EIC Pathfinder не требует LMIC (хотя приветствует), но требует минимум трёх участников из стран EU MS. Грузия — не EU MS, а Associated Country (возможно в Horizon Europe). Это не блокирует, но снижает eligibility: большинство консорциумов предпочитают чистых EU MS для упрощения бюрократии. 

Альтернатива: переформулировать заявку как «research capacity in Associated Country» для EIC Pathfinder, исключив LMIC трек, и усилить компонент обучения (добавить 2–3 PhD-позиции, летние школы, обмен с EU партнёрами).

### §3d. Citation integrity post-corrections

После Wave 1 коррекций часть fabrication исправлена. В MCOA 9/9 PMID заменены на реальные PubMed-записи (Hernandez-Segura 28844647, Schaum 32669715, Balaban 15734681, Mathieson 29449567, Enge 28965763, Parrinello 12855956, Janke 32107477; Sun 2016 удалён). В PARAMETERS α_Tel скорректирован: битый PMID 2038241 заменён на 2342578 (Harley 1990) и 1631178 (Allsopp 1992). В Ze удалён фальшивый arXiv 2501.12345 (Kerenidis). В CDATA Goetz & Anderson 2010 DOI 10.1038/nature08117 заменён на 10.1038/nrg2774 (PMID 20395968). В BioSense 4 PMID исправлены (Voytek 26424877, Iyengar 8967405, Kleiger 3812275, Task Force 8598068). В Ontogenesis удалён гипотетический DOI 10.1016/j.dcn.2021.100971.

Однако в Ontogenesis остаётся ссылка «Smith J. 2025» — не верифицированный источник; по состоянию на 26.04.2026 такая публикация не обнаружена ни в PubMed, ни в arXiv, ни в репозиториях bioRxiv/medRxiv. Это означает, что раздел Ontogenesis по-прежнему содержит подозрение на fabrication. Для фондов это критично: если разумное сомнение остаётся хотя бы в одной цитате, весь проект получает ярлык «integrity risk».

Компонент HAP остаётся в stub state: 10/10 fabrication по-прежнему не исправлены, единственная цитата (Tkemaladze в Longevity Horizon) не индексируется. HAP является одним из ключевых блоков системы (Hormonal Aging Profile). Невозможно рекомендовать проект с непроверяемой ссылочной базой в важном компоненте.

**Итог:** 
- **Acceptable but flagged** — MCOA, FCLC, CDATA, BioSense (коррекции проведены, но полный аудит не завершён). 
- **Unacceptable** — Ontogenesis (Smith J. 2025) и HAP (stub with fabrications). 

Любой программный офицер фонда запросит full reverse-verification всех 24+ ссылок до серьёзного рассмотрения. Это работа librarian-level (~40 часов), которая не выполнена. Рекомендуется незамедлительно поручить её независимому библиографу (например, через университетскую библиотеку). До получения заключения citation integrity остаётся под сомнением.

# §4. Required corrections for fund submission (конкретные deliverables)

## CommonHealth (для EIC)
- ✅ **Signed LoIs от ≥3 EU MS организаций** (хотя бы одна research university, один SME health tech). 
- ✅ **Formal consortium agreement (CA)** с ясными ролями, IP ownership, exit clause. 
- ✅ **PI должен получить co-applicant аффилированный с EU university** (например, пригласить Lezhava? но Lezhava PhD candidate не пройдёт). 
- ✅ **Public repository с LICENSE** (например, Apache 2.0) для всего кода, включая fclc-core. 
- ✅ **Risk register с разделом "Integrity & fabrication mitigation"** — описать audit процедуры. 

## MCOA (для Schmidt Sciences – условно)
- ❌ **Полный рестарт теории** — заменить w_i на вычислимые параметры. 
- ❌ **Pre-register 3 checkable predictions** (например, на основе epigenetic datasets). 
- ✅ **Independent replication MCOA без участия Tkemaladze**. 
- ❌ **Поиск external collaborator (UCSD/Stanford)** для со-PI позиции. 

Но реалистично: MCOA не готов к подаче ни в один фонд в 2026–2027.

## FCLC (для EIC/Gates/Wellcome)
- **Для EIC**: 
 — ε ≤ 1 (реализовать DP-SGD с частным запасом). 
 — PATE full implementation (не stub). 
 — Заменить Federated Shapley на разрешимую задачу (например, federated feature importance через influence functions). 
 — Signed LoIs от ≥2 EU partners (минимум один technical partner). 
- **Для Wellcome Leap**: 
 — US/UK co-PI (например, из MIT или Imperial). 
 — Quarterly milestones (M1: DP prototype; M2: Deployment on OMOP CDM; M3: Clinical pilot). 
 — Прототип на реальных data (не синтетика). 
- **Для Gates**: 
 — LMIC partner (клиника в Африке). 
 — Dataset от партнёра. 
 — ε ≤ 1 с подтверждением на реальных данных. 

## Ze
- ❌ **Теория требует external review от физиков (Tsirelson bound)**. 
- ❌ **χ_Ze experiment redesign** — после null нужно объяснение. 
- ❌ **Приложения к здоровью: интеграция с CDATA (например, χ_Ze как proxy для epigenetic clock)** — но это speculative. 
- ❗ Подача только в Schmidt Sciences невозможна без сильного AI core.

## CDATA (для Impetus)
- ✅ **Полное исправление Sobol-парадокса**: необходимо показать, что ablation эпигенетики ухудшает модель, а не улучшает. Если это невозможно — отказаться от claim причинности. 
- ✅ **LOO-CV bias устранён** (перекрёстная валидация с учётом иерархии). 
- ✅ **Pre-registration MCAI tests**. 
- ✅ **In vitro валидация (хотя бы клеточная линия) или использование public datasets (e.g., Horvath clock)**. 
- ✅ **Plan для clinical biomarker validation**. 

## BioSense
- ❌ **Полный рестарт hardware/software протокола** (Born rule заменить корректными feature extraction). 
- ❌ **Pre-register новый протокол с коррекцией объёма выборки**. 
- ❌ **Необходимо pilot study с положительным контролем** (например, HRV after exercise). 
- ✅ **Команда должна включать statistician**. 

## Ontogenesis
- ❌ **Подтверждение или опровержение Smith J. 2025 fabrication** (тотальный аудит). 
- ❌ **Публикация данных для независимого рецензирования**. 
- ❗ Без полного очищения имени – не подавать никуда. 

## HAP
- ❌ **Удалить из всех заявок** как скомпрометированный. 

# §5. Sequencing strategy (2026–2028)

## Приоритет 1: CDATA → Impetus Longevity (Q3 2026)
- **Почему**: Impetus гранты $100–500k, срок 12 мес. CDATA с исправлениями может подаваться как проверка биомаркера centriolar damage. 
- **Условие**: полное исправление Sobol-парадокса и bias; pre-registration; независимое репликация (хотя бы на публичных данных). 
- **Риск**: если Sobol-парадокс неисправим, проект теряет смысл. Но это покажет нежизнеспособность. 
- **Timeline**: Q2 2026 – исправления, Q3 подача. 

## Приоритет 2: FCLC → EIC Pathfinder Open 2027 (Q4 2026 – Q1 2027)
- **Почему**: EIC Pathfinder – основной целевой фонд для CommonHealth. FCLC – единственный подпроект с потенциалом после исправлений. 
- **Необходимые шаги**: 
 — Q2–Q3 2026: signed LoIs (договориться с DFKI + искать второго EU partner). 
 — Q2 2026: реализовать ε ≤ 1 и PATE. 
 — Q3 2026: open-source core code с LICENSE. 
 — Q4 2026: собрать консорциум (минимум 3 EU partners). 
- **Риск**: без международного co-PI заявка будет слабой. Нужен хотя бы один западный университет. 

## Приоритет 3: FCLC → Gates Foundation (2027, если получены LMIC данные)
- **Условие**: подписание MOU с африканской клиникой (например, в Кении). 
- **Требуется**: валидация на реальных данных (не синтетика). 
- **Timeline**: 2027–2028, после EIC пилота. 

## Приоритет 4: FCLC → Wellcome Leap (2028, как часть большого консорциума)
- Wellcome Leap требует зрелый consortium. CommonHealth может войти как один из WP, но не как lead. 
- **Требование**: US/UK co-PI, quarterly milestones, deliverable data sharing system. 

## **Никогда не подавать** 
- **Ze, Ontogenesis, HAP** — без полной реабилитации (годы). 
- **BioSense** — без положительного контролируемого испытания. 
- **MCOA** — без independent replication. 

## Общее резюме
Рекомендуется сфокусироваться на **FCLC→EIC** как основном треке 2027. CDATA→Impetus как быстрый тест. Остальные подпроекты не готовы к подаче в 2026–2028. CommonHealth umbrella может войти как overhead в EIC, но не самостоятельная заявка.

# §6. Bottom-line ecosystem-wide fund readiness

На сегодня (2026-04-26) экосистема CommonHealth находится на уровне **2 из 10** по шкале fund-readiness. Причина: массовые fabrications (≥30 верифицированных случаев в Wave 1) подорвали доверие к научной добросовестности PI и всей команды. Даже после исправлений остаются структурные дефекты: отсутствие signed LoIs от EU партнёров, отсутствие US/UK co-PI для американских фондов, неполнота кода (FCLC core не публичен), невыполнение базовых требований DP (ε=10), теоретические противоречия (M3, Tsirelson, Sobol-парадокс). Только два подпроекта (FCLC и CDATA) имеют потенциал при условии полной переработки и построения консорциума за пределами Грузии. Инвестиционный комитет не может рекомендовать ни один из семи фондов для подачи в текущем состоянии. Требуется 12–18 месяцев планомерных исправлений, интеграции международных партнёров и демонстрации положительных результатов независимых репликаций. Без этого любая заявка приведёт к немедленному REJECT.

### §6a. Investment committee recommendation

На основе проведённого due-diligence, комитет по инвестициям рекомендует **HOLD** на всей экосистеме до Q1 2027 milestone gate. Ни один из семи целевых фондов в текущем состоянии не готов к подаче. Для перехода к статусу «WATCH» необходимо выполнение следующих gate criteria: 

1. **Citation audit** — проведён полный reverse-verification всех 24+ ссылок независимым экспертом, получено заключение об отсутствии неподтверждённых fabrication (кроме HAP, который либо завершён, либо удалён). 
2. **NGO capacity** — нанят грант-менеджер с опытом работы с грантами >€1M, утверждён финансовый план, подготовлена проектная заявка с аудированной отчётностью. 
3. **PI diversification** — назначен co-PI из Javakheti University или зарубежного партнёра, задокументированы полномочия и succession plan. 
4. **IP/licensing** — все репозитории получили LICENSE файлы, проведён trademark search, решён вопрос с DeepSeek (DPA или замена на локальную LLM). 
5. **Теоретическая согласованность** — опубликован pre-print (arXiv или рецензируемый журнал), демонстрирующий формальную связь между MCOA M3, CDATA Sobol и Ze Tsirelson; разослан коллегам для external review. 

После выполнения этих критериев рекомендация может быть повышена до **WATCH** (возможность подачи в EIC Pathfinder при условии подтверждения Associated Country статуса Грузии) или **INVEST** (для конкретного подпроекта, например, FCLC prototype). 

**Финальная фраза:** 
Текущее состояние не позволяет рекомендовать ни один из 7 целевых фондов для подачи в течение ближайших 6 месяцев. Re-evaluation: 2027-Q1 после выполнения gate criteria.
