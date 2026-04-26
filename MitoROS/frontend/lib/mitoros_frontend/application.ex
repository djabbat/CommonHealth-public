defmodule MitoROSFrontend.Application do
  use Application

  @impl true
  def start(_type, _args) do
    children = [
      MitoROSFrontendWeb.Telemetry,
      {Phoenix.PubSub, name: MitoROSFrontend.PubSub},
      MitoROSFrontendWeb.Endpoint,
      {Finch, name: MitoROSFrontend.Finch, pools: %{default: [size: 10]}}
    ]

    opts = [strategy: :one_for_one, name: MitoROSFrontend.Supervisor]
    Supervisor.start_link(children, opts)
  end

  @impl true
  def config_change(changed, _new, removed) do
    MitoROSFrontendWeb.Endpoint.config_change(changed, removed)
    :ok
  end
end