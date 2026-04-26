defmodule EpigeneticDriftFrontendWeb.BackendClient do
  @moduledoc """
  HTTP client for communicating with the Rust backend API.
  Implements retry logic, telemetry, and graceful degradation.
  """

  use Tesla
  require Logger

  alias EpigeneticDriftFrontendWeb.Telemetry

  @default_base_url "http://localhost:3007"
  @timeout 30_000
  @max_retries 3

  plug Tesla.Middleware.BaseUrl, base_url()
  plug Tesla.Middleware.Headers, [{"user-agent", "epigeneticdrift-frontend"}]
  plug Tesla.Middleware.JSON
  plug Tesla.Middleware.Timeout, timeout: @timeout
  plug Tesla.Middleware.Retry,
    delay: 500,
    max_retries: @max_retries,
    should_retry: &should_retry/1

  defp base_url do
    Application.get_env(:epigeneticdrift_frontend, __MODULE__)[:base_url] || @default_base_url
  end

  defp should_retry({:ok, %{status: status}}) when status >= 500, do: true
  defp should_retry({:ok, %{status: status}}) when status >= 400, do: false
  defp should_retry({:ok, _}), do: false
  defp should_retry({:error, _}), do: true

  @spec get_entities(String.t()) :: {:ok, list()} | {:error, any()}
  def get_entities(type) do
    :telemetry.span([:epigeneticdrift, :backend_client, :request], %{endpoint: "/api/entities"}, fn ->
      case get("/api/entities", query: [type: type]) do
        {:ok, %Tesla.Env{status: 200, body: body}} ->
          {{:ok, body}, %{status: 200}}

        {:ok, %Tesla.Env{status: status}} ->
          {{:error, :bad_status}, %{status: status}}

        {:error, reason} ->
          {{:error, reason}, %{status: 0}}
      end
    end)
  end

  @spec get_entity(String.t()) :: {:ok, map()} | {:error, any()}
  def get_entity(id) do
    case get("/api/entities/#{id}") do
      {:ok, %Tesla.Env{status: 200, body: body}} ->
        {:ok, body}

      {:ok, %Tesla.Env{status: 404}} ->
        {:error, :not_found}

      {:ok, %Tesla.Env{status: status}} ->
        {:error, {:bad_status, status}}

      {:error, reason} ->
        {:error, reason}
    end
  end

  @spec get_parameters() :: list()
  def get_parameters do
    case get("/api/parameters") do
      {:ok, %Tesla.Env{status: 200, body: body}} -> body
      _ -> []
    end
  end

  @spec get_time_series() :: list()
  def get_time_series do
    case get("/api/timeseries") do
      {:ok, %Tesla.Env{status: 200, body: body}} -> body
      _ -> []
    end
  end

  @spec get_entity_time_series(String.t()) :: list()
  def get_entity_time_series(id) do
    case get("/api/entities/#{id}/timeseries") do
      {:ok, %Tesla.Env{status: 200, body: body}} -> body
      _ -> []
    end
  end

  @spec get_sensitivity_analysis(String.t()) :: list()
  def get_sensitivity_analysis(id) do
    case get("/api/entities/#{id}/sensitivity") do
      {:ok, %Tesla.Env{status: 200, body: body}} -> body
      _ -> []
    end
  end

  @spec delete_entity(String.t()) :: :ok | {:error, any()}
  def delete_entity(id) do
    case delete("/api/entities/#{id}") do
      {:ok, %Tesla.Env{status: 204}} -> :ok
      {:ok, %Tesla.Env{status: status}} -> {:error, {:bad_status, status}}
      {:error, reason} -> {:error, reason}
    end
  end

  @spec recalculate_d4(String.t()) :: {:ok, map()} | {:error, any()}
  def recalculate_d4(id) do
    case post("/api/entities/#{id}/recalculate", %{}) do
      {:ok, %Tesla.Env{status: 200, body: body}} -> {:ok, body}
      {:ok, %Tesla.Env{status: status}} -> {:error, {:bad_status, status}}
      {:error, reason} -> {:error, reason}
    end
  end

  @spec measure_health() :: :ok
  def measure_health do
    case get("/health") do
      {:ok, %Tesla.Env{status: 200}} ->
        :telemetry.execute([:epigeneticdrift, :backend_client, :health], %{status: :healthy}, %{})

      _ ->
        :telemetry.execute([:epigeneticdrift, :backend_client, :health], %{status: :unhealthy}, %{})
    end

    :ok
  end

  def child_spec(_opts) do
    %{id: __MODULE__, start: {__MODULE__, :start_link, []}, type: :supervisor}
  end

  def start_link do
    Tesla.client([], adapter())
  end

  defp adapter do
    if Mix.env() == :test do
      {Tesla.Adapter.Mock, []}
    else
      {Tesla.Adapter.Finch, name: EpigeneticDriftFrontend.Finch, receive_timeout: @timeout}
    end
  end
end