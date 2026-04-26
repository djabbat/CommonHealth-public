defmodule McoaWeb.Application do
  @moduledoc false

  use Application

  @impl true
  def start(_type, _args) do
    children = [
      McoaWebWeb.Telemetry,
      {Phoenix.PubSub, name: McoaWeb.PubSub},
      {Finch, name: McoaWeb.Finch},
      McoaWebWeb.Endpoint
    ]

    opts = [strategy: :one_for_one, name: McoaWeb.Supervisor]
    Supervisor.start_link(children, opts)
  end

  @impl true
  def config_change(changed, _new, removed) do
    McoaWebWeb.Endpoint.config_change(changed, removed)
    :ok
  end
end
