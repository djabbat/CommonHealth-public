# AUDIT PACKET — Iqalto_lms

Path: `/home/oem/Desktop/Iqalto/lms`  Date: 2026-05-08

## Size & file counts
```
120K	/home/oem/Desktop/Iqalto/lms
```
**Extensions:** .ex=11, .exs=3
## Tree (depth=2, max 200 entries)
```
.
./priv
./priv/repo
./mix.exs
./lib
./lib/iqalto_web
./lib/iqalto
```
## Detected stack: **Phoenix/Elixir**
## Core files

### `mix.exs` (2056 chars)
```exs
defmodule Iqalto.MixProject do
  use Mix.Project

  def project do
    [
      app:             :iqalto,
      version:         "0.1.0",
      elixir:          "~> 1.15",
      elixirc_paths:   elixirc_paths(Mix.env()),
      start_permanent: Mix.env() == :prod,
      aliases:         aliases(),
      deps:            deps(),
    ]
  end

  def application do
    [
      mod:   {Iqalto.Application, []},
      extra_applications: [:logger, :runtime_tools],
    ]
  end

  defp elixirc_paths(:test), do: ["lib", "test/support"]
  defp elixirc_paths(_),     do: ["lib"]

  defp deps do
    [
      {:phoenix,             "~> 1.7"},
      {:phoenix_ecto,        "~> 4.4"},
      {:ecto_sql,            "~> 3.10"},
      {:postgrex,            ">= 0.0.0"},
      {:phoenix_html,        "~> 4.0"},
      {:phoenix_live_reload, "~> 1.2",      only: :dev},
      {:phoenix_live_view,   "~> 0.20"},
      {:floki,               ">= 0.30.0",  only: :test},
      {:esbuild,             "~> 0.8",      runtime: Mix.env() == :dev},
      {:tailwind,            "~> 0.2",      runtime: Mix.env() == :dev},
      {:swoosh,              "~> 1.5"},
      {:finch,               "~> 0.13"},
      {:telemetry_metrics,   "~> 0.6"},
      {:telemetry_poller,    "~> 1.0"},
      {:jason,               "~> 1.2"},
      {:plug_cowboy,         "~> 2.5"},
      {:guardian,            "~> 2.3"},
      {:bcrypt_elixir,       "~> 3.0"},
      {:rustler,             "~> 0.32"},
    ]
  end

  defp aliases do
    [
      setup:           ["deps.get", "ecto.setup", "assets.setup", "assets.build"],
      "ecto.setup":    ["ecto.create", "ecto.migrate", "run priv/repo/seeds.exs"],
      "ecto.reset":    ["ecto.drop", "ecto.setup"],
      test:            ["ecto.create --quiet", "ecto.migrate --quiet", "test"],
      "assets.setup":  ["tailwind.install --if-missing", "esbuild.install --if-missing"],
      "assets.build":  ["tailwind iqalto", "esbuild iqalto"],
      "assets.deploy": ["tailwind iqalto --minify", "esbuild iqalto --minify", "phx.digest"],
    ]
  end
end

```
### code `lib/iqalto_web/router.ex`
```
defmodule IqaltoWeb.Router do
  use IqaltoWeb, :router

  import IqaltoWeb.UserAuth

  pipeline :browser do
    plug :accepts, ["html"]
    plug :fetch_session
    plug :fetch_live_flash
    plug :put_root_layout, html: {IqaltoWeb.Layouts, :root}
    plug :protect_from_forgery
    plug :put_secure_browser_headers
    plug :fetch_current_user
  end

  pipeline :api do
    plug :accepts, ["json"]
  end

  pipeline :authenticated do
    plug :require_authenticated_user
  end

  # ── Public routes ─────────────────────────────────────────────────────────
  scope "/", IqaltoWeb do
    pipe_through :browser

    get  "/",        PageController, :home
    get  "/login",   SessionController, :new
    post "/login",   SessionController, :create
    delete "/logout", SessionController, :delete
    get  "/register", RegistrationController, :new
    post "/register", RegistrationController, :create
  end

  # ── Authenticated LiveView routes ─────────────────────────────────────────
  scope "/", IqaltoWeb do
    pipe_through [:browser, :authenticated]

    live "/dashboard",          DashboardLive,    :index
    live "/simulator",          SimLauncherLive,  :index
    live "/arteli/:session_id", ArteliLive,       :index
    live "/profile",            ProfileLive,      :index
  end

  # ── JSON API ──────────────────────────────────────────────────────────────
  scope "/api/v1", IqaltoWeb.API do
    pipe_through :api

    post "/sessions",              SessionController,  :create
    post "/progress",              ProgressController, :submit
    get  "/progress/:user_id",     ProgressController, :index
    get  "/progress/:user_id/:craft/:level", ProgressController, :show
  end

  # ── Dev routes ────────────────────────────────────────────────────────────
  if Mix.env() in [:dev, :test] do
    import Phoenix.LiveDashboard.Router
    scope "/" do
      pipe_through :browser
      live_dashboard "/dev/dashboard", metrics: IqaltoWeb.Telemetry
    end
  end
end

```
## Code volume
| ext | files | bytes |
|---|---|---|
| .ex | 11 | 29179 |
| .exs | 3 | 4072 |