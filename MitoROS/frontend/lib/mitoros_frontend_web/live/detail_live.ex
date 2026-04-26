defmodule MitoROSFrontendWeb.DetailLive do
  use MitoROSFrontendWeb, :live_view

  alias MitoROSFrontendWeb.BackendClient

  @impl true
  def mount(%{"entity_id" => entity_id}, _session, socket) do
    if connected?(socket), do: schedule_refresh()

    {:ok,
     socket
     |> assign_defaults()
     |> load_entity(entity_id)}
  end

  @impl true
  def handle_params(%{"entity_id" => entity_id}, _uri, socket) do
    {:noreply,
     socket
     |> assign(entity_id: entity_id)
     |> load_entity(entity_id)}
  end

  @impl true
  def handle_event("refresh", _params, socket) do
    {:noreply, load_entity(socket.assigns.entity_id, socket)}
  end

  @impl true
  def handle_info(:refresh, socket) do
    schedule_refresh()
    {:noreply, load_entity(socket.assigns.entity_id, socket)}
  end

  defp assign_defaults(socket) do
    assign(socket,
      page_title: "Entity Details",
      loading: true,
      error: nil,
      entity: nil,
      entity_id: nil,
      related_data: [],
      raw_data: nil
    )
  end

  defp load_entity(entity_id, socket \\ nil) do
    socket = if socket, do: assign(socket, loading: true), else: socket

    Task.start_link(fn ->
      case BackendClient.fetch_entity(entity_id) do
        {:ok, entity} ->
          send(self(), {:entity_loaded, entity})

        {:error, reason} ->
          send(self(), {:entity_load_failed, reason})
      end
    end)

    socket || assign_defaults(%{socket | assigns: %{socket.assigns | entity_id: entity_id}})
  end

  @impl true
  def handle_info({:entity_loaded, entity}, socket) do
    socket =
      socket
      |> assign(
        loading: false,
        entity: entity,
        error: nil,
        page_title: "Details: #{entity.name}",
        raw_data: Jason.encode!(entity.raw_data, pretty: true)
      )
      |> load_related_data(entity.type)

    {:noreply, socket}
  end

  defp handle_info({:entity_load_failed, reason}, socket) do
    {:noreply,
     assign(socket,
       loading: false,
       error: "Failed to load entity: #{inspect(reason)}",
       entity: nil
     )}
  end

  defp handle_info({:related_data_loaded, data}, socket) do
    {:noreply, assign(socket, related_data: data)}
  end

  defp load_related_data(socket, "parameter") do
    Task.start_link(fn ->
      case BackendClient.fetch_parameter_sensitivity(socket.assigns.entity_id) do
        {:ok, sensitivity} -> send(self(), {:related_data_loaded, sensitivity})
        {:error, _} -> :ok
      end
    end)

    socket
  end

  defp load_related_data(socket, "lineage") do
    Task.start_link(fn ->
      case BackendClient.fetch_lineage_cells(socket.assigns.entity_id) do
        {:ok, cells} -> send(self(), {:related_data_loaded, cells})
        {:error, _} -> :ok
      end
    end)

    socket
  end

  defp load_related_data(socket, _type), do: socket

  defp schedule_refresh do
    Process.send_after(self(), :refresh, 60_000)
  end

  @impl true
  def render(assigns) do
    ~H"""
    <div class="space-y-6">
      <.header>
        <:title>
          <%= if @entity do %>
            <%= @entity.name %>
          <% else %>
            Loading...
          <% end %>
        </:title>
        <:actions>
          <.button
            variant="outline"
            phx-click="refresh"
            disabled={@loading}
          >
            Refresh
          </.button>
          <.link navigate={~p"/"} class="button button--outline">
            Back to Dashboard
          </.link>
        </:actions>
      </.header>

      <%= if @error do %>
        <.error_alert>
          <%= @error %>
        </.error_alert>
      <% end %>

      <%= if @loading do %>
        <div class="flex h-64 items-center justify-center">
          <.spinner size="lg" />
          <span class="ml-4 text-zinc-600">Loading entity details...</span>
        </div>
      <% else %>
        <%= if @entity do %>
          <div class="grid grid-cols-1 gap-8 lg:grid-cols-3">
            <div class="lg:col-span-2">
              <div class="rounded-lg border border-zinc-200 bg-white p-6">
                <h3 class="mb-4 text-lg font-semibold text-zinc-900">Entity Information</h3>
                <dl class="grid grid-cols-1 gap-4 sm:grid-cols-2">
                  <div>
                    <dt class="text-sm font-medium text-zinc-500">Type</dt>
                    <dd class="mt-1 text-sm text-zinc-900"><%= @entity.type %></dd>
                  </div>
                  <div>
                    <dt class="text-sm font-medium text-zinc-500">ID</dt>
                    <dd class="mt-1 font-mono text-sm text-zinc-900"><%= @entity.id %></dd>
                  </div>
                  <div>
                    <dt class="text-sm font-medium text-zinc-500">Created</dt>
                    <dd class="mt-1 text-sm text-zinc-900"><%= @entity.created_at %></dd>
                  </div>
                  <div>
                    <dt class="text-sm font-medium text-zinc-500">Updated</dt>
                    <dd class="mt-1 text-sm text-zinc-900"><%= @entity.updated_at %></dd>
                  </div>
                </dl>

                <div class="mt-6">
                  <h4 class="font-medium text-zinc-900">Description</h4>
                  <p class="mt-2 text-zinc-700"><%= @entity.description %></p>
                </div>

                <%= if @entity.type == "parameter" do %>
                  <div class="mt-6">
                    <h4 class="font-medium text-zinc-900">Parameter Details</h4>
                    <div class="mt-4 grid grid-cols-2 gap-4">
                      <div>
                        <span class="text-sm font-medium text-zinc-500">Current Value</span>
                        <div class="mt-1 text-2xl font-bold text-zinc-900"><%= @entity.value %></div>
                      </div>
                      <div>
                        <span class="text-sm font-medium text-zinc-500">Unit</span>
                        <div class="mt-1 text-lg text-zinc-900"><%= @entity.unit %></div>
                      </div>
                    </div>
                    <div class="mt-4">
                      <span class="text-sm font-medium text-zinc-500">Source</span>
                      <p class="mt-1 text-sm text-zinc-700"><%= @entity.source %></p>
                    </div>
                    <div class="mt-4">
                      <span class="text-sm font-medium text-zinc-500">Status</span>
                      <span class={"ml-2 rounded-full px-3 py-1 text-xs font-medium #{status_color(@entity.status)}"}>
                        <%= @entity.status %>
                      </span>
                    </div>
                  </div>
                <% end %>

                <%= if @entity.type == "lineage" do %>
                  <div class="mt-6">
                    <h4 class="font-medium text-zinc-900">Lineage Details</h4>
                    <div class="mt-4 grid grid-cols-2 gap-4">
                      <div>
                        <span class="text-sm font-medium text-zinc-500">Founder Heteroplasmy</span>
                        <div class="mt-1 text-2xl font-bold text-zinc-900"><%= @entity.founder_heteroplasmy %>%</div>
                      </div>
                      <div>
                        <span class="text-sm font-medium text-zinc-500">Current Cells</span>
                        <div class="mt-1 text-2xl font-bold text-zinc-900"><%= @entity.cell_count %></div>
                      </div>
                      <div>
                        <span class="text-sm font-medium text-zinc-500">Total Divisions</span>
                        <div class="mt-1 text-lg text-zinc-900"><%= @entity.total_divisions %></div>
                      </div>
                      <div>
                        <span class="text-sm font-medium text-zinc-500">Current Heteroplasmy</span>
                        <div class="mt-1 text-lg text-zinc-900"><%= @entity.current_heteroplasmy %>%</div>
                      </div>
                    </div>
                  </div>
                <% end %>
              </div>

              <div class="mt-8">
                <h3 class="mb-4 text-lg font-semibold text-zinc-900">Raw Data</h3>
                <div class="rounded-lg border border-zinc-200 bg-zinc-50 p-4">
                  <pre class="overflow-x-auto text-sm text-zinc-800"><code><%= @raw_data %></code></pre>
                </div>
              </div>
            </div>

            <div class="space-y-8">
              <%= if @related_data && Enum.any?(@related_data) do %>
                <div class="rounded-lg border border-zinc-200 bg-white p-6">
                  <h3 class="mb-4 text-lg font-semibold text-zinc-900">Related Data</h3>
                  <div class="space-y-4">
                    <%= for item <- @related_data do %>
                      <div class="rounded-lg border border-zinc-100 p-4">
                        <div class="flex items-center justify-between">
                          <div>
                            <h4 class="font-medium text-zinc-900"><%= item.name %></h4>