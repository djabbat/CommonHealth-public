defmodule OntogenesisFrontend.Application do
  use Application

  @impl true
  def start(_type, _args) do
    children = [
      OntogenesisFrontendWeb.Telemetry,
      {DNSCluster, query: Application.get_env(:ontogenesis_frontend, :dns_cluster) || :ignore},
      {Phoenix.PubSub, name: OntogenesisFrontend.PubSub},
      OntogenesisFrontendWeb.Endpoint
    ]

    opts = [strategy: :one_for_one, name: OntogenesisFrontend.Supervisor]
    Supervisor.start_link(children, opts)
  end

  @impl true
  def config_change(changed, _new, removed) do
    OntogenesisFrontendWeb.Endpoint.config_change(changed, removed)
    :ok
  end
end