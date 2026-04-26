defmodule EpigeneticDriftFrontendWeb.DashboardLive do
  use EpigeneticDriftFrontendWeb, :live_view

  alias EpigeneticDriftFrontendWeb.BackendClient
  alias EpigeneticDriftFrontendWeb.Components

  @impl true
  def mount(_params, _session, socket) do
    if connected?(socket), do: send(self(), :load_data)

    socket =
      socket
      |> assign(:page_title, "Epigenetic Drift Dashboard")
      |> assign(:loading, true)
      |> assign(:error, nil)
      |> assign(:entities, [])
      |> assign(:summary, %{})
      |> assign(:parameters, [])
      |> assign(:time_series, [])

    {:ok, socket}
  end

  @impl true
  def handle_info(:load_data, socket) do
    socket = assign(socket, :loading, true)

    case BackendClient.get_entities("counter") do
      {:ok, entities} ->
        summary = calculate_summary(entities)
        parameters = BackendClient.get_parameters() || []
        time_series = BackendClient.get_time_series() || []

        socket =
          socket
          |> assign(:entities, entities)
          |> assign(:summary, summary)
          |> assign(:parameters, parameters)
          |> assign(:time_series, time_series)
          |> assign(:loading, false)
          |> assign(:error, nil)

        {:noreply, socket}

      {:error, reason} ->
        socket =
          socket
          |> assign(:loading, false)
          |> assign(:error, "Failed to load data: #{inspect(reason)}")
          |> put_flash(:error, "Backend connection failed")

        {:noreply, socket}
    end
  end

  @impl true
  def handle_event("refresh", _params, socket) do
    send(self(), :load_data)
    {:noreply, assign(socket, :loading, true)}
  end

  def handle_event("delete_entity", %{"id" => id}, socket) do
    case BackendClient.delete_entity(id) do
      :ok ->
        send(self(), :load_data)
        {:noreply, put_flash(socket, :info, "Entity deleted")}

      {:error, reason} ->
        {:noreply, put_flash(socket, :error, "Delete failed: #{inspect(reason)}")}
    end
  end

  defp calculate_summary(entities) do
    %{
      total: length(entities),
      avg_d4: calculate_average(entities, "d4"),
      max_d4: calculate_max(entities, "d4"),
      min_d4: calculate_min(entities, "d4"),
      recent: Enum.take(entities, 5)
    }
  end

  defp calculate_average(entities, field) do
    values = Enum.map(entities, &Map.get(&1, field, 0))
    if values == [], do: 0, else: Enum.sum(values) / length(values)
  end

  defp calculate_max(entities, field) do
    entities
    |> Enum.map(&Map.get(&1, field, 0))
    |> Enum.max(fn -> 0 end)
  end

  defp calculate_min(entities, field) do
    entities
    |> Enum.map(&Map.get(&1, field, 0))
    |> Enum.min(fn -> 0 end)
  end

  @impl true
  def render(assigns) do
    ~H"""
    <div class="space-y-6">
      <.header>
        <:title>Epigenetic Drift Counter Dashboard</:title>
        <:actions>
          <.button click="refresh" variant="secondary" disabled={@loading}>
            <.spinner :if={@loading} />
            Refresh Data
          </.button>
        </:actions>
      </.header>

      <.alert :if={@error} type="error">
        <%= @error %>
      </.alert>

      <div class="grid grid-cols-1 gap-6 md:grid-cols-2 lg:grid-cols-4">
        <.metric_card
          id="total-entities"
          label="Total Entities"
          value={@summary.total}
          description="Registered counter instances"
        />
        <.metric_card
          id="avg-d4"
          label="Average D₄"
          value={@summary.avg_d4}
          unit="units"
          description="Mean epigenetic drift"
          trend="up"
        />
        <.metric_card
          id="max-d4"
          label="Max D₄"
          value={@summary.max_d4}
          unit="units"
          description="Maximum observed drift"
        />
        <.metric_card
          id="active"
          label="Active Counters"
          value={length(Enum.filter(@entities, & &1.active))}
          description="Currently tracking"
        />
      </div>

      <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
        <div class="lg:col-span-2">
          <.card id="entities-list">
            <:title>Recent Epigenetic Drift Entities</:title>
            <:actions>
              <.live_patch to="/counter_registry" class="text-sm font-medium text-indigo-600 hover:text-indigo-500">
                View All →
              </.live_patch>
            </:actions>
            <.table headers={["ID", "Tissue", "D₄(n,t)", "Divisions (n)", "Time (t)", "Status", "Actions"]}>
              <:rows :let={entity} rows={@summary.recent}>
                <tr>
                  <td class="px-6 py-4 whitespace-nowrap text-sm font-mono text-gray-900"><%= entity.id %></td>
                  <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500"><%= entity.tissue || "N/A" %></td>
                  <td class="px-6 py-4 whitespace-nowrap text-sm">
                    <.progress_bar
                      id={"d4-#{entity.id}"}
                      label="D₄"
                      value={entity.d4 || 0}
                      min={0}
                      max={100}
                      unit="%"
                    />
                  </td>
                  <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-900"><%= entity.divisions || 0 %></td>
                  <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-900"><%= entity.time_years || 0 %> yrs</td>
                  <td class="px-6 py-4 whitespace-nowrap">
                    <span class={"px-2 inline-flex text-xs leading-5 font-semibold rounded-full #{status_color(entity.status)}"}>
                      <%= entity.status || "unknown" %>
                    </span>
                  </td>
                  <td class="px-6 py-4 whitespace-nowrap text-sm font-medium">
                    <.live_patch to={"/counter/#{entity.id}"} class="text-indigo-600 hover:text-indigo-900">
                      Details
                    </.live_patch>
                    <button
                      phx-click="delete_entity"
                      phx-value-id={entity.id}
                      class="ml-4 text-red-600 hover:text-red-900"
                      onclick="return confirm('Delete this entity?')"
                    >
                      Delete
                    </button>
                  </td>
                </tr>
              </:rows>
            </.table>
          </.card>
        </div>

        <div>
          <.card id="parameter-summary">
            <:title>Counter #4 Parameters</:title>
            <.parameter_table parameters={@parameters} />
          </.card>
        </div>
      </div>

      <.card id="equation-display">
        <:title>Kinetic Equation: D₄(n, t) = D₄,₀ + β₄·(t / τ₄) + α₄·(n / n₄*) + γ₄ · I(others)</:title>
        <div class="p-4 bg-gray-50 rounded-lg font-mono text-sm">
          <div class="grid grid-cols-2 gap-4">
            <div>
              <div class="font-bold">Time-dominant component:</div>
              <div>β₄ = <%= get_parameter(@parameters, "beta4") %></div>
              <div>τ₄ = <%= get_parameter(@parameters, "tau4") %> years</div>
            </div>
            <div>
              <div class="font-bold">Replication component:</div>
              <div>α₄ = <%= get_parameter(@parameters, "alpha4") %></div>
              <div>n₄* = <%= get_parameter(@parameters, "n4_star") %> divisions</div>
            </div>
          </div>
          <div class="mt-4 text-xs text-gray-600">
            Note: All γ parameters default to 0 per CORRECTIONS_2026-04-22 canon.
          </div>
        </div>
      </.card>
    </div>
    """
  end

  defp status_color("active"), do: "bg-green-100 text-green-800"
  defp status_color("inactive"), do: "bg-gray-100 text-gray-800"
  defp status_color("error"), do: "bg-red-100 text-red-800"
  defp status_color(_), do: "bg-yellow-100 text-yellow-800"

  defp get_parameter(parameters, key) do
    case Enum.find(parameters, &(&1.key == key)) do
      nil -> "—"
      param -> param.value
    end
  end
end