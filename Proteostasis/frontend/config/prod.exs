import Config

config :proteostasis_frontend, ProteostasisFrontendWeb.Endpoint,
  url: [host: System.get_env("PHX_HOST", "localhost"), port: 4008],
  cache_static_manifest: "priv/static/cache_manifest.json",
  http: [
    port: String.to_integer(System.get_env("PORT", "4008")),
    transport_options: [socket_opts: [:inet6]]
  ],
  server: true,
  force_ssl: [rewrite_on: [:x_forwarded_proto]]

config :logger,
  level: :info,
  backends: [:console],
  compile_time_purge_matching: [
    [level_lower_than: :info]
  ]

config :phoenix, :serve_endpoints, true