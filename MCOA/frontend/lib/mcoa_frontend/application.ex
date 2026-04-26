defmodule MCOAFrontend.Application do
  use Application

  @impl true
  def start(_type, _args) do
    children = [
      MCOAFrontendWeb.Telemetry,
      {Phoenix.PubSub, name: MCOAFrontend.PubSub},
      MCOAFrontendWeb.Endpoint,
      {Registry, keys: :unique, name: MCOAFrontend.BackendClient.Registry}
    ]

    opts = [strategy: :one_for_one, name: MCOAFrontend.Supervisor]
    Supervisor.start_link(children, opts)
  end

  @impl true
  def config_change(changed, _new, removed) do
    MCOAFrontendWeb.Endpoint.config_change(changed, removed)
    :ok
  end
end