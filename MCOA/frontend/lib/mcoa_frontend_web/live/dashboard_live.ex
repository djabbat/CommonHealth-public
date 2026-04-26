defmodule MCOAFrontendWeb.DashboardLive do
  use MCOAFrontendWeb, :live_view
  alias MCOAFrontend.BackendClient

  @impl true
  def mount(_params, _session, socket) do
    if connected?(socket) do
      schedule_refresh()
    end

    {:ok,
     socket
     |> assign(page_title: "MCOA Dashboard")
     |> assign(current_page: :dashboard)
     |> assign(loading: true)
     |> assign(counters: [])
     |> assign(tissues: [])
     |> assign(coupling_matrix: %{})
     |> assign(simulations: [])
     |> assign(backend_status: :unknown)
     |> assign(last_updated: nil)
     |> load_data(), temporary_assigns: []}
  end

  @impl true
  def handle_params(params, _uri, socket) do
    page = Map.get(params, "page", "dashboard")
    current_page = if page == "registry", do: :registry, else: :dashboard

    {:noreply,
     socket
     |> assign(current_page: current_page)
     |> assign(page_title: if(current_page == :registry, do: "Counter Registry", else: "MCOA Dashboard"))}
  end

  defp load_data(socket) do
    with {:ok, counters} <- BackendClient.list_counters(),
         {:ok, tissues} <- BackendClient.list_tissues(),
         {:ok, coupling} <- BackendClient.get_coupling_matrix(),
         {:ok, simulations} <- BackendClient.list_simulations() do
      assign(socket,
        counters: counters,
        tissues: tissues,
        coupling_matrix: coupling,
        simulations: Enum.take(simulations, 5),
        loading: false,
        backend_status: :healthy,
        last_updated: DateTime.utc_now()
      )
    else
      {:error, _reason} ->
        assign(socket,
          loading: false,
          backend_status: :unhealthy,
          last_updated: DateTime.utc_now()
        )
    end
  end

  @impl true
  def handle_info(:refresh, socket) do
    schedule_refresh()
    {:noreply, load_data(socket)}
  end

  @impl true
  def handle_event("refresh", _params, socket) do
    {:noreply, load_data(socket)}
  end

  defp schedule_refresh do
    Process.send_after(self(), :refresh, 30_000)
  end

  @impl true
  def render(assigns) do
    ~H"""
    <div class="space-y-6">
      <.alert :if={@backend_status == :unhealthy} type="warning">
        Backend API unavailable at <%= Application.get_env(:mcoa_frontend, :backend_url) %>.
        Displaying static reference data from CONCEPT.md.
      </.alert>

      <div class="flex justify-between items-center">
        <div>
          <h1 class="text-3xl font-bold text-gray-900">
            Multi-Counter Architecture Dashboard
          </h1>
          <p class="mt-2 text-gray-600">
            Canonical counters with division/time kinetics and tissue weighting
          </p>
        </div>
        <div class="text-right">
          <button
            phx-click="refresh"
            class="inline-flex items-center px-4 py-2 border border-transparent text-sm font-medium rounded-md shadow-sm text-white bg-blue-600 hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500"
          >
            Refresh
          </button>
          <p class="mt-2 text-sm text-gray-500">
            Last updated: <%= if @last_updated, do: Calendar.strftime(@last_updated, "%H:%M:%S UTC"), else: "Never" %>
          </p>
        </div>
      </div>

      <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
        <div class="lg:col-span-2 space-y-6">
          <.card title="Canonical Counters">
            <.table headers={["ID", "Counter", "α (div)", "β (time)", "Γ Coupling", "Critical Threshold", "Status"]}>
              <%= for counter <- @counters do %>
                <tr>
                  <td class="px-6 py-4 whitespace-nowrap text-sm font-medium text-gray-900">
                    <a
                      href={"/counter/#{counter.id}"}
                      class="text-blue-600 hover:text-blue-900 hover:underline"
                    >
                      <%= counter.id %>
                    </a>
                  </td>
                  <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-900 font-medium">
                    <%= counter.name %>
                  </td>
                  <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-900">
                    <%= Float.round(counter.alpha, 4) %>
                  </td>
                  <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-900">
                    <%= Float.round(counter.beta, 4) %>
                  </td>
                  <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-900">
                    <%= if counter.coupling_strength do %>
                      <.progress_bar value={counter.coupling_strength} max={1.0} />
                    <% else %>
                      N/A
                    <% end %>
                  </td>
                  <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-900">
                    <%= Float.round(counter.critical_threshold, 2) %>
                  </td>
                  <td class="px-6 py-4 whitespace-nowrap">
                    <span class={
                      [
                        "inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium",
                        counter.status == "active" && "bg-green-100 text-green-800",
                        counter.status == "provisional" && "bg-yellow-100 text-yellow-800",
                        counter.status == "inactive" && "bg-gray-100 text-gray-800"
                      ]
                    }>
                      <%= String.capitalize(counter.status) %>
                    </span>
                  </td>
                </tr>
              <% end %>
            </.table>
          </.card>

          <.card title="Tissue-Specific Weights (Constraint: Σw = 1.0)">
            <.table headers={["Tissue"] ++ Enum.map(@counters, & &1.name)}>
              <%= for tissue <- @tissues do %>
                <tr>
                  <td class="px-6 py-4 whitespace-nowrap text-sm font-medium text-gray-900">
                    <%= tissue.name %>
                  </td>
                  <%= for counter <- @counters do %>
                    <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-900">
                      <%= weight = tissue.weights[counter.id] %>
                      <%= if weight do %>
                        <.progress_bar value={weight} max={1.0} label={Float.to_string(Float.round(weight, 3))} />
                      <% else %>
                        N/A
                      <% end %>
                    </td>
                  <% end %>
                </tr>
              <% end %>
            </.table>
          </.card>
        </div>

        <div class="space-y-6">
          <.card title="Coupling Matrix Γ (Acceleration)">
            <div class="text-sm text-gray-600 mb-4">
              Γ<sub>ij</sub> = rate at which counter j accelerates counter i
            </div>
            <div class="grid grid-cols-6 gap-1">
              <div class="col-span-1"></div>
              <%= for counter <- @counters do %>
                <div class="text-xs font-medium text-gray-700 truncate text-center" title={counter.name}>
                  <%= String.at(counter.name, 0) %>
                </div>
              <% end %>

              <%= for row_counter <- @counters do %>
                <div class="text-xs font-medium text-gray-700 truncate" title={row_counter.name}>
                  <%= String.at(row_counter.name, 0) %>
                </div>
                <%= for col_counter <- @counters do %>
                  <div class={
                    [
                      "text-xs p-2 text-center",
                      @coupling_matrix[row_counter.id][col_counter.id] == 0 && "bg-gray-100 text-gray-500",
                      @coupling_matrix[row_counter.id][col_counter.id] > 0 && "bg-red-100 text-red-800",
                      @coupling_matrix[row_counter.id][col_counter.id] < 0 && "bg-blue-100 text-blue-800"
                    ]
                  }>
                    <%= Float.round(@coupling_matrix[row_counter.id][col_counter.id] || 0, 2) %>
                  </div>
                <% end %>
              <% end %>
            </div>
          </.card>

          <.card title="Recent Simulations">
            <.table headers={["ID", "Test", "Status", "Progress"]}>
              <%= for sim <- @simulations do %>
                <tr>
                  <td class="px-6 py-4 whitespace-nowrap text-sm font-medium text-gray-900">
                    <%= String.slice(sim.id, 0..7) %>
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
                        sim.status == "failed" && "bg-red-100 text-red-800",
                        sim.status == "pending" && "bg-yellow-100 text-yellow-800"
                      ]
                    }>
                      <%= String.capitalize(sim.status) %>
                    </span>
                  </td>
                  <td class="px-6 py-4 whitespace-nowrap">
                    <.progress_bar
                      value={sim.progress || 0}
                      max={1.0}
                      label={"#{Float.round((sim.progress || 0) * 100, 1)}%"}
                    />
                  </td>
                </tr>
              <% end %>
            </.table>
          </.card>

          <.card title="MCOA Axioms">
            <ul class="space-y-3 text-sm">
              <li class="flex">
                <div class="flex-shrink-0 w-6 h-6 text-blue-500">M1</div>
                <div class="ml-2 text-gray-700">Parallel counters (≥2 distinct processes)</div>
              </li>
              <li class="flex">
                <div class="flex-shrink-0 w-6 h-6 text-blue-500">M2</div>
                <div class="ml-2 text-gray-700">Dimensional consistency: α·n + β·t reduced to common form</div>
              </li>
              <li class="flex">
                <div class="flex-shrink-0 w-6 h-6 text-blue-500">M3</div>
                <div class="ml-2 text-gray-700">A-priori tissue weighting (no post-hoc fitting)</div>
              </li>
              <li class="flex">
                <div class="flex-shrink-0 w-6 h-6 text-blue-500">M4</div>
                <div class="ml-2 text-gray-700">Falsifiability is first-class</div>
              </li>
            </ul>
          </.card>
        </div>
      </div>

      <%= if @current_page == :registry do %>
        <.card title="Counter Registry">
          <div class="text-gray-600 mb-4">
            All counters must be registered with reference scales n* and τ fixed a priori from independent cell-biological knowledge.
          </div>
          <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
            <%= for counter <- @counters do %>
              <div class="border border-gray-200 rounded-lg p-4">
                <div class="flex justify-between items-start">
                  <div>
                    <h3 class="font-bold text-lg text-gray-900"><%= counter.name %></h3>
                    <p class="text-sm text-gray-600 mt-1"><%= counter.description %></p>
                  </div>
                  <span class={
                    [
                      "inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium",
                      counter.status == "active" && "bg-green-100 text-green-800",
                      counter.status == "provisional" && "bg-yellow-100 text-yellow-800",
                      counter.status == "inactive" && "bg-gray-100 text-gray-800"
                    ]
                  }>
                    <%= String.capitalize(counter.status) %>
                  </span>
                </div>
                <div class="mt-4 grid grid-cols-2 gap-4 text-sm">
                  <div>
                    <div class="font-medium text-gray-700">Reference Scales:</div>
                    <div class="mt-1">
                      <div>n* = <%= counter.reference_divisions || "N/A" %> divisions</div>
                      <div>τ = <%= counter.reference_time || "N/A" %> seconds</div>
                    </div>
                  </div>
                  <div>
                    <div class="font-medium text-gray-700">Drift Rates:</div>
                    <div class="mt-1">
                      <div>α = <%= Float.round(counter.alpha, 4) %> per division-equivalent</div>
                      <div>β = <%= Float.round(counter.beta, 4) %> per time-equivalent</div>
                    </div>
                  </div>
                </div>
              </div>
            <% end %>
          </div>
        </.card>
      <% end %>
    </div>
    """
  end
end