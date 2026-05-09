# PLAN IMPROVEMENTS — LC_realtime

## P0 (Blockers)

### 1. Add Rust native module for social-server bridge
**Action:** Create a Rust crate under `native/` (e.g. `social_bridge`) that handles `pg_notify` subscription and forwards events to Phoenix channels via NIF or sidecar. Update `mix.exs` to compile the Rust code via `rustler_precompiled` or bundle as a port.  
**Files:** `native/social_bridge/Cargo.toml`, `native/social_bridge/src/`, `mix.exs`, `config/config.exs`  
**Effort:** L | **Risk:** High — requires Rust knowledge, changes deployment pipeline, may break build.

### 2. Fix CORS origin for production
**Action:** Replace `origin: "*"` with a list of allowed origins from config (`config/prod.exs`). Add `cors_origins` env variable in `runtime.exs`.  
**Files:** `lib/longevitycommon_web/endpoint.ex`, `config/prod.exs`, `config/runtime.exs`  
**Effort:** S | **Risk:** Medium — misconfiguration can block legitimate clients.

### 3. Implement FeedNotifier with proper pg_notify handling
**Action:** Write `LongevityCommonRealtime.FeedNotifier` (GenServer) that subscribes to the `social_feeds` channel via Postgrex `pg_notify`, handles disconnects with exponential backoff, and broadcasts events via `Phoenix.PubSub`. Add to supervision tree.  
**Files:** `lib/longevitycommon_realtime/feed_notifier.ex`, `lib/longevitycommon_realtime/application.ex`  
**Effort:** M | **Risk:** High — without this, real-time feed delivery is broken.

### 4. Provide production runtime configuration (`runtime.exs`)
**Action:** Complete `config/runtime.exs` with required env variables: `DATABASE_URL`, `SECRET_KEY_BASE`, `PHX_HOST`, `ALLOWED_ORIGINS`. Ensure it’s loaded in `application.ex` via `config_change`.  
**Files:** `config/runtime.exs`, `config/prod.exs`  
**Effort:** S | **Risk:** Medium — missing config makes deployment impossible.

## P1 (Important)

### 5. Sync Elixir version constraint with Dockerfile
**Action:** Bump `mix.exs` Elixir requirement from `~> 1.14` to `~> 1.17` (or `~> 1.17-otp-27`) to match the builder image.  
**Files:** `mix.exs`  
**Effort:** S | **Risk:** Low

### 6. Add HEALTHCHECK instruction to Dockerfile
**Action:** Insert `HEALTHCHECK --interval=30s --timeout=3s CMD curl -f http://localhost:4500/health || exit 1` before `CMD`.  
**Files:** `Dockerfile`  
**Effort:** S | **Risk:** Low

### 7. Run container as non‑root user
**Action:** Add `RUN adduser --disabled-password --gecos '' app && USER app` in the final stage of Dockerfile. Adjust file permissions if needed.  
**Files:** `Dockerfile`  
**Effort:** S | **Risk:** Low

### 8. Add comprehensive tests for FeedNotifier and socket channels
**Action:** Create test files under `test/longevitycommon_realtime/` for `FeedNotifier` (pg_notify subscription, reconnect) and `test/longevitycommon_web/` for `UserSocket` (connect, join). Use `ExUnit` + `Phoenix.ChannelTest`.  
**Files:** `test/` (multiple new files), `mix.exs` (ensure test paths)  
**Effort:** M | **Risk:** Medium — missing tests allow regressions.

### 9. Implement UserSocket and channel routes
**Action:** Write `LongevityCommonRealtimeWeb.UserSocket` with `connect/2` (JWT verification) and `id/1`. Define channel topics (e.g. `feed:*`). Add socket mount in `endpoint.ex` (already exists if file present, otherwise ensure).  
**Files:** `lib/longevitycommon_web/user_socket.ex`, `lib/longevitycommon_web/router.ex` (add `channel` routes)  
**Effort:** M | **Risk:** Medium — without socket, client cannot receive real-time updates.

## P2 (Nice‑to‑have)

### 10. Enhance health endpoint with deeper checks
**Action:** Extend `HealthController.index` to verify DB connectivity (via `Repo.query`), PubSub status, and FeedNotifier alive state. Return structured JSON.  
**Files:** `lib/longevitycommon_web/controllers/health_controller.ex`, `lib/longevitycommon_web/router.ex`  
**Effort:** S | **Risk:** Low

### 11. Add retry logic and structured logging for pg_notify
**Action:** In `FeedNotifier`, implement exponential backoff on connection loss, log warnings/errors with metadata, and expose a telemetry event for monitorability.  
**Files:** `lib/longevitycommon_realtime/feed_notifier.ex`  
**Effort:** M | **Risk:** Low

### 12. Use a dedicated pg_notify library
**Action:** Replace raw `Postgrex` LISTEN/NOTIFY with `postgrex_notify` or `pg_notify` hex package to simplify subscription management and auto‑reconnect.  
**Files:** `mix.exs`, `lib/longevitycommon_realtime/feed_notifier.ex`  
**Effort:** S | **Risk:** Low