defmodule ProteostasisFrontendWeb.Endpoint do
  use Phoenix.Endpoint, otp_app: :proteostasis_frontend

  @session_options [
    store: :cookie,
    key: "_proteostasis_frontend_key",
    signing_salt: "Su9NFdKu",
    same_site: "Lax"
  ]

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
  plug ProteostasisFrontendWeb.Router
end