import Config

config :epigeneticdrift_frontend, EpigeneticDriftFrontendWeb.Endpoint,
  debug_errors: true,
  code_reloader: true,
  check_origin: false,
  watchers: [
    esbuild: {Esbuild, :install_and_run, [:default, ~w(--sourcemap=inline --watch)]},
    tailwind: {Tailwind, :install_and_run, [:default, ~w(--watch)]}
  ],
  live_reload: [
    patterns: [
      ~r"priv/static/.*(js|css|png|jpeg|jpg|gif|svg)$",
      ~r"lib/epigeneticdrift_frontend_web/(controllers|live|components)/.*(ex|heex)$"
    ]
  ]

config :epigeneticdrift_frontend, EpigeneticDriftFrontendWeb.Endpoint,
  http: [ip: {127, 0, 0, 1}, port: String.to_integer(System.get_env("PORT") || "4007")],
  secret_key_base: "zDkYJkAKPhZZZHL1rFLLM2BnjTl6lDzNMEjfJzSfRbL0dpIdrCwrn5sW6N4hjKtM",
  server: true

config :logger, :console, format: "[$level] $message\n"
config :phoenix_live_reload, :debug, true
config :phoenix, :stacktrace_depth, 20