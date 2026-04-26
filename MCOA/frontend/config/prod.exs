import Config

config :mcoa_frontend, MCOAFrontendWeb.Endpoint,
  url: [host: "mcoa.example.com", port: 443],
  http: [
    port: 4002,
    transport_options: [socket_opts: [:inet6]]
  ],
  cache_static_manifest: "priv/static/cache_manifest.json",
  force_ssl: [hsts: true]

config :logger, level: :info

config :phoenix, :serve_endpoints, true