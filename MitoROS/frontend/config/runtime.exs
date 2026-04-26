import Config

if config_env() == :prod do
  host = System.get_env("PHX_HOST", "example.com")
  port = String.to_integer(System.get_env("PORT", "4006"))

  config :mitoros_frontend, MitoROSFrontendWeb.Endpoint,
    url: [host: host, port: 443, scheme: "https"],
    http: [
      ip: {0, 0, 0, 0, 0, 0, 0, 0},
      port: port
    ],
    secret_key_base: System.get_env("SECRET_KEY_BASE")

  backend_url = System.fetch_env!("BACKEND_URL")
  config :mitoros_frontend, backend_url: backend_url
end