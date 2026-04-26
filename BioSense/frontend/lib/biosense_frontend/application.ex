defmodule BioSenseFrontend.Application do
  use Application

  @impl true
  def start(_type, _args) do
    children = [
      {Phoenix.PubSub, name: BioSenseFrontend.PubSub},
      BioSenseFrontendWeb.Endpoint,
      {BioSenseFrontendWeb.Telemetry, []}
    ]

    opts = [strategy: :one_for_one, name: BioSenseFrontend.Supervisor]
    Supervisor.start_link(children, opts)
  end

  @impl true
  def config_change(changed, _new, removed) do
    BioSenseFrontendWeb.Endpoint.config_change(changed, removed)
    :ok
  end
end