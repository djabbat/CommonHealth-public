defmodule EpigeneticDriftFrontendWeb.Endpoint do
  use Phoenix.Endpoint, otp_app: :epigeneticdrift_frontend

  socket "/live", Phoenix.LiveView.Socket,
    websocket: [connect_info: [session: @session_options]],
    longpoll: [connect_info: [session: @session_options]]

  plug Plug.RequestId
  plug Plug.Telemetry, event_prefix: [:phoenix, :endpoint]

  plug Plug.Parsers,
    parsers: [:urlencoded, :multipart, :json],
    pass: ["*/*"],
    json_decoder: Jason,
    length: 10_000_000

  plug Plug.MethodOverride
  plug Plug.Head
  plug RemoteIp

  plug Plug.Session,
    store: :cookie,
    key: "_epigeneticdrift_frontend_key",
    signing_salt: "u1c8HEKt",
    extra: "SameSite=Lax"

  plug EpigeneticDriftFrontendWeb.Router
end