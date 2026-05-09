# deploy/web-shared/ — cross-subdomain assets

Серверные ассеты, которые инжектятся через nginx `sub_filter` или
включаются темой OJS на ВСЕХ longevity.ge поддоменах.

## Файлы

- **`eco-inject.js`** — общий sticky header (LongevityCommon brand +
 навигация по экосистеме) + dark/light переключатель + dark theme
 CSS overrides для PKP/OJS, Tailwind, кастомных стилей.

 Грузится на каждом поддомене из `https://longevity.ge/eco-inject.js`.
 Cookie `lc_theme` на `.longevity.ge` синхронизирует тему между
 поддоменами.

 Тема, навигация и dark mode — единственный механизм; не дублировать
 в темах OJS / Phoenix / Vite.

## Deploy на server

Канонический путь на сервере: `/home/jaba/web/longevity/eco-inject.js`
(симлинки/копии в `/home/jaba/web/ngo/`, и т.д. для legacy путей).

```bash
scp deploy/web-shared/eco-inject.js \
 server:/home/jaba/web/longevity/eco-inject.js
ssh server "sudo cp /home/jaba/web/longevity/eco-inject.js \
 /home/jaba/web/ngo/eco-inject.js"
```

После деплоя — bump `?v=` query string в OJS теме / nginx snippet,
иначе CF и браузеры закэшируют старую версию.

## Bump cache version

После любого изменения JS:

```bash
ssh server 'cd /home/jaba/web/longevity && \
 grep -rl "eco-inject.js?v=" plugins/themes/ | \
 xargs sed -i "s/eco-inject.js?v=[0-9]*/eco-inject.js?v=$(date +%s | tail -c 4)/g"'
```
