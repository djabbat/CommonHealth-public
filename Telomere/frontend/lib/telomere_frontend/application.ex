defmodule TelomereFrontend.Application do
  use Application

  @impl true
  def start(_type, _args) do
    children = [
      {Phoenix.PubSub, name: TelomereFrontend.PubSub},
      TelomereFrontendWeb.Telemetry,
      TelomereFrontendWeb.Endpoint
    ]

    opts = [strategy: :one_for_one, name: TelomereFrontend.Supervisor]
    Supervisor.start_link(children, opts)
  end

  @impl true
  def config_change(changed, _new, removed) do
    TelomereFrontendWeb.Endpoint.config_change(changed, removed)
    :ok
  end
end