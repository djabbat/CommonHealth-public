# AUDIT PACKET — Regenesis

Path: `/home/oem/Desktop/Regenesis`  Date: 2026-05-08

## Size & file counts
```
115M	/home/oem/Desktop/Regenesis
```
**Extensions:** .md=44, .docx=23, .pdf=19, .xlsx=3, .pptx=2, (noext)=1
## Tree (depth=2, max 200 entries)
```
.
./Pinekan
./Pinekan/PARAMETERS.md
./Pinekan/Recipe_Pinekan
./Pinekan/README.md
./Pinekan/UPGRADE.md
./Pinekan/Logistics
./Pinekan/TODO.md
./Pinekan/MEMORY.md
./Pinekan/CLAUDE.md
./Pinekan/LINKS.md
./Pinekan/CONCEPT.md
./Pinekan/MAP.md
./Pinekan/KNOWLEDGE.md
./Pinekan/Marketing
./PARAMETERS.md
./Materials
./Materials/Список экспериментов.docx
./Materials/005Newest system CO2 fractional laser--catalog.pdf
./Materials/REGENESIS_PINXEL-V_Clinical_Explanation_RU.docx
./Materials/REGENESIS_Premium_Protocol_RU.docx
./Materials/Centriole transplantation_.docx
./Materials/Recepturae
./Materials/Organism Ageing AcАudio.docx
./Materials/სტატიები
./Materials/catalog--vacuum mrico needle RF machine.pdf
./Materials/七色纳米蓝色包装光谱仪说明书.pdf
./Materials/5代光谱仪(英文版).pdf
./Materials/SilkAesthetic_Industry_Event_Program_Trilingual_FINAL.docx
./Materials/Projects.docx
./Materials/Омолаживающий Кожу Крем.docx
./Materials/REGENESIS_Booklet_Final_SilkAesthetic_GE.docx
./Materials/CO2_Laser_Aftercare_Silkaesthetic_Final.docx
./Materials/Proposals
./Materials/Pets
./Materials/REGENESIS_CO2_Fractional_Laser_Clinical_Explanation_RU.docx
./Materials/REGENESIS_A4_Luxury_Flyer_GEO.docx
./Materials/Hevolution
./Materials/Letter Templates.docx
./Materials/ბიზნეს გეგმა
./Materials/Presentations
./Materials/REGENESIS_Clinical_Protocol_RU.docx
./Materials/Lab. Equipment.xlsx
./README.md
./CONCEPT_CODE_AUDIT_2026-04-21.md
./UPGRADE.md
./TODO.md
./MEMORY.md
./CLAUDE.md
./LINKS.md
./docs
./docs/TRUE_MISMATCHES_Regenesis.md
./docs/literature_daily
./docs/CORRECTION_CANDIDATES.md
./docs/META_ANALYSIS_Regenesis.md
./docs/PEER_REVIEW_Regenesis.md
./docs/REFERENCE_AUDIT_Regenesis.md
./CONCEPT.md
./MAP.md
./KNOWLEDGE.md
```
## Detected stack: **unknown**
## Core files

### `CLAUDE.md` (507 chars)
```md
# Regenesis — Фитомедицинские протоколы

## 📌 Правило: DeepSeek для нетехнических задач

**Код — Claude. Всё остальное — DeepSeek API.**
Примеры: протоколы на латыни, клинические обоснования, переводы, редактура.
**Ключ:** `~/.aim_env → DEEPSEEK_API_KEY` · **Вход:** `~/Desktop/AIM/llm.py`
**Модели:** `deepseek-chat` (быстро) · `deepseek-reasoner` (клинические обоснования)

---

## Проект

14 фитомедицинских протоколов (латынь). Интеграция с AIM через `regenesis_protocol.py`.
Статус: 🟢 Документируется.

```
### `Pinekan/CLAUDE.md` (1988 chars)
```md
# CLAUDE.md — Pinekan (subproject of Regenesis)

## Identity

**Project:** Pinekan — `Pinekan` рецептура и логистика как подпроект Regenesis
**Location:** `~/Desktop/Regenesis/Pinekan/`
**Parent:** `~/Desktop/Regenesis/` (rejuvenation lab concept)

## Source of truth

`Pinekan/CONCEPT.md` — concept-уровень.
Старший CONCEPT — `Regenesis/CONCEPT.md` (rejuvenation umbrella).

## Что Pinekan собой представляет

Конкретная нутрицевтическая рецептура (входит в Track B Regenesis — integrative protocols). Подпроект включает:

- `CONCEPT.md`, `KNOWLEDGE.md`, `MAP.md`, `MEMORY.md`, `LINKS.md`, `PARAMETERS.md`, `README.md` — ядро
- `Recipe_Pinekan/` — собственно рецепт + ingredients + dosage
- `Logistics/` — закупка, упаковка, regulatory
- `Marketing/` — позиционирование (но коммерческий запуск через Marketing umbrella, не здесь)

## Связи

- **Regenesis Track B** — Pinekan = одна из 8 nutraceutical protocols в Track B
- **WLRAbastumani** — Pinekan может предлагаться как часть Standard/Deep Rejuvenation programs
- **Marketing umbrella / Books** — публикация brand-narrative как part of Books/Diets/ или отдельная книга
- **drjaba.com** — possible direct sales channel (Stripe), если Pinekan получит регистрацию

## Правила

1. **Подпроект НЕ имеет собственного git** (правило `feedback_subproject_git_rule`). Изменения коммитятся в parent `Regenesis-private` repo.
2. **Безопасность:** клинические утверждения — только с phrasing "интегративная нутрицевтика", **не "лекарство"** (без регистрации это запрещено в Грузии).
3. **DeepSeek для всего нетехнического** (рецептурные тексты, marketing copy, регуляторная документация) — `~/Desktop/AIM/llm.py`.
4. **Любые изменения dosage / ingredients** — обсуждать с пользователем перед записью в `Recipe_Pinekan/`.

## Что НЕ менять без явного approval

1. `Pinekan/CONCEPT.md` — концепт продукта
2. `Recipe_Pinekan/` — формула
3. Связь с Regenesis Track B (структурная — нельзя выводить Pinekan в самостоятельный umbrella без обсуждения)

```
### `README.md` (1649 chars)
```md
# Regenesis — Integrative Medicine Protocols

**Author:** Dr. Jaba Tkemaladze
**Brand:** Regenesis (luxury spa/clinic)
**Partner:** WLRAbastumani clinic, SilkAesthetic brand
**Status:** Active — protocols in clinical use

---

## What Is Regenesis

A luxury integrative medicine brand built on:
- 14 phytomedicinal protocols (`Materials/Recepturae/`, Latin names)
- Clinical device protocols (CO2 laser, RF micro-needling)
- Nutrition and breathing protocols
- Stem cell rejuvenation theory (connected to CDATA)

---

## Protocol Quick Reference

| Protocol | Category |
|----------|---------|
| Syrupus Pinus | Antiseptic phytotherapy |
| 108 Spirationes | Breathing meditation |
| Protocollum Nutritionis | Complete nutrition protocol |
| Diaeta Sana | Dietary correction (baseline healthy diet) |
| Diaeta Recta | Dietary correction (therapeutic elimination) |
| Triticum Activatum | Activated wheat (Tkemaladze method) |
| Cannabis Pasta | Topical/internal cannabis |
| Syrupus Rosae | Cardioprotective |
| Decoctum Urologicum | Urological |
| Decoctum Amygdali | Metabolic |
| Fermentum Vini | Fermented beverage |
| Protocollum Pondi | Weight management |
| Tincturae et Consuetudines | Combined tinctures |
| 5 Ieradi Kveba | Six-meal protocol |

**Total: 14 protocol files in `Materials/Recepturae/`** (8 from CONCEPT Track B table + 6 nutrition/dietary/supplementary).

---

## Languages

Patient materials in: Georgian (KA), Russian (RU), English (EN)
Protocol Latin names are the canonical identifiers.

---

## AIM Integration

Protocols are accessible in AIM via `treatment_recommender.py`.
See `MAP.md` for full ecosystem connections.

```
### `Pinekan/README.md` (1080 chars)
```md
# PINEKAN — Авторский фитобальзам

**Regenesis · Phytopharmacy Line**

---

Pinekan — функциональный фитобальзам ручного приготовления, авторская рецептура д-ра Джабы Ткемаладзе. Разработан на основе доказательной фитомедицины, прошёл 3 раунда независимого peer review.

**12 растительных компонентов. 3 раунда независимого peer review. Глиняная бутылка 360 мл. Сургучная печать.**

*БАД. Не является лекарственным средством. Перед применением проконсультируйтесь с врачом.*

---

## Состав

Адаптогенный комплекс: родиола, лимонник, элеутерококк.
Фитотоническая основа: дягиль, ягель, можжевельник, арония, водяника, сосновые почки.
Специализированные: гинкго (экстракт 24/6), девясил (инулин), солодка.

## Область применения

Может использоваться для поддержки организма при общем упадке сил, в периоды повышенных нагрузок и восстановления. Рекомендуется предварительная консультация со специалистом.

## Рекомендуемый способ приёма

10 мл один раз в день (утром, до еды). Курс 30 дней. По рекомендации врача.

## Контакт

Д-р Джаба Ткемаладзе · djabbat@gmail.com · drjaba.com

```
### `CONCEPT.md` (14946 chars)
```md
Ниже представлена **финальная концепция Regenesis на русском языке** с полным внедрением всех рекомендаций, сформулированных в ходе трех раундов peer review.

---

# Regenesis — КОНЦЕПЦИЯ (Финальная версия)

**Проект:** Regenesis  
**Автор:** Д-р Джаба Ткемаладзе  
**Бренд:** Regenesis — люксовая интегративная медицина & спа  
**Статус:** Активен — протоколы документированы, клиническое внедрение продолжается  
**Партнерская клиника:** WLRAbastumani  

---

## Основная идея

**Regenesis** — это бренд люксовой интегративной медицины, предлагающий протоколы регенеративной медицины, основанные на доказательной базе и объединяющие:

- Фитомедицину (травяные препараты в латинской рецептуре)
- Теорию стволовых клеток и науку о rejuvenation (омоложении)
- Биофизические методы лечения (CO2 фракционный лазер, RF-микронидлинг)
- Дыхательные и телесно-ориентированные практики
- Нутрициологические протоколы
- Косметологические и антивозрастные процедуры

Бренд работает на пересечении древней природной медицины и передовых исследований клеточной биологии (CDATA — центриолярная теория старения). Все протоколы разработаны д-ром Ткемаладзе и предназначены для клинического применения.

---

## Стратификация протоколов

Протоколы Regenesis разделены на два четких клинических трека для обеспечения прозрачности, безопасности и соответствия регуляторным требованиям.

### Трек A — Доказательная медицина

Процедуры с установленной клинической доказательной базой, выполняемые в соответствии с международными стандартами.

| Протокол | Описание |
|----------|----------|
| CO2 фракционный лазер | Лазерная шлифовка кожи, омоложение, лечение рубцов |
| RF-микронидлинг (PINXEL-V) | Индукция коллагена, подтяжка кожи |
| Постпроцедурный уход | Стандартизированный уход за кожей и протоколы восстановления |

Для этих процедур требуется стандартное информированное согласие в соответствии с клинической практикой.

### Трек B — Интегративные протоколы (авторская методика)

Практики, разработанные д-ром Ткемаладзе на основе клинического опыта, традиционных знаний и новых научных гипотез (CDATA). Эти методы предлагаются как дополнительные подходы, а не замена стандартной медицинской помощи.

| Файл | Протокол | Ключевое действие |
|------|----------|-------------------|
| `Syrupus_Pinus.md` | Сосновый сироп | Антисептическое: грибы, дрожжи, микоплазма, туберкулезная палочка; легкие и урогенитальный тракт |
| `108_Spirationes.md` | 108 дыханий | Медитативная дыхательная практика — 6 сенсорных каналов, 32 точки тела, 4 треугольника |
| `Decoctum_Urologicum.md` | Урологический отвар | Поддержка урологического здоровья |
| `Cannabis_Pasta.md` | Каннабисная паста | Препарат из каннабиса для местного и внутреннего применения |
| `Syrupus_Rosae.md` | Розовый сироп | Кардиопротекторное / адаптогенное действие |
| `Fermentum_Vini.md` | Ферментация вина | Протокол приготовления ферментированного напитка |
| `5_Jeradi_Kveba.md` | 5 приемов пищи | Протокол шестиразового питания |
| `Triticum_Activatum.md` | Активированная пшеница | Метод активации пшеницы по Ткемаладзе |

**Всего: 8 интегративных протоколов**

---

## Нутрициологический протокол — пересмотренная структура

Первоначальная единая жесткая диета была перестроена в два отдельных подхода:

### 1. Базовое здоровое питание (для всех пациентов)

Сбалансированный подход, ориентированный на цельные продукты, без избыточных ограничений:

| Категория | Рекомендации |
|-----------|--------------|
| Овощи | Все сезонные овощи рекомендуются; акцент на разнообразие |
| Фрукты | Сезонные фрукты в умеренных количествах, предпочтительно до обеда |
| Злаки | Цельные злаки (рис, гречка, овес) разрешены |
| Белок | Дикая рыба, ягненок, яйца (термически обработанные), бобовые |
| Жиры | Оливковое масло, нерафинированные масла разрешены в умеренных количествах |
| Молочные продукты | По желанию; при переносимости — ферментированные в небольших количествах |
| Напитки | Вода, натуральный кофе (cold brew), травяные чаи |

### 2. Терапевтическая элиминационная диета (интегративный протокол)

Структурированная элиминационная диета продолжительностью 4–6 недель, предназначенная для пациентов с подозрением на пищевую чувствительность, хроническое воспаление или по специальным клиническим показаниям. *Не предназначена для пожизненного применения.*

**Ограничения в элиминационной фазе:**
- Промышленные хлебобулочные изделия (временно)
- Свинина, птичья кожа, переработанное мясо
- Все молочные продукты (временно)
- Капустные, пасленовые (томаты, баклажаны, картофель)
- Цитрусовые, бананы, виноград (временно)
- Алкоголь (за исключением небольших количеств натурального вина после 3-й недели при наличии показаний)

**Фаза reintroduction (повторного введения):** Продукты систематически вводятся каждые 3–4 дня для выявления индивидуальных триггеров. Всем пациентам предоставляется стандартизированный дневник питания и симптомов.

**Лабораторный мониторинг (на исходном уровне и после завершения элиминационной фазы):**
- Витамин D
- Витамин B12
- Кальций
- Железо (ферритин, гемоглобин)
- Витамин C
- Цинк
- Магний

Пациенты с подтвержденной целиакией или тяжелым синдромом раздраженного кишечника (СРК) проходят элиминационную диету под наблюдением гастроэнтеролога.

---

## Протокол «108 дыханий» — клинический контекст

Медитативная последовательность из 108 дыханий, работающая через 6 сенсорных каналов:

1. **Зрение** — правый глаз, левый глаз, гениталии (треугольник)
2. **Слух** — правое ухо, левое ухо, копчик (треугольник)
3. **Обоняние** — правая ноздря, левая ноздря, солнечное сплетение (треугольник)
4. **Вкус** — зубы правой челюсти, зубы левой челюсти, яремная ямка (треугольник)
5. **Кинестезия** — 20 пальцев рук и ног последовательно (20 дыханий)
6. **Интуиция** — все 32 точки одновременно: прошлое / будущее / настоящее (3 дыхания)

**Клинический контекст:** Дыхание с частотой примерно 6 циклов в минуту (0,1 Гц) ассоциировано с улучшением вариабельности сердечного ритма (ВСР) и тонуса блуждающего нерва. Несмотря на традиционное происхождение протокола, ритм дыхания соответствует установленным психофизиологическим принципам. Пациентам с неконтролируемой артериальной гипертензией, тяжелыми тревожными расстройствами или после недавних хирургических вмешательств следует проконсультироваться с врачом перед началом практики.

---

## CDATA и исследования стволовых клеток — раскрытие статуса

Научный фундамент Regenesis включает текущие исследования центриолярной теории старения (CDATA) и трансплантации молодых стволовых клеток (YSSC).

**Текущий статус:**
- **Исследования in vitro:** Активны
- **Животные модели:** Ограниченные данные
- **Клиническое применение на людях:** В настоящее время не предлагается в качестве клинической услуги. Только исследовательская фаза.

Любое будущее расширение в сторону терапии на основе стволовых клеток будет осуществляться в рамках отдельных исследовательских протоколов с полным этическим одобрением и расширенным информированным согласием. Regenesis в настоящее время не продвигает и не предоставляет терапию стволовыми клетками.

---

## Взаимодействие лекарственных средств и фитопрепаратов (Трек B)

Следующие взаимодействия идентифицированы и оцениваются при первичном приеме пациента:

| Протокол | Потенциальное взаимодействие | Уровень риска |
|----------|------------------------------|---------------|
| `Syrupus_Pinus` (Сосна) | Может влиять на функцию почек; осторожность с НПВП, ингибиторами АПФ, диуретиками | Умеренный |
| `Cannabis_Pasta` | Взаимодействие с ферментами CYP450 (3A4, 2C9); влияет на противосудорожные, антидепрессанты, антикоагулянты, бензодиазепины | Высокий |
| `Fermentum_Vini` (вино) | Ингибиторы МАО (гипертензивный криз); метронидазол (дисульфирамоподобная реакция); антикоагулянты (потенцирование) | Высокий |
| `Syrupus_Rosae` (роза) | Известных взаимодействий минимально; осторожность с антикоагулянтами из-за теоретического содержания витамина K | Низкий |
| `Decoctum_Urologicum` | Может изменять pH мочи; осторожность с препаратами, выводящимися путем канальцевой секреции | Умеренный |

Все пациенты, принимающие хронические лекарственные препараты, проходят полный обзор медикаментозной терапии перед началом любого протокола Трека B.

---

## Противопоказания — интегративные протоколы

Следующие группы пациентов **не подходят** для протоколов Трека B (фитомедицина, элиминационная диета, интенсивные дыхательные практики) без явного разрешения лечащего врача:

- Беременность и грудное вскармливание
- Активное онкологическое заболевание (любое)
- Тяжелые аутоиммунные заболевания (СКВ, активный ревматоидный артрит и др.)
- Тяжелые нарушения функции почек или печени
- Пациенты, принимающие антикоагулянты (высокий риск взаимодействий)
- Пациенты, принимающие ингибиторы МАО (из-за взаимодействия с Fermentum Vini)
- Пациенты с расстройствами пищевого поведения в анамнезе (ограничительные диетические протоколы противопоказаны)
- Дети младше 18 лет
- Пациенты с неконтролируемой эпилепсией или судорожными расстройствами (взаимодействие каннабиса с противосудорожными препаратами)

Для всех остальных пациентов участие требует подписания **Расширенного информированного согласия на интегративные протоколы**, в котором подробно описываются традиционная природа методов, потенциальные риски (включая взаимодействия лекарств и фитопрепаратов), а также отсутствие регуляторного одобрения терапевтических заявлений в определенных юрисдикциях.

---

## Юрисдикционный дисклеймер

Регуляторный статус интегративных протоколов варьируется в зависимости от юрисдикции:

| Регион | Статус |
|--------|--------|
| **Грузия (клиническая площадка)** | Практика в рамках интегративной медицины; пациенты информированы о традиционном применении |
| **Европейский союз** | Фитотерапевтические средства регулируются как традиционные травяные лекарственные препараты; терапевтические заявления ограничены |
| **Соединенные Штаты** | Статус пищевых добавок; отсутствие одобрения FDA для заявлений о лечении заболеваний |
| **Другие страны** | Пациентам рекомендуется ознакомиться с местным законодательством в отношении травяных препаратов и препаратов на основе каннабиса |

Regenesis работает в полном соответствии с применимым местным законодательством на своей клинической площадке (WLRAbastumani, Грузия).

---

## Связь с WLRAbastumani

Протоколы Regenesis внедрены в клинике WLRAbastumani (Абастумани, Грузия). Брендирование под названием «SilkAesthetic» используется в контексте спа- и эстетической медицины. Пациентские материалы доступны на трех языках: грузинском, русском, английском.

**Операционное разделение:**
- Процедуры Трека A выполняются как стандартные медицинские услуги.
- Протоколы Трека B предлагаются как дополнительные услуги интегративной медицины с отдельной документацией и согласием.

---

## Интеграция с экосистемой AIM

| Система | Связь |
|---------|-------|
…<truncated 81 more lines>…
```
### `Pinekan/CONCEPT.md` (2103 chars)
```md
# Pinekan — КОНЦЕПЦИЯ

**Проект:** Pinekan
**Подпроект:** Regenesis
**Автор:** Д-р Джаба Ткемаладзе
**Статус:** Активен — рецептура одобрена (peer review 3 раунда, 2026-03-31)

---

## Основная идея

**Pinekan** — функциональный фитобальзам ручного приготовления, авторская рецептура д-ра Джабы Ткемаладзе на основе доказательной фитомедицины. Разработан для поддержки организма при хронической усталости, снижении адаптационных резервов и когнитивных функций. Статус: **БАД / функциональная пищевая добавка** (целевой статус — traditional herbal medicinal product, EU THR).

Концептуальная триада:
1. **Авторство** — авторская рецептура практикующего врача-фитофармаколога, 3-раундовый независимый peer review
2. **Доказательность** — все компоненты с клинически подтверждёнными дозами и источниками (EMA/HMPC, ESCOP, Cochrane)
3. **Форма** — глиняные бутылки ручной работы с сургучной печатью; аптекарский премиум-формат

---

## Целевая аудитория

- Пациенты WLRAbastumani и клиники Regenesis с жалобами на снижение энергии и адаптационного резерва
- Люди с хронической усталостью, в периоде восстановления после заболеваний
- Снижение когнитивных функций и концентрации при хроническом стрессе
- Профилактические курсы для людей 45+ в контексте коррекции биологического возраста (Ze-теория)

---

## Позиционирование в экосистеме Regenesis

Pinekan — первый фитоаптекарский продукт Regenesis.
Стратегия: лечебный бальзам → линейка (Pinekan Light, Pinekan Forte) → экспорт в аптеки класса люкс.
Совместим с протоколами Regenesis (антиоксидантный, адаптогенный треки).

---

## Уникальное торговое предложение

- Рецептура, разработанная практикующим врачом (не маркетолог)
- Прошла 3-раундовый независимый peer review
- Глиняная тара ручной работы — не косметика, а аптека
- Сургучная печать = символ аутентичности и защита от фальсификации
- Срок хранения 2 года без консервантов

---

## Ключевые документы

- `Retseptura_Pinekan/RECIPE.md` — полная рецептура (одобрена peer review 2026-03-31)
- `PARAMETERS.md` — ингредиенты, дозировки, источники
- `TODO.md` — план производства и вывода на рынок

```
### `MAP.md` (2379 chars)
```md
# Regenesis — MAP (Ecosystem Connections)

---

## Project Connections

```
Regenesis (protocols hub)
    |
    |--[treatment_recommender]----> AIM (medical AI)
    |   Recepturae protocols        → prescriptions for patients
    |   Nutrition protocol          → dietary recommendations in patient records
    |   108 Spirationes             → Ze biofeedback integration (planned)
    |
    |--[centrosomal aging theory]--> CDATA
    |   Young stem cell basis        → scientific foundation for rejuvenation claims
    |   Centriole biology            → aging biomarkers
    |
    |--[publications]--------------> OJS (longevity.ge)
    |   Clinical protocols           → Annals of Rejuvenation Science
    |   Case studies                 → Longevity Horizon
    |
    |--[Ze breathing]--------------> ZeAnastasis
    |   108 Spirationes at 0.1 Hz    → Ze biofeedback at v* (planned connection)
    |   Breathing as Ze control      → Ze-breathing protocol document (planned)
    |
    |--[clinic deployment]---------> WLRAbastumani
        SilkAesthetic brand          → physical protocols at spa/clinic
        Three-language materials     → KA / RU / EN patient handouts
```

---

## Materials Organization

```
Materials/
    Recepturae/              — 14 protocols (primary content, Latin names)
    სტატიები/               — source scientific articles (PDF)
    Presentations/           — slide decks for clinics/investors
    Pets/                    — veterinary stem cell protocols
    Proposals/               — Hevolution grant and research proposals
    Hevolution/              — unlimited periodic reverse rejuvenation technology
    ბიზნეს გეგმა/            — business plan for rejuvenation clinic
```

---

## AIM Integration Points

| AIM Module | Regenesis Data Used |
|-----------|-------------------|
| `treatment_recommender.py` | Recepturae protocols → prescriptions |
| `patient_intake.py` | Nutrition protocol → dietary compliance check |
| `medical_system.py` | Display Regenesis protocols in patient session |
| `ze_biofeedback.py` (planned) | 108 Spirationes → breathing pacing guide |

---

## Domain

| Service | Connection |
|---------|-----------|
| `regenesis.drjaba.com` (planned) | Regenesis brand website |
| `longevity.ge` | OJS publication of clinical papers |
| WLRAbastumani clinic | Physical deployment |

---

*Last updated: 2026-03-28*

```
### `Pinekan/MAP.md` (1506 chars)
```md
# Pinekan — MAP

**Подпроект:** Regenesis | v1.0 | 2026-03-31

---

## Структура проекта

```
Regenesis/
└── Pinekan/
    ├── CONCEPT.md              — идея, цели, позиционирование
    ├── TODO.md                 — задачи производства и вывода на рынок
    ├── PARAMETERS.md           — все 12+1 ингредиентов, дозы, технология
    ├── MAP.md                  — этот файл
    ├── KNOWLEDGE.md            — научная база, ссылки
    ├── MEMORY.md               — история решений
    ├── LINKS.md                — поставщики, контакты
    ├── README.md               — публичное описание продукта
    ├── UPGRADE.md              — идеи для развития
    ├── Recipe_Pinekan/
    │   └── RECIPE.md           — полная рецептура (одобрена PR v1.0, 2026-03-31)
    ├── Logistics/
    │   └── Pinekan_Logistics.xlsx — смета сырья, оборудования, упаковки (партия 50 бут.)
    └── Marketing/
        └── MARKETING_GUIDE.md  — руководство по продажам (Абастумани + интернет)
```

---

## Связи с экосистемой

```
Regenesis (бренд)
└── Pinekan (фитоаптека)
    ├── WLRAbastumani (клиника, точка продаж)
    ├── drjaba.com (интернет-магазин)
    └── CDATA (научный контекст: Ze-статус, антивозрастной трек)
```

---

## Статус компонентов

| Компонент | Статус |
|---|---|
| Рецептура | ✅ Одобрена (3 раунда peer review) |
| Тара (глина + сургуч) | ⬜ Поиск производителя |
| Сырьё (закупка) | ⬜ Запланирована |
| Пробная партия | ⬜ Запланирована |
| Регистрация ТМ | ⬜ Запланирована |
| Продажи | ⬜ После пробной партии |

```
### `PARAMETERS.md` (2442 chars)
```md
# Regenesis — PARAMETERS

Clinical and protocol parameters.

---

## Pine Cone Syrup (Syrupus Pinus)

| Parameter | Value |
|-----------|-------|
| Collection | May–June, young cones 2–5 cm, still soft |
| Layering | 3–4 cm cone pulp + 2–3 cm sugar, alternate |
| Maceration | Until mid-October (no frost exposure) |
| Dose | 1–2 teaspoons in 100–200 ml cooled boiled water |
| Frequency | 1–2 times/day, 2 hours after eating |
| Duration | 3–90 days depending on condition |
| Warning | NEVER mix with spruce or other conifer cones — dangerous |

---

## 108 Breaths (108 Spirationes)

| Parameter | Value |
|-----------|-------|
| Total breaths | 108 |
| Channels | 6 (sight, hearing, smell, taste, kinesthesia, intuition) |
| Body points | 32 individual + 4 triangle planes |
| Structure | 5 breaths per point × (4 channels × 4 points) + 20 finger/toe + 3 synthesis |
| Breath cycle | Slow inhale — retention — soft exhale |
| Position | Savasana (corpse pose) |
| Timing | After any physical exercise or stress; before eating |

---

## Nutrition Protocol Forbidden List (v. 09.03.2026)

| Category | Forbidden Items |
|----------|----------------|
| Yeast/Bread | Commercial yeast (baker's, brewer's, thermophilic); all industrial bread |
| Meat | Pork; game; blood; poultry skin; belly fat |
| Dairy | All milk; kefir; yogurt; sour cream; cottage cheese; unsalted cheese |
| Eggs | Raw or undercooked |
| Grains | Buckwheat; corn; soy; peas |
| Vegetables | Cabbage; salad leaves; tomatoes; potatoes; eggplant; raw carrots; sweet potato |
| Fruit | Honey; all juices; grapes; citrus; bananas; persimmon; apricot; figs; raisins; cherry; mulberry; melon; any fruit after noon |
| Oils | Olive oil; unrefined oils; margarine |
| Drinks | Boiled coffee; tea; beer; champagne; spirits; alkaline mineral water; juices; sodas |

---

## CO2 Fractional Laser Parameters

| Parameter | Notes |
|-----------|-------|
| Device | CO2 fractional laser (catalog in Materials) |
| Indication | Skin rejuvenation, scar treatment |
| Aftercare | Per `CO2_Laser_Aftercare_Silkaesthetic_Final.docx` |
| Brand | SilkAesthetic / Regenesis |

---

## Ze-Breathing Connection

| Parameter | Value |
|-----------|-------|
| Optimal breathing frequency | 0.1 Hz (6 breaths/min) |
| Effect | Maximum HRV → Ze stream → v ≈ v* |
| 108 breaths duration | ~18 minutes at 0.1 Hz |
| Ze metric target | v = 0.35–0.45 (healthy range) |

---

*Last updated: 2026-03-28*

```
### `Pinekan/PARAMETERS.md` (2424 chars)
```md
# Pinekan — PARAMETERS

Все компоненты финального одобренного рецепта (v1.0, 2026-03-31).

---

## Ингредиенты — полная таблица

| № | Компонент | Лат. название | Часть | г/л | мг/10 мл | Ступень экстракции | Источник обоснования |
|---|---|---|---|---|---|---|---|
| 1 | Родиола розовая | *Rhodiola rosea* L. | Корневище + корни | 30 | 300 | I (65%) | EMA/HMPC, multiple RCT |
| 2 | Лимонник китайский | *Schisandra chinensis* | Плоды | 30 | 300 | II (45%) | Panossian 2016, Phytomedicine |
| 3 | Элеутерококк | *Eleutherococcus senticosus* | Корневище + корни | 25 | 250 | I (65%) | EMA/HMPC |
| 4 | Дягиль лекарственный | *Angelica archangelica* L. | Корень | 20 | 200 | I (65%) | Традиция, ESCOP |
| 5 | Ягель (кладония) | *Cladonia spp.* | Слоевище | 15 | 150 | II (45%) | Традиционное использование. Клинические данные ограничены |
| 6 | Можжевельник | *Juniperus communis* L. | Шишкоягоды | 12 | 120 | II (45%) | ESCOP, EMA/HMPC |
| 7 | Арония черноплодная | *Aronia melanocarpa* | Плоды | 40 | 400 | II (45%) | EFSA NDA panel |
| 8 | Водяника чёрная | *Empetrum nigrum* L. | Побеги с листьями | 15 | 150 | II (45%) | Традиционное использование. Клинические данные ограничены |
| 9 | Сосна обыкновенная | *Pinus sylvestris* L. | Молодые почки | 10 | 100 | II (45%) | ESCOP. Клинические данные ограничены |
| 10 | Гинкго билоба | *Ginkgo biloba* L. | Экстракт 24/6 | 24 | 240 | II (45%) | Cochrane 2007, EMA/HMPC |
| 11 | Девясил высокий | *Inula helenium* L. | Корень | 20 | 200 | I (65%) | Inulin: EFSA |
| 12 | Солодка голая | *Glycyrrhiza glabra* L. | Корень | 5 | 50 | I (65%) | ESCOP (min. dose) |

---

## Технологические параметры

| Параметр | Значение |
|---|---|
| Конечная крепость продукта | 42% vol. (зерновой спирт) |
| Экстракция I (корни) | 65% спирт, 30 дней, 18–22°C |
| Экстракция II (ягоды/листья) | 45% спирт, 30 дней, 18–22°C |
| Стабилизация | 7–10 дней в стекле |
| Фильтрация финальная | 5 мкм |
| Стандарт. экстракты | Вводятся в купаж после мацерации, до стабилизации (гинкго 24/6) |
| Розлив | Глиняные бутылки 360 мл + сургучная печать |
| Срок хранения | 2 года (закрытая), 3–4 мес (открытая) |
| Температура хранения | +5…+20°C, темнота |

---

## Клинические параметры

| Параметр | Значение |
|---|---|
| Разовая доза | 10 мл |
| Кратность | 1 раз в день (утро, до еды) |
| Курс | 30 дней |
| Перерыв | 2–3 недели |
| Целевая популяция | Взрослые 18+, не беременные, без гипертонии |

```
### `UPGRADE.md` (394 chars)
```md
# UPGRADE.md — Regenesis

Suggestions for project development from external analysis, literature, and cross-project review.

**Format:**
```
## [YYYY-MM-DD] Title
**Source:** [what triggered this]
**Status:** [ ] proposed | [✓ approved YYYY-MM-DD] | [✓✓ implemented YYYY-MM-DD]
```

---

## Pending proposals

*(No pending proposals yet — add from literature review or cross-project analysis)*

```
### `Pinekan/UPGRADE.md` (1079 chars)
```md
# Pinekan — UPGRADE

Идеи для развития продукта и линейки.

---

## [2026-03-31] Линейка Pinekan
**Source:** Концепция продукта
**Status:** [ ] proposed

- Pinekan Classic — текущий рецепт (астения, реконвалесценция)
- Pinekan Forte — удвоенная доза адаптогенов (спортсмены, экстремальный стресс)
- Pinekan Sleep — с валерианой и мелиссой (без тонизирующих), вечерний приём
- Pinekan Junior (на основе) — безалкогольный сироп на водной основе (18-)

---

## [2026-03-31] Сертификация и экспорт
**Source:** Бизнес-стратегия Regenesis
**Status:** [ ] proposed

Получить сертификат фитопрепарата в Грузии (GMP-lite).
Выход на рынок Балтии (Латвия, Эстония) — тематическая аутентичность.
Европейская сертификация как traditional herbal medicinal product (EU THR).

---

## [2026-03-31] Клиническое исследование
**Source:** Научная репутация Regenesis
**Status:** [ ] proposed

Малое пилотное исследование (n=30): Pinekan vs placebo при синдроме хронической усталости.
Первичный endpoint: FSS-шкала (Fatigue Severity Scale) через 30 дней.
Публикация в журнале интегративной медицины.

```
### `TODO.md` (2252 chars)
```md
# Regenesis — TODO

## Правило: DeepSeek для нетехнических задач

**Код — Claude. Всё остальное — DeepSeek API.**
Протоколы на латыни, клинические обоснования, переводы, редактура.
**Ключ:** `~/.aim_env → DEEPSEEK_API_KEY` · **Вход:** `~/Desktop/AIM/llm.py`

---

## P0 — Документирование протоколов

- [x] 14 протоколов Recepturae в `Materials/Recepturae/` (Markdown, trilingual)
- [ ] Проверить каждый протокол: наличие EN / RU / KA версий — добавить недостающие языки
- [ ] Создать сводный PDF-буклет всех 14 протоколов (для пациентов)
- [ ] Добавить латинские названия к оставшимся без них

## P1 — Клинические материалы

- [ ] Перевести `REGENESIS_Clinical_Protocol_RU.docx` → EN и KA
- [ ] Создать краткий одностраничный Quick Reference Sheet для каждого протокола
- [ ] Добавить клиническое обоснование к каждому протоколу Recepturae (DeepSeek-reasoner)

## P2 — Интеграция с AIM

- [ ] Убедиться что `treatment_recommender.py` использует актуальные Regenesis протоколы
- [ ] Связать Ze-дыхательный протокол (108 Spirationes) с Ze-биофидбэком:
  - 108 Дыханий = 108 циклов 0.1 Гц → максимальная HRV → Ze v ≈ v*
  - Задокументировать связь в KNOWLEDGE.md

## P3 — Стволовые клетки / Rejuvenation track

- [ ] Обзор статей в `Materials/სტატიები/` и `Materials/Pets/Articles/`
- [ ] Оформить теоретическую базу омоложения в один документ (для OJS/Annals)
- [ ] Оценить реалистичность плана из `Materials/Proposals/`

## P4 — Публикации

- [ ] Статья в Annals of Rejuvenation Science: клинические результаты Regenesis протоколов
- [ ] Статья: Pine Cone Syrup — клинические данные

---

## P5 — Consistency gap (добавлено 2026-04-21 аудитом)

CONCEPT.md §«Структура репозитория» перечисляет 4 приложения, которых нет на диске. Либо создать их, либо убрать из CONCEPT:

- [ ] `INFORMED_CONSENT.md` — расширенная форма согласия для Track B
- [ ] `CONTRAINDICATIONS.md` — подробный список противопоказаний (черновик уже в CONCEPT §Противопоказания)
- [ ] `HERB_DRUG_INTERACTIONS.md` — таблицы взаимодействий (черновик уже в CONCEPT §Взаимодействие)
- [ ] `REINTRODUCTION_DIARY.md` — шаблон дневника (уже в CONCEPT §Приложение 1)

- [ ] Pinekan subproject: добавить `CLAUDE.md` (единственный недостающий из 10-file core)

---

*Last updated: 2026-04-21*

```
### `Pinekan/TODO.md` (2090 chars)
```md
# Pinekan — TODO

**Подпроект:** Regenesis | **Статус:** Рецептура одобрена, начало производства

---

## Приоритеты

### 🔴 СРОЧНО
- [ ] **Консультация с юристом/регулятором:** определить процедуру регистрации БАД в Грузии (Национальный центр контроля заболеваний / Агентство регулирования медицинской деятельности)
- [ ] Закупить растительное сырьё: родиола, лимонник, элеутерококк, дягиль, девясил, солодка, гинкго (стандарт. экстракт 24/6)
- [ ] Найти поставщика редкого сырья: ягель (Cladonia), водяника (Empetrum), дягиль (Etsy, Финляндия, Эстония)
- [ ] Найти гончара/мастерскую — глиняные бутылки ручной работы (объём 360 мл, наполнение 300 мл)
- [ ] Разработать дизайн этикетки (состав, КИ, QR) + печать/штамп с эмблемой Адигени (телескоп, солнце, луна, 7 звёзд)
- [ ] Заказать аптекарский сургуч (~760 г на партию 50 бутылок)

### 🟡 БЛИЖАЙШИЕ ШАГИ
- [ ] Пробная партия (первая: 50 бутылок × 300 мл = 15 литров) — технологическая отработка
- [ ] Органолептическая оценка (вкус, цвет, аромат) пробной партии
- [ ] Лабораторный анализ: содержание спирта (42% vol.), микробиологическая чистота
- [ ] Разработать чек-лист контроля качества на каждой стадии (входящее сырьё, мацерат, розлив)
- [ ] Запросить коммерческие предложения и образцы у 2–3 поставщиков по ключевым компонентам (особенно гинкго, ягель)
- [ ] Зарегистрировать ТМ «PINEKAN» (Грузия)
- [ ] ✅ Себестоимость рассчитана (Logistics/Pinekan_Logistics.xlsx): ~$17–31/бутылка, розница $60–90

### 🟢 СРЕДНЕСРОЧНО
- [ ] Изучить требования EU THR (traditional herbal medicinal product) — консультант по регуляторике ЕС
- [ ] Проработать вход в аптечные сети Грузии («Аптека №1», PSP Pharma) — скрипт + условия
- [ ] Разработать протокол назначения Pinekan в рамках Regenesis-консультаций
- [ ] Фотосъёмка продукта (контент для drjaba.com и WLRAbastumani)
- [ ] Запустить в продажу через WLRAbastumani и сайт Regenesis
- [ ] Разработать линейку: Pinekan Classic → Pinekan Forte (двойная доза адаптогенов) → Pinekan Sleep (с добавлением валерианы, без тонизирующих)
- [ ] Написать статью о бальзаме для публикации (NEEDTOWRITE)

```
### `KNOWLEDGE.md` (3877 chars)
```md
# Regenesis — KNOWLEDGE BASE

Accumulated clinical and theoretical knowledge.

---

## Why Commercial Yeast Is Forbidden

Tkemaladze Nutrition Protocol basis: commercial Saccharomyces cerevisiae (baker's, brewer's, thermophilic) colonizes the gut and persists. Unlike natural yeasts, thermophilic strains survive bread baking temperatures. Consequence: dysbiosis → intestinal permeability → systemic inflammation → autoimmune cascades.

This is why the protocol:
1. Forbids all commercial bread
2. Permits only naturally fermented products (Fermentum Vini — natural wine)
3. Recommends activated wheat (Triticum Activatum) as safe grain source

---

## Why Casein Is Forbidden

Cow's milk casein (β-casein A1) → digests into β-casomorphin-7 → crosses intestinal barrier → molecular mimicry with self-proteins → antibody production → autoimmune reactions. Tkemaladze protocol eliminates all casein sources (milk, kefir, yogurt, cottage cheese, unsalted cheese).

---

## 108 Breaths — Ze Connection

The 108 Breaths protocol done at 0.1 Hz (6 cycles/min) matches the resonant frequency for maximum HRV (heart rate variability). This is physiologically significant:
- At 0.1 Hz, RSA (respiratory sinus arrhythmia) amplitude is maximized
- Maximum HRV → ANS balance → Ze HRV stream approaches v* ≈ 0.35–0.45
- The 108 breaths at this rate takes ~18 minutes — a full ANS reset

Clinical translation: 108 Spirationes is not mystical — it is a precisely dosed parasympathetic activation protocol that can be quantified with Ze metrics.

Connection to ZeAnastasis: this breathing protocol is a candidate for the Ze biofeedback closed-loop system. Target: v → v*, τ → max.

---

## Stem Cell Theory (Regenesis Scientific Foundation)

Based on Tkemaladze's centriole aging theory (CDATA):
- Stem cells accumulate old centrioles with each division (asymmetric inheritance)
- Old centrioles → impaired spindle function → asymmetric division defects
- Net effect: stem cell pool ages → organ regeneration capacity declines
- Solution: transplantation of young stem cells (YSSCs) from gametogenesis in vitro

**Key published result:** Bone marrow transplantation from young to old mice extends lifespan (article in Materials). This is the experimental validation of the YSSC approach.

**Regenesis clinical protocol** builds on this: rejuvenation therapies target the stem cell niche, not just the symptom.

---

## Pine Cone Syrup — Mechanism

Active compounds in young (green) pine cones: terpenes, resin acids, flavonoids, vitamins C and P.
Maceration in sugar (osmotic extraction) concentrates these over months.
Solar activation enhances extraction and potentially bioactivation of certain compounds.

Antimicrobial spectrum (claimed):
- Antifungal: Candida species, dermatophytes
- Antibacterial: Klebsiella pneumoniae, Mycobacterium tuberculosis (TB bacillus)
- Anti-mycoplasma activity

Clinical targets: pulmonary infections, urogenital infections.
Safety note: Abies (spruce/fir) and other conifer cones have different and potentially toxic alkaloid profiles — NEVER substitute.

---

## Hevolution Grant Opportunity

Document: `Materials/Hevolution/The Development of an Unlimited Periodic Reverse Rejuvenation Technology.docx`

Hevolution Foundation (Saudi Arabia) funds longevity research up to $1M+.
Proposal angle: unlimited periodic reverse rejuvenation through young stem cell transplantation.
Status: document drafted, not yet submitted (as of 2026-03-28).

---

## Veterinary Track

Stem cell protocols for dogs (`Materials/Pets/`):
- Autologous mesenchymal stromal cell injection for osteoarthritis in dogs
- Sex-specific preservation of neuromuscular function via multipotent adult stem cell transplantation

Clinical veterinary data can provide faster proof-of-concept for human protocols (shorter approval timelines, willing owners).

---

*Last updated: 2026-03-28*

```
### `Pinekan/KNOWLEDGE.md` (2507 chars)
```md
# Pinekan — KNOWLEDGE

Научная база рецептуры (одобренный вариант v1.0).

---

## Ключевые источники по компонентам

| Компонент | Источник | Вывод |
|---|---|---|
| Родиола розовая | Panossian A. 2010, Phytother Res | Адаптоген, снижает усталость в RCT |
| Лимонник кит. | Panossian 2016, Phytomedicine | Адаптоген + гепатопротектор, без CYP-взаимодействий |
| Элеутерококк | EMA/HMPC 2016 | Астения, реконвалесценция — одобрен EU |
| Гинкго (24/6) | EMA/HMPC Assessment 2014 (EMA/HMPC/321097/2012); Yang G. et al. 2016 BMC Complement Altern Med | Улучшение когниции и микроциркуляции, 240 мг/сут — терапевтическая доза |
| Девясил (инулин) | EFSA J 2015 | Пребиотик, поддержка Bifidobacterium |
| Арония | Jurikova T. 2017, Molecules | Высшее содержание антоцианов среди ягод |
| Дягиль | ESCOP Monographs 2003 | Спазмолитик ЖКТ, тонизирующее |
| Можжевельник | EMA/HMPC 2010 (EMA/HMPC/212030/2008) | Диуретик, КИ: заболевания почек — не применять >4 нед. |
| Ягель (*Cladonia*) | Традиционное использование; данных EMA/HMPC нет | Противовоспалительное. Клинические данные ограничены |
| Водяника (*Empetrum*) | Традиционное использование | Флавоноиды, уросептик. Клинические данные ограничены |
| Сосновые почки | ESCOP | Отхаркивающее, фитонциды. Клинические данные ограничены |
| Солодка | ESCOP Monograph; EMA/HMPC | Противовоспалительное, мин. доза (50 мг/день) безопасна; макс. курс 4 нед. |


---

## История тёмных лечебных бальзамов

Традиция тёмных спиртовых фитобальзамов в глиняных бутылках — аптекарская форма XVIII–XIX вв., объединявшая фитотерапию и форму доставки с длительным сроком хранения. Авторская рецептура Pinekan развивает эту форму, наполняя её доказательной адаптогенной композицией.

---

## Предполагаемый механизм действия (теоретическая модель)

> ⚠️ *Данный раздел описывает предполагаемый механизм действия, основанный на фармакологических свойствах отдельных компонентов. Клиническая синергия для данного конкретного состава в целом не исследована.*

1. **Адаптогенный трек** (родиола + лимонник + элеутерококк): нормализация оси HPA (кортизол ↓), усиление ATP-синтеза, защита митохондрий
2. **Антиоксидантный трек** (арония + ягель + сосна + водяника): нейтрализация свободных радикалов, снижение системного воспаления
3. **Нейропротекторный трек** (гинкго 240 мг + лимонник): улучшение мозгового кровотока, нейропластичность, когнитивные функции
4. **Пребиотический трек** (девясил/инулин + дягиль): нормализация микробиоты → ось кишечник-мозг → иммунитет и настроение

```
### `MEMORY.md` (1802 chars)
```md
# Regenesis — SESSION MEMORY

---

## State as of 2026-03-28

- 14 Recepturae protocols documented in `Materials/Recepturae/` (Markdown, 3 languages each)
- Clinical device protocols: CO2 laser, RF micro-needling documented (RU)
- Marketing materials: Georgian booklet, flyer, trilingual event program
- Stem cell research track: articles, proposals, presentations in Materials/
- Business plan: `Materials/ბიზნეს გეგმა/`
- Core project files (CONCEPT, README, TODO, PARAMETERS, MAP, MEMORY, LINKS, KNOWLEDGE) created 2026-03-28

---

## Protocol Naming Convention

All protocol files use Latin names (canonical identifiers):
- `Syrupus_Pinus.md` — not "PineConeProtocol.md"
- `108_Spirationes.md` — numbered (108 = sacred number in Georgian/Eastern tradition)
- `Protocollum_Nutritionis.md` — not "NutritionProtocol.md"

Version tracking: Protocollum_Nutritionis is v. 09.03.2026 (expanded).

---

## Key Clinical Facts to Remember

1. **Pine Cone Syrup** — only PINE (Pinus sylvestris) cones. Spruce cones are dangerous.
2. **Nutrition protocol** — the forbidden list is comprehensive. Key logic: commercial yeast = dysbiosis; casein = autoimmune; cooked coffee = hepatotoxic; industrial bread = yeast + gluten combo.
3. **108 Breaths** — done AFTER exercise, not before. Prevents cortisol spike, adrenal fatigue.
4. **Activated wheat** (Triticum Activatum) — Tkemaladze's specific preparation method.

---

## Decisions

- Text/article tasks → DeepSeek API (deepseek-reasoner for clinical justifications)
- Code tasks → Claude
- Protocol language: Latin names canonical, trilingual content (KA/RU/EN)

---

## Next Session Tasks

- Verify all 14 Recepturae have complete trilingual content
- Create Quick Reference Sheet PDF for clinic
- Connect 108 Spirationes to Ze biofeedback documentation

---

```
### `Pinekan/MEMORY.md` (2056 chars)
```md
# Pinekan — MEMORY

История решений по проекту.

---

## 2026-03-31 — Создание проекта и одобрение рецептуры

**Решение:** Проект Pinekan создан как подпроект Regenesis. Цель — лечебный тёмный бальзам в глиняных бутылках с сургучной печатью.

**Peer review рецепта:**
- Раунд 1: 25 ингредиентов → убраны зверобой (CYP450), полынь (туйон), кора дуба (длительный приём), готу кола; снижено количество компонентов
- Раунд 2: Гинкго доза увеличена 100→240 мг
- Раунд 3: ✅ ОДОБРЕН

**Финальная рецептура:** 12 компонентов. Две ступени экстракции. Без дубовой выдержки.

**Название продукта:** Pinekan (предложено пользователем, происхождение не уточнено).

---

## 2026-04-01 — Peer review всей документации + ключевые изменения

**Изменения рецептуры:**
- Прополис **убран** из состава
- Гинкго: перенесён из Экстракции II → вводится в купаж (Шаг 4.1), не подлежит мацерации
- Крепость зафиксирована: 42% vol.
- Добавлены КИ: почечная недостаточность (можжевельник), сахарный диабет
- Ограничение курса: солодка и можжевельник — не более 4 нед. непрерывно
- Взаимодействия структурированы: «абсолютные КИ» vs «требует врачебного контроля»

**Изменения упаковки и брендинга:**
- Бутылка: 360 мл (наполнение 300 мл = 30 доз = 1 курс)
- Серебряный сургуч → просто аптекарский сургуч
- Убраны все ссылки на рижский бальзам и «балтийскую традицию»
- Позиционирование: авторская рецептура д-ра Ткемаладзе (не производная)
- Эмблема: телескоп + ☀ + ☽ + 7 звёзд (Адигенский р-н, Абастумани/Месхетия)
- Цитата: «აქეთ მზე — იქით მთვარე — უკან კიდევ ვარსკვლავები»

**Статус продукта (юридический):** БАД / функциональная пищевая добавка (до EU THR)
- «Показания» и «Дозировка» → «Область применения» и «Способ приёма»
- Дисклеймер «Не является лекарственным средством» — везде

**Peer review документации 2026-04-01:**
- Научный (RECIPE/PARAMETERS/KNOWLEDGE): Раунд 1 → НЕ ОДОБРЕН → Раунд 2 → ✅ ОДОБРЕН
- Бизнес (CONCEPT/README/TODO/LINKS/MARKETING): Раунд 1 → НЕ ОДОБРЕН → Раунд 2 → ✅ ОДОБРЕН

**Созданы:** Logistics/Pinekan_Logistics.xlsx, Marketing/MARKETING_GUIDE.md

```
### `LINKS.md` (2465 chars)
```md
# Regenesis — LINKS

---

## Internal Files

| File | Path |
|------|------|
| Clinical protocol (RU) | `Materials/REGENESIS_Clinical_Protocol_RU.docx` |
| Premium protocol (RU) | `Materials/REGENESIS_Premium_Protocol_RU.docx` |
| CO2 laser aftercare | `Materials/CO2_Laser_Aftercare_Silkaesthetic_Final.docx` |
| Georgian booklet | `Materials/REGENESIS_Booklet_Final_SilkAesthetic_GE.docx` |
| All 14 Recepturae | `Materials/Recepturae/*.md` |
| AIM treatment recommender | `~/Desktop/AIM/treatment_recommender.py` |

---

## Scientific References (Stem Cell Track)

| Article | Location |
|---------|---------|
| Hayflick replicative senescence | `Materials/სტატიები/heiflick 1.pdf` + `heiflick 2.pdf` |
| Stvolovie kletki review 2016 | `Materials/სტატიები/Stvolovie kletki_Obzor_2016.pdf` |
| Effect on lifespan of bone marrow transplantation | `Materials/სტატიები/Effect on lifespan...pdf` |
| Muscle-derived stem cells in progeria | `Materials/სტატიები/Muscle-derived stem...pdf` |
| Embryonic origins of adult stem cells | `Materials/სტატიები/Embryonic origins...pdf` |
| Hydractinia as stem cell model | `Materials/სტატიები/Hydractinia...pdf` |

---

## Priority Self-Citations (for any Regenesis publication)

1. PMID 36583780 — https://pubmed.ncbi.nlm.nih.gov/36583780/
2. PMID 20480236 — https://pubmed.ncbi.nlm.nih.gov/20480236/
3. Zenodo CDATA — https://doi.org/10.5281/zenodo.19174506
4. Zenodo Ze — https://doi.org/10.5281/zenodo.19174630

---

## Equipment References

| Equipment | File |
|-----------|------|
| CO2 fractional laser catalog | `Materials/005Newest system CO2 fractional laser--catalog.pdf` |
| Vacuum micro needle RF machine | `Materials/catalog--vacuum mrico needle RF machine.pdf` |
| Spectrometer manuals | `Materials/七色纳米蓝色包装光谱仪说明书.pdf`, `Materials/5代光谱仪(英文版).pdf` |
| Lab equipment list | `Materials/Lab. Equipment.xlsx`, `Materials/Pets/lab. Equipment.xlsx` |

---

## Publication Targets

| Journal | Venue | Type |
|---------|-------|------|
| Annals of Rejuvenation Science | longevity.ge | Clinical protocols, case studies |
| Longevity Horizon | longevity.ge | Theoretical/conceptual |

---

## Brand / Business

| Resource | URL/Path |
|----------|---------|
| WLRAbastumani clinic | (partner clinic) |
| SilkAesthetic brand | (spa/aesthetic medicine) |
| Hevolution Foundation | https://hevolution.com (grant opportunity) |
| Business plan | `Materials/ბიზნეს გეგმა/ბიზნეს გეგმა.docx` |

---

*Last updated: 2026-03-28*

```
### `Pinekan/LINKS.md` (1107 chars)
```md
# Pinekan — LINKS

## Поставщики сырья (найти)

- [ ] Родиола, элеутерококк, лимонник — российские/китайские поставщики (Altai Herbs, iHerb bulk)
- [ ] Гинкго стандарт. экстракт 24/6 — Indena, Schwabe Pharma (wholesale)

- [ ] Редкое сырьё (ягель, водяника, дягиль) — Финляндия/Эстония (Etsy, прямой импорт)

## Тара

- [ ] Гончарная мастерская Грузии (глиняные бутылки ручной работы)
- [ ] Поставщик сургуча (аптекарский, тёмно-красный/чёрный)

## Регуляторные ссылки

### Грузия
- Национальный центр контроля заболеваний и общественного здоровья Грузии (NCDC): ncdc.ge
- Агентство регулирования медицинской деятельности Грузии: moh.gov.ge
- [ ] Уточнить: процедура регистрации БАД / нутрицевтиков в Грузии

### Международные (EU/WHO)
- EMA/HMPC монографии: https://www.ema.europa.eu/en/human-regulatory/herbal-products
- ESCOP Monographs: https://escop.com/escop-monographs/
- WHO Monographs on Selected Medicinal Plants Vol 1–4
- EU Traditional Herbal Medicinal Products (THR): Directive 2004/24/EC

## Конкуренты (изучить)


- Becherovka (Чехия), Jägermeister (Германия) — международный класс бальзамов

```