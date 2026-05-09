## P0 (Blockers)

**1. Устранить дублирование nginx-конфигов**  
- Создать скрипт `sync-nginx-configs.sh` (или Makefile target), копирующий файлы из `nginx/` в `server-state/nginx-vhosts/` и `server-state/nginx-snippets/` с проверкой контрольной суммы; затем удалить дубликаты или заменить их симлинками.  
- **Файлы:** `server-state/nginx-vhosts/*.conf`, `server-state/nginx-snippets/*.conf`, `nginx/*.conf`  
- **Трудоёмкость:** S · **Риск:** Низкий  

**2. Автоматизировать деплой eco-inject.js с атомарной сменой версии**  
- Написать скрипт `deploy-eco-inject.sh`, копирующий файл на сервер, заменяющий все вхождения `eco-inject.js?v=` (в nginx-сниппетах, Phoenix layout, статическом контенте) на новый timestamp и перезагружающий соответствующие сервисы.  
- **Файлы:** `web-shared/eco-inject.js`, `web-shared/README.md`, `server-state/nginx-snippets/eco-inject.conf`, `apps/aim_web/lib/aim_web_web/components/layouts/root.html.heex` (если есть), `server-state/web-content/ngo/index.html`, `server-state/web-content/longevitycommon-landing/index.html`  
- **Трудоёмкость:** M · **Риск:** Средний (может пропустить нестандартные ссылки)  

**3. Удалить мёртвый артефакт `docker-compose-all.OLD-pre-v5.6.yml`**  
- Убедиться, что файл не используется, затем удалить из корня или переместить в `archive/` с коротким README.  
- **Файлы:** `docker-compose-all.OLD-pre-v5.6.yml`  
- **Трудоёмкость:** S · **Риск:** Низкий  

**4. Добавить проверку статуса systemd-сервиса после перезапуска**  
- В `systemd/README.md` (секция «Rebuild after editing source») после `sudo systemctl start` добавить `systemctl status --no-pager` и рекомендацию проверить `journalctl -n 20`.  
- **Файлы:** `systemd/README.md`  
- **Трудоёмкость:** S · **Риск:** Низкий  

## P1 (Important)

**1. Удалить или документировать неиспользуемые скрипты `scripts/`**  
- Удалить `deploy_all.sh` и `deploy-app-native.sh`, если они устарели; иначе добавить comment о назначении и интеграции в процесс.  
- **Файлы:** `scripts/deploy_all.sh`, `scripts/deploy-app-native.sh`  

**2. Согласовать описания bootstrap в `systemd/README.md` и `server-state/README.md`**  
- Перенести полное описание первичного развёртывания в `systemd/README.md`, в `server-state/README.md` оставить краткий обзор с ссылкой.  
- **Файлы:** `systemd/README.md`, `server-state/README.md`  

**3. Добавить версионирование nginx-сниппетов**  
- В начало каждого `.conf` файла в `nginx/` добавить комментарий с версией (например, `# version: 1.0.0`) и датой.  
- **Файлы:** `nginx/*.conf`, `server-state/nginx-snippets/*.conf`  

## P2 (Nice-to-Have)

**1. Упростить таблицу Cloudflare DNS в `server-state/README.md`**  
- Заменить детальную таблицу на ссылку на панель Cloudflare или добавить дату последней синхронизации.  
- **Файлы:** `server-state/README.md`  

*Все предложения соответствуют ограничению стека (Rust/Phoenix, Python только для legacy).*