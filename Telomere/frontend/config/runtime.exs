import Config

if config_env() == :prod do
  config :telomere_frontend, TelomereFrontendWeb.Endpoint,
    http: [
      ip: {0, 0, 0, 0, 0, 0, 0, 0},
      port: String.to_integer(System.get_env("PORT", "4005"))
    ],
    secret_key_base: System.fetch_env!("SECRET_KEY_BASE")
end