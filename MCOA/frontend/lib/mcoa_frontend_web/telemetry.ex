defmodule MCOAFrontendWeb.Telemetry do
  use Supervisor
  import Telemetry.Metrics

  def start_link(arg) do
    Supervisor.start_link(__MODULE__, arg, name: __MODULE__)
  end

  @impl true
  def init(_arg) do
    children = [
      {:telemetry_poller, measurements: periodic_measurements(), period: 10_000},
      {TelemetryMetricsPrometheus.Core, [metrics: metrics()]}
    ]

    Supervisor.init(children, strategy: :one_for_one)
  end

  defp metrics do
    [
      counter("phoenix.endpoint.stop.duration",
        unit: {:native, :millisecond}
      ),
      counter("phoenix.router_dispatch.stop.duration",
        tags: [:route],
        unit: {:native, :millisecond}
      ),
      summary("phoenix.live_view.mount.stop.duration",
        tags: [:view],
        unit: {:native, :millisecond}
      ),
      summary("mcoa_frontend.backend_client.request.duration",
        tags: [:endpoint, :status],
        unit: {:native, :millisecond}
      ),
      counter("mcoa_frontend.backend_client.request.count",
        tags: [:endpoint, :status]
      ),
      counter("mcoa_frontend.backend_client.request.error")
    ]
  end

  defp periodic_measurements do
    [
      {MCOAFrontend.BackendClient, :measure_health, []}
    ]
  end
end