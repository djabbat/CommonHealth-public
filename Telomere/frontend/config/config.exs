import Config

config :telomere_frontend,
  ecto_repos: [],
  backend_url: System.get_env("BACKEND_URL", "http://localhost:3005")

config :telomere_frontend, TelomereFrontendWeb.Endpoint,
  url: [host: "localhost"],
  render_errors: [
    formats: [html: TelomereFrontendWeb.ErrorHTML, json: TelomereFrontendWeb.ErrorJSON],
    layout: false
  ],
  pubsub_server: TelomereFrontend.PubSub,
  live_view: [signing_salt: "qkfJXV8q"]

config :telomere_frontend, TelomereFrontendWeb.Telemetry,
  metrics_prefix: "telomere_frontend",
  period: 30_000

config :logger, :console,
  format: "$time $metadata[$level] $message\n",
  metadata: [:request_id]

config :phoenix, :json_library, Jason

config :esbuild,
  version: "0.19.11",
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

config :telomere_frontend, TelomereFrontendWeb.Clients.BackendClient,
  base_url: System.get_env("BACKEND_URL", "http://localhost:3005"),
  timeout: 15_000,
  retry: [max_retries: 3, delay: 500]

import_config "#{config_env()}.exs"