defmodule MitoROSFrontendWeb.Endpoint do
  use Phoenix.Endpoint, otp_app: :mitoros_frontend

  socket "/live", Phoenix.LiveView.Socket,
    websocket: [connect_info: [session: @session_options]],
    longpoll: false

  plug Plug.RequestId
  plug Plug.Telemetry, event_prefix: [:phoenix, :endpoint]

  plug Plug.Parsers,
    parsers: [:urlencoded, :multipart, :json],
    pass: ["*/*"],
    json_decoder: Jason

  plug Plug.MethodOverride
  plug Plug.Head
  plug Plug.Session, @session_options
  plug MitoROSFrontendWeb.Router

  @session_options [
    store: :cookie,
    key: "_mitoros_frontend_key",
    signing_salt: "NV6T0qps",
    same_site: "Lax"
  ]
end