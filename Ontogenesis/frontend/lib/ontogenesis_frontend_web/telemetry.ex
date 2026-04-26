defmodule OntogenesisFrontendWeb.Telemetry do
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
      summary("ontogenesis_frontend.backend_client.request.duration",
        tags: [:endpoint],
        unit: {:native, :millisecond}
      ),
      counter("ontogenesis_frontend.backend_client.request.count",
        tags: [:endpoint, :status]
      ),
      last_value("ontogenesis_frontend.live_view.count")
    ]
  end

  defp periodic_measurements do
    [
      {OntogenesisFrontendWeb.Metrics, :dispatch_metrics, []}
    ]
  end
end

defmodule OntogenesisFrontendWeb.Metrics do
  def dispatch_metrics do
    :telemetry.execute([:ontogenesis_frontend, :live_view], %{count: Phoenix.LiveView.Server.count()}, %{})
  end
end