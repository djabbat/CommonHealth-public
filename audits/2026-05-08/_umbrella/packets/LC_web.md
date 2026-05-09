# AUDIT PACKET — LC_web

Path: `/home/oem/Desktop/LongevityCommon/web`  Date: 2026-05-08

## Size & file counts
```
1,3M	/home/oem/Desktop/LongevityCommon/web
```
**Extensions:** .tsx=14, .ts=7, .json=4, (noext)=2, .html=1, .md=1, .sh=1, .svg=1, .mjs=1
## Tree (depth=2, max 200 entries)
```
.
./deploy
./deploy/nginx
./deploy/scripts
./tsconfig.json
./index.html
./tsconfig.node.json
./package-lock.json
./public
./public/icon.svg
./RUST_PHOENIX_MIGRATION_PLAN.md
./package.json
./src
./src/main.tsx
./src/hooks
./src/App.tsx
./src/types
./src/components
./src/pages
./src/store
./Dockerfile
./scripts
./scripts/gen-icons.mjs
./vite.config.ts
./dist
./dist/registerSW.js
./dist/index.html
./dist/manifest.webmanifest
./dist/workbox-9c191d2f.js
./dist/sw.js
./dist/icon.svg
./dist/assets
```
## Detected stack: **Node/JS, PHP**
## Core files

### `package.json` (964 chars)
```json
{
  "name": "longevitycommon-web",
  "private": true,
  "version": "0.1.0",
  "type": "module",
  "scripts": {
    "dev": "vite",
    "build": "tsc && vite build",
    "preview": "vite preview",
    "lint": "eslint src --ext ts,tsx",
    "gen-icons": "node scripts/gen-icons.mjs"
  },
  "dependencies": {
    "@tanstack/react-query": "^5.28.0",
    "axios": "^1.6.8",
    "clsx": "^2.1.0",
    "date-fns": "^3.6.0",
    "html-to-image": "^1.11.11",
    "phoenix": "^1.8.7",
    "react": "^18.3.1",
    "react-dom": "^18.3.1",
    "react-router-dom": "^6.22.0",
    "recharts": "^2.12.2",
    "zustand": "^4.5.2"
  },
  "devDependencies": {
    "@types/phoenix": "^1.6.7",
    "@types/react": "^18.3.1",
    "@types/react-dom": "^18.3.0",
    "@typescript-eslint/eslint-plugin": "^7.3.0",
    "@vitejs/plugin-react": "^4.2.1",
    "eslint": "^8.57.0",
    "sharp": "^0.33.3",
    "typescript": "^5.4.2",
    "vite": "^5.2.0",
    "vite-plugin-pwa": "^0.19.8"
  }
}

```
### `Dockerfile` (465 chars)
```
FROM node:20-alpine AS builder
WORKDIR /app
COPY package.json package-lock.json* ./
RUN npm ci || npm install
COPY . .
RUN npm run build

FROM nginx:1.27-alpine
COPY --from=builder /app/dist /usr/share/nginx/html
COPY <<'EOF' /etc/nginx/conf.d/default.conf
server {
  listen 80;
  server_name _;
  root /usr/share/nginx/html;
  index index.html;
  location / { try_files $uri $uri/ /index.html; }
  location /api/ { proxy_pass http://server:8080; }
}
EOF
EXPOSE 80

```
## Code volume
| ext | files | bytes |
|---|---|---|
| .tsx | 14 | 69613 |
| .ts | 7 | 8682 |