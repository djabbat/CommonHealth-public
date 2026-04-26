import Config

config :proteostasis_frontend, ProteostasisFrontendWeb.Endpoint,
  url: [host: "localhost"],
  render_errors: [
    formats: [html: ProteostasisFrontendWeb.ErrorHTML, json: ProteostasisFrontendWeb.ErrorJSON],
    layout: false
  ],
  pubsub_server: ProteostasisFrontend.PubSub,
  live_view: [signing_salt: "d6mYw8hq"]

config :proteostasis_frontend, ProteostasisFrontendWeb.Telemetry,
  metrics_prefix: "proteostasis.frontend",
  namespace: ProteostasisFrontend

config :logger, :console,
  format: "$time $metadata[$level] $message\n",
  metadata: [:request_id]

config :phoenix, :json_library, Jason

config :esbuild,
  version: "0.19.7",
  default: [
    args:
      ~w(js/app.js --bundle --target=es2020 --outdir=../priv/static/assets --external:/fonts/* --external:/images/*),
    cd: Path.expand("../assets", __DIR__),
    env: %{"NODE_PATH" => Path.expand("../deps", __DIR__)}
  ]

config :tailwind,
  version: "3.4.0",
  default: [
    args: ~w(
      --config=tailwind.config.js
      --input=css/app.css
      --output=../priv/static/assets/app.css
    ),
    cd: Path.expand("../assets", __DIR__)
  ]

config :proteostasis_frontend, :backend,
  url: System.get_env("BACKEND_URL", "http://localhost:3008"),
  timeout: 30_000,
  retry: [max_attempts: 3, base_backoff: 100, max_backoff: 5_000]

config :proteostasis_frontend, ProteostasisFrontendWeb.BackendClient,
  adapter: Req,
  timeout: 30_000

if config_env() == :prod do
  config :proteostasis_frontend, ProteostasisFrontendWeb.Endpoint,
    cache_static_manifest: "priv/static/cache_manifest.json",
    server: true
end