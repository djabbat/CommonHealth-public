defmodule OntogenesisFrontendWeb.Clients.BackendClient do
  @moduledoc """
  HTTP client for communicating with the Rust backend API.
  Implements proper error handling, telemetry, and graceful degradation.
  """

  use Telemetry.Timer

  @backend_url Application.compile_env(:ontogenesis_frontend, :backend_url, "http://localhost:3011")

  @doc """
  Fetches dashboard data from backend.
  """
  @spec get_dashboard_data() :: {:ok, map()} | {:error, term()}
  def get_dashboard_data do
    :telemetry.span([:ontogenesis_frontend, :backend_client, :request], %{endpoint: "/api/dashboard"}, fn ->
      result = execute_request(:get, "/api/dashboard")
      {result, %{endpoint: "/api/dashboard", status: extract_status(result)}}
    end)
  end

  @doc """
  Fetches a single entity with related entities.
  """
  @spec get_entity(String.t()) :: {:ok, map()} | {:error, term()}
  def get_entity(id) do
    :telemetry.span([:ontogenesis_frontend, :backend_client, :request], %{endpoint: "/api/entities/#{id}"}, fn ->
      result = execute_request(:get, "/api/entities/#{id}")
      {result, %{endpoint: "/api/entities/#{id}", status: extract_status(result)}}
    end)
  end

  @doc """
  Fetches parameters for a specific domain.
  """
  @spec get_parameters_by_domain(String.t()) :: {:ok, list(map())} | {:error, term()}
  def get_parameters_by_domain(domain) do
    :telemetry.span([:ontogenesis_frontend, :backend_client, :request], 
      %{endpoint: "/api/parameters?domain=#{domain}"}, 
      fn ->
        result = execute_request(:get, "/api/parameters?domain=#{domain}")
        {result, %{endpoint: "/api/parameters?domain=#{domain}", status: extract_status(result)}}
      end)
  end

  @doc """
  Checks backend health status.
  """
  @spec health_check() :: {:ok, map()} | {:error, term()}
  def health_check do
    :telemetry.span([:ontogenesis_frontend, :backend_client, :request], %{endpoint: "/health"}, fn ->
      result = execute_request(:get, "/health", timeout: 5_000)
      {result, %{endpoint: "/health", status: extract_status(result)}}
    end)
  end

  defp execute_request(method, path, opts \\ []) do
    url = @backend_url <> path
    timeout = Keyword.get(opts, :timeout, 30_000)

    case Req.request(
           method: method,
           url: url,
           receive_timeout: timeout,
           retry: :transient,
           retry_delay: fn attempt -> 100 * :math.pow(2, attempt) end,
           max_retries: 3
         ) do
      {:ok, %{status: status, body: body}} when status in 200..299 ->
        {:ok, body}

      {:ok, %{status: status, body: body}} ->
        {:error, {:http_error, status, body}}

      {:error, %Mint.TransportError{reason: :econnrefused}} ->
        {:error, :backend_unreachable}

      {:error, %Mint.TransportError{reason: reason}} ->
        {:error, {:transport_error, reason}}

      {:error, reason} ->
        {:error, reason}
    end
  rescue
    e in RuntimeError -> {:error, {:runtime_error, e.message}}
  end

  defp extract_status({:ok, _}), do: :success
  defp extract_status({:error, _}), do: :error
  defp extract_status(_), do: :unknown
end

defmodule OntogenesisFrontendWeb.Components.BackendStatus do
  use Phoenix.Component

  attr :status, :atom, required: true

  def status(assigns) do
    {bg_color, text_color, icon, message} = case @status do
      :healthy -> 
        {"bg-green-50", "text-green-800", 
         ~H"<Heroicons.check_circle class='h-5 w-5 text-green-400' />", 
         "Backend connected"}
      
      :unhealthy -> 
        {"bg-red-50", "text-red-800",
         ~H"<Heroicons.x_circle class='h-5 w-5 text-red-400' />",
         "Backend connection failed"}
      
      :unknown -> 
        {"bg-yellow-50", "text-yellow-800",
         ~H"<Heroicons.exclamation_triangle class='h-5 w-5 text-yellow-400' />",
         "Backend status unknown"}
    end

    ~H"""
    <div class={"rounded-md p-4 mb-4 #{bg_color}"}>
      <div class="flex">
        <div class="flex-shrink-0">
          <%= icon %>
        </div>
        <div class="ml-3">
          <p class={"text-sm font-medium #{text_color}"}>
            <%= message %>
          </p>
          <p class={"mt-1 text-sm #{text_color}"}>
            Backend URL: <%= Application.get_env(:ontogenesis_frontend, :backend_url) %>
          </p>
        </div>
      </div>
    </div>
    """
  end
end