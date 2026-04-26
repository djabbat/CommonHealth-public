defmodule MitoROSFrontendWeb.DashboardLive do
  use MitoROSFrontendWeb, :live_view

  alias MitoROSFrontendWeb.BackendClient
  alias MitoROSFrontendWeb.Components.ConnectivityStatus

  @impl true
  def mount(_params, _session, socket) do
    if connected?(socket), do: schedule_refresh()

    {:ok,
     socket
     |> assign_defaults()
     |> load_initial_data()}
  end

  @impl true
  def handle_params(_params, _uri, socket) do
    {:noreply, socket}
  end

  @impl true
  def handle_event("refresh", _params, socket) do
    {:noreply, load_initial_data(socket)}
  end

  @impl true
  def handle_event("select_tissue", %{"tissue" => tissue}, socket) do
    {:noreply,
     socket
     |> assign(:selected_tissue, tissue)
     |> load_tissue_data(tissue)}
  end

  @impl true
  def handle_info(:refresh, socket) do
    schedule_refresh()
    {:noreply, load_initial_data(socket)}
  end

  defp assign_defaults(socket) do
    assign(socket,
      page_title: "MitoROS Dashboard",
      loading: true,
      error: nil,
      parameters: [],
      selected_tissue: "liver",
      tissues: [
        {"Liver", "liver"},
        {"Heart", "heart"},
        {"Brain", "brain"},
        {"Muscle", "muscle"},
        {"Intestine", "intestine"},
        {"HSC", "hsc"}
      ],
      sobol_data: [],
      lineages: [],
      time_series: %{},
      v_value: nil,
      connectivity: %{status: :unknown, latency: nil}
    )
  end

  defp load_initial_data(socket) do
    socket =
      socket
      |> assign(loading: true)
      |> push_event("loading_start", %{})

    Task.start_link(fn ->
      with {:ok, params} <- BackendClient.fetch_parameters(),
           {:ok, sobol} <- BackendClient.fetch_sobol_sensitivity(),
           {:ok, lineages} <- BackendClient.fetch_hsc_lineages(),
           {:ok, time_series} <- BackendClient.fetch_time_series(socket.assigns.selected_tissue),
           {:ok, v} <- BackendClient.fetch_v_value() do
        send(self(), {:initial_data_loaded, params, sobol, lineages, time_series, v})
      else
        {:error, reason} ->
          send(self(), {:data_load_failed, reason})
      end
    end)

    socket
  end

  defp load_tissue_data(socket, tissue) do
    Task.start_link(fn ->
      case BackendClient.fetch_time_series(tissue) do
        {:ok, time_series} -> send(self(), {:tissue_data_loaded, tissue, time_series})
        {:error, reason} -> send(self(), {:tissue_data_failed, tissue, reason})
      end
    end)

    socket
  end

  @impl true
  def handle_info({:initial_data_loaded, params, sobol, lineages, time_series, v}, socket) do
    socket =
      socket
      |> assign(
        loading: false,
        parameters: params,
        sobol_data: sobol,
        lineages: lineages,
        time_series: time_series,
        v_value: v,
        error: nil
      )
      |> push_event("loading_end", %{})

    {:noreply, socket}
  end

  defp handle_info({:tissue_data_loaded, tissue, time_series}, socket) do
    if socket.assigns.selected_tissue == tissue do
      {:noreply,
       socket
       |> assign(time_series: time_series)
       |> push_event("loading_end", %{})}
    else
      {:noreply, socket}
    end
  end

  defp handle_info({:data_load_failed, reason}, socket) do
    socket =
      socket
      |> assign(loading: false, error: "Failed to load data: #{inspect(reason)}")
      |> push_event("loading_end", %{})

    {:noreply, socket}
  end

  defp handle_info({:tissue_data_failed, _tissue, reason}, socket) do
    socket =
      socket
      |> put_flash(:error, "Failed to load tissue data: #{inspect(reason)}")
      |> push_event("loading_end", %{})

    {:noreply, socket}
  end

  defp handle_info({:connectivity_update, status}, socket) do
    {:noreply, assign(socket, connectivity: status)}
  end

  defp schedule_refresh do
    Process.send_after(self(), :refresh, 30_000)
  end

  @impl true
  def render(assigns) do
    ~H"""
    <div class="space-y-8">
      <.header>
        <:title>Mitochondrial ROS & mtDNA Damage (Counter #3)</:title>
        <:subtitle>
          Multi-Counter Architecture of Aging · Formalized kinetic model parameterized from 24 studies
        </:subtitle>
      </.header>

      <.live_component
        id="connectivity"
        module={ConnectivityStatus}
        backend_url={Application.get_env(:mitoros_frontend, :backend_url)}
      />

      <%= if @error do %>
        <.error_alert>
          <%= @error %>
          <button phx-click="refresh" class="ml-2 font-medium underline">Try again</button>
        </.error_alert>
      <% end %>

      <%= if @loading do %>
        <div class="flex h-64 items-center justify-center">
          <.spinner size="lg" />
          <span class="ml-4 text-zinc-600">Loading MitoROS model data...</span>
        </div>
      <% else %>
        <div class="grid grid-cols-1 gap-8 lg:grid-cols-3">
          <div class="lg:col-span-2">
            <div class="mb-8">
              <.h2>Model Parameters</.h2>
              <p class="mt-2 text-zinc-600">
                Kinetic equation: D₃(n, t) = D₃₀ + α₃·(n/n₃*) + β₃·(t/τ₃) + γ₃·I(other counters)
              </p>
              <p class="mt-1 text-sm text-zinc-500">
                Note: γ₃ = 0 by default (null hypothesis of independence) per CORRECTIONS_2026-04-22 §1.3
              </p>
            </div>

            <div class="grid grid-cols-1 gap-6 sm:grid-cols-2">
              <%= for param <- @parameters do %>
                <.parameter_card parameter={param} />
              <% end %>
            </div>

            <div class="mt-12">
              <.h2>Sensitivity Analysis</.h2>
              <p class="mt-2 text-zinc-600">
                Sobol global sensitivity indices for model parameters (first order and total order effects)
              </p>
              <.sobol_sensitivity
                data={@sobol_data}
                parameters={["D₃₀", "α₃", "n₃*", "β₃", "τ₃", "γ₃"]}
              />
            </div>
          </div>

          <div class="space-y-8">
            <div class="rounded-lg border border-zinc-200 bg-white p-6">
              <h3 class="mb-4 text-lg font-semibold text-zinc-900">Tissue Selection</h3>
              <div class="space-y-2">
                <%= for {label, value} <- @tissues do %>
                  <button
                    phx-click="select_tissue"
                    phx-value-tissue={value}
                    class={"w-full rounded-lg px-4 py-3 text-left transition-colors #{if @selected_tissue == value, do: "bg-blue-50 text-blue-700", else: "hover:bg-zinc-50"}"}
                  >
                    <div class="flex items-center justify-between">
                      <span><%= label %></span>
                      <%= if @selected_tissue == value do %>
                        <svg class="h-5 w-5" fill="currentColor" viewBox="0 0 20 20">
                          <path fill-rule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z" clip-rule="evenodd" />
                        </svg>
                      <% end %>
                    </div>
                  </button>
                <% end %>
              </div>
            </div>

            <div class="rounded-lg border border-zinc-200 bg-white p-6">
              <h3 class="mb-4 text-lg font-semibold text-zinc-900">Damage Trajectory</h3>
              <div class="aspect-video w-full">
                <%= if map_size(@time_series) > 0 do %>
                  <!-- Time series chart would be rendered here with a charting library -->
                  <div class="flex h-full items-center justify-center rounded-lg bg-zinc-50">
                    <div class="text-center">
                      <div class="text-sm text-zinc-500">Damage D₃(t) for <%= @selected_tissue %></div>
                      <div class="mt-2 text-3xl font-bold text-zinc-900">
                        <%= :erlang.float_to_binary(@time_series.current_value, decimals: 3) %>
                      </div>
                      <div class="mt-1 text-sm text-zinc-600">
                        at t = <%= @time_series.current_time %> years
                      </div>
                    </div>
                  </div>
                <% else %>
                  <div class="flex h-full items-center justify-center rounded-lg bg-zinc-50">
                    <div class="text-center text-zinc-500">No time series data available</div>
                  </div>
                <% end %>
              </div>
            </div>

            <.hsc_lineage_tracking lineages={@lineages} />

            <div class="rounded-lg border border-zinc-200 bg-white p-6">
              <h3 class="mb-4 text-lg font-semibold text-zinc-900">Validation Metric</h3>
              <div class="space-y-4">
                <div>
                  <div class="text-sm font-medium text-zinc-500">Ze Validation Statistic</div>
                  <div class="mt-1 flex items-baseline">
                    <div class="text-3xl font-bold text-zinc-900">
                      v = <%= if @v_value, do: :erlang.float_to_binary(@v_value, decimals: 4), else: "N/A" %>
                    </div>
                  </div>
                  <p class="mt-2 text-sm text-zinc-600">
                    Computed as v = Nₛ/(N-1) where Nₛ is number of successful predictions and N is total samples.
                    Higher values indicate better model validation against experimental data.
                  </p>
                </div>
              </div>
            </div>
          </div>
        </div>
      <% end %>
    </div>
    """
  end
end