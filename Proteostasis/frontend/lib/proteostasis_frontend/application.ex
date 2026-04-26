defmodule ProteostasisFrontend.Application do
  @moduledoc false

  use Application

  @impl true
  def start(_type, _args) do
    children = [
      ProteostasisFrontendWeb.Telemetry,
      ProteostasisFrontend.PubSub,
      {Phoenix.PubSub, name: ProteostasisFrontend.PubSub},
      ProteostasisFrontendWeb.Endpoint,
      {Finch, name: ProteostasisFrontend.Finch, pools: %{default: [conn_max_idle_time: 10_000]}}
    ]

    opts = [strategy: :one_for_one, name: ProteostasisFrontend.Supervisor]
    Supervisor.start_link(children, opts)
  end

  @impl true
  def config_change(changed, _new, removed) do
    ProteostasisFrontendWeb.Endpoint.config_change(changed, removed)
    :ok
  end
end