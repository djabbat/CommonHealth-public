defmodule MCOAFrontend.BackendClient do
  use Tesla
  require Logger

  @timeout Application.get_env(:mcoa_frontend, :api_timeout)
  @backend_url Application.get_env(:mcoa_frontend, :backend_url)

  plug Tesla.Middleware.BaseUrl, @backend_url
  plug Tesla.Middleware.JSON
  plug Tesla.Middleware.Timeout, timeout: @timeout

  plug Tesla.Middleware.Telemetry,
    metadata: %{client: :mcoa_backend},
    log: {__MODULE__, :log_level}

  plug Tesla.Middleware.Retry,
    delay: 500,
    max_retries: 3,
    max_delay: 4_000,
    should_retry: fn
      {:ok, %{status: status}} when status in 500..599 -> true
      {:error, _} -> true
      _ -> false
    end

  @doc """
  Fetch all canonical counters from backend
  """
  def list_counters do
    case get("/api/v1/counters") do
      {:ok, %Tesla.Env{status: 200, body: body}} ->
        {:ok, Enum.map(body, &normalize_counter/1)}

      {:ok, %Tesla.Env{status: status}} ->
        Logger.error("Backend returned status #{status} for counters")
        {:error, :backend_error}

      {:error, reason} ->
        Logger.error("Failed to fetch counters: #{inspect(reason)}")
        fallback_counters()
    end
  end

  @doc """
  Fetch single counter by ID
  """
  def get_counter(counter_id) do
    case get("/api/v1/counters/#{counter_id}") do
      {:ok, %Tesla.Env{status: 200, body: body}} ->
        {:ok, normalize_counter(body)}

      {:ok, %Tesla.Env{status: 404}} ->
        {:error, :not_found}

      {:ok, %Tesla.Env{status: status}} ->
        Logger.error("Backend returned status #{status} for counter #{counter_id}")
        {:error, :backend_error}

      {:error, reason} ->
        Logger.error("Failed to fetch counter #{counter_id}: #{inspect(reason)}")
        {:error, :connection_error}
    end
  end

  @doc """
  Fetch all tissues with their weights
  """
  def list_tissues do
    case get("/api/v1/tissues") do
      {:ok, %Tesla.Env{status: 200, body: body}} ->
        {:ok, Enum.map(body, &normalize_tissue/1)}

      {:ok, %Tesla.Env{status: status}} ->
        Logger.error("Backend returned status #{status} for tissues")
        {:error, :backend_error}

      {:error, reason} ->
        Logger.error("Failed to fetch tissues: #{inspect(reason)}")
        fallback_tissues()
    end
  end

  @doc """
  Fetch coupling matrix Γ
  """
  def get_coupling_matrix do
    case get("/api/v1/coupling") do
      {:ok, %Tesla.Env{status: 200, body: body}} ->
        {:ok, normalize_coupling_matrix(body)}

      {:ok, %Tesla.Env{status: status}} ->
        Logger.error("Backend returned status #{status} for coupling matrix")
        {:error, :backend_error}

      {:error, reason} ->
        Logger.error("Failed to fetch coupling matrix: #{inspect(reason)}")
        fallback_coupling_matrix()
    end
  end

  @doc """
  Fetch recent simulations
  """
  def list_simulations do
    case get("/api/v1/simulations?limit=10") do
      {:ok, %Tesla.Env{status: 200, body: body}} ->
        {:ok, Enum.map(body, &normalize_simulation/1)}

      {:ok, %Tesla.Env{status: status}} ->
        Logger.error("Backend returned status #{status} for simulations")
        {:error, :backend_error}

      {:error, reason} ->
        Logger.error("Failed to fetch simulations: #{inspect(reason)}")
        fallback_simulations()
    end
  end

  @doc """
  Measure backend health for telemetry
  """
  def measure_health do
    start_time = System.monotonic_time()
    result = get("/health")
    duration = System.monotonic_time() - start_time

    status =
      case result do
        {:ok, %Tesla.Env{status: 200}} -> :healthy
        _ -> :unhealthy
      end

    :telemetry.execute([:mcoa_frontend, :backend_client, :health_check], %{duration: duration}, %{status: status})
  end

  defp log_level(env) do
    case env.status do
      status when status in 400..599 -> :error
      _ -> :debug
    end
  end

  # Fallback data from CONCEPT.md and PARAMETERS.md when backend is unavailable
  defp fallback_counters do
    counters = [
      %{
        id: "telomere",
        name: "Telomere",
        description: "Division-dominant counter, Hayflick limit per cell type",
        canonical_index: 1,
        alpha: 0.02,
        beta: 0.002,
        reference_divisions: 50,
        reference_time: 31_536_000,
        reference_source: "Hayflick & Moorhead 1961",
        critical_threshold: 0.85,
        validation_status: "validated",
        status: "active",
        coupling_strength: 0.3,
        measurement_method: "qFISH, qPCR",
        subproject: "Telomere"
      },
      %{
        id: "centriolar",
        name: "Centriolar polyglutamylation",
        description: "Division + time counter, TTLL/CCP balance",
        canonical_index: 2,
        alpha: 0.015,
        beta: 0.005,
        reference_divisions: 65,
        reference_time: 15_768_000,
        reference_source: "Tkemaladze 2023 (PMID 36583780)",
        critical_threshold: 0.80,
        validation_status: "provisional",
        status: "active",
        coupling_strength: 0.2,
        measurement_method: "GT335 mass-spec",
        subproject: "CDATA"
      },
      %{
        id: "mitochondrial",
        name: "Mitochondrial ROS / mtDNA",
        description: "Time-dominant counter for post-mitotic cells",
        canonical_index: 3,
        alpha: 0.0,
        beta: 0.01,
        reference_divisions: nil,
        reference_time: 1_209_600,
        reference_source: "Bratic & Larsson 2013",
        critical_threshold: 0.75,
        validation_status: "validated",
        status: "active",
        coupling_strength: 0.3,
        measurement_method: "MitoSOX, 8-OHdG ELISA",
        subproject: "MitoROS"
      },
      %{
        id: "epigenetic",
        name: "Epigenetic drift",
        description: "Time-dominant, Horvath clock / DunedinPACE",
        canonical_index: 4,
        alpha: 0.0,
        beta: 0.008,
        reference_divisions: nil,
        reference_time: 31_536_000,
        reference_source: "Horvath 2013, Belsky 2022",
        critical_threshold: 0.70,
        validation_status: "validated",
        status: "active",
        coupling_strength: 0.1,
        measurement_method: "DNA methylation array",
        subproject: "EpigeneticDrift"
      },
      %{
        id: "proteostasis",
        name: "Proteostasis collapse",
        description: "Mixed division/time, protein half-life dependent",
        canonical_index: 5,
        alpha: 0.005,
        beta: 0.006,
        reference_divisions: "cell-specific",
        reference_time: "protein-half-life-dependent",
        reference_source: "open",
        critical_threshold: 0.75,
        validation_status: "theoretical",
        status: "provisional",
        coupling_strength: 0.2,
        measurement_method: "Aggregation assays, proteomics",
        subproject: "Proteostasis"