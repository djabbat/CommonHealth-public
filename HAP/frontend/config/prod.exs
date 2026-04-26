import Config

config :hap_frontend, HAPFrontendWeb.Endpoint,
  http: [
    ip: {0, 0, 0, 0, 0, 0, 0, 0},
    port: String.to_integer(System.get_env("PORT") || "4010")
  ],
  url: [host: System.get_env("HOST", "example.com"), port: 80],
  cache_static_manifest: "priv/static/cache_manifest.json"

config :logger, level: :info