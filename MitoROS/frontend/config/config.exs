import Config

config :mitoros_frontend,
  ecto_repos: [],
  backend_url: System.get_env("BACKEND_URL", "http://localhost:3006")

config :mitoros_frontend, MitoROSFrontendWeb.Endpoint,
  url: [host: "localhost"],
  adapter: Bandit.PhoenixAdapter,
  render_errors: [
    formats: [html: MitoROSFrontendWeb.ErrorHTML, json: MitoROSFrontendWeb.ErrorJSON],
    layout: false
  ],
  pubsub_server: MitoROSFrontend.PubSub,
  live_view: [signing_salt: "YFPhOQtP"],
  check_origin: false

config :logger, :console,
  format: "$time $metadata[$level] $message\n",
  metadata: [:request_id]

config :phoenix, :json_library, Jason

config :esbuild,
  version: "0.17.11",
  default: [
    args:
      ~w(js/app.js --bundle --target=es2017 --outdir=../priv/static/assets --external:/fonts/* --external:/images/*),
    cd: Path.expand("../assets", __DIR__),
    env: %{"NODE_PATH" => Path.expand("../deps", __DIR__)}
  ]

config :tailwind,
  version: "3.3.2",
  default: [
    args: ~w(
      --config=tailwind.config.js
      --input=css/app.css
      --output=../priv/static/assets/app.css
    ),
    cd: Path.expand("../assets", __DIR__)
  ]

import_config "#{config_env()}.exs"