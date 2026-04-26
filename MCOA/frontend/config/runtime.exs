import Config

config :mcoa_frontend, MCOAFrontendWeb.Endpoint,
  url: [host: System.get_env("PHX_HOST") || "localhost", port: 4002],
  http: [
    port: String.to_integer(System.get_env("PORT") || "4002"),
    transport_options: [socket_opts: [:inet6]]
  ]

config :mcoa_frontend,
  backend_url: System.get_env("BACKEND_URL") || "http://localhost:3002",
  api_timeout: String.to_integer(System.get_env("API_TIMEOUT") || "15000")

if config_env() == :prod do
  secret_key_base = System.get_env("SECRET_KEY_BASE")

  if is_nil(secret_key_base) do
    raise """
    environment variable SECRET_KEY_BASE is missing.
    You can generate one by calling: mix phx.gen.secret
    """
  end

  config :mcoa_frontend, MCOAFrontendWeb.Endpoint,
    secret_key_base: secret_key_base,
    server: true
end