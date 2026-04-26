defmodule ProteostasisFrontendWeb.DashboardLive do
  use ProteostasisFrontendWeb, :live_view

  alias ProteostasisFrontendWeb.BackendClient
  alias ProteostasisFrontendWeb.CoreComponents

  @impl true
  def mount(_params, _session, socket) do
    if connected?(socket), do: schedule_refresh()

    socket =
      socket
      |> assign(:page_title, "Proteostasis Counter Dashboard")
      |> assign(:current_page, :dashboard)
      |> assign(:loading, true)
      |> assign(:error, nil)
      |> assign(:parameters, [])
      |> assign(:simulation_data, nil)
      |> assign(:counter_registry, [])

    {:ok, socket, temporary_assigns: [parameters: []]}
  end

  @impl true
  def handle_params(_params, _uri, socket) do
    load_data(socket)
  end

  @impl true
  def handle_event("refresh", _params, socket) do
    load_data(socket)
  end

  @impl true
  def handle_event("run_simulation", %{"divisions" => divisions, "time" => time}, socket) do
    with {divisions, ""} <- Integer.parse(divisions),
         {time, ""} <- Integer.parse(time),
         {:ok, simulation} <- BackendClient.run_simulation(divisions, time) do
      socket =
        socket
        |> assign(:simulation_data, simulation)
        |> put_flash(:info, "Simulation completed")

      {:noreply, socket}
    else
      _ ->
        {:noreply, put_flash(socket, :error, "Invalid input for simulation")}
    end
  end

  @impl true
  def handle_info(:refresh, socket) do
    schedule_refresh()
    load_data(socket)
  end

  defp load_data(socket) do
    socket = assign(socket, :loading, true)

    case BackendClient.fetch_parameters() do
      {:ok, parameters} ->
        case BackendClient.fetch_counter_registry() do
          {:ok, registry} ->
            socket
            |> assign(:parameters, parameters)
            |> assign(:counter_registry, registry)
            |> assign(:loading, false)
            |> assign(:error, nil)
            |> noreply()

          {:error, error} ->
            socket
            |> assign(:error, "Failed to load counter registry: #{inspect(error)}")
            |> assign(:loading, false)
            |> noreply()
        end

      {:error, error} ->
        socket
        |> assign(:error, "Failed to load parameters: #{inspect(error)}")
        |> assign(:loading, false)
        |> noreply()
    end
  end

  defp schedule_refresh do
    Process.send_after(self(), :refresh, 30_000)
  end

  defp noreply(socket), do: {:noreply, socket}

  def render(assigns) do
    ~H"""
    <div class="space-y-8">
      <.header>
        Proteostasis Counter #5 Dashboard
        <:subtitle>
          Multi-Counter Architecture of Aging — Formal quantification of proteostasis collapse
        </:subtitle>
        <:actions>
          <.button phx-click="refresh" class="bg-indigo-600 hover:bg-indigo-700">
            Refresh
          </.button>
        </:actions>
      </.header>

      <.flash_group flash={@flash} />

      <.section>
        <:title>Counter Registry</:title>
        <:content>
          <.table id="counter-registry" rows={@counter_registry}>
            <:col :let={counter} label="Counter">
              <.badge color="blue">#<%= counter.id %></.badge> <%= counter.name %>
            </:col>
            <:col :let={counter} label="Equation">
              <code class="text-sm"><%= counter.equation %></code>
            </:col>
            <:col :let={counter} label="Status">
              <.badge color={status_color(counter.status)}><%= counter.status %></.badge>
            </:col>
            <:col :let={counter} label="Actions">
              <.link navigate={~p"/detail/#{counter.id}"} class="text-indigo-600 hover:text-indigo-900">
                Details
              </.link>
            </:col>
          </.table>
        </:content>
      </.section>

      <.section>
        <:title>Proteostasis Collapse Parameters</:title>
        <:subtitle>
          D₅(n, t) = D₅,₀ + α₅ · (n / n₅*) + β₅ · (t / τ₅) + γ₅ · I(other counters)
        </:subtitle>
        <:content>
          <.spinner :if={@loading} />
          <div :if={@error} class="rounded-md bg-red-50 p-4">
            <div class="flex">
              <div class="ml-3">
                <h3 class="text-sm font-medium text-red-800">Error</h3>
                <div class="mt-2 text-sm text-red-700">
                  <p><%= @error %></p>
                </div>
              </div>
            </div>
          </div>

          <div :if={!@loading && !@error} class="overflow-hidden shadow ring-1 ring-black ring-opacity-5 sm:rounded-lg">
            <table class="min-w-full divide-y divide-gray-300">
              <thead class="bg-gray-50">
                <tr>
                  <th scope="col" class="py-3.5 pl-4 pr-3 text-left text-sm font-semibold text-gray-900 sm:pl-6">
                    Parameter
                  </th>
                  <th scope="col" class="px-3 py-3.5 text-left text-sm font-semibold text-gray-900">
                    Description
                  </th>
                  <th scope="col" class="px-3 py-3.5 text-left text-sm font-semibold text-gray-900">
                    Value
                  </th>
                  <th scope="col" class="px-3 py-3.5 text-left text-sm font-semibold text-gray-900">
                    Units
                  </th>
                  <th scope="col" class="px-3 py-3.5 text-left text-sm font-semibold text-gray-900">
                    Status
                  </th>
                  <th scope="col" class="relative py-3.5 pl-3 pr-4 sm:pr-6">
                    <span class="sr-only">Actions</span>
                  </th>
                </tr>
              </thead>
              <tbody class="divide-y divide-gray-200 bg-white">
                <tr :for={param <- @parameters}>
                  <td class="whitespace-nowrap py-4 pl-4 pr-3 text-sm font-medium text-gray-900 sm:pl-6">
                    <code><%= param.symbol %></code>
                  </td>
                  <td class="px-3 py-4 text-sm text-gray-500"><%= param.description %></td>
                  <td class="px-3 py-4 text-sm text-gray-500">
                    <span :if={param.value != nil}><%= param.value %></span>
                    <span :if={param.value == nil} class="italic text-gray-400">TBD</span>
                  </td>
                  <td class="px-3 py-4 text-sm text-gray-500"><%= param.units %></td>
                  <td class="px-3 py-4 text-sm">
                    <.badge color={param_status_color(param.status)}>
                      <%= param.status %>
                    </.badge>
                  </td>
                  <td class="relative whitespace-nowrap py-4 pl-3 pr-4 text-right text-sm font-medium sm:pr-6">
                    <.link
                      navigate={~p"/detail/#{param.id}"}
                      class="text-indigo-600 hover:text-indigo-900"
                    >
                      Details<span class="sr-only">, <%= param.symbol %></span>
                    </.link>
                  </td>
                </tr>
              </tbody>
            </table>
          </div>
        </:content>
      </.section>

      <.section>
        <:title>Simulation</:title>
        <:content>
          <div class="grid grid-cols-1 gap-6 lg:grid-cols-2">
            <div class="rounded-lg bg-white p-6 shadow">
              <h3 class="text-lg font-medium leading-6 text-gray-900">Run Simulation</h3>
              <form phx-submit="run_simulation" class="mt-4 space-y-4">
                <div>
                  <label for="divisions" class="block text-sm font-medium text-gray-700">
                    Cell Divisions (n)
                  </label>
                  <input
                    type="number"
                    name="divisions"
                    id="divisions"
                    value="50"
                    class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500 sm:text-sm"
                  />
                </div>
                <div>
                  <label for="time" class="block text-sm font-medium text-gray-700">
                    Time (t, years)
                  </label>
                  <input
                    type="number"
                    name="time"
                    id="time"
                    value="10"
                    step="0.1"
                    class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500 sm:text-sm"
                  />
                </div>
                <.button type="submit" class="w-full">
                  Run Simulation
                </.button>
              </form>
            </div>

            <div :if={@simulation_data} class="rounded-lg bg-white p-6 shadow">
              <h3 class="text-lg font-medium leading-6 text-gray-900">Results</h3>
              <dl class="mt-4 grid grid-cols-1 gap-4 sm:grid-cols-2">
                <div class="rounded-lg bg-gray-50 px-4 py-5 sm:p-6">
                  <dt class="truncate text-sm font-medium text-gray-500">D₅ Damage Load</dt>
                  <dd class="mt-1 text-3xl font-semibold tracking-tight text-gray-900">
                    <%= Float.round(@simulation_data.damage_load, 3) %>
                  </dd>
                </div>
                <div class="rounded-lg bg-gray-50 px-4 py-5 sm:p-6">
                  <dt class="truncate text-sm font-medium text-gray-500">Normalized</dt>
                  <dd class="mt-1 text-3xl font-semibold tracking-tight text-gray-900">
                    <%= Float.round(@simulation_data.normalized, 3) %>
                  </dd>
                </div>
              </dl>
              <div class="mt-6">
                <h4 class="text-sm font-medium text-gray-900">Breakdown</h4>
                <ul role="list" class="mt-3 space-y-3">
                  <li :for={{term, value} <- @simulation_data.breakdown} class="flex items-center justify-between">
                    <span class="text-sm text-gray-500"><%= term %></span>
                    <span class="text-sm font-medium text-gray-900"><%= Float.round(value, 4) %></span>
                  </li>
                </ul>
              </div>
            </div>
          </div>
        </:content>
      </.section>
    </div>
    """
  end

  defp section(assigns) do
    ~H"""
    <div class="space-y-6">
      <div>
        <h2 class="text-xl font-semibold text-gray-900">
          <%= render_slot(@title) %>
        </h2>
        <p :if={@subtitle != []} class="mt-1 text-sm text-gray-500">
          <%= render_slot(@subtitle) %>
        </p>
      </div>
      <div class="rounded-lg border border-gray-200 bg-white shadow-sm">
        <div class="p-6">
          <%= render_slot(@content) %>
        </div>
      </div>
    </div>
    """
  end

  defp param_status_color("TBD"), do: "yellow"
  defp param_status_color("Estimated"), do: "blue"
  defp param_status_color("Hypothetical"), do: "purple"
  defp param_status_color("Default"), do: "gray"
  defp param_status_color("Measured"), do: "green"
  defp param_status_color(_), do: "gray"

  defp status_color("active"), do: "green"
  defp status_color("draft"), do: "yellow"
  defp status_color("archived"), do: "gray"
  defp status_color(_), do: "gray"
end