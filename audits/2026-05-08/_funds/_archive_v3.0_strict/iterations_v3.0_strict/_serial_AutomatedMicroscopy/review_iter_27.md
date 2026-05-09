# Review of AutomatedMicroscopy

## Verdict
**REVISE_MAJOR**

## Scores (1-5)
- Premise: 3
- Method: 2
- Evidence: 2
- Falsif: 4
- Deliv: 3
- Novelty: 4
- Risk: 2
- RefIntegrity: 2
- EvidenceDepth: 2
- MethodDepth: 3
- Reproducibility: 2

## Checklist (✓/✗ + объяснение по каждому из 12 условий)

1. **Операционализированная фальсифицируемость (числовые пороги)**  
   ✓ Чёткие пороги: concordance >0.80, uptime >0.90, contamination <0.03, cost ≤$4,500, power analysis для concordance с N=286.

2. **Pre-registration plan (OSF placeholder + дата)**  
   ✓ Placeholder `osf.io/TBD` и дата 2026-06-01 указаны.

3. **Sample size calc (power analysis)**  
   ✓ Для concordance расчёт с формулой и подстановкой. Для contamination N=TBD, но primary endpoint покрыт.

4. **Risk matrix ≥5 rows**  
   ✓ Две матрицы (по 6 строк). Выполнено.

5. **Limitations section**  
   ✓ Явный раздел в CONCEPT.md и EVIDENCE.md (8+ пунктов).

6. **Consortium / collaboration plan**  
   ✓ Таблица с партнёрами, ролями и статусами (LongevityCommon, Bristol, Zeiss, FLIR, ThorLabs, OpenTrons). У некоторых статус TBD, но план есть.

7. **Reference reality + match**  
   ✗ **Проблемы:**  
   - Утверждение "AI-operated microscopy has precedents" опирается на Burger, Boiko, Bran — все из химии, не microscopy. Несоответствие.  
   - В EVIDENCE.md и CONCEPT.md есть [Placeholder] вместо реальных ссылок (например, "Low-cost microscope automation: [Placeholder: e.g., OpenTrons...]").  
   - Некоторые ссылки на manufacturer spec без peer-review, но это допустимо.  
   *Решение: требуется замена указанных ссылок на microscopy-specific и удаление placeholder’ов.*

8. **Отсутствие фабрикационных маркеров**  
   ✗ **Явные маркеры:**  
   - В EVIDENCE.md: `[Reference needed — placeholder]`, `[Reference removed during audit — placeholder]`.  
   - В конце EVIDENCE.md и CONCEPT.md: `[Placeholder: e.g., OpenTrons, µManager, or similar]` и `[Author(s), Year, Journal, DOI TBD]`.  
   *Маркеры не удалены; требуется их замена на реальные ссылки или удаление текста.*

9. **Внутренняя согласованность core-документов**  
   ✓ Методы и цели соответствуют; THEORY.md и CONCEPT.md не противоречат. PARAMETERS.md и OPEN_PROBLEMS.md — стабы, но не содержат конфликтов.

10. **Evidence base depth**  
    ✗ **Нарушения:**  
    - Ключевое утверждение "AI-operated microscopy feasible" не имеет ≥3 независимых источников именно по microscopy.  
    - Нет систематического обзора или мета-анализа (написано "не найден").  
    - Противоречия только упомянуты, но не обсуждены с конкре