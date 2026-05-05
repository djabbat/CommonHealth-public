import Config

config :biosense_web,
  backend_url: System.get_env("BIOSENSE_BACKEND_URL") || "http://127.0.0.1:4101"

if config_env() == :prod do
  secret_key_base =
    System.get_env("SECRET_KEY_BASE") ||
      raise "SECRET_KEY_BASE missing — generate with `mix phx.gen.secret`."

  host = System.get_env("PHX_HOST") || "localhost"
  port = String.to_integer(System.get_env("PORT") || "4100")

  config :biosense_web, BiosenseWebWeb.Endpoint,
    url: [host: host, port: 443, scheme: "https"],
    http: [ip: {127, 0, 0, 1}, port: port],
    secret_key_base: secret_key_base,
    server: true
end
