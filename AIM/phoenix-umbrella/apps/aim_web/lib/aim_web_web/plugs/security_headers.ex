defmodule AimWeb.Plugs.SecurityHeaders do
  @moduledoc """
  Adds CSP, HSTS (in prod), Referrer-Policy, X-Content-Type-Options,
  X-Frame-Options. CSP is permissive enough for LiveView WebSocket + inline
  styles but blocks third-party scripts.
  """
  import Plug.Conn

  @csp_dev "default-src 'self'; " <>
           "script-src 'self' 'unsafe-inline' 'unsafe-eval'; " <>
           "style-src 'self' 'unsafe-inline'; " <>
           "img-src 'self' data: blob:; " <>
           "font-src 'self' data:; " <>
           "connect-src 'self' ws: wss:; " <>
           "object-src 'none'; " <>
           "base-uri 'self'"

  @csp_prod "default-src 'self'; " <>
            "script-src 'self'; " <>
            "style-src 'self' 'unsafe-inline'; " <>
            "img-src 'self' data:; " <>
            "font-src 'self'; " <>
            "connect-src 'self' wss:; " <>
            "object-src 'none'; " <>
            "base-uri 'self'; " <>
            "frame-ancestors 'none'"

  def init(opts), do: opts

  def call(conn, _opts) do
    prod? = Application.get_env(:aim_web, :env, :dev) == :prod || System.get_env("AIM_ENV") == "prod"

    conn
    |> put_resp_header("content-security-policy", if(prod?, do: @csp_prod, else: @csp_dev))
    |> put_resp_header("x-content-type-options", "nosniff")
    |> put_resp_header("referrer-policy", "strict-origin-when-cross-origin")
    |> put_resp_header("x-frame-options", "DENY")
    |> maybe_hsts(prod?)
  end

  defp maybe_hsts(conn, true),
    do: put_resp_header(conn, "strict-transport-security", "max-age=63072000; includeSubDomains")
  defp maybe_hsts(conn, false), do: conn
end
