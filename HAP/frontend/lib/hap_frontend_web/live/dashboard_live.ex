defmodule HAPFrontendWeb.DashboardLive do
  use HAPFrontendWeb, :live_view
  require Logger

  alias HAPFrontendWeb.BackendClient
  alias HAPFrontendWeb.CoreComponents

  @impl true
  def mount(_params, _session, socket) do
    if connected?(socket), do: Process.send_after(self(), :load_data, 100)

    socket =
      socket
      |> assign(:loading, true)
      |> assign(:concept, nil)
      |> assign(:parameters, [])
      |> assign(:knowledge, [])
      |> assign(:error, nil)

    {:ok, socket}
  end

  @impl true
  def handle_info(:load_data, socket) do
    with {:ok, concept} <- BackendClient.get_concept(),
         {:ok, parameters} <- BackendClient.get_parameters(),
         {:ok, knowledge} <- BackendClient.get_knowledge() do
      {:noreply,
       socket
       |> assign(:concept, concept)
       |> assign(:parameters, parameters)
       |> assign(:knowledge, knowledge)
       |> assign(:loading, false)
       |> assign(:error, nil)}
    else
      {:error, reason} ->
        Logger.error("Failed to load dashboard data: #{inspect(reason)}")
        {:noreply,
         socket
         |> assign(:error, "Failed to load data from backend: #{reason}")
         |> assign(:loading, false)}
    end
  end

  @impl true
  def render(assigns) do
    ~H"""
    <div class="space-y-8">
      <.live_header title="HAP Dashboard" />

      <%= if @loading do %>
        <.spinner />
      <% else %>
        <%= if @error do %>
          <.error_alert message={@error} />
        <% end %>

        <div class="grid grid-cols-1 lg:grid-cols-2 gap-8">
          <!-- Left Column -->
          <div class="space-y-8">
            <!-- Central Hypothesis -->
            <div class="bg-blue-50 border-l-4 border-blue-500 p-4 rounded-r-lg">
              <h3 class="text-lg font-semibold text-blue-900 mb-2">Central Hypothesis</h3>
              <p class="text-blue-800">
                <%= if @concept do %>
                  <%= @concept.central_hypothesis %>
                <% else %>
                  Loading...
                <% end %>
              </p>
            </div>

            <!-- Key Data Table -->
            <div>
              <h3 class="text-xl font-bold text-gray-900 mb-4">Taxonomic Correlation (Meta-analysis)</h3>
              <.table
                rows={[
                  %{taxon: "Vertebrates", organ: "Liver", regulators: "Bile acids (FXR/TGR5)", affect: "Yes", status: "✅"},
                  %{taxon: "Insects", organ: "Fat body + pericardial cells", regulators: "Ecdysteroids (EcR/USP)", affect: "Yes", status: "✅"},
                  %{taxon: "Cephalopods", organ: "Hepatopancreas", regulators: "Bile pigments + steroid hormones", affect: "Yes", status: "✅"},
                  %{taxon: "Annelids", organ: "None (chloragogen cells)", regulators: "None", affect: "No", status: "✅"},
                  %{taxon: "Nematodes", organ: "None", regulators: "None", affect: "No", status: "✅"},
                  %{taxon: "Flatworms", organ: "None", regulators: "None", affect: "No", status: "✅"}
                ]}
                columns={[
                  %{key: :taxon, label: "Taxon"},
                  %{key: :organ, label: "Hepatic Organ"},
                  %{key: :regulators, label: "Steroid Regulators"},
                  %{key: :affect, label: "Affect"},
                  %{key: :status, label: "Status"}
                ]}
              />
              <p class="text-sm text-gray-600 mt-2">
                32 taxa with affect → 32 have hepatic organ. 24 taxa without hepatic organ → 0 have affect.
                Correlation: 100% (p < 0.0001)
              </p>
            </div>
          </div>

          <!-- Right Column -->
          <div class="space-y-8">
            <!-- BHCA Assessment -->
            <div>
              <h3 class="text-xl font-bold text-gray-900 mb-4">BHCA Assessment</h3>
              <div class="bg-gray-50 rounded-lg p-4">
                <div class="mb-4">
                  <div class="flex justify-between mb-1">
                    <span class="text-sm font-medium text-gray-700">Strength of Association</span>
                    <span class="text-sm font-medium text-gray-700">2/3</span>
                  </div>
                  <div class="w-full bg-gray-200 rounded-full h-2">
                    <div class="bg-blue-600 h-2 rounded-full" style="width: 66%"></div>
                  </div>
                </div>
                <div class="text-center">
                  <span class="text-2xl font-bold text-gray-900">Average Score: ~1.9</span>
                  <p class="text-sm text-gray-600 mt-1">Weak-moderate support according to Bradford-Hill criteria</p>
                </div>
              </div>
            </div>

            <!-- Key Parameters -->
            <div>
              <h3 class="text-xl font-bold text-gray-900 mb-4">Key Parameters</h3>
              <div class="space-y-3">
                <%= for param <- Enum.take(@parameters, 5) do %>
                  <div class="flex justify-between items-center border-b pb-2">
                    <span class="text-gray-700"><%= param.parameter %></span>
                    <div class="flex items-center space-x-2">
                      <span class="font-semibold"><%= param.value %> <%= param.unit %></span>
                      <.status_badge status={param.status}>
                        <%= param.status %>
                      </.status_badge>
                    </div>
                  </div>
                <% end %>
                <div class="pt-2">
                  <.link
                    navigate={~p"/parameters"}
                    class="text-blue-600 hover:text-blue-800 font-medium"
                  >
                    View all #{length(@parameters)} parameters →
                  </.link>
                </div>
              </div>
            </div>

            <!-- Key Concepts -->
            <div>
              <h3 class="text-xl font-bold text-gray-900 mb-4">Key Concepts</h3>
              <ul class="space-y-3">
                <%= for concept <- Enum.take(@knowledge, 4) do %>
                  <li class="flex items-start">
                    <svg class="h-5 w-5 text-green-500 mr-2 mt-0.5" fill="currentColor" viewBox="0 0 20 20">
                      <path fill-rule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z" clip-rule="evenodd"/>
                    </svg>
                    <span class="text-gray-700"><%= concept.title %></span>
                  </li>
                <% end %>
              </ul>
              <div class="pt-2">
                <.link
                  navigate={~p"/knowledge"}
                  class="text-blue-600 hover:text-blue-800 font-medium"
                >
                  Explore domain knowledge →
                </.link>
              </div>
            </div>
          </div>
        </div>
      <% end %>
    </div>
    """
  end

  defp live_header(assigns) do
    ~H"""
    <header class="bg-white shadow">
      <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        <div class="flex justify-between h-16">
          <div class="flex">
            <div class="flex-shrink-0 flex items-center">
              <h1 class="text-2xl font-bold text-gray-900">
                HAP: HepatoEmotions Project
              </h1>
            </div>
            <nav class="hidden sm:ml-6 sm:flex sm:space-x-8">
              <.link
                :for={item <- @nav}
                patch={elem(item, 1)}
                class="border-transparent text-gray-500 hover:border-gray-300 hover:text-gray-700 inline-flex items-center px-1 pt-1 border-b-2 text-sm font-medium"
              >
                <%= elem(item, 0) %>
              </link>
            </nav>
          </div>
          <div class="flex items-center">
            <span class="inline-flex items-center px-3 py-0.5 rounded-full text-sm font-medium bg-blue-100 text-blue-800">
              v4.0 (STRONG)
            </span>
          </div>
        </div>
      </div>
    </header>
    """
  end
end