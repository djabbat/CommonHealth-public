defmodule OntogenesisFrontendWeb.DashboardLive do
  use OntogenesisFrontendWeb, :live_view

  alias OntogenesisFrontendWeb.Components.BackendStatus
  alias OntogenesisFrontendWeb.CoreComponents

  @impl true
  def mount(_params, _session, socket) do
    if connected?(socket) do
      :timer.send_interval(30_000, self(), :refresh_data)
    end

    {:ok, 
      socket
      |> assign(:page_title, "Ontogenesis Dashboard")
      |> assign(:loading, true)
      |> assign(:error, nil)
      |> assign(:parameters, [])
      |> assign(:phases, [])
      |> assign(:metamorphosis_count, 0)
      |> assign(:backend_status, :unknown)
      |> load_data()
    }
  end

  @impl true
  def handle_info(:refresh_data, socket) do
    {:noreply, socket |> load_data()}
  end

  @impl true
  def handle_event("refresh", _params, socket) do
    {:noreply, socket |> assign(:loading, true) |> load_data()}
  end

  defp load_data(socket) do
    socket = case OntogenesisFrontendWeb.Clients.BackendClient.get_dashboard_data() do
      {:ok, data} ->
        socket
        |> assign(:parameters, data.parameters || [])
        |> assign(:phases, data.phases || [])
        |> assign(:metamorphosis_count, data.metamorphosis_count || 0)
        |> assign(:loading, false)
        |> assign(:error, nil)
        |> assign(:backend_status, :healthy)

      {:error, reason} ->
        socket
        |> assign(:loading, false)
        |> assign(:error, "Backend error: #{inspect(reason)}")
        |> assign(:backend_status, :unhealthy)
    end

    socket
  end

  @impl true
  def render(assigns) do
    ~H"""
    <div class="space-y-6">
      <!-- Header -->
      <div class="md:flex md:items-center md:justify-between">
        <div class="flex-1 min-w-0">
          <h2 class="text-2xl font-bold leading-7 text-gray-900 sm:text-3xl sm:truncate">
            Ontogenesis v4.2 — Developmental prequel to MCOA
          </h2>
          <div class="mt-1 flex flex-col sm:flex-row sm:flex-wrap sm:mt-0 sm:space-x-6">
            <div class="mt-2 flex items-center text-sm text-gray-500">
              <Heroicons.calendar class="flex-shrink-0 mr-1.5 h-5 w-5 text-gray-400" />
              Coverage: 0–120 years (zygote → death)
            </div>
            <div class="mt-2 flex items-center text-sm text-gray-500">
              <Heroicons.cube class="flex-shrink-0 mr-1.5 h-5 w-5 text-gray-400" />
              3 modules: Ontogenesis · Mesogenesis · Gerontogenesis
            </div>
            <div class="mt-2 flex items-center text-sm text-gray-500">
              <Heroicons.chart_bar class="flex-shrink-0 mr-1.5 h-5 w-5 text-gray-400" />
              Algorithm: LCS (Latent Change Score)
            </div>
          </div>
        </div>
        <div class="mt-4 flex md:mt-0 md:ml-4">
          <.button phx-click="refresh" class="inline-flex items-center">
            <Heroicons.arrow_path class="-ml-0.5 mr-2 h-4 w-4" />
            Refresh
          </.button>
        </div>
      </div>

      <!-- Backend Status -->
      <BackendStatus.status status={@backend_status} />

      <!-- Error Display -->
      <%= if @error do %>
        <CoreComponents.error_alert title="Data loading failed">
          <p><%= @error %></p>
          <p class="mt-1">Please check backend connectivity at <%= Application.get_env(:ontogenesis_frontend, :backend_url) %></p>
        </CoreComponents.error_alert>
      <% end %>

      <!-- Loading Spinner -->
      <%= if @loading do %>
        <div class="flex justify-center py-12">
          <CoreComponents.spinner />
        </div>
      <% else %>

        <!-- Key Metrics -->
        <div class="grid grid-cols-1 gap-5 sm:grid-cols-3">
          <div class="bg-white overflow-hidden shadow rounded-lg">
            <div class="px-4 py-5 sm:p-6">
              <dt class="text-sm font-medium text-gray-500 truncate">Total Parameters</dt>
              <dd class="mt-1 text-3xl font-semibold text-gray-900">
                <%= length(@parameters) %>
              </dd>
            </div>
          </div>

          <div class="bg-white overflow-hidden shadow rounded-lg">
            <div class="px-4 py-5 sm:p-6">
              <dt class="text-sm font-medium text-gray-500 truncate">Neurobiological Phases</dt>
              <dd class="mt-1 text-3xl font-semibold text-gray-900">5</dd>
            </div>
          </div>

          <div class="bg-white overflow-hidden shadow rounded-lg">
            <div class="px-4 py-5 sm:p-6">
              <dt class="text-sm font-medium text-gray-500 truncate">Metamorphoses Detected</dt>
              <dd class="mt-1 text-3xl font-semibold text-gray-900">
                <%= @metamorphosis_count %>
              </dd>
            </div>
          </div>
        </div>

        <!-- Phases Section -->
        <div class="bg-white shadow rounded-lg">
          <div class="px-4 py-5 sm:px-6">
            <h3 class="text-lg leading-6 font-medium text-gray-900">Five Neurobiological Phases</h3>
            <p class="mt-1 max-w-2xl text-sm text-gray-500">
              Based on Nat Commun 2025 DOI 10.1038/s41467-025-65974-8
            </p>
          </div>
          <div class="border-t border-gray-200">
            <div class="px-4 py-5 sm:p-6 space-y-4">
              <div class="flex flex-wrap gap-4 justify-center">
                <CoreComponents.phase_indicator phase="I" age_range="0–9 years" />
                <CoreComponents.phase_indicator phase="II" age_range="9–32 years" />
                <CoreComponents.phase_indicator phase="III" age_range="32–66 years" />
                <CoreComponents.phase_indicator phase="IV" age_range="66–83 years" />
                <CoreComponents.phase_indicator phase="V" age_range="83–120 years" />
              </div>
              <div class="mt-4 text-sm text-gray-600">
                <p>Turnpoints: ~9, ~32, ~66, ~83 years (empirical, subject to refinement)</p>
                <p class="mt-1">Three-phase Frolkis model (Ontogenesis/Mesogenesis/Gerontogenesis) remains as larger-scale overlay</p>
              </div>
            </div>
          </div>
        </div>

        <!-- Parameters Table -->
        <div class="bg-white shadow rounded-lg">
          <div class="px-4 py-5 sm:px-6">
            <h3 class="text-lg leading-6 font-medium text-gray-900">Quantitative Parameters</h3>
            <p class="mt-1 max-w-2xl text-sm text-gray-500">
              All domains with measured, estimated, and TBD status
            </p>
          </div>
          <div class="border-t border-gray-200 overflow-x-auto">
            <table class="min-w-full divide-y divide-gray-200">
              <thead class="bg-gray-50">
                <tr>
                  <th scope="col" class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                    Domain
                  </th>
                  <th scope="col" class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                    Parameter
                  </th>
                  <th scope="col" class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                    Value / Range
                  </th>
                  <th scope="col" class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                    Status
                  </th>
                  <th scope="col" class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                    Action
                  </th>
                </tr>
              </thead>
              <tbody class="bg-white divide-y divide-gray-200">
                <%= for param <- Enum.take(@parameters, 10) do %>
                  <tr>
                    <td class="px-6 py-4 whitespace-nowrap text-sm font-medium text-gray-900">
                      <%= param.domain %>
                    </td>
                    <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                      <%= param.name %>
                    </td>
                    <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                      <%= param.range %>
                    </td>
                    <td class="px-6 py-4 whitespace-nowrap">
                      <CoreComponents.badge status={param.status}>
                        <%= param.status %>
                      </CoreComponents.badge>
                    </td>
                    <td class="px-6 py-4 whitespace-nowrap text-sm font-medium">
                      <.link 
                        navigate={~p"/detail/#{param.id}"}
                        class="text-indigo-600 hover:text-indigo-900"
                      >
                        Details
                      </.link>
                    </td>
                  </tr>
                <% end %>
              </tbody>
            </table>
            <%= if length(@parameters) > 10 do %>
              <div class="px-6 py-4 border-t border-gray-200">
                <.link 
                  navigate={~p"/detail/parameters"}
                  class="text-sm text-indigo-600 hover:text-indigo-900"
                >
                  View all <%= length(@parameters) %> parameters →
                </.link>
              </div>
            <% end %>
          </div>
        </div>

        <!-- Algorithm Configuration -->
        <div class="bg-white shadow rounded-lg">
          <div class="px-4 py-5 sm:px-6">
            <h3 class="text-lg leading-6 font-medium text-gray-900">Algorithm Configuration</h3>
            <p class="mt-1 max-w-2xl text-sm text-gray-500">
              LCS model with cross-domain coupling
            </p>
          </div>
          <div class="border-t border-gray-200 px-4 py-5 sm:p-6">
            <dl class="grid grid-cols-1 gap-x-4 gap-y-6 sm:grid-cols-3">
              <div>
                <dt class="text-sm font-medium text-gray-500">Coupling coefficient (γᵢ)</dt>
                <dd class="mt-1 text-sm text-gray-900">0 (CORRECTIONS §1.3 null hypothesis)</dd>
              </div>
              <div>
                <dt class="text-sm font-medium text-gray-500">Metamorphosis radius</dt>
                <dd class="mt-1 text-sm text-gray-900">6 months</dd>
              </div>
              <div>
                <dt class="text-sm font-medium text-gray-500">FDR threshold (q*)</dt>
                <dd class="mt-1 text-sm text-gray-900">0.05 (BH procedure)</dd>
              </div>
            </dl>
            <div class="mt-6 text-sm text-gray-600">
              <p>Cross-domain coupling equation: Δmorphologyₜ = αₘ + βₘ·Mₜ₋₁ + γₘₚ·Pₜ₋₁ + γₘf·Fₜ₋₁ + γₘₛ·Sₜ₋₁ + ζₜ</p>
              <p class="mt-1">Where M=morphology, P=physiology, F=psychology, S=sociology</p>
            </div>
          </div>
        </div>

        <!-- Important Notice -->
        <div class="rounded-md bg-yellow-50 p-4">
          <div class="flex">
            <div class="flex-shrink-0">
              <Heroicons.exclamation_circle class="h-5 w-5 text-yellow-400" />
            </div>
            <div class="ml-3">
              <h3 class="text-sm font-medium text-yellow-800">Research Use Only Notice</h3>
              <div class="mt-2 text-sm text-yellow-700">
                <p>• All simulations use aggregated, anonymized synthetic data</p>
                <p>• 120‑year limit is pragmatic simulation threshold (Barbi et al. Science 2018 shows mortality plateau after 105)</p>
                <p>• BioSense displays raw sensor streams only — no χ_Ze validated biomarker claims</p>
                <p>• No Health Score widget — CORRECTIONS_2026‑04‑22 applied</p>
                <p>• For Ze displays: v = N_S/(N‑1)</p>
              </div>
            </div>
          </div>
        </div>
      <% end %>
    </div>
    """
  end
end