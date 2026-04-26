import Config

config :biosense_frontend, BioSenseFrontendWeb.Endpoint,
  url: [host: System.get_env("PHX_HOST", "localhost"), port: 4004],
  cache_static_manifest: "priv/static/cache_manifest.json"

config :logger, level: :info