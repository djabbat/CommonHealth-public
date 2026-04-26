defmodule OntogenesisFrontendWeb.Endpoint do
  use Phoenix.Endpoint, otp_app: :ontogenesis_frontend

  @session_options [
    store: :cookie,
    key: "_ontogenesis_frontend_key",
    signing_salt: "KHIdrIFa",
    same_site: "Lax"
  ]

  socket "/live", Phoenix.LiveView.Socket, websocket: [connect_info: [session: @session_options]]

  plug Plug.RequestId
  plug Plug.Telemetry, event_prefix: [:phoenix, :endpoint]

  plug Plug.Parsers,
    parsers: [:urlencoded, :multipart, :json],
    pass: ["*/*"],
    json_decoder: Jason

  plug Plug.MethodOverride
  plug Plug.Head
  plug Plug.Session, @session_options
  plug OntogenesisFrontendWeb.Router
end