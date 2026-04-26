defmodule ProteostasisFrontendWeb.Telemetry do
  use Supervisor
  import Telemetry.Metrics

  def start_link(arg) do
    Supervisor.start_link(__MODULE__, arg, name: __MODULE__)
  end

  @impl true
  def init(_arg) do
    children = [
      {TelemetryMetricsPrometheus.Core, [metrics: metrics()]},
      {:telemetry_poller, measurements: periodic_measurements(), period: 10_000}
    ]

    Supervisor.init(children, strategy: :one_for_one)
  end

  def metrics do
    [
      counter("phoenix.endpoint.start.duration",
        unit: {:native, :millisecond}
      ),
      counter("phoenix.endpoint.stop.duration",
        unit: {:native, :millisecond}
      ),
      summary("phoenix.router_dispatch.stop.duration",
        tags: [:route],
        unit: {:native, :millisecond}
      ),
      summary("proteostasis_frontend.backend_client.request.duration",
        tags: [:endpoint, :status],
        unit: {:native, :millisecond}
      ),
      counter("proteostasis_frontend.backend_client.request.count",
        tags: [:endpoint, :status]
      ),
      counter("proteostasis_frontend.backend_client.request.error"),
      last_value("vm.memory.total", unit: {:byte, :kilobyte}),
      last_value("vm.total_run_queue_lengths.total"),
      last_value("vm.total_run_queue_lengths.cpu"),
      last_value("vm.total_run_queue_lengths.io")
    ]
  end

  defp periodic_measurements do
    [
      {ProteostasisFrontendWeb.BackendClient, :measure_health, []}
    ]
  end
end