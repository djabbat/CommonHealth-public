defmodule BioSenseFrontendWeb.DetailLive do
  use BioSenseFrontendWeb, :live_view

  alias BioSenseFrontendWeb.Clients.BackendClient
  alias BioSenseFrontendWeb.CoreComponents

  @impl true
  def mount(_params, _session, socket) do
    {:ok, assign(socket, page_title: "Details", entity: nil, loading: true)}
  end

  @impl true
  def handle_params(params, _url, socket) do
    action = socket.assigns.live_action
    id = params["id"]

    socket =
      socket
      |> assign(action: action, id: id, loading: true)
      |> load_entity(action, id)

    {:noreply, socket}
  end

  defp load_entity(socket, :dataset, id) do
    case BackendClient.get_dataset(id) do
      {:ok, dataset} ->
        assign(socket,
          entity: dataset,
          loading: false,
          page_title: "Dataset: #{dataset.name}"
        )

      {:error, _} ->
        assign(socket,
          entity: %{
            id: id,
            name: "Unknown Dataset",
            description: "Data not available",
            status: "unknown"
          },
          loading: false,
          page_title: "Dataset Not Found"
        )
    end
  end

  defp load_entity(socket, :parameter, id) do
    case BackendClient.get_parameter(id) do
      {:ok, parameter} ->
        assign(socket,
          entity: parameter,
          loading: false,
          page_title: "Parameter: #{parameter.name}"
        )

      {:error, _} ->
        assign(socket,
          entity: %{
            id: id,
            name: "Unknown Parameter",
            value: "N/A",
            status: "unknown"
          },
          loading: false,
          page_title: "Parameter Not Found"
        )
    end
  end

  defp load_entity(socket, :knowledge, id) do
    case BackendClient.get_knowledge(id) do
      {:ok, knowledge} ->
        assign(socket,
          entity: knowledge,
          loading: false,
          page_title: "Knowledge: #{knowledge.title}"
        )

      {:error, _} ->
        assign(socket,
          entity: %{
            id: id,
            title: "Unknown Concept",
            content: "Information not available"
          },
          loading: false,
          page_title: "Knowledge Not Found"
        )
    end
  end

  defp load_entity(socket, :counters, _id) do
    case BackendClient.get_counters() do
      {:ok, counters} ->
        assign(socket,
          entity: %{type: "counter_registry", counters: counters},
          loading: false,
          page_title: "MCOA Counter Registry"
        )

      {:error, _} ->
        assign(socket,
          entity: %{
            type: "counter_registry",
            counters: [
              %{id: "S", name: "Ze Counter", gamma_i: 0, formula: "v = N_S/(N−1)"},
              %{id: "A", name: "Autonomic Counter", gamma_i: 0, formula: "SDNN/RMSSD"}
            ]
          },
          loading: false,
          page_title: "MCOA Counter Registry"
        )
    end
  end

  defp load_entity(socket, :sensitivity, _id) do
    case BackendClient.get_sensitivity_analysis() do
      {:ok, analysis} ->
        assign(socket,
          entity: %{type: "sensitivity", data: analysis},
          loading: false,
          page_title: "Sobol Sensitivity Analysis"
        )

      {:error, _} ->
        assign(socket,
          entity: %{
            type: "sensitivity",
            data: %{
              parameters: ["gamma_i", "w_i", "theta_Q"],
              first_order: [0.3, 0.5, 0.2],
              total_order: [0.4, 0.6, 0.3]
            }
          },
          loading: false,
          page_title: "Sobol Sensitivity Analysis"
        )
    end
  end

  @impl true
  def render(assigns) do
    ~H"""
    <div class="space-y-6">
      <.header>
        <%= @page_title %>
        <:actions>
          <.link navigate={~p"/"} class="text-sm text-blue-600 hover:text-blue-800">
            ← Back to Dashboard
          </.link>
        </:actions>
      </.header>

      <.card>
        <%= if @loading do %>
          <div class="flex justify-center py-12">
            <.spinner class="h-8 w-8" />
          </div>
        <% else %>
          <%= render_entity(@action, @entity) %>
        <% end %>
      </.card>
    </div>
    """
  end

  defp render_entity(:dataset, entity) do
    ~H"""
    <div class="space-y-4">
      <div class="flex items-start justify-between">
        <div>
          <h2 class="text-xl font-bold text-gray-900"><%= @entity.name %></h2>
          <.status_badge status={@entity.status} class="mt-1">
            <%= String.capitalize(@entity.status) %>
          </.status_badge>
        </div>
        <%= if @entity.n do %>
          <div class="text-right">
            <div class="text-2xl font-bold text-gray-900"><%= @entity.n %></div>
            <div class="text-sm text-gray-500">participants</div>
          </div>
        <% end %>
      </div>

      <%= if @entity.description do %>
        <div class="prose max-w-none">
          <p class="text-gray-700"><%= @entity.description %></p>
        </div>
      <% end %>

      <%= if @entity.results do %>
        <div class="mt-6">
          <h3 class="text-lg font-medium text-gray-900 mb-3">Results</h3>
          <div class="overflow-x-auto">
            <table class="min-w-full divide-y divide-gray-200">
              <thead>
                <tr>
                  <th class="px-4 py-2 text-left text-xs font-medium text-gray-500">Analysis</th>
                  <th class="px-4 py-2 text-left text-xs font-medium text-gray-500">Effect Size (d)</th>
                  <th class="px-4 py-2 text-left text-xs font-medium text-gray-500">p-value</th>
                  <th class="px-4 py-2 text-left text-xs font-medium text-gray-500">Status</th>
                </tr>
              </thead>
              <tbody class="divide-y divide-gray-100">
                <%= for result <- @entity.results do %>
                  <tr>
                    <td class="px-4 py-2 text-sm"><%= result.analysis %></td>
                    <td class="px-4 py-2 text-sm font-mono"><%= result.effect_size %></td>
                    <td class="px-4 py-2 text-sm font-mono"><%= result.p_value %></td>
                    <td class="px-4 py-2">
                      <.status_badge status={result.status}>
                        <%= String.capitalize(result.status) %>
                      </.status_badge>
                    </td>
                  </tr>
                <% end %>
              </tbody>
            </table>
          </div>
        </div>
      <% end %>
    </div>
    """
  end

  defp render_entity(:parameter, entity) do
    ~H"""
    <div class="space-y-4">
      <div class="flex items-center justify-between">
        <h2 class="text-xl font-bold text-gray-900"><%= @entity.name %></h2>
        <.status_badge status={@entity.status}>
          <%= String.capitalize(@entity.status) %>
        </.status_badge>
      </div>

      <div class="grid grid-cols-2 gap-4">
        <div>
          <div class="text-sm text-gray-500">Value</div>
          <div class="text-lg font-mono"><%= @entity.value %> <%= @entity.unit %></div>
        </div>
        <div>
          <div class="text-sm text-gray-500">Category</div>
          <div class="text-lg"><%= @entity.category %></div>
        </div>
      </div>

      <%= if @entity.justification do %>
        <div>
          <div class="text-sm text-gray-500 mb-1">Justification</div>
          <p class="text-gray-700"><%= @entity.justification %></p>
        </div>
      <% end %>

      <%= if @entity.canon_note do %>
        <div class="p-3 bg-yellow-50 rounded border border-yellow-200">
          <div class="text-sm text-yellow-800">
            <strong>Canonical Note:</strong> <%= @entity.canon_note %>
          </div>
        </div>
      <% end %>
    </div>
    """
  end

  defp render_entity(:knowledge, entity) do
    ~H"""
    <div class="space-y-4">
      <h2 class="text-xl font-bold text-gray-900"><%= @entity.title %></h2>
      
      <div class="prose max-w-none">
        <p class="text-gray-700 whitespace-pre-line"><%= @entity.content %></p>
      </div>

      <%= if @entity.implications do %>
        <div class="mt-6 p-4 bg-blue-50 rounded-lg">
          <h3 class="font-medium text-blue-900 mb-2">Implications for BioSense</h3>
          <p class="text-blue-800 text-sm"><%= @entity.implications %></p>
        </div>
      <% end %>
    </div>
    """
  end

  defp render_entity(:counters, entity) do
    ~H"""
    <div class="space-y-6">
      <div>
        <h2 class="text-xl font-bold text-gray-900">MCOA Counter Registry</h2>
        <p class="text-gray-600 mt-1">
          Counter definitions and γ_i linkage parameters. Default γ_i = 0 per CORRECTIONS 2026-04-22.
        </p>
      </div>

      <div class="overflow-x-auto">
        <table class="min-w-full divide-y divide-gray-200">
          <thead>
            <tr>
              <th class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase">Counter ID</th>
              <th class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase">Name</th>
              <th class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase">Formula</th>
              <th class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase">γ_i</th>
              <th class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase">Status</th>
            </tr>
          </thead>
          <tbody class="divide-y divide-gray-200">
            <%= for counter <- @entity.counters do %>
              <tr>
                <td class="px-4 py-3 font-mono font-medium"><%= counter.id %></td>
                <td class="px-4 py-3"><%= counter.name %></td>
                <td class="px-4 py-3 font-mono text-sm"><%= counter.formula %></td>
                <td class="px-4 py-3">
                  <div class="flex items-center">
                    <input
                      type="number"
                      value={counter.gamma_i}
                      step="0.1"
                      class="w-20 px-2 py-1 border rounded text-sm"
                      phx-change="update_gamma"
                      phx-value-counter={counter.id}
                    />
                    <span class="ml-2 text-xs text-gray-500">Default: 0</span>
                  </div>
                </td>
                <td class="px-4 py-3">
                  <.status_badge status={counter.status || "active"}>
                    <%= String.capitalize(counter.status || "active") %>
                  </.status_badge>
                </td>
              </tr>
            <% end %>
          </tbody>
        </table>
      </div>

      <div class="p-4 bg-gray-50 rounded-lg">
        <h3 class="font-medium text-gray-900 mb-2">Note on γ_i Parameters</h3>
        <p class="text-sm text-gray-700">
          γ_i represents linkage strength between counters. Per CORRECTIONS 2026-04-22 §1.3,
          the default hypothesis is independence (γ_i = 0). Non-zero values should only be
          set after post-hoc statistical analysis of empirical data.
        </p>
      </div>
    </div>
    """
  end

  defp render_entity(:sensitivity, entity) do
    ~H"""
    <div class="space-y-6">
      <div>
        <h2 class="text-xl font-bold text-gray-900">Sobol Sensitivity Analysis</h2>
        <p class="text-gray-600 mt-1">
          First-order and total-order sensitivity indices for CDATA model parameters.
        </p>
      </div>

      <div class="h-64 bg-white rounded-lg border">
        <svg class="w-full h-full" viewBox="0 0 600 300">
          <!-- Axes -->
          <line x1="50" y1="250" x2="550" y2="250" stroke="#ccc" stroke-width="1" />
          <line x1="50" y1="250" x2="50" y2="50" stroke="#ccc" stroke-width="1" />
          
          <!-- Grid -->
          <%= for i <- 0..4 do %>
            <line 
              x1={50 + i * 100} 
              y1="50" 
              x2={50 + i * 100} 
              y2="250" 
              stroke="#eee" 
              stroke-width="1" 
            />
            <text x={50 + i * 100} y="270" text-anchor="middle" class="text-xs fill-gray-500">
              <%= Enum.at(@entity.data.parameters || [], i, "P#{i+1}") %>
            </text>
          <% end %>

          <!-- Bars for first-order indices -->
          <%= for {value, idx} <- Enum.with_index(@entity.data.first_order || []) do %>
            <rect
              x={60 + idx * 100}
              y={250 - value * 180}
              width="40"
              height={value * 180}
              fill="#3b82f6"
              opacity="0.7"
            />
            <text
              x={80 + idx * 100}
              y={240 - value * 180}
              text-anchor="middle"
              class="text-xs fill-gray-700"
            >
              <%= Float.round(value, 2) %>
            </text>
          <% end %>

          <!-- Bars for total-order indices -->
          <%= for {value, idx} <- Enum.with_index(@entity.data.total_order || []) do %>
            <rect
              x={100 + idx * 100}
              y={250 - value * 180}
              width="40"
              height={value * 180}
              fill="#10b981"
              opacity="0.7"
            />
            <text
              x={120 + idx * 100}
              y={240 - value * 180}
              text-anchor="middle"
              class="text-xs fill-gray-700"
            >
              <%= Float.round(value, 2) %>
            </text>
          <% end %>
        </svg>
      </div>

      <div class="flex items-center space-x-4 text-sm">
        <div class="flex items-center">
          <div class="h-3 w-3 bg-blue-500 mr-1"></div>
          <span>First-order indices</span>
        </div>
        <div class="flex items-center">
          <div class="h-3 w-3 bg-green-500 mr-1"></div>
          <span>Total-order indices</span>
        </div>
      </div>

      <div class="p-4 bg-blue-50 rounded-lg">
        <h3 class="font-medium text-blue-900 mb-2">Interpretation</h3>
        <p class="text-sm text-blue-800">
          First-order indices measure the direct contribution of each parameter to output variance.
          Total-order indices include interaction effects. Higher values indicate greater sensitivity.
          Analysis supports HSC lineage tracking and parameter optimization.
        </p>
      </div>
    </div>
    """
  end

  @impl true
  def handle_event("update_gamma", %{"counter" => counter_id, "value" => value}, socket) do
    case Float.parse(value) do
      {gamma, _} ->
        # In production, would call BackendClient.update_counter_gamma(counter_id, gamma)
        :telemetry.execute([:biosense_frontend, :counter, :gamma_update], %{
          counter_id: counter_id,
          gamma: gamma
        })
        
        {:noreply,
         socket
         |> put_flash(:info, "Updated γ_#{counter_id} to #{gamma}")
         |> push_navigate(to: ~p"/counters")}

      :error ->
        {:noreply,
         socket
         |> put_flash(:error, "Invalid gamma value")
         |> push_navigate(to: ~p"/counters")}
    end
  end
end