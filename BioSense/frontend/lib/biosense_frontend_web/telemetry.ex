defmodule BioSenseFrontendWeb.Telemetry do
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
      summary("phoenix.live_view.mount.stop.duration",
        unit: {:native, :millisecond}
      ),
      summary("biosense_frontend.backend_client.request.duration",
        unit: {:native, :millisecond}
      ),
      counter("biosense_frontend.backend_client.request.count"),
      counter("biosense_frontend.backend_client.request.error.count")
    ]
  end

  defp periodic_measurements do
    [
      {__MODULE__, :measure_backend_health, []}
    ]
  end

  def measure_backend_health do
    case BioSenseFrontendWeb.Clients.BackendClient.health_check() do
      {:ok, %{"status" => "healthy"}} ->
        :telemetry.execute([:biosense_frontend, :backend, :health], %{value: 1})

      {:error, _} ->
        :telemetry.execute([:biosense_frontend, :backend, :health], %{value: 0})
    end
  end
end