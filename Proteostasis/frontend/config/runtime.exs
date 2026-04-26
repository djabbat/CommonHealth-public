import Config

config :proteostasis_frontend, ProteostasisFrontendWeb.Endpoint,
  http: [
    ip: {0, 0, 0, 0, 0, 0, 0, 0},
    port: String.to_integer(System.get_env("PORT", "4008"))
  ],
  secret_key_base: System.fetch_env!("SECRET_KEY_BASE"),
  live_view: [signing_salt: System.get_env("LIVE_VIEW_SALT", "d6mYw8hq")]

config :proteostasis_frontend, :backend,
  url: System.get_env("BACKEND_URL", "http://localhost:3008"),
  timeout: String.to_integer(System.get_env("BACKEND_TIMEOUT", "30000"))

if config_env() == :prod do
  config :opentelemetry,
    traces_exporter: :otlp,
    processors: [
      otel_batch_processor: %{
        exporter: {:opentelemetry_exporter, %{endpoints: [{:http, 'localhost', 4318, []}]}}
      }
    ]
end