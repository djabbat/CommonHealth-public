# Архитектура и дизайн: Модель Proteostasis (Counter #5)

## 1. Обзор архитектуры

Модель Proteostasis реализована как **Python-пакет `proteostasis`**, который является подмодулем общей вычислительной платформы MCOA. Архитектура следует принципам **модульности, проверяемости и воспроизводимости**. Модель отделена от данных, а все параметры вынесены в конфигурационные файлы.

## 2. Структура каталогов

```
proteostasis/
├── README.md # Этот файл
├── pyproject.toml # Зависимости и метаданные пакета
├── src/
│ └── proteostasis/
│ ├── __init__.py
│ ├── core.py # Основной класс модели ProteostasisCounter
│ ├── kinetics.py # Функции кинетики (f(n), g(t))
│ ├── coupling.py # Функции и матрицы связи γ
│ ├── parameters.py # Классы для загрузки и валидации параметров
│ └── metrics.py # Функции для вычисления биомаркеров D₅
├── tests/
│ ├── __init__.py
│ ├── test_core.py # Юнит-тесты основной модели
│ ├── test_kinetics.py
│ └── test_parameters.py
├── data/
│ ├── literature_params.yaml # Параметры из литературы (см. PARAMETERS.md)
│ ├── experimental_calibration.yaml # Параметры, полученные калибровкой
│ └── validation_datasets/ # Ссылки/скрипты загрузки внешних данных
├── scripts/
│ ├── calibrate.py # Скрипт калибровки модели на данных
│ ├── sensitivity_analysis.py # Анализ чувствительности (Sobol)
│ └── run_falsification_tests.py # Автоматизация тестов из OPEN_PROBLEMS.md
└── docs/
 ├── api.md # Автогенерируемая документация API
 └── examples.ipynb Jupyter-ноутбуки с примерами использования
```

## 3. API контракты

### 3.1. Основной класс `ProteostasisCounter`

```python
class ProteostasisCounter:
 """
 Модель MCOA Counter #5 (Collapse of Proteostasis).
 """

 def __init__(self, params: Union[dict, str, Path]):
 """
 Инициализация модели с параметрами.
 Args:
 params: Может быть словарём, путём к YAML-файлу или именем пресета
 ('literature', 'default').
 """
 self.params = self._load_and_validate_params(params)
 self.gamma_vector = self.params['coupling']['gamma_vector'] # Вектор γ₅ⱼ

 def calculate_D5(self, n: Union[float, np.ndarray], t: Union[float, np.ndarray],
 D_other: Optional[np.ndarray] = None) -> Union[float, np.ndarray]:
 """
 Рассчитывает повреждение D₅(n, t) с учётом связей.
 Args:
 n: Число делений (скаляр или массив).
 t: Хронологическое время (в годах, скаляр или массив).
 D_other: Массив повреждений других счётчиков [D₁, D₂, ..., D₉].
 Если None, член связи игнорируется.
 Returns:
 Значение D₅. Если n и t - массивы, возвращается массив той же формы.
 """
 # 1. Базовое повреждение
 D = self.params['D5_0']

 # 2. n-linked компонент
 n_crit = self.params['n5_star']
 if n_crit > 0:
 D += self.params['alpha5'] * (n / n_crit)

 # 3. t-linked компонент
 tau = self.params['tau5']
 if tau > 0:
 D += self.params['beta5'] * (t / tau)

 # 4. Член связей (согласно CORRECTIONS, по умолчанию gamma = 0)
 if D_other is not None:
 # D_other[0] соответствует D₁, D_other[1] - D₂ и т.д.
 # gamma_vector[0] соответствует γ₅₁, gamma_vector[1] - γ₅₂ и т.д.
 coupling_effect = np.dot(self.gamma_vector[:len(D_other)], D_other)
 D += coupling_effect

 # 5. Нелинейный порог? (Может быть добавлен позже как опция)
 # if D > self.params['collapse_threshold']:
 # D += self.params['gamma55'] * D # Положительная обратная связь

 return D

 def _load_and_validate_params(self, params_spec) -> dict:
 # Загружает параметры и проверяет обязательные поля.
 # Вызывает ошибку, если необходимые параметры отсутствуют.
 ...
```

### 3.2. Модуль `parameters`

```python
def load_preset(preset_name: str) -> dict:
 """
 Загружает именованный набор параметров.
 Доступные пресеты: 'literature', 'default_zero', 'sensitivity_baseline'.
 """
 PRESETS = {
 'default_zero': {'D5_0': 0.0, 'alpha5': 0.0, 'beta5': 0.0, 'n5_star': 1.0,
 'tau5': 1.0, 'coupling': {'gamma_vector': [0.0]*9}},
 'literature': ..., # Загружает data/literature_params.yaml
 }
 return PRESETS[preset_name]

def calibrate_to_dataset(dataset_path: Path, param_bounds: dict) -> dict:
 """
 Калибрует параметры модели на предоставленном наборе данных.
 Возвращает словарь с подобранными параметрами.
 """
 # Использует методы оптимизации (e.g., scipy.optimize).
 # dataset_path указывает на CSV/JSON с колонками: n, t, D5_measured, [D_other...].
 ...
```

### 3.3. Модуль `coupling`

```python
def calculate_gamma_from_data(df: pd.DataFrame, counter_cols: list) -> np.ndarray:
 """
 Оценивает вектор γ₅ⱼ из продольных или поперечных данных.
 Использует регрессионный анализ (например, Ridge regression), чтобы найти,
 как изменения D_other предсказывают изменения D₅.
 Args:
 df: DataFrame с колонками 'D5' и D1, D2, ..., D9.
 counter_cols: Список названий колонок для D_other (например, ['D1', 'D2']).
 Returns:
 Вектор коэффициентов γ длиной len(counter_cols).
 """
 # В соответствии с CORRECTIONS, это ПОСТ-ХОК анализ.
 # Если модель не объясняет вариацию, возвращаются нули.
 ...
```

## 4. Контракты данных

* **Входные данные для калибровки:** Файлы CSV должны содержать как минимум колонки `n` (число делений), `t` (время в годах), `D5_measured` (измеренное значение прокси для *D₅*). Опционально: `D1`, `D2`, ... (повреждения других счётчиков).
* **Выходные данные модели:** Модель возвращает скаляр или массив `D5`. Для интеграции в MCOA, вызывающий код использует тканеспецифичный вес `w₅` из общего конфигурационного файла MCOA.
* **Файлы параметров:** Используется формат YAML для удобочитаемости. Обязательные секции: `base_parameters`, `tissue_specific` (словарь по тканям), `coupling`.

## 5. Интеграция с MCOA Core

Пакет `proteostasis` регистрируется в основном реестре MCOA. Вызов для интеграции:

```python
from mcoa.core import MCOA_Model
from proteostasis import ProteostasisCounter

# Загрузка общей конфигурации MCOA
config = load_mcoa_config('mcoa_config.yaml')

# Создание экземпляра счётчика #5 с его параметрами
proteostasis_counter = ProteostasisCounter(config['counters']['5']['params'])

# Регистрация в модели MCOA
mcoa_model = MCOA_Model
mcoa_model.register_counter(5, proteostasis_counter, weight=config['tissues']['brain']['w5'])
```

---
*Эта архитектура позволяет независимо развивать модель Proteostasis, тестировать её и интегрировать в более широкую систему. Все компоненты фальсифицируемы и заменяемы.*