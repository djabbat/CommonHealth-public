defmodule HAPFrontendWeb.DetailLive do
  use HAPFrontendWeb, :live_view
  require Logger

  alias HAPFrontendWeb.BackendClient
  alias HAPFrontendWeb.CoreComponents

  @impl true
  def mount(params, _session, socket) do
    view_type = Map.get(params, "view", "concept")

    socket =
      socket
      |> assign(:view_type, view_type)
      |> assign(:loading, true)
      |> assign(:data, nil)
      |> assign(:error, nil)

    if connected?(socket) do
      Process.send_after(self(), {:load_data, view_type}, 100)
    end

    {:ok, socket}
  end

  @impl true
  def handle_params(params, _uri, socket) do
    view_type = Map.get(params, "view", "concept")
    {:noreply, assign(socket, :view_type, view_type)}
  end

  @impl true
  def handle_info({:load_data, view_type}, socket) do
    load_function = case view_type do
      "concept" -> &BackendClient.get_concept/0
      "parameters" -> &BackendClient.get_parameters/0
      "knowledge" -> &BackendClient.get_knowledge/0
    end

    case load_function.() do
      {:ok, data} ->
        {:noreply,
         socket
         |> assign(:data, data)
         |> assign(:loading, false)
         |> assign(:error, nil)}

      {:error, reason} ->
        Logger.error("Failed to load #{view_type} data: #{inspect(reason)}")
        {:noreply,
         socket
         |> assign(:error, "Failed to load #{view_type} data: #{reason}")
         |> assign(:loading, false)}
    end
  end

  @impl true
  def render(assigns) do
    ~H"""
    <div class="space-y-6">
      <.live_header title={page_title(@view_type)} />

      <%= if @loading do %>
        <.spinner />
      <% else %>
        <%= if @error do %>
          <.error_alert message={@error} />
        <% end %>

        <%= case @view_type do %>
          <% "concept" -> %>
            <.concept_view data={@data} />
          <% "parameters" -> %>
            <.parameters_view data={@data} />
          <% "knowledge" -> %>
            <.knowledge_view data={@data} />
        <% end %>
      <% end %>
    </div>
    """
  end

  defp page_title("concept"), do: "Concept Details"
  defp page_title("parameters"), do: "Parameters & Measurements"
  defp page_title("knowledge"), do: "Domain Knowledge"
  defp page_title(_), do: "Details"

  defp concept_view(assigns) do
    ~H"""
    <div class="prose max-w-none">
      <div class="mb-8 p-4 bg-yellow-50 border-l-4 border-yellow-500 rounded-r-lg">
        <h3 class="text-lg font-semibold text-yellow-900 mb-2">Version: CONCEPT v4.0 (STRONG)</h3>
        <p class="text-yellow-800">
          Status: Working hypothesis, empirically supported, accepted for publication (Biological Reviews, IF ~10)
        </p>
      </div>

      <h2 class="text-2xl font-bold text-gray-900 mb-4">1. Central Hypothesis (Final Formulation)</h2>
      <div class="bg-blue-50 p-4 rounded-lg mb-6">
        <p class="text-blue-900 font-medium">
          <%= @data.central_hypothesis %>
        </p>
      </div>

      <h2 class="text-2xl font-bold text-gray-900 mb-4">2. Operational Definitions</h2>
      <div class="grid grid-cols-1 md:grid-cols-2 gap-6 mb-8">
        <div class="bg-white border border-gray-200 rounded-lg p-4">
          <h3 class="font-bold text-gray-900 mb-2">Affective State (Behavioral Criteria)</h3>
          <ul class="list-disc pl-5 space-y-1">
            <:for item <- @data.affective_criteria>
              <li class="text-gray-700"><%= item %></li>
            </:for>
          </ul>
        </div>
        <div class="bg-white border border-gray-200 rounded-lg p-4">
          <h3 class="font-bold text-gray-900 mb-2">Hepatic Organ (Functional Definition)</h3>
          <div class="space-y-3">
            <div>
              <span class="font-medium text-gray-800">(A) Secretes steroid regulatory molecules:</span>
              <ul class="list-disc pl-5 mt-1 space-y-1">
                <:for item <- @data.hepatic_criteria_a>
                  <li class="text-gray-700"><%= item %></li>
                </:for>
              </ul>
            </div>
            <div>
              <span class="font-medium text-gray-800">(B) Barrier-detoxification function:</span>
              <ul class="list-disc pl-5 mt-1 space-y-1">
                <:for item <- @data.hepatic_criteria_b>
                  <li class="text-gray-700"><%= item %></li>
                </:for>
              </ul>
            </div>
          </div>
        </div>
      </div>

      <h2 class="text-2xl font-bold text-gray-900 mb-4">3. BBB-Permeable Molecules: Key Mechanism</h2>
      <.table
        rows={@data.bbb_molecules}
        columns={[
          %{key: :category, label: "Category"},
          %{key: :examples, label: "Examples"},
          %{key: :bbb_permeability, label: "BBB Permeability"},
          %{key: :source, label: "Source"},
          %{key: :affective_modulation, label: "Affective Modulation"}
        ]}
      />
    </div>
    """
  end

  defp parameters_view(assigns) do
    ~H"""
    <div>
      <div class="mb-6 p-4 bg-gray-50 rounded-lg">
        <p class="text-gray-700">
          <strong>Note:</strong> Most parameters related to the formal damage model are TBD, as the hypothesis is at the conceptual development stage. Active parameterization is expected after funding (2028-2029).
        </p>
      </div>

      <.table
        rows={@data}
        columns={[
          %{key: :parameter, label: "Parameter"},
          %{key: :value, label: "Value"},
          %{key: :unit, label: "Unit"},
          %{key: :source, label: "Source / Justification"},
          %{
            key: :status,
            label: "Status",
            render: fn row ->
              assigns = %{status: row.status}
              ~H"""
              <.status_badge status={@status}>
                <%= @status %>
              </.status_badge>
              """
            end
          }
        ]}
      />

      <div class="mt-8 grid grid-cols-1 md:grid-cols-2 gap-6">
        <div class="bg-white border border-gray-200 rounded-lg p-4">
          <h3 class="font-bold text-gray-900 mb-3">Status Distribution</h3>
          <div class="space-y-2">
            <:for status <- ["Measured", "Estimated", "TBD", "Canonical"]>
              <% count = Enum.count(@data, &(&1.status == status)) %>
              <div class="flex justify-between items-center">
                <span class="text-gray-700"><%= status %></span>
                <span class="font-semibold"><%= count %> parameters</span>
              </div>
            </:for>
          </div>
        </div>
        <div class="bg-white border border-gray-200 rounded-lg p-4">
          <h3 class="font-bold text-gray-900 mb-3">Key Canonical Parameters</h3>
          <div class="space-y-3">
            <:for param <- Enum.filter(@data, &(&1.status in ["Canonical", "Measured"])) |> Enum.take(3)>
              <div class="border-b pb-2">
                <div class="font-medium text-gray-900"><%= param.parameter %></div>
                <div class="text-sm text-gray-600"><%= param.value %> <%= param.unit %></div>
              </div>
            </:for>
          </div>
        </div>
      </div>
    </div>
    """
  end

  defp knowledge_view(assigns) do
    ~H"""
    <div class="space-y-8">
      <:for concept <- @data>
        <div class="bg-white border border-gray-200 rounded-lg p-6">
          <h3 class="text-xl font-bold text-gray-900 mb-3"><%= concept.title %></h3>
          <div class="prose max-w-none text-gray-700">
            <p><%= concept.description %></p>
          </div>
          <:if concept.details>
            <div class="mt-4 pl-4 border-l-4 border-gray-300">
              <:for detail <- concept.details>
                <p class="text-gray-600 mb-2"><%= detail %></p>
              </:for>
            </div>
          </:if>
        </div>
      </:for>
    </div>
    """
  end

  defp live_header(assigns) do
    ~H"""
    <div class="flex justify-between items-center mb-6">
      <div>
        <h1 class="text-3xl font-bold text-gray-900"><%= @title %></h1>
        <div class="mt-2 flex space-x-4">
          <.link
            navigate={~p"/concept"}
            class={"px-3 py-1 rounded-full text-sm font-medium #{if @view_type == "concept", do: "bg-blue-100 text-blue-800", else: "bg-gray-100 text-gray-800"}"}
          >
            Concept
          </.link>
          <.link
            navigate={~p"/parameters"}
            class={"px-3 py-1 rounded-full text-sm font-medium #{if @view_type == "parameters", do: "bg-blue-100 text-blue-800", else: "bg-gray-100 text-gray-800"}"}
          >
            Parameters
          </.link>
          <.link
            navigate={~p"/knowledge"}
            class={"px-3 py-1 rounded-full text-sm font-medium #{if @view_type == "knowledge", do: "bg-blue-100 text-blue-800", else: "bg-gray-100 text-gray-800"}"}
          >
            Knowledge
          </.link>
        </div>
      </div>
      <div class="text-sm text-gray-500">
        Data loaded from backend
      </div>
    </div>
    """
  end
end