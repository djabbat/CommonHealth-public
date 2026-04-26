import Config

config :mcoa_frontend, MCOAFrontendWeb.Endpoint,
  url: [host: "localhost"],
  render_errors: [
    formats: [html: MCOAFrontendWeb.ErrorHTML, json: MCOAFrontendWeb.ErrorJSON],
    layout: false
  ],
  pubsub_server: MCOAFrontend.PubSub,
  live_view: [signing_salt: "default_salt_placeholder"],
  http: [port: 4002]

config :mcoa_frontend,
  backend_url: System.get_env("BACKEND_URL", "http://localhost:3002"),
  api_timeout: 15_000

config :logger, :console,
  format: "$time $metadata[$level] $message\n",
  metadata: [:request_id]

config :phoenix, :json_library, Jason

config :esbuild,
  version: "0.19.7",
  default: [
    args: ~w(js/app.js --bundle --target=es2020 --outdir=../priv/static/assets),
    cd: Path.expand("../assets", __DIR__),
    env: %{"NODE_PATH" => Path.expand("../deps", __DIR__)}
  ]

config :tailwind,
  version: "3.3.6",
  default: [
    args: ~w(
      --config=tailwind.config.js
      --input=css/app.css
      --output=../priv/static/assets/app.css
    ),
    cd: Path.expand("../assets", __DIR__)
  ]

import_config "#{config_env()}.exs"