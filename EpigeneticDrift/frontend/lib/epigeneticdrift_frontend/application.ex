defmodule EpigeneticDriftFrontend.Application do
  use Application

  @impl true
  def start(_type, _args) do
    children = [
      EpigeneticDriftFrontendWeb.Telemetry,
      {Phoenix.PubSub, name: EpigeneticDriftFrontend.PubSub},
      EpigeneticDriftFrontendWeb.Endpoint,
      {Task.Supervisor, name: EpigeneticDriftFrontend.TaskSupervisor}
    ]

    opts = [strategy: :one_for_one, name: EpigeneticDriftFrontend.Supervisor]
    Supervisor.start_link(children, opts)
  end

  @impl true
  def config_change(changed, _new, removed) do
    EpigeneticDriftFrontendWeb.Endpoint.config_change(changed, removed)
    :ok
  end
end