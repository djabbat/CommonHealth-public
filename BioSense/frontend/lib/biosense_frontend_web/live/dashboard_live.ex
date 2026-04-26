defmodule BioSenseFrontendWeb.DashboardLive do
  use BioSenseFrontendWeb, :live_view

  alias BioSenseFrontendWeb.Clients.BackendClient
  alias BioSenseFrontendWeb.CoreComponents

  @impl true
  def mount(_params, _session, socket) do
    if connected?(socket), do: schedule_refresh()

    {:ok,
     socket
     |> assign(
       page_title: "BioSense Dashboard",
       last_updated: nil,
       sensor_streams: %{},
       datasets: [],
       parameters: [],
       backend_status: :unknown
     )
     |> load_data()}
  end

  @impl true
  def handle_params(params, _url, socket) do
    {:noreply, apply_action(socket, socket.assigns.live_action, params)}
  end

  @impl true
  def handle_info(:refresh, socket) do
    schedule_refresh()
    {:noreply, socket |> load_data()}
  end

  @impl true
  def handle_event("refresh", _params, socket) do
    {:noreply, socket |> load_data()}
  end

  defp apply_action(socket, :index, _params) do
    socket
    |> assign(page_title: "BioSense Dashboard")
  end

  defp load_data(socket) do
    socket
    |> load_sensor_streams()
    |> load_datasets()
    |> load_parameters()
    |> check_backend()
    |> assign(last_updated: DateTime.utc_now())
  end

  defp load_sensor_streams(socket) do
    case BackendClient.get_sensor_streams() do
      {:ok, streams} ->
        assign(socket, sensor_streams: streams)

      {:error, _} ->
        assign(socket, sensor_streams: %{
          eeg: generate_fallback_stream(128, 0, 50),
          hrv: generate_fallback_stream(60, 800, 1200),
          olfaction: generate_fallback_stream(30, 0, 100)
        })
    end
  end

  defp load_datasets(socket) do
    case BackendClient.get_datasets() do
      {:ok, datasets} ->
        assign(socket, datasets: datasets)

      {:error, _} ->
        assign(socket, datasets: [
          %{
            id: "cuban",
            name: "Cuban Normative EEG",
            n: 97,
            status: "exploratory",
            note: "Wideband analysis (≤19.1 Hz)"
          },
          %{
            id: "dortmund",
            name: "Dortmund Vital Study",
            n: 60,
            status: "null",
            note: "Gamma 25–35 Hz: confirmatory null"
          },
          %{
            id: "lemon",
            name: "MPI-LEMON",
            n: 30,
            status: "null",
            note: "Alpha 8–13 Hz: pre-registered null"
          }
        ])
    end
  end

  defp load_parameters(socket) do
    case BackendClient.get_parameters() do
      {:ok, parameters} ->
        assign(socket, parameters: parameters)

      {:error, _} ->
        assign(socket, parameters: [
          %{
            category: "Hardware",
            name: "EEG Sampling Rate",
            value: "≥ 1000",
            unit: "Hz",
            status: "planned"
          },
          %{
            category: "Signal",
            name: "HRV Window Length",
            value: "300",
            unit: "s",
            status: "validated"
          },
          %{
            category: "Theory",
            name: "Ze Speed Formula",
            value: "v = N_S/(N−1)",
            unit: "",
            status: "canonical"
          }
        ])
    end
  end

  defp check_backend(socket) do
    case BackendClient.health_check() do
      {:ok, %{"status" => "healthy"}} ->
        assign(socket, backend_status: :healthy)

      _ ->
        assign(socket, backend_status: :unhealthy)
    end
  end

  defp generate_fallback_stream(count, min, max) do
    1..count
    |> Enum.map(fn _ -> min + :rand.uniform(max - min) end)
  end

  defp schedule_refresh do
    Process.send_after(self(), :refresh, 5000)
  end

  @impl true
  def render(assigns) do
    ~H"""
    <div class="space-y-8">
      <.header>
        BioSense Dashboard
        <:subtitle>
          Real-time sensor streams — Measurement Layer (EEG · HRV · Olfaction)
        </:subtitle>
        <:actions>
          <.button phx-click="refresh" variant="outline">
            <.icon name="hero-arrow-path" class="h-4 w-4 mr-2" />
            Refresh
          </.button>
        </:actions>
      </.header>

      <.card title="Backend Status">
        <div class="flex items-center">
          <div class={[
            "h-3 w-3 rounded-full mr-2",
            @backend_status == :healthy && "bg-green-500",
            @backend_status == :unhealthy && "bg-red-500",
            @backend_status == :unknown && "bg-yellow-500"
          ]}></div>
          <span class="text-sm">
            Backend: <%= humanize_backend_status(@backend_status) %>
          </span>
          <span class="ml-auto text-xs text-gray-500">
            Updated: <%= format_datetime(@last_updated) %>
          </span>
        </div>
      </.card>

      <.card title="Sensor Streams">
        <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
          <.sensor_stream
            title="EEG Raw Signal"
            data={@sensor_streams[:eeg] || []}
            unit="μV"
          />
          <.sensor_stream
            title="HRV RR Intervals"
            data={@sensor_streams[:hrv] || []}
            unit="ms"
          />
          <.sensor_stream
            title="Olfaction VOC Levels"
            data={@sensor_streams[:olfaction] || []}
            unit="ppm"
          />
        </div>
        <p class="mt-4 text-sm text-gray-600">
          Note: Displaying raw sensor streams only. No χ_Ze biomarker claims per CORRECTIONS 2026-04-22.
        </p>
      </.card>

      <.card title="Validated Datasets">
        <div class="overflow-x-auto">
          <table class="min-w-full divide-y divide-gray-200">
            <thead>
              <tr>
                <th class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase">Dataset</th>
                <th class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase">N</th>
                <th class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase">Status</th>
                <th class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase">Note</th>
              </tr>
            </thead>
            <tbody class="divide-y divide-gray-200">
              <%= for dataset <- @datasets do %>
                <tr>
                  <td class="px-4 py-3">
                    <.link
                      navigate={~p"/datasets/#{dataset.id}"}
                      class="text-blue-600 hover:text-blue-800"
                    >
                      <%= dataset.name %>
                    </.link>
                  </td>
                  <td class="px-4 py-3"><%= dataset.n %></td>
                  <td class="px-4 py-3">
                    <.status_badge status={dataset.status}>
                      <%= String.capitalize(dataset.status) %>
                    </.status_badge>
                  </td>
                  <td class="px-4 py-3 text-sm text-gray-600"><%= dataset.note %></td>
                </tr>
              <% end %>
            </tbody>
          </table>
        </div>
      </.card>

      <.card title="Key Parameters">
        <div class="space-y-4">
          <%= for param <- @parameters do %>
            <div class="flex items-center justify-between py-2 border-b border-gray-100 last:border-0">
              <div>
                <span class="font-medium"><%= param.name %></span>
                <span class="ml-2 text-xs px-2 py-1 bg-gray-100 rounded">
                  <%= param.category %>
                </span>
              </div>
              <div class="text-right">
                <span class="font-mono"><%= param.value %></span>
                <%= if param.unit != "" do %>
                  <span class="text-sm text-gray-500 ml-1"><%= param.unit %></span>
                <% end %>
                <.status_badge status={param.status} class="ml-2">
                  <%= String.capitalize(param.status) %>
                </.status_badge>
              </div>
            </div>
          <% end %>
        </div>
      </.card>

      <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
        <.link navigate={~p"/counters"} class="block">
          <.card title="MCOA Counter Registry">
            <p class="text-sm text-gray-600">
              View and manage MCOA counter definitions and γ_i linkage parameters.
            </p>
            <div class="mt-4">
              <.button variant="outline" class="w-full">Manage Counters</.button>
            </div>
          </.card>
        </.link>

        <.link navigate={~p"/sensitivity"} class="block">
          <.card title="Sobol Sensitivity Analysis">
            <p class="text-sm text-gray-600">
              Interactive visualization of parameter sensitivity for CDATA models.
            </p>
            <div class="mt-4">
              <.button variant="outline" class="w-full">View Analysis</.button>
            </div>
          </.card>
        </.link>
      </div>
    </div>
    """
  end

  defp humanize_backend_status(:healthy), do: "Healthy"
  defp humanize_backend_status(:unhealthy), do: "Unhealthy"
  defp humanize_backend_status(:unknown), do: "Unknown"

  defp format_datetime(nil), do: "Never"
  defp format_datetime(dt) do
    Calendar.strftime(dt, "%H:%M:%S")
  end
end