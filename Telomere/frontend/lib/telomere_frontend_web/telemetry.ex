defmodule TelomereFrontendWeb.Telemetry do
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
      last_value("phoenix.endpoint.stop.duration",
        unit: {:native, :millisecond}
      ),
      last_value("vm.memory.total", unit: {:byte, :kilobyte}),
      last_value("vm.total_run_queue_lengths.total"),
      last_value("vm.total_run_queue_lengths.cpu"),
      last_value("vm.total_run_queue_lengths.io"),
      summary("telomere_frontend.backend_client.request.duration",
        unit: {:native, :millisecond}
      ),
      counter("telomere_frontend.backend_client.request.errors")
    ]
  end

  defp periodic_measurements do
    [
      {__MODULE__, :emit_backend_health, []}
    ]
  end

  def emit_backend_health do
    case TelomereFrontendWeb.Clients.BackendClient.health_check() do
      {:ok, %{status: 200}} ->
        :telemetry.execute([:telomere_frontend, :backend, :health], %{status: 1}, %{})

      _error ->
        :telemetry.execute([:telomere_frontend, :backend, :health], %{status: 0}, %{})
    end
  end
end