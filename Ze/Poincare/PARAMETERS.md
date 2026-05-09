# Poincaré — PARAMETERS

Ключевые формулы, константы и хронологические точки проекта.

---

## Ze-параметры проекта

| Параметр | Значение | Источник |
|----------|----------|---------|
| **v*** (активный агент) | ≈ 0.456 | Tkemaladze (2026), |
| **v*** (пассивный счётчик) | 1 − ln2 ≈ 0.3069 | Tkemaladze (2026) |
| **Δv** (цена агентности) | ≈ 0.1491 | v*_active − v*_passive |
| **τ_Z** | ℕ, уменьшается при T-событиях | Постулат 3 |
| **T-событие** | неправильный прогноз → τ_Z − 1 | Постулат 3 |
| **S-событие** | правильный прогноз → τ_Z без изменений | Постулат 3 |
| **Strategy A** | θ_Z(t+1) = θ_Z(t) + δ | внутренняя фильтрация |
| **Strategy B** | U_Z = argmax_U Tr(M_{j*}†M_{j*} · Uρ_Z U†) | ниш-конструкция |

---

## Отображение Пуанкаре → Ze

| Явление Пуанкаре | Ze-эквивалент | Формальное значение |
|-----------------|---------------|---------------------|
| Форсированный анализ | S-доминирование | v → 0 |
| Подсознательная инкубация | Ze-поток при v* | v ≈ 0.456, τ → max |
| Момент озарения (omnibus) | T-burst | скачок v → 1 |
| Верификация | Возврат к равновесию | v → v* |
| «Бесплодные усилия» | Глубокое S-доминирование | v ≈ 0, τ^info → 0 |
| τ-трансфер (аналогия) | Перенос структурированной интуиции | τ из одного домена → другой |
| τ-накопление (итерации) | Рост через T-события | каждая ошибка → τ − 1, но содержание растёт |

---

## Хронология Пуанкаре

| Год | Событие | Ze-интерпретация |
|-----|---------|-----------------|
| 1880 | Первая статья о кривых ОДУ | Начало геометрического Ze-потока |
| **1881** | **Озарение на ступеньке омнибуса** (фуксовы функции = неевклидова геометрия) | **Архетип T-burst** |
| 1881–84 | Переписка с Клейном, серия о фуксовых функциях | Конкурентный τ-трансфер |
| 1890 | Задача трёх тел → гомоклиническое переплетение | Хаос как предел Ze-предсказуемости |
| 1892–99 | Les Méthodes Nouvelles (3 тома) | Геометрическое пространство состояний |
| **1895** | **Analysis Situs** | Топология = инварианты τ-структуры |
| 1895–1904 | 5 дополнений к Analysis Situs | τ-накопление через исправления |
| 1902 | La Science et l'Hypothèse | Конвенционализм = фиксация θ_Z |
| **1904** | **Гипотеза Пуанкаре** | Предел геометрической Ze-интуиции |
| 1905 | «Sur la dynamique de l'électron» | Полная математика СТО без физической реинтерпретации |
| 1908 | Science et Méthode, гл. 3 | Самоотчёт о Ze-потоке |
| 1912 | Смерть (54 года) | τ_Z = 0 |

---

## Ключевые формулы Пуанкаре

```
Характеристика Эйлера: χ = V − E + F = 2 − 2g
Теорема Пуанкаре-Бендиксона: фазовый портрет планарных ОДУ
Инвариант Лоренца (1905): x² + y² + z² − c²t² = const
Группа Лоренца как группа вращений в ℝ⁴ (с воображаемым временем ict)
```

---

## Самоцитирование (использовать в статьях проекта)

```
Tkemaladze, J. (2026). Ze Theory as an Interpretive Framework for Quantum Mechanics.
 Longevity Horizon, 2(4). DOI: 
Tkemaladze, J. (2026). Mathematical formalism of Ze.
 Longevity Horizon, 2(2). DOI: 
Tkemaladze, J. (2026). Emergence of the Minkowski Metric from Ze Dynamics.
 Longevity Horizon, 2(4). DOI: 
```

---

*Обновлено: 2026-04-03*

## Pre-registration plan

**Pre-registration:** OSF project https://osf.io/TBD [плейсхолдер — реальный OSF-проект будет создан до публикации; дата регистрации будет соответствовать дате создания проекта]. (planned registration date: 2026-06-01)

**Design overview:** Historical case study of Poincaré's mathematical insights (n=3 episodes: Fuchsian functions, Analysis Situs, celestial mechanics). Each episode will be coded independently by two raters for: (a) presence of incubation period, (b) suddenness of insight, (c) match to Ze-theory T-burst criteria. Inter-rater reliability (Cohen's κ ≥ 0.70) will be reported. Primary outcome: proportion of episodes consistent with Ze-theory predictions (threshold: ≥2/3). Secondary: qualitative narrative synthesis.

## Sample size calculation

**Power analysis (placeholder):** For a binomial test of proportion (H₀: p=0.5, H₁: p>0.5), with α=0.05, power=0.80, and expected effect size of 0.8 (proportion of episodes consistent with Ze-theory), required n = 10 episodes. Given historical constraints (n=3 available episodes), the study is underpowered for frequentist inference. Therefore, results will be reported as exploratory with descriptive statistics and Bayesian analysis (Beta(1,1) prior, posterior probability of p>0.5).

**Formula sketch:** n = (Z_α/2 + Z_β)² · p(1-p) / (p - p₀)² = (1.96 + 0.84)² · 0.8·0.2 / (0.8 - 0.5)² ≈ 10.4 → n=10 required; actual n=3.
