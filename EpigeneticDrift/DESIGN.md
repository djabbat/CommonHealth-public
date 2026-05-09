# Архитектура и дизайн проекта EpigeneticDrift

## 1. Обзор

Проект реализует вычислительную модель MCOA Counter #4 (Epigenetic Drift) для симуляций, анализа чувствительности и интеграции с экспериментальными данными. Код написан в основном на **Python** с использованием научного стека (NumPy, SciPy, pandas), отдельные скрипты анализа — на **R**. Архитектура модульная, с чётким разделением теории, параметров, симулятора и анализа.

## 2. Дерево файлов

```
EpigeneticDrift/
├── README.md # Этот файл (краткий обзор)
├── THEORY.md # Формальная теория
├── EVIDENCE.md # Проверенные ссылки и данные
├── OPEN_PROBLEMS.md # Открытые проблемы
├── PARAMETERS.md # Параметры (таблица)
├── DESIGN.md # Архитектура (этот файл)
├── AGENTS.md # Инструкции для LLM
├── JOURNAL.md # Хронологический журнал
├── ROADMAP.md # План развития
├── pyproject.toml # Зависимости Python
├── requirements.txt # (Альтернатива) Зависимости
├── src/ # Исходный код
│ ├── __init__.py
│ ├── core/ # Ядро модели
│ │ ├── __init__.py
│ │ ├── axioms.py # Реализация аксиом как функций
│ │ ├── equation.py # Класс и функции для уравнения D₄
│ │ └── normalizers.py # Функции f_i для нормализации выхода
│ ├── parameters/ # Работа с параметрами
│ │ ├── __init__.py
│ │ ├── loader.py # Загрузка параметров из YAML/таблиц
│ │ └── validator.py # Проверка границ и согласованности
│ ├── simulator/ # Симулятор MCOA
│ │ ├── __init__.py
│ │ ├── tissue_sim.py # Симуляция L_tissue для одной ткани
│ │ └── multi_counter_sim.py # Совместная симуляция нескольких счетчиков
│ ├── analysis/ # Скрипты анализа
│ │ ├── __init__.py
│ │ ├── sensitivity.py # Анализ чувствительности (Соболь, Моррис)
│ │ ├── fitting.py # Подгонка параметров под данные
│ │ └── coupling_estimator.py # Оценка коэффициентов связи γ
│ ├── utils/ # Утилиты
│ │ ├── __init__.py
│ │ ├── data_loader.py # Загрузка экспериментальных данных
│ │ └── visualization.py # Построение графиков
│ └── interfaces/ # Внешние API
│ ├── __init__.py
│ ├── mcoa_api.py # API для интеграции в общую MCOA
│ └── cli.py # Командный интерфейс
├── data/ # Данные (не коммитить большие файлы)
│ ├── synthetic/ # Синтетические данные для тестов
│ ├── experimental/ # Экспериментальные данные (ссылки, малые файлы)
│ └── results/ # Результаты симуляций и анализа
├── tests/ # Юнит-тесты
│ ├── __init__.py
│ ├── test_core.py
│ ├── test_parameters.py
│ └── test_simulator.py
├── notebooks/ # Jupyter-ноутбуки для исследования
│ ├── 01_Model_Exploration.ipynb
│ ├── 02_Sensitivity_Analysis.ipynb
│ └── 03_Coupling_Estimation.ipynb
├── scripts/ # Исполняемые скрипты
│ ├── run_batch_simulation.py
│ └── generate_figures.R
└── config/ # Конфигурационные файлы
 ├── default_params.yaml # Параметры по умолчанию
 └── tissue_weights.yaml # Веса w_i для разных тканей
```

## 3. API контракты

### 3.1. Ядро: Модуль `core.equation`

```python
class EpigeneticDriftCounter:
 """
 Реализует уравнение состояния D₄.
 """
 def __init__(self, D4_0: float = 0.0, beta4: float = 1.0, tau4: float = 10.0,
 alpha4: float = 0.05, n4_star: float = 50.0, gamma_dict: Optional[Dict[str, float]] = None):
 # ... инициализация параметров ...
 # gamma_dict: {"gamma_43": 0.12, "gamma_45": 0.08, ...}

 def compute_state(self, t: Union[float, np.ndarray], n: Union[float, np.ndarray],
 other_states: Optional[Dict[str, float]] = None) -> Union[float, np.ndarray]:
 """
 Вычисляет D₄(n, t).

 Args:
 t: Хронологическое время (годы).
 n: Число делений.
 other_states: Словарь состояний других счетчиков, например {"D3": 2.5, "D5": 1.8}.

 Returns:
 Значение D₄.
 """
 # Вычисляет член времени: D4_0 + beta4 * (t / tau4)
 # Вычисляет член делений: alpha4 * (n / n4_star)
 # Вычисляет член связи: sum(gamma_4j * other_states.get(f"D{j}", 0))
 # Возвращает сумму.
```

### 3.2. Симулятор: Модуль `simulator.tissue_sim`

```python
def simulate_tissue_aging(tissue_type: str, time_range: np.ndarray,
 division_rates: Optional[np.ndarray] = None,
 counter_params: Dict[str, Any] = None) -> pd.DataFrame:
 """
 Симулирует L_tissue для заданной ткани во времени.

 Args:
 tissue_type: Ключ из config/tissue_weights.yaml (например, "muscle", "blood").
 time_range: Массив временных точек (годы).
 division_rates: Опционально, массив скоростей деления (делений/год) для каждой точки времени.
 counter_params: Словарь с параметрами всех задействованных счетчиков.

 Returns:
 DataFrame с колонками: time, L_tissue, D4, D2, D3, ... (состояния счетчиков).
 """
 # 1. Загружает веса w_i для данной ткани.
 # 2. Инициализирует объекты всех счетчиков.
 # 3. Для каждого времени вычисляет n (интегрируя division_rates).
 # 4. Вычисляет состояние каждого счетчика.
 # 5. Вычисляет L_tissue = sum(w_i * f_i(D_i)).
 # 6. Возвращает DataFrame.
```

### 3.3. API для интеграции в MCOA: Модуль `interfaces.mcoa_api`

```python
def get_counter_definition -> Dict:
 """Возвращает каноническое определение счетчика #4 для регистрации в MCOA."""
 return {
 "id": 4,
 "name": "Epigenetic Drift",
 "equation": "D4(n,t) = D4_0 + beta4*(t/tau4) + alpha4*(n/n4*) + sum(gamma_4j * Dj)",
 "parameters": ["D4_0", "beta4", "tau4", "alpha4", "n4_star", "gamma_43", "gamma_45", ...],
 "normalizer": "f4(D4) = (D4 - D4_min) / (D4_max - D4_min)" # Пример
 }

def compute_D4_for_mcoa(t: float, n: float, other_states: Dict[int, float]) -> float:
 """Функция-обёртка, которую вызывает главный симулятор MCOA."""
 # Преобразует other_states из формата {2: 1.2, 3: 0.8} в {"D2": 1.2, ...}
 # Создает объект EpigeneticDriftCounter с параметрами по умолчанию или загруженными.
 # Возвращает compute_state(t, n, other_states).
```

## 4. Конфигурация

Параметры хранятся в YAML для удобства редактирования и версионирования.

**`config/default_params.yaml`**
```yaml
epigenetic_drift:
 D4_0: 0.0
 beta4: 1.0
 tau4: 10.0
 bounds: [7.0, 15.0]
 alpha4: 0.05
 bounds: [0.01, 0.15]
 n4_star: 50.0
 bounds: [20.0, 100.0]
 couplings:
 gamma_43: 0.0 # По умолчанию 0
 bounds: [-0.5, 0.5]
 gamma_45: 0.0
 bounds: [-0.5, 0.5]
 gamma_42: 0.0
 bounds: [-0.5, 0.5]
```

## 5. Зависимости

Основные Python-пакеты (зафиксированы в `pyproject.toml`):
- numpy>=1.21
- scipy>=1.7
- pandas>=1.3
- matplotlib>=3.5
- seaborn>=0.11
- pyyaml>=6.0
- SALib>=1.4 (анализ чувствительности)
- pytest>=7.0 (для тестов)

Для R-скриптов: `scripts/generate_figures.R` зависит от `ggplot2`, `dplyr`, `cowplot`.

---