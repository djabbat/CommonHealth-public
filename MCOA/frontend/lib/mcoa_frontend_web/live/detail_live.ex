defmodule MCOAFrontendWeb.DetailLive do
  use MCOAFrontendWeb, :live_view
  alias MCOAFrontend.BackendClient

  @impl true
  def mount(_params, _session, socket) do
    {:ok, assign(socket, counter: nil, tissues: [], simulations: [], loading: true, not_found: false)}
  end

  @impl true
  def handle_params(%{"counter_id" => counter_id}, _uri, socket) do
    case BackendClient.get_counter(counter_id) do
      {:ok, counter} ->
        tissues = fetch_tissue_weights_for_counter(counter_id)
        simulations = fetch_simulations_for_counter(counter_id)

        {:noreply,
         assign(socket,
           counter: counter,
           tissues: tissues,
           simulations: simulations,
           loading: false,
           not_found: false,
           page_title: "#{counter.name} - MCOA Counter",
           current_page: :dashboard
         )}

      {:error, :not_found} ->
        {:noreply, assign(socket, loading: false, not_found: true, page_title: "Counter Not Found")}

      {:error, _} ->
        {:noreply,
         assign(socket,
           loading: false,
           not_found: false,
           page_title: "Error Loading Counter"
         )}
    end
  end

  defp fetch_tissue_weights_for_counter(counter_id) do
    case BackendClient.list_tissues() do
      {:ok, tissues} ->
        Enum.map(tissues, fn tissue ->
          weight = tissue.weights[counter_id]
          Map.put(tissue, :weight, weight)
        end)
        |> Enum.filter(& &1.weight)

      _ -> []
    end
  end

  defp fetch_simulations_for_counter(counter_id) do
    case BackendClient.list_simulations() do
      {:ok, simulations} ->
        Enum.filter(simulations, fn sim ->
          sim.counters && counter_id in sim.counters
        end)
        |> Enum.take(10)

      _ -> []
    end
  end

  @impl true
  def render(assigns) do
    ~H"""
    <div class="space-y-6">
      <.alert :if={@not_found} type="error">
        Counter not found. Please check the counter ID and try again.
      </.alert>

      <.alert :if={@loading} type="info">
        Loading counter data...
      </.alert>

      <%= if @counter && !@loading do %>
        <div class="flex justify-between items-start">
          <div>
            <h1 class="text-3xl font-bold text-gray-900">
              <%= @counter.name %>
            </h1>
            <p class="mt-2 text-gray-600">
              <%= @counter.description %>
            </p>
            <div class="mt-4 flex items-center space-x-4">
              <span class={
                [
                  "inline-flex items-center px-3 py-1 rounded-full text-sm font-medium",
                  @counter.status == "active" && "bg-green-100 text-green-800",
                  @counter.status == "provisional" && "bg-yellow-100 text-yellow-800",
                  @counter.status == "inactive" && "bg-gray-100 text-gray-800"
                ]
              }>
                <%= String.capitalize(@counter.status) %>
              </span>
              <span class="text-sm text-gray-500">
                ID: <%= @counter.id %> · Canonical #<%= @counter.canonical_index || "N/A" %>
              </span>
            </div>
          </div>
          <a
            href="/"
            class="inline-flex items-center px-4 py-2 border border-gray-300 shadow-sm text-sm font-medium rounded-md text-gray-700 bg-white hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500"
          >
            ← Back to Dashboard
          </a>
        </div>

        <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
          <div class="lg:col-span-2 space-y-6">
            <.card title="Kinetic Parameters">
              <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
                <div>
                  <h4 class="font-medium text-gray-700 mb-3">Reference Scales (a priori)</h4>
                  <dl class="space-y-2">
                    <div class="flex justify-between">
                      <dt class="text-sm text-gray-600">n* (divisions)</dt>
                      <dd class="text-sm font-medium text-gray-900">
                        <%= @counter.reference_divisions || "Not applicable" %>
                      </dd>
                    </div>
                    <div class="flex justify-between">
                      <dt class="text-sm text-gray-600">τ (seconds)</dt>
                      <dd class="text-sm font-medium text-gray-900">
                        <%= if @counter.reference_time do %>
                          <%= @counter.reference_time %> seconds
                          <span class="text-gray-500 ml-1">
                            (~<%= div(@counter.reference_time, 86_400) %> days)
                          </span>
                        <% else %>
                          Not applicable
                        <% end %>
                      </dd>
                    </div>
                    <div class="flex justify-between">
                      <dt class="text-sm text-gray-600">Source</dt>
                      <dd class="text-sm text-gray-900">
                        <%= @counter.reference_source || "Literature" %>
                      </dd>
                    </div>
                  </dl>
                </div>

                <div>
                  <h4 class="font-medium text-gray-700 mb-3">Drift Rates (dimensionless)</h4>
                  <dl class="space-y-2">
                    <div class="flex justify-between">
                      <dt class="text-sm text-gray-600">α (per division-equivalent)</dt>
                      <dd class="text-sm font-medium text-gray-900">
                        <%= Float.round(@counter.alpha, 4) %>
                      </dd>
                    </div>
                    <div class="flex justify-between">
                      <dt class="text-sm text-gray-600">β (per time-equivalent)</dt>
                      <dd class="text-sm font-medium text-gray-900">
                        <%= Float.round(@counter.beta, 4) %>
                      </dd>
                    </div>
                    <div class="flex justify-between">
                      <dt class="text-sm text-gray-600">Critical Threshold</dt>
                      <dd class="text-sm font-medium text-gray-900">
                        <%= Float.round(@counter.critical_threshold, 2) %>
                        <.progress_bar
                          value={@counter.critical_threshold}
                          max={1.0}
                          label={Float.to_string(Float.round(@counter.critical_threshold, 2))}
                        />
                      </dd>
                    </div>
                  </dl>
                </div>
              </div>
            </.card>

            <.card title="Damage Accumulation Formula">
              <div class="font-mono text-sm bg-gray-50 p-4 rounded-lg">
                D<sub><%= @counter.canonical_index %></sub>(n, t) =
                D<sub><%= @counter.canonical_index %>₀</sub> +
                α<sub><%= @counter.canonical_index %></sub> · (n / n<sub><%= @counter.canonical_index %></sub>*) +
                β<sub><%= @counter.canonical_index %></sub> · (t / τ<sub><%= @counter.canonical_index %></sub>) +
                γ<sub><%= @counter.canonical_index %></sub> · I(other counters)
              </div>
              <div class="mt-4 text-sm text-gray-600">
                <p class="mb-2">Where:</p>
                <ul class="list-disc pl-5 space-y-1">
                  <li>n = number of divisions</li>
                  <li>t = time in seconds</li>
                  <li>I(other counters) = Σ<sub>j≠i</sub> γ<sub>ij</sub> · D<sub>j</sub></li>
                </ul>
              </div>
            </.card>

            <.card title="Tissue Weight Distribution">
              <.table headers={["Tissue", "Weight (w)", "Contribution to L_tissue"]}>
                <%= for tissue <- @tissues do %>
                  <tr>
                    <td class="px-6 py-4 whitespace-nowrap text-sm font-medium text-gray-900">
                      <%= tissue.name %>
                    </td>
                    <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-900">
                      <%= Float.round(tissue.weight, 3) %>
                    </td>
                    <td class="px-6 py-4 whitespace-nowrap">
                      <.progress_bar
                        value={tissue.weight}
                        max={1.0}
                        label={Float.to_string(Float.round(tissue.weight, 3))}
                      />
                    </td>
                  </tr>
                <% end %>
              </.table>
            </.card>
          </div>

          <div class="space-y-6">
            <.card title="Counter Metadata">
              <dl class="space-y-3">
                <div>
                  <dt class="text-sm font-medium text-gray-600">MCOA Subproject</dt>
                  <dd class="mt-1 text-sm text-gray-900">
                    <%= @counter.subproject || "Standalone" %>
                  </dd>
                </div>
                <div>
                  <dt class="text-sm font-medium text-gray-600">Nature</dt>
                  <dd class="mt-1 text-sm text-gray-900">
                    <%= if @counter.alpha > 0 and @counter.beta > 0 do %>
                      Mixed division/time
                    <% else if @counter.alpha > 0 do %>
                      Division-dominant
                    <% else if @counter.beta > 0 do %>
                      Time-dominant
                    <% else %>
                      Unknown
                    <% end %>
                  </dd>
                </div>
                <div>
                  <dt class="text-sm font-medium text-gray-600">Measurement Method</dt>
                  <dd class="mt-1 text-sm text-gray-900">
                    <%= @counter.measurement_method || "Not specified" %>
                  </dd>
                </div>
                <div>
                  <dt class="text-sm font-medium text-gray-600">Validation Status</dt>
                  <dd class="mt-1">
                    <span class={
                      [
                        "inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium",
                        @counter.validation_status == "validated" && "bg-green-100 text-green-800",
                        @counter.validation_status == "provisional" && "bg-yellow-100 text-yellow-800",
                        @counter.validation_status == "theoretical" && "bg-gray-100 text-gray-800"
                      ]
                    }>
                      <%= String.capitalize(@counter.validation_status || "theoretical") %>
                    </span>
                  </dd>
                </div>
              </dl>
            </.card>

            <.card title="Related Simulations">
              <.table headers={["ID", "Test", "Status"]}>
                <%= for sim <- @simulations do %>
                  <tr>
                    <td class="px-6 py-4 whitespace-nowrap text-sm font-medium text-gray-900">
                      <a
                        href="#"
                        class="text-blue-600 hover:text-blue-900 hover:underline"
                        phx-click="show_simulation"
                        phx-value-id={sim.id}
                      >
                        <%= String.slice(sim.id, 0..7) %>
                      </a>
                    </td>
                    <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-900">
                      <%= sim.test_name %>
                    </td>
                    <td class="px-6 py-4 whitespace-nowrap">
                      <span class={
                        [
                          "inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium",
                          sim.status == "completed" && "bg-green-100 text-green-800",
                          sim.status == "running" && "bg-blue-100 text-blue-800",
                          sim.status == "failed" && "bg-red-100 text-red-800"
                        ]
                      }>
                        <%= String.capitalize(sim.status) %>
                      </span>
                    </td>
                  </tr>
                <% end %>
              </.table>
            </.card>

            <.card title="Falsifiability Tests">
              <div class="text-sm text-gray-600">
                This counter participates in the following canonical tests:
              </div>
              <ul class="mt-3 space-y-2">
                <%= for test <- (@counter.falsifiability_tests || []) do %>
                  <li class="flex items-start">
                    <div class="flex-shrink-0 h-6 w-6 rounded-full bg-blue-100 text-blue-800 flex items-center justify-center text-xs font-medium">
                      <%= test.number %>
                    </div>
                    <div class="ml-3">
                      <div class="text-sm font-medium text-gray-900">
                        <%= test.name %>
                      </div>
                      <div class="text-xs text-gray-500">
                        <%= test.description %>
                      </div>
                    </div>
                  </li>
                <% end %>
              </ul>
            </.card>
          </div>
        </div>
      <% end %>
    </div>
    """
  end

  @impl true
  def handle_event("show_simulation", %{"id" => simulation_id}, socket) do
    {:noreply,
     socket
     |> put_flash(:info, "Simulation #{String.slice(simulation_id, 0..8)} selected")
     |> push_patch(to: "/")}
  end
end