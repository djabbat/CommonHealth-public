import Config

config :hap_frontend,
  ecto_repos: [],
  backend_url: System.get_env("BACKEND_URL", "http://localhost:3010")

config :hap_frontend, HAPFrontendWeb.Endpoint,
  url: [host: "localhost"],
  render_errors: [
    formats: [html: HAPFrontendWeb.ErrorHTML, json: HAPFrontendWeb.ErrorJSON],
    layout: false
  ],
  pubsub_server: HAPFrontend.PubSub,
  live_view: [signing_salt: "WE0vH/Xs"]

config :logger, :console,
  format: "$time $metadata[$level] $message\n",
  metadata: [:request_id]

config :phoenix, :json_library, Jason

import_config "#{config_env()}.exs"