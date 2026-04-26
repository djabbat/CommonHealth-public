defmodule EpigeneticDriftFrontendWeb.DetailLive do
  use EpigeneticDriftFrontendWeb, :live_view

  alias EpigeneticDriftFrontendWeb.BackendClient

  @impl true
  def mount(%{"id" => id}, _session, socket) do
    if connected?(socket), do: send(self(), {:load_entity, id})

    socket =
      socket
      |> assign(:page_title, "Loading...")
      |> assign(:loading, true)
      |> assign(:entity, nil)
      |> assign(:error, nil)
      |> assign(:time_series, [])
      |> assign(:sensitivity, [])

    {:ok, socket}
  end

  @impl true
  def handle_info({:load_entity, id}, socket) do
    case BackendClient.get_entity(id) do
      {:ok, entity} ->
        time_series = BackendClient.get_entity_time_series(id) || []
        sensitivity = BackendClient.get_sensitivity_analysis(id) || []

        socket =
          socket
          |> assign(:entity, entity)
          |> assign(:time_series, time_series)
          |> assign(:sensitivity, sensitivity)
          |> assign(:page_title, "Counter #{entity.id}")
          |> assign(:loading, false)

        {:noreply, socket}

      {:error, reason} ->
        socket =
          socket
          |> assign(:loading, false)
          |> assign(:error, "Failed to load entity: #{inspect(reason)}")

        {:noreply, socket}
    end
  end

  @impl true
  def handle_event("recalculate", %{"id" => id}, socket) do
    case BackendClient.recalculate_d4(id) do
      {:ok, updated} ->
        socket =
          socket
          |> assign(:entity, updated)
          |> put_flash(:info, "Recalculated D₄ = #{updated.d4}")

        {:noreply, socket}

      {:error, reason} ->
        {:noreply, put_flash(socket, :error, "Recalculation failed: #{inspect(reason)}")}
    end
  end

  @impl true
  def render(assigns) do
    ~H"""
    <div class="space-y-6">
      <.header>
        <:title>
          <%= if @entity do %>
            Counter #4: <%= @entity.id %>
          <% else %>
            Loading...
          <% end %>
        </:title>
        <:actions>
          <.live_patch to="/" class="text-sm font-medium text-gray-500 hover:text-gray-700">
            ← Back to Dashboard
          </.live_patch>
        </:actions>
      </.header>

      <.alert :if={@loading} type="info">
        <.spinner /> Loading entity data...
      </.alert>

      <.alert :if={@error} type="error">
        <%= @error %>
      </.alert>

      <%= if @entity do %>
        <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
          <div class="lg:col-span-2">
            <.card id="entity-details">
              <:title>Entity Details</:title>
              <:actions>
                <.button
                  click="recalculate"
                  phx-value-id={@entity.id}
                  variant="secondary"
                  class="ml-2"
                >
                  Recalculate D₄
                </.button>
              </:actions>
              <div class="overflow-x-auto">
                <table class="min-w-full divide-y divide-gray-200">
                  <thead class="bg-gray-50">
                    <tr>
                      <th scope="col" class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">Property</th>
                      <th scope="col" class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">Value</th>
                      <th scope="col" class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">Description</th>
                    </tr>
                  </thead>
                  <tbody class="bg-white divide-y divide-gray-200">
                    <tr>
                      <td class="px-6 py-4 whitespace-nowrap text-sm font-medium text-gray-900">ID</td>
                      <td class="px-6 py-4 whitespace-nowrap text-sm font-mono text-gray-900"><%= @entity.id %></td>
                      <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">Unique identifier</td>
                    </tr>
                    <tr>
                      <td class="px-6 py-4 whitespace-nowrap text-sm font-medium text-gray-900">D₄(n,t)</td>
                      <td class="px-6 py-4 whitespace-nowrap">
                        <.progress_bar
                          id="d4-value"
                          label="Epigenetic Drift"
                          value={@entity.d4}
                          min={0}
                          max={100}
                          unit=" units"
                        />
                      </td>
                      <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">Current drift state (normalized)</td>
                    </tr>
                    <tr>
                      <td class="px-6 py-4 whitespace-nowrap text-sm font-medium text-gray-900">Tissue</td>
                      <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-900"><%= @entity.tissue || "N/A" %></td>
                      <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">Source tissue type</td>
                    </tr>
                    <tr>
                      <td class="px-6 py-4 whitespace-nowrap text-sm font-medium text-gray-900">Divisions (n)</td>
                      <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-900"><%= @entity.divisions || 0 %></td>
                      <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">Cumulative cell divisions</td>
                    </tr>
                    <tr>
                      <td class="px-6 py-4 whitespace-nowrap text-sm font-medium text-gray-900">Time (t)</td>
                      <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-900"><%= @entity.time_years || 0 %> years</td>
                      <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">Chronological time</td>
                    </tr>
                    <tr>
                      <td class="px-6 py-4 whitespace-nowrap text-sm font-medium text-gray-900">D₄,₀</td>
                      <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-900"><%= @entity.d4_0 || 0 %></td>
                      <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">Baseline epigenetic state</td>
                    </tr>
                    <tr>
                      <td class="px-6 py-4 whitespace-nowrap text-sm font-medium text-gray-900">Created</td>
                      <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-900">
                        <%= @entity.inserted_at |> DateTime.from_naive!("Etc/UTC") |> DateTime.to_iso8601() %>
                      </td>
                      <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">First observation</td>
                    </tr>
                  </tbody>
                </table>
              </div>
            </.card>

            <.card id="time-series" class="mt-6">
              <:title>Drift Progression Over Time</:title>
              <%= if @time_series == [] do %>
                <div class="p-4 text-center text-gray-500">No time-series data available</div>
              <% else %>
                <div id="time-series-plot" phx-hook="Plotly" data-data={Jason.encode!(@time_series)}></div>
              <% end %>
            </.card>
          </div>

          <div>
            <.card id="parameter-values">
              <:title>Parameter Values</:title>
              <div class="space-y-4">
                <div class="p-3 bg-blue-50 rounded-lg">
                  <div class="text-sm font-medium text-blue-900">β₄ (time coefficient)</div>
                  <div class="text-2xl font-bold text-blue-700"><%= @entity.beta4 || 1.0 %></div>
                </div>
                <div class="p-3 bg-green-50 rounded-lg">
                  <div class="text-sm font-medium text-green-900">τ₄ (time constant)</div>
                  <div class="text-2xl font-bold text-green-700"><%= @entity.tau4 || 10.0 %> years</div>
                </div>
                <div class="p-3 bg-purple-50 rounded-lg">
                  <div class="text-sm font-medium text-purple-900">α₄ (division coefficient)</div>
                  <div class="text-2xl font-bold text-purple-700"><%= @entity.alpha4 || "TBD" %></div>
                </div>
                <div class="p-3 bg-yellow-50 rounded-lg">
                  <div class="text-sm font-medium text-yellow-900">n₄* (characteristic divisions)</div>
                  <div class="text-2xl font-bold text-yellow-700"><%= @entity.n4_star || 50 %></div>
                </div>
              </div>
            </.card>

            <.card id="couplings" class="mt-6">
              <:title>Counter Couplings (γ)</:title>
              <div class="space-y-2 text-sm">
                <div class="flex justify-between">
                  <span class="text-gray-700">γ₄,₁ (Centriolar):</span>
                  <span class="font-mono">0</span>
                </div>
                <div class="flex justify-between">
                  <span class="text-gray-700">γ₄,₂ (Telomere):</span>
                  <span class="font-mono">0</span>
                </div>
                <div class="flex justify-between">
                  <span class="text-gray-700">γ₄,₃ (MitoROS):</span>
                  <span class="font-mono">0</span>
                </div>
                <div class="flex justify-between">
                  <span class="text-gray-700">γ₄,₅ (Proteostasis):</span>
                  <span class="font-mono">0</span>
                </div>
                <div class="flex justify-between">
                  <span class="text-gray-700">γ₄,₄ (Autocatalytic):</span>
                  <span class="font-mono">0</span>
                </div>
              </div>
              <div class="mt-4 p-2 bg-gray-50 rounded text-xs text-gray-600">
                All γ parameters default to 0 per CORRECTIONS_2026-04-22 canon §1.3
              </div>
            </card>

            <.card id="sensitivity" class="mt-6">
              <:title>Sobol Sensitivity Indices</:title>
              <%= if @sensitivity == [] do %>
                <div class="p-4 text-center text-gray-500">No sensitivity analysis</div>
              <% else %>
                <div class="space-y-2">
                  <%= for {param, index} <- @sensitivity do %>
                    <div class="flex items-center">
                      <div class="w-24 text-sm text-gray-700 truncate"><%= param %></div>
                      <div class="flex-1 ml-2">
                        <div class="h-2 bg-gray-200 rounded-full overflow-hidden">
                          <div class="h-full bg-indigo-600" style={"width: #{index * 100}%"}></div>
                        </div>
                      </div>
                      <div class="w-10 text-right text-sm font-mono"><%= Float.round(index, 3) %></div>
                    </div>
                  <% end %>
                </div>
              <% end %>
            </.card>
          </div>
        </div>
      <% end %>
    </div>
    """
  end
end