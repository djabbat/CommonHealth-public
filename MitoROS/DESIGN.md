# Архитектура и дизайн проекта MitoROS

## 1. Обзор

Проект MitoROS реализует вычислительную модель Counter #3 в рамках экосистемы LongevityCommon. Кодовая база предназначена для:
1. **Симуляции** траекторий накопления повреждений \( D_3(n, t) \) по заданным параметрам.
2. **Анализа экспериментальных данных** (уровни гетероплазмии, 8-oxo-dG) для оценки параметров модели.
3. **Интеграции** с другими счётчиками MCOA через определённые API.

## 2. Дерево файлов

```
MitoROS/
├── README.md # Общее описание (этот файл верхнего уровня)
├── docs/
│ ├── THEORY.md # Формальная теория
│ ├── EVIDENCE.md # Эмпирическая база
│ ├── OPEN_PROBLEMS.md # Открытые проблемы
│ ├── PARAMETERS.md # Таблица параметров
│ └── JOURNAL.md # Журнал изменений
├── src/ # Исходный код
│ ├── core/
│ │ ├── __init__.py
│ │ ├── counter.py # Класс MitoROSCounter
│ │ ├── kinetics.py # Функции для D3(n,t), f3(D3)
│ │ └── parameters.py # Загрузка и валидация параметров
│ ├── simulation/
│ │ ├── __init__.py
│ │ ├── simulator.py # Симулятор для одной ткани/клетки
│ │ └── monte_carlo.py # Стохастические симуляции (дрейф)
│ ├── analysis/
│ │ ├── __init__.py
│ │ ├── fitting.py # Подгонка параметров под данные
│ │ ├── sensitivity.py # Анализ чувствительности Sobol/Morris
│ │ └── visualization.py # Построение графиков
│ └── data_processing/
│ ├── __init__.py
│ ├── heteroplasmy_tools.py # Чтение/обработка данных ddPCR/NGS
│ └── oxidative_damage_tools.py # Обработка данных LC-MS/MS
├── tests/ # Юнит-тесты и интеграционные тесты
│ ├── __init__.py
│ ├── test_counter.py
│ ├── test_kinetics.py
│ └── test_fitting.py
├── examples/ # Jupyter-ноутбуки с примерами
│ ├── 01_basic_simulation.ipynb
│ ├── 02_parameter_fitting.ipynb
│ └── 03_sensitivity_analysis.ipynb
├── data/ # Данные (в .gitignore, структура для ссылок)
│ ├── external/ # Референсные данные из литературы
│ ├── processed/ # Обработанные данные
│ └── raw/ # Сырые данные (не коммитить)
├── config/ # Конфигурационные файлы
│ ├── default_params.yaml # Параметры по умолчанию
│ └── tissue_profiles.yaml # Предустановленные тканеспецифичные профили (w3, τ3 и т.д.)
└── environment.yml # Conda environment для воспроизводимости
```

## 3. API контракты

### 3.1. Класс `MitoROSCounter` (src/core/counter.py)

```python
class MitoROSCounter:
 """
 Реализация MCOA Counter #3.
 """
 def __init__(self, params: Dict[str, float], tissue_type: str = "generic"):
 """
 Инициализация счётчика.
 Args:
 params: Словарь с параметрами (alpha, beta, tau, n_star, etc.).
 Может быть загружен из config/default_params.yaml.
 tissue_type: Тип ткани для выбора тканеспецифичных констант.
 """
 self.params = self._validate_params(params)
 self.tissue = tissue_type
 self._state = {"D": self.params.get("D0", 0.0), "n": 0, "t": 0.0}

 def step(self, delta_n: int = 0, delta_t: float = 0, other_counters: Dict[int, float] = None) -> float:
 """
 Обновляет состояние счётчика на заданное число делений и время.
 Args:
 delta_n: Изменение в числе делений.
 delta_t: Изменение во времени (в годах).
 other_counters: Словарь {counter_id: D_value} для учёта связей (Γ).
 Если None, связи игнорируются (по умолчанию).
 Returns:
 Новое значение D3.
 """
 # Вычисление приращения по формуле D3(n,t)
 dD_division = self.params["alpha"] * (delta_n / self.params["n_star"])
 dD_time = self.params["beta"] * (delta_t / self.params["tau"])
 dD_coupling = 0.0
 if other_counters:
 # Вычисление вклада связей (реализация по умолчанию возвращает 0)
 dD_coupling = self._compute_coupling(other_counters)
 delta_D = dD_division + dD_time + dD_coupling

 # Обновление состояния
 self._state["n"] += delta_n
 self._state["t"] += delta_t
 self._state["D"] += delta_D
 return self._state["D"]

 def get_contribution(self) -> float:
 """
 Вычисляет вклад этого счётчика в общий фенотип старения L_tissue.
 Returns:
 w3 * f3(D3)
 """
 from .kinetics import f3_contribution
 w = self._get_tissue_weight(self.tissue) # Из конфига
 return w * f3_contribution(self._state["D"], self.params)

 def _compute_coupling(self, other_counters: Dict[int, float]) -> float:
 """Вычисляет член связи. Базовая реализация возвращает 0."""
 # В будущем может загружаться матрица Γ из конфига
 return 0.0

 # ... другие вспомогательные методы
```

### 3.2. Функции кинетики (src/core/kinetics.py)

```python
def d3_accumulation(n: float, t: float, params: Dict) -> float:
 """Прямой расчёт D3 по основному уравнению (без учёта текущего состояния)."""
 D0 = params.get("D0", 0.0)
 alpha = params.get("alpha", 0.0)
 n_star = params.get("n_star", 1e4)
 beta = params.get("beta", 0.1)
 tau = params.get("tau", 10.0)
 return D0 + alpha * (n / n_star) + beta * (t / tau)

def f3_contribution(D3: float, params: Dict) -> float:
 """Функция вклада f3. По умолчанию — линейная, с опцией сигмоиды."""
 f_type = params.get("contribution_function", "linear")
 if f_type == "linear":
 return D3
 elif f_type == "sigmoid":
 k = params.get("k", 10.0)
 threshold = params.get("D_threshold", 0.5)
 return 1 / (1 + np.exp(-k * (D3 - threshold)))
 else:
 raise ValueError(f"Unknown contribution function: {f_type}")
```

### 3.3. API для интеграции с MCOA Core

MCOA Core (отдельный проект) будет обращаться к MitoROS через следующий интерфейс:

```python
# Примерный вызов из MCOA Core
from MitoROS.src.core.counter import MitoROSCounter

# Инициализация счётчика для конкретной ткани
mito_params = load_params("config/muscle_params.yaml")
counter_3 = MitoROSCounter(mito_params, tissue_type="skeletal_muscle")

# В цикле симуляции MCOA:
for step in simulation_steps:
 # MCOA Core вычисляет delta_n и delta_t для ткани
 D3_value = counter_3.step(delta_n=current_delta_n, delta_t=current_delta_t)
 contribution_3 = counter_3.get_contribution
 # MCOA Core суммирует contribution_3 с вкладами других счётчиков
```

## 4. Конфигурация

Параметры модели хранятся в YAML-файлах для лёгкого изменения и версионирования.

**default_params.yaml:**
```yaml
# Параметры по умолчанию (generic tissue)
D0: 0.0
alpha: 0.0002
n_star: 5000
beta: 0.12
tau: 15.0
contribution_function: "sigmoid"
k: 15.0
D_threshold: 0.4
# Коэффициенты связи (все 0 по канону)
gamma:
 from_counter_1: 0.0
 from_counter_2: 0.0
 from_counter_4: 0.0
```

**tissue_profiles.yaml:**
```yaml
# Определения тканеспецифичных профилей
tissue_profiles:
 skeletal_muscle:
 weight_w3: 0.25 # Априорный вес w3 для мышцы
 tau: 12.0 # Скорректированное характеристическое время
 beta: 0.15 # Скорректированная скорость
 comment: "Постмитотическая, высокий метаболизм"
 intestinal_epithelium:
 weight_w3: 0.05
 alpha: 0.0003
 n_star: 3000
 tau: 50.0 # Очень большое, так как обновление быстрое
 beta: 0.02
 comment: "Высокая пролиферация, низкий окислительный метаболизм"
 neuron:
 weight_w3: 0.30
 alpha: 0.0
 tau: 8.0
 beta: 0.18
 comment: "Постмитотическая, очень высокий метаболизм, низкая репарация"

## Compliance with CORRECTIONS_2026‑04‑22

- All references to MCOA Test 2 and χ_Ze have been removed.
- Coupling coefficients Γ are set to zero by default (hypothesis of independence).
- No self‑citations (Tkemaladze, Chichinadze).

## Risk Monitoring Module

**File:** `src/analysis/risk_monitor.py`

**Purpose:** Track mitigation progress and update risk matrix dynamically during project execution.

**API:**
- `update_risk(risk_id, new_probability, new_impact)` — update risk status after milestone
- `get_risk_matrix()` — return current risk matrix as DataFrame
- `log_mitigation(risk_id, action, date)` — record completed mitigation steps

**Usage:** Called after each experiment milestone (P0-1, P0-2, P1-1, P2-1) to reassess risks.

## 5. RiskMonitor Module

```python
class RiskMonitor:
    def __init__(self, risk_matrix: pd.DataFrame):
        self.risk_matrix = risk_matrix
        self.status = {row['Risk ID']: {'probability': row['Probability'], 'impact': row['Impact'], 'mitigation': row['Mitigation']} for _, row in risk_matrix.iterrows()}
    
    def update_risk(self, risk_id: str, p: str, i: str):
        if risk_id in self.status:
            self.status[risk_id]['probability'] = p
            self.status[risk_id]['impact'] = i
    
    def get_status(self) -> dict:
        return self.status
```
This module is called after each experiment from OPEN_PROBLEMS.md to update risk assessments based on empirical results.
