import Config

config :ontogenesis_frontend, OntogenesisFrontendWeb.Endpoint,
  url: [host: System.get_env("PHX_HOST") || "localhost", port: 4011],
  cache_static_manifest: "priv/static/cache_manifest.json"

config :logger, level: :info

config :ontogenesis_frontend, OntogenesisFrontendWeb.Endpoint,
  secret_key_base: System.get_env("SECRET_KEY_BASE") || "8nQ7+LmT53oVUBk9Dy+WlJYHdhw7MIIM/J/bt5Z2uQxHhnyfzGrDYzf7L8gsLYUw"

backend_url = System.get_env("BACKEND_URL") || "http://localhost:3011"
config :ontogenesis_frontend, :backend_url, backend_url