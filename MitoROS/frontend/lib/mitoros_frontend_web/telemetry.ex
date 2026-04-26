defmodule MitoROSFrontendWeb.Telemetry do
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
      counter("mitoros_frontend.backend_client.request.count",
        tags: [:status]
      ),
      last_value("vm.memory.total", unit: :byte),
      last_value("vm.system_counts.process_count"),
      summary("mitoros_frontend.live_view.mount.duration",
        tags: [:live_view],
        unit: {:native, :millisecond}
      )
    ]
  end

  defp periodic_measurements do
    [
      {MitoROSFrontendWeb.BackendClient, :measure_health, []}
    ]
  end
end