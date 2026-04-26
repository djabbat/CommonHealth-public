defmodule BioSenseFrontendWeb.Clients.BackendClient do
  @moduledoc """
  HTTP client for communicating with the BioSense Rust backend API.
  """

  use Tesla

  require Logger

  @base_url Application.compile_env(:biosense_frontend, :backend_url, "http://localhost:3004")
  @timeout 10_000

  plug Tesla.Middleware.BaseUrl, @base_url
  plug Tesla.Middleware.Timeout, timeout: @timeout
  plug Tesla.Middleware.JSON
  plug Tesla.Middleware.Logger
  plug Tesla.Middleware.Retry,
    delay: 500,
    max_retries: 3,
    max_delay: 4_000,
    should_retry: fn
      {:ok, %{status: status}} when status in [500, 502, 503, 504] -> true
      {:ok, _} -> false
      {:error, _} -> true
    end

  @doc """
  Health check for backend service.
  """
  def health_check do
    :telemetry.span(
      [:biosense_frontend, :backend_client, :request],
      %{endpoint: "/health"},
      fn ->
        result = get("/health")
        {result, %{endpoint: "/health", method: "GET"}}
      end
    )
  end

  @doc """
  Fetch real-time sensor streams.
  """
  def get_sensor_streams do
    with_telemetry("/api/sensor/streams", "GET", fn ->
      case get("/api/sensor/streams") do
        {:ok, %Tesla.Env{status: 200, body: body}} ->
          {:ok, body}

        {:ok, env} ->
          Logger.error("Unexpected response from sensor streams: #{inspect(env)}")
          {:error, :invalid_response}

        {:error, reason} ->
          Logger.error("Failed to fetch sensor streams: #{inspect(reason)}")
          {:error, :connection_failed}
      end
    end)
  end

  @doc """
  Fetch dataset metadata.
  """
  def get_datasets do
    with_telemetry("/api/datasets", "GET", fn ->
      case get("/api/datasets") do
        {:ok, %Tesla.Env{status: 200, body: body}} ->
          {:ok, body}

        {:error, reason} ->
          Logger.warning("Failed to fetch datasets, using fallback: #{inspect(reason)}")
          {:error, :connection_failed}
      end
    end)
  end

  @doc """
  Fetch specific dataset by ID.
  """
  def get_dataset(id) do
    with_telemetry("/api/datasets/#{id}", "GET", fn ->
      case get("/api/datasets/#{id}") do
        {:ok, %Tesla.Env{status: 200, body: body}} ->
          {:ok, body}

        {:ok, %Tesla.Env{status: 404}} ->
          {:error, :not_found}

        {:error, reason} ->
          Logger.error("Failed to fetch dataset #{id}: #{inspect(reason)}")
          {:error, :connection_failed}
      end
    end)
  end

  @doc """
  Fetch parameters.
  """
  def get_parameters do
    with_telemetry("/api/parameters", "GET", fn ->
      case get("/api/parameters") do
        {:ok, %Tesla.Env{status: 200, body: body}} ->
          {:ok, body}

        {:error, reason} ->
          Logger.warning("Failed to fetch parameters, using fallback: #{inspect(reason)}")
          {:error, :connection_failed}
      end
    end)
  end

  @doc """
  Fetch specific parameter by ID.
  """
  def get_parameter(id) do
    with_telemetry("/api/parameters/#{id}", "GET", fn ->
      case get("/api/parameters/#{id}") do
        {:ok, %Tesla.Env{status: 200, body: body}} ->
          {:ok, body}

        {:ok, %Tesla.Env{status: 404}} ->
          {:error, :not_found}

        {:error, reason} ->
          Logger.error("Failed to fetch parameter #{id}: #{inspect(reason)}")
          {:error, :connection_failed}
      end
    end)
  end

  @doc """
  Fetch knowledge entries.
  """
  def get_knowledge(id) do
    with_telemetry("/api/knowledge/#{id}", "GET", fn ->
      case get("/api/knowledge/#{id}") do
        {:ok, %Tesla.Env{status: 200, body: body}} ->
          {:ok, body}

        {:ok, %Tesla.Env{status: 404}} ->
          {:error, :not_found}

        {:error, reason} ->
          Logger.error("Failed to fetch knowledge #{id}: #{inspect(reason)}")
          {:error, :connection_failed}
      end
    end)
  end

  @doc """
  Fetch MCOA counters.
  """
  def get_counters do
    with_telemetry("/api/mcoa/counters", "GET", fn ->
      case get("/api/mcoa/counters") do
        {:ok, %Tesla.Env{status: 200, body: body}} ->
          {:ok, body}

        {:error, reason} ->
          Logger.warning("Failed to fetch counters, using fallback: #{inspect(reason)}")
          {:error, :connection_failed}
      end
    end)
  end

  @doc """
  Fetch Sobol sensitivity analysis results.
  """
  def get_sensitivity_analysis do
    with_telemetry("/api/cdata/sensitivity", "GET", fn ->
      case get("/api/cdata/sensitivity") do
        {:ok, %Tesla.Env{status: 200, body: body}} ->
          {:ok, body}

        {:error, reason} ->
          Logger.warning("Failed to fetch sensitivity analysis, using fallback: #{inspect(reason)}")
          {:error, :connection_failed}
      end
    end)
  end

  defp with_telemetry(endpoint, method, fun) do
    start_time = System.monotonic_time()

    try do
      fun.()
    after
      duration = System.monotonic_time() - start_time
      :telemetry.execute([:biosense_frontend, :backend_client, :request, :duration], %{
        duration: duration
      })
    end
  end
end