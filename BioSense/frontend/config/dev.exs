import Config

config :biosense_frontend, BioSenseFrontendWeb.Endpoint,
  debug_errors: true,
  code_reloader: true,
  check_origin: false,
  watchers: [
    esbuild: {Esbuild, :install_and_run, [:default, ~w(--sourcemap=inline --watch)]},
    tailwind: {Tailwind, :install_and_run, [:default, ~w(--watch)]}
  ]

config :biosense_frontend, BioSenseFrontendWeb.Endpoint,
  live_reload: [
    patterns: [
      ~r"lib/biosense_frontend_web/(controllers|live|components)/.*(ex|heex)$"
    ]
  ]

config :logger, level: :debug
config :phoenix, :stacktrace_depth, 20