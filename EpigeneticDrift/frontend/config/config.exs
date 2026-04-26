import Config

config :epigeneticdrift_frontend,
  ecto_repos: [],
  backend_url: System.get_env("BACKEND_URL", "http://localhost:3007")

config :epigeneticdrift_frontend, EpigeneticDriftFrontendWeb.Endpoint,
  url: [host: "localhost"],
  render_errors: [
    formats: [html: EpigeneticDriftFrontendWeb.ErrorHTML, json: EpigeneticDriftFrontendWeb.ErrorJSON],
    layout: false
  ],
  pubsub_server: EpigeneticDriftFrontend.PubSub,
  live_view: [signing_salt: System.get_env("LV_SALT", "GENERATE_ME")]

config :epigeneticdrift_frontend, EpigeneticDriftFrontendWeb.BackendClient,
  base_url: System.get_env("BACKEND_URL", "http://localhost:3007"),
  timeout: 30_000,
  retry: [max_attempts: 3, delay: 500]

config :logger, :console,
  format: "$time $metadata[$level] $message\n",
  metadata: [:request_id]

config :phoenix, :json_library, Jason
config :phoenix, :stacktrace_depth, 20

config :sentry,
  dsn: System.get_env("SENTRY_DSN"),
  environment_name: Mix.env(),
  enable_source_code_context: true,
  root_source_code_path: File.cwd!(),
  tags: %{app: "epigeneticdrift_frontend"},
  included_environments: [:prod]

config :phoenix_live_dashboard,
  metrics: EpigeneticDriftFrontendWeb.Telemetry,
  additional_pages: []

if config_env() == :prod, do: import_config("prod.exs")
if config_env() == :dev, do: import_config("dev.exs")