defmodule EpigeneticDriftFrontendWeb.Telemetry do
  use Supervisor
  import Telemetry.Metrics

  def start_link(arg) do
    Supervisor.start_link(__MODULE__, arg, name: __MODULE__)
  end

  @impl true
  def init(_arg) do
    children = [
      {:telemetry_poller, measurements: periodic_measurements(), period: 10_000}
    ]

    Supervisor.init(children, strategy: :one_for_one)
  end

  def metrics do
    [
      summary("phoenix.endpoint.stop.duration",
        unit: {:native, :millisecond}
      ),
      summary("phoenix.router_dispatch.stop.duration",
        tags: [:route],
        unit: {:native, :millisecond}
      ),
      counter("phoenix.endpoint.stop.duration",
        tags: [:status],
        unit: {:native, :millisecond}
      ),
      last_value("vm.memory.total", unit: {:byte, :kilobyte}),
      last_value("vm.system_counts.process_count"),
      summary("epigeneticdrift.backend_client.request.duration",
        tags: [:endpoint, :status],
        unit: {:native, :millisecond}
      ),
      counter("epigeneticdrift.backend_client.request.count",
        tags: [:endpoint, :status]
      ),
      counter("epigeneticdrift.live_view.mount.count",
        tags: [:live_view]
      )
    ]
  end

  defp periodic_measurements do
    [
      {EpigeneticDriftFrontendWeb.BackendClient, :measure_health, []}
    ]
  end
end