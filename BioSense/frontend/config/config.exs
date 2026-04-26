import Config

config :biosense_frontend,
  ecto_repos: [],
  backend_url: System.get_env("BACKEND_URL", "http://localhost:3004")

config :biosense_frontend, BioSenseFrontendWeb.Endpoint,
  url: [host: "localhost"],
  render_errors: [
    formats: [html: BioSenseFrontendWeb.ErrorHTML, json: BioSenseFrontendWeb.ErrorJSON],
    layout: false
  ],
  pubsub_server: BioSenseFrontend.PubSub,
  live_view: [signing_salt: "sJgPkL3d"]

config :logger, :console,
  format: "$time $metadata[$level] $message\n",
  metadata: [:request_id]

config :phoenix, :json_library, Jason

import_config "#{config_env()}.exs"