import Config

config :telomere_frontend, TelomereFrontendWeb.Endpoint,
  url: [host: System.get_env("PHX_HOST", "localhost"), port: 4005],
  cache_static_manifest: "priv/static/cache_manifest.json",
  force_ssl: [hsts: true]

config :logger, level: :info
config :phoenix, :serve_endpoints, true