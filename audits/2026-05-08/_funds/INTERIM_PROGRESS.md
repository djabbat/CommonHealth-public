# Funds-perspective audit — прогресс на 2026-05-08

## Статус фаз

| Фаза | Статус | Артефакт |
|---|---|---|
| 1. Inventory | ✅ done | `inventory/inventory.json` |
| 2. Reference extraction | ✅ done | `refs/refs.json` (643 refs, 354 unique) |
| 3. Reference verification | ✅ done | `refs/REFERENCE_AUDIT_FINAL.md` |
| 4. Per-subproject deep peer review (v1) | 🔄 idle (3/13 done) | `reviews_v1/` |
| 5. Ecosystem meta-review (v1) | ⏸ pending | scripted, ждёт окончания v1 |
| 6. Apply patches (safe-mode) | ✅ done | 12 файлов аннотированы REF_AUDIT баннерами |
| 7. v2 reviews + final meta | ⏸ pending | scripted, ждёт окончания v1 |

## Ключевые находки уже сейчас

### Reference verification (Phase 3)
- **PMID:** 218/219 OK (1 не найден: PMID 27249423)
- **DOI:** 96/132 в Crossref + 3 Zenodo (DataCite) + 1 internal (10.65649/) + **21 FAKE/MISSING**
- **arXiv/Zenodo:** все валидны
- **🔴 19 PMID↔DOI mismatches** — реальный PMID указывает на статью, не имеющую отношения к описанию в тексте (button shear 2D materials вместо AMIE/Nature 2024; ovine astrovirus вместо Centor; pulmonary embolism Geneva вместо right one).

### Худшие файлы (ranked)
| Файл | Mismatches | Fake DOIs | Total |
|---|---|---|---|
| `AIM/docs/diffdiagnosis/EVIDENCE.md` | 12 | 4 | 16 |
| `AutomatedMicroscopy/EVIDENCE.md` | 3 | 1 | 4 |
| `CDATA/EVIDENCE.md` | 0 | 6 | 6 |
| `Proteostasis/EVIDENCE.md` | 0 | 2 | 2 |
| `Ze/UPGRADE.md`, `Ze/MEMORY.md`, `Ze/CONCEPT.md`, `Ze/KNOWLEDGE.md` | 0 | 4 (Zenodo+bioRxiv 10.64898 fake) | 4 |
| `CDATA/CONCEPT.md` | 0 | 1 | 1 |
| `CytogeneticTree/KNOWLEDGE.md` | 0 | 1 | 1 |
| `HAP/LINKS.md` | 0 | 1 | 1 |
| `AIM/docs/ssa/EVIDENCE.md` | 0 | 1 | 1 |

### Подозрительный prefix `10.64898/...`
Используется в `Ze/CONCEPT.md` и `Ze/KNOWLEDGE.md` для ссылки на "BrainYears bioRxiv 2026". Однако bioRxiv DOI prefix — `10.1101`. Prefix `10.64898` относится к другой организации, либо это **выдуманный DOI**.

### Per-subproject early v1 reviews (3 done)
- **AIM** → REJECT, 1.4/5 средний по фондам. P0: 13 mismatches kill evidence base; нет pre-registration; ключевые компоненты vapor.

## Применённые safe patches
12 файлов получили баннер `<!-- REF_AUDIT_2026-05-08 -->` в начале + 36 inline маркеров `<!-- [REF_AUDIT_2026-05-08: PMID X on PubMed = different paper. Manual correction required.] -->`. Это reversible — не удаляет contentingen, не меняет claims, только сигнализирует.

**НЕ применены auto patches** (7 кандидатов): большинство из них заменили бы fake DOI на DOI **другой** статьи (например AMIE Nature 2024 → button shear 2D materials), что хуже чем исходное состояние. Каждый требует ручной проверки.

## Что осталось
1. Дождаться окончания v1 reviews (~10-15 мин total)
2. Запустить `run_full_pipeline.sh` (meta-v1 → v2 reviews → meta-v2 final)
3. Сделать executive summary для PI

## Где смотреть
- Авдит ссылок: `~/Desktop/AUDIT_FUNDS_2026-05-08/refs/REFERENCE_AUDIT_FINAL.md`
- Реview-ы: `~/Desktop/AUDIT_FUNDS_2026-05-08/reviews_v1/*.md`
- План патчей: `~/Desktop/AUDIT_FUNDS_2026-05-08/patches/PATCHES_PLAN.md`, `SAFE_PATCHES_PLAN.md`
- Логи: `~/Desktop/AUDIT_FUNDS_2026-05-08/logs/run_reviews.log`
