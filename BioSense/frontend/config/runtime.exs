import Config

if config_env() == :prod do
  config :biosense_frontend, BioSenseFrontendWeb.Endpoint,
    server: true,
    http: [port: String.to_integer(System.get_env("PORT", "4004"))],
    secret_key_base: System.fetch_env!("SECRET_KEY_BASE")

  config :biosense_frontend,
    backend_url: System.fetch_env!("BACKEND_URL")
end