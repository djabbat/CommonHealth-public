import Config

config :epigeneticdrift_frontend, EpigeneticDriftFrontendWeb.Endpoint,
  url: [host: System.get_env("PHX_HOST", "example.com"), port: 443],
  http: [
    ip: {0, 0, 0, 0, 0, 0, 0, 0},
    port: String.to_integer(System.get_env("PORT", "4007")),
    transport_options: [socket_opts: [:inet6]]
  ],
  secret_key_base: System.get_env("SECRET_KEY_BASE"),
  server: true

config :epigeneticdrift_frontend, EpigeneticDriftFrontendWeb.BackendClient,
  base_url: System.fetch_env!("BACKEND_URL"),
  timeout: 60_000

config :logger, level: :info

config :sentry,
  dsn: System.fetch_env!("SENTRY_DSN"),
  environment_name: "production",
  release: System.get_env("RELEASE_VERSION", "0.1.0")