defmodule HAPFrontendWeb.BackendClient do
  @moduledoc """
  HTTP client for communicating with the Rust backend API.
  """

  use GenServer
  require Logger

  @backend_url Application.compile_env(:hap_frontend, :backend_url)

  @spec get_concept() :: {:ok, map()} | {:error, String.t()}
  def get_concept() do
    request(:get, "/api/concept")
  end

  @spec get_parameters() :: {:ok, list(map())} | {:error, String.t()}
  def get_parameters() do
    request(:get, "/api/parameters")
  end

  @spec get_knowledge() :: {:ok, list(map())} | {:error, String.t()}
  def get_knowledge() do
    request(:get, "/api/knowledge")
  end

  @spec measure_requests() :: :ok
  def measure_requests() do
    :telemetry.execute([:hap_frontend, :backend_client, :measure], %{count: 1})
  end

  defp request(method, path) do
    url = @backend_url <> path
    start_time = System.monotonic_time()

    result = Req.request(
      method: method,
      url: url,
      receive_timeout: 10_000,
      retry: :transient,
      retry_delay: fn attempt -> 100 * attempt end,
      retry_max_attempts: 3
    )

    duration = System.monotonic_time() - start_time
    duration_ms = System.convert_time_unit(duration, :native, :millisecond)

    case result do
      {:ok, %{status: 200, body: body}} ->
        :telemetry.execute(
          [:hap_frontend, :backend_client, :request],
          %{duration: duration_ms},
          %{endpoint: path, status: "success"}
        )
        {:ok, body}

      {:ok, %{status: status, body: body}} ->
        error_msg = "HTTP #{status}: #{inspect(body)}"
        :telemetry.execute(
          [:hap_frontend, :backend_client, :request],
          %{duration: duration_ms},
          %{endpoint: path, status: "error"}
        )
        {:error, error_msg}

      {:error, %Mint.TransportError{reason: :econnrefused}} ->
        :telemetry.execute(
          [:hap_frontend, :backend_client, :error],
          %{count: 1},
          %{error_type: "connection_refused"}
        )
        {:error, "Backend connection refused at #{@backend_url}"}

      {:error, %Mint.TransportError{reason: reason}} ->
        :telemetry.execute(
          [:hap_frontend, :backend_client, :error],
          %{count: 1},
          %{error_type: "transport_error"}
        )
        {:error, "Transport error: #{inspect(reason)}"}

      {:error, reason} ->
        :telemetry.execute(
          [:hap_frontend, :backend_client, :error],
          %{count: 1},
          %{error_type: "unknown"}
        )
        {:error, "Unknown error: #{inspect(reason)}"}
    end
  end

  # GenServer implementation for connection pooling and state management
  def start_link(opts) do
    GenServer.start_link(__MODULE__, opts, name: __MODULE__)
  end

  @impl true
  def init(_opts) do
    {:ok, %{}}
  end
end