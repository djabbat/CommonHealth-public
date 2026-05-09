## План улучшений (React‑стек, без миграции на LiveView)

### P0 — Блокеры

| № | Пункт | Затронутые файлы | Трудоёмкость | Риск |
|---|-------|------------------|--------------|------|
| 0.1 | **Вынести nginx‑конфиг из heredoc в отдельный файл** — заменить `COPY <<'EOF'` на `COPY deploy/nginx/default.conf /etc/nginx/conf.d/default.conf`; добавить `deploy/nginx/default.conf` с содержимым server‑блока. | `Dockerfile`, `deploy/nginx/default.conf` (создать) | S | низкий |
| 0.2 | **Создать единый API‑клиент на TypeScript** — `src/api/client.ts` (instance axios с `baseURL`, перехватчиками для обработки ошибок и авторизации); переписать все существующие прямые вызовы axios в компонентах на использование этого клиента. | `src/api/client.ts`, все `.tsx`/`.ts`, где используется `axios` | M | средний — требуется рефакторинг существующих вызовов, возможны регрессии |

### P1 — Важно

| № | Пункт | Затронутые файлы | Трудоёмкость |
|---|-------|------------------|--------------|
| 1.1 | **Добавить `engines` в `package.json`** — указать `"node": ">=20"` и `"npm": ">=10"`. | `package.json` | S |
| 1.2 | **Настроить unit‑тестирование (Jest + React Testing Library)** — установить зависимости (`jest`, `@testing-library/react`, `@testing-library/jest-dom`, `ts-jest`), создать `jest.config.ts`, написать тесты для ключевых компонентов (хотя бы 2–3) и для API‑клиента (mock‑сервер через `msw` или осмеяние axios). | `package.json`, `jest.config.ts`, `src/**/__tests__/*.test.tsx`, `src/api/__tests__/client.test.ts` | M |
| 1.3 | **Добавить CI (GitHub Actions)** — workflow на push/PR с шагами: установка зависимостей, ESLint, TypeScript‑проверка (`tsc --noEmit`), сборка (`npm run build`), тесты (`npm test`). | `.github/workflows/ci.yml` | L |
| 1.4 | **Удалить избыточные артефакты** — удалить `scripts/gen-icons.mjs`, а также `sharp` из `devDependencies` (иконки генерируются через `vite-plugin-pwa`); проверить, что иконки в `public/` корректно подхватываются. | `scripts/gen-icons.mjs`, `package.json` | S |

### P2 — Nice‑to‑have

| № | Пункт | Затронутые файлы | Трудоёмкость |
|---|-------|------------------|--------------|
| 2.1 | **Улучшить структуру проекта** — перейти от плоского разделения к feature‑based: `src/features/<feature>/` (компоненты, hooks, types) и `src/shared/` (общие утилиты, UI‑компоненты, конфиги). | `src/` (реорганизация каталогов) | M |
| 2.2 | **Добавить pre‑commit хуки** — установить `husky` и `lint-staged`, настроить автоматический запуск ESLint и форматирования (Prettier) при коммите. | `package.json`, `.husky/pre-commit` | S |
| 2.3 | **Сгенерировать TypeScript‑типы из Rust‑бэкенда** — если бэкенд предоставляет OpenAPI‑спецификацию, добавить скрипт `npm run gen-api-types`, вызывающий `openapi-typescript`; типы складывать в `src/api/types/`. | `package.json`, `src/api/types/`, `scripts/gen-api-types.mjs` | L |

Все пункты **P0** и **P1** закрывают замечания последней проверки: сохранён React‑стек, создан API‑клиент, добавлены тесты и CI, исправлен Dockerfile. Пункты **P2** повышают поддерживаемость и автоматизацию.