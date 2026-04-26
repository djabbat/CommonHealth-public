import Config

if config_env() == :prod do
  config :ontogenesis_frontend, OntogenesisFrontendWeb.Endpoint,
    server: true,
    http: [port: String.to_integer(System.get_env("PORT") || "4011")],
    url: [host: System.get_env("PHX_HOST") || "localhost", port: 4011]

  secret_key_base =
    System.get_env("SECRET_KEY_BASE") ||
      raise """
      environment variable SECRET_KEY_BASE is missing.
      You can generate one by calling: mix phx.gen.secret
      """

  config :ontogenesis_frontend, OntogenesisFrontendWeb.Endpoint,
    secret_key_base: secret_key_base
end