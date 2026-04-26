defmodule TelomereFrontendWeb.DetailLive do
  use TelomereFrontendWeb, :live_view

  alias TelomereFrontendWeb.Clients.BackendClient

  @impl true
  def mount(_params, _session, socket) do
    {:ok, assign(socket, page_title: "Detail View", detail_data: nil, loading: true)}
  end

  @impl true
  def handle_params(%{"parameter_id" => param_id}, _uri, socket) do
    detail_data = fetch_parameter_detail(param_id)

    {:noreply,
     socket
     |> assign(
       detail_data: detail_data,
       loading: false,
       page_title: "Parameter: #{detail_data.name}",
       view_type: :parameter
     )}
  end

  @impl true
  def handle_params(%{"counter_id" => counter_id}, _uri, socket) do
    detail_data = fetch_counter_detail(counter_id)

    {:noreply,
     socket
     |> assign(
       detail_data: detail_data,
       loading: false,
       page_title: "Counter #{counter_id}: #{detail_data.name}",
       view_type: :counter
     )}
  end

  @impl true
  def handle_params(_params, _uri, socket) do
    {:noreply, assign(socket, detail_data: nil, loading: false)}
  end

  @impl true
  def render(assigns) do
    ~H"""
    <div>
      <.link
        navigate={~p"/"}
        class="inline-flex items-center text-sm font-medium text-blue-600 hover:text-blue-500 mb-6"
      >
        <Heroicons.arrow_left class="mr-1.5 h-4 w-4" />
        Back to Dashboard
      </.link>

      <div :if={@loading} class="flex justify-center py-12">
        <div class="text-center">
          <Heroicons.arrow_path class="h-12 w-12 text-gray-400 animate-spin mx-auto" />
          <p class="mt-4 text-sm text-gray-500">Loading detail data...</p>
        </div>
      </div>

      <div :if={not @loading and @detail_data}>
        <.render_detail detail_data={@detail_data} view_type={@view_type} />
      </div>
    </div>
    """
  end

  defp render_detail(assigns) do
    case assigns.view_type do
      :parameter -> render_parameter_detail(assigns)
      :counter -> render_counter_detail(assigns)
    end
  end

  defp render_parameter_detail(assigns) do
    ~H"""
    <div class="bg-white shadow overflow-hidden sm:rounded-lg">
      <div class="px-4 py-5 sm:px-6">
        <div class="flex items-center justify-between">
          <div>
            <h1 class="text-2xl font-bold text-gray-900"><%= @detail_data.name %></h1>
            <p class="mt-1 text-sm text-gray-500">Parameter ID: <%= @detail_data.id %></p>
          </div>
          <span class={status_badge_class(@detail_data.status)}>
            <%= String.upcase(@detail_data.status) %>
          </span>
        </div>
      </div>
      <div class="border-t border-gray-200">
        <dl>
          <div class="bg-gray-50 px-4 py-5 sm:grid sm:grid-cols-3 sm:gap-4 sm:px-6">
            <dt class="text-sm font-medium text-gray-500">Value Range</dt>
            <dd class="mt-1 text-sm text-gray-900 sm:mt-0 sm:col-span-2">
              <span class="font-mono text-lg"><%= @detail_data.value %></span>
              <span class="text-gray-500 ml-2"><%= @detail_data.unit %></span>
            </dd>
          </div>
          <div class="bg-white px-4 py-5 sm:grid sm:grid-cols-3 sm:gap-4 sm:px-6">
            <dt class="text-sm font-medium text-gray-500">Description</dt>
            <dd class="mt-1 text-sm text-gray-900 sm:mt-0 sm:col-span-2">
              <%= @detail_data.description %>
            </dd>
          </div>
          <div class="bg-gray-50 px-4 py-5 sm:grid sm:grid-cols-3 sm:gap-4 sm:px-6">
            <dt class="text-sm font-medium text-gray-500">Source / Justification</dt>
            <dd class="mt-1 text-sm text-gray-900 sm:mt-0 sm:col-span-2">
              <div class="space-y-2">
                <p><%= @detail_data.source %></p>
                <div class="text-xs text-gray-500">
                  Verified in mega-audit 2026-04-22
                </div>
              </div>
            </dd>
          </div>
          <div class="bg-white px-4 py-5 sm:grid sm:grid-cols-3 sm:gap-4 sm:px-6">
            <dt class="text-sm font-medium text-gray-500">Biological Context</dt>
            <dd class="mt-1 text-sm text-gray-900 sm:mt-0 sm:col-span-2">
              <%= @detail_data.context || "See CONCEPT.md for detailed biological mechanism" %>
            </dd>
          </div>
          <div class="bg-gray-50 px-4 py-5 sm:grid sm:grid-cols-3 sm:gap-4 sm:px-6">
            <dt class="text-sm font-medium text-gray-500">Measurement Method</dt>
            <dd class="mt-1 text-sm text-gray-900 sm:mt-0 sm:col-span-2">
              <%= @detail_data.measurement || "Varies by study (qPCR, TRF, Flow-FISH, STELA)" %>
            </dd>
          </div>
        </dl>
      </div>
    </div>
    """
  end

  defp render_counter_detail(assigns) do
    ~H"""
    <div class="space-y-6">
      <div class="bg-white shadow overflow-hidden sm:rounded-lg">
        <div class="px-4 py-5 sm:px-6">
          <div class="flex items-center justify-between">
            <div>
              <h1 class="text-2xl font-bold text-gray-900">Counter #<%= @detail_data.id %>: <%= @detail_data.name %></h1>
              <p class="mt-1 text-sm text-gray-500">MCOA Counter Registry Entry</p>
            </div>
            <span class={counter_status_class(@detail_data.status)}>
              <%= String.upcase(@detail_data.status) %>
            </span>
          </div>
        </div>
        <div class="border-t border-gray-200 px-4 py-5 sm:p-0">
          <dl class="sm:divide-y sm:divide-gray-200">
            <div class="py-4 sm:py-5 sm:grid sm:grid-cols-3 sm:gap-4 sm:px-6">
              <dt class="text-sm font-medium text-gray-500">Full Description</dt>
              <dd class="mt-1 text-sm text-gray-900 sm:mt-0 sm:col-span-2">
                <%= @detail_data.description %>
              </dd>
            </div>
            <div class="py-4 sm:py-5 sm:grid sm:grid-cols-3 sm:gap-4 sm:px-6">
              <dt class="text-sm font-medium text-gray-500">Kinetic Equation</dt>
              <dd class="mt-1 text-sm text-gray-900 sm:mt-0 sm:col-span-2 font-mono bg-gray-50 p-4 rounded">
                <%= @detail_data.equation %>
              </dd>
            </div>
            <div class="py-4 sm:py-5 sm:grid sm:grid-cols-3 sm:gap-4 sm:px-6">
              <dt class="text-sm font-medium text-gray-500">Current State Estimation</dt>
              <dd class="mt-1 text-sm text-gray-900 sm:mt-0 sm:col-span-2">
                <div class="flex items-center">
                  <div class="w-full bg-gray-200 rounded-full h-4">
                    <div
                      class={progress_bar_color(@detail_data.progress)}
                      style={"width: #{@detail_data.progress}%"}
                    >
                    </div>
                  </div>
                  <span class="ml-4 text-sm font-medium text-gray-700">
                    <%= @detail_data.progress %>% of critical threshold
                  </span>
                </div>
                <div class="mt-2 text-sm text-gray-500">
                  Current value: <%= @detail_data.current_value %> <%= @detail_data.unit %>
                </div>
              </dd>
            </div>
            <div class="py-4 sm:py-5 sm:grid sm:grid-cols-3 sm:gap-4 sm:px-6">
              <dt class="text-sm font-medium text-gray-500">Coupling with Other Counters</dt>
              <dd class="mt-1 text-sm text-gray-900 sm:mt-0 sm:col-span-2">
                <div class="space-y-3">
                  <p class="text-xs text-gray-500 mb-2">Applying CORRECTIONS_2026-04-22 §1.3: γ_i = 0 for scaffold projects</p>
                  <table class="min-w-full divide-y divide-gray-300">
                    <thead>
                      <tr>
                        <th class="px-3 py-2 text-left text-xs font-medium text-gray-500">Target Counter</th>
                        <th class="px-3 py-2 text-left text-xs font-medium text-gray-500">Γ<sub><%= @detail_data.id %>,j</sub></th>
                        <th class="px-3 py-2 text-left text-xs font-medium text-gray-500">Status</th>
                        <th class="px-3 py-2 text-left text-xs font-medium text-gray-500">Hypothesis</th>
                      </tr>
                    </thead>
                    <tbody class="divide-y divide-gray-200">
                      <tr>
                        <td class="whitespace-nowrap px-3 py-2 text-sm">Centriolar (1)</td>
                        <td class="whitespace-nowrap px-3 py-2 text-sm font-mono">0</td>
                        <td class="whitespace-nowrap px-3 py-2">
                          <span class="inline-flex items-center rounded bg-gray-100 px-2 py-0.5 text-xs font-medium text-gray-800">
                            Default
                          </span>
                        </td>
                        <td class="px-3 py-2 text-sm text-gray-500">Centriolar aberrations → Telomere shortening acceleration</td>
                      </tr>
                      <tr class="bg-yellow-50">
                        <td class="whitespace-nowrap px-3 py-2 text-sm">MitoROS (3)</td>
                        <td class="whitespace-nowrap px-3 py-2 text-sm font-mono">TBD (>0 expected)</td>
                        <td class="whitespace-nowrap px-3 py-2">
                          <span class="inline-flex items-center rounded bg-yellow-100 px-2 py-0.5 text-xs font-medium text-yellow-800">
                            Priority for measurement
                          </span>
                        </td>
                        <td class="px-3 py-2 text-sm text-gray-500">ROS → Oxidative damage to telomeric G-overhang</td>
                      </tr>
                      <tr>
                        <td class="whitespace-nowrap px-3 py-2 text-sm">Epigenetic Drift (4)</td>
                        <td class="whitespace-nowrap px-3 py-2 text-sm font-mono">0</td>
                        <td class="whitespace-nowrap px-3 py-2">
                          <span class="inline-flex items-center rounded bg-gray-100 px-2 py-0.5 text-xs font-medium text-gray-800">
                            Default
                          </span>
                        </td>
                        <td class="px-3 py-2 text-sm text-gray-500">Epigenetic changes → Shelterin silencing</td>
                      </tr>
                      <tr>
                        <td class="whitespace-nowrap px-3 py-2 text-sm">Proteostasis (5)</td>
                        <td class="whitespace-nowrap px-3 py-2 text-sm font-mono">0</td>
                        <td class="whitespace-nowrap px-3 py-2">
                          <span class="inline-flex items-center rounded bg-gray-100 px-2 py-0.5 text-xs font-medium text-gray-800">
                            Default
                          </span>
                        </td>
                        <td class="px-3 py-2 text-sm text-gray-500">Proteostasis collapse → Shelterin complex dysfunction</td>
                      </tr>
                    </tbody>
                  </table>
                </div>
              </dd>
            </div>
          </dl>
        </div>
      </div>

      <div class="bg-blue-50 border border-blue-200 rounded-lg p-4">
        <div class="flex">
          <Heroicons.information_circle class="h-5 w-5 text-blue-400" />
          <div class="ml-3">
            <h3 class="text-sm font-medium text-blue-800">MCOA Tissue Aging Load Equation</h3>
            <div class="mt-2 text-sm text-blue-700">
              <p class="font-mono">L_tissue(n,t) = Σ_i w_i(tissue)·f_i(D_i(n,t))</p>
              <p class="mt-2">
                This counter contributes as: w₂(tissue)·f₂(D₂(n,t))
              </p>
              <p class="mt-1 text-xs text-blue-600">
                Weight coefficient w₂(tissue) requires a priori determination based on cell turnover rate and tissue-specific stress levels (Axiom M3)
              </p>
            </div>
          </div>
        </div>
      </div>
    </div>
    """
  end

  defp fetch_parameter_detail(param_id) do
    case param_id do
      "D20" ->
        %{
          id: "D20",
          name: "D₂,₀ (Baseline Telomere Length)",
          value: "10-15",
          unit: "kb",
          description: "Initial telomere length at birth or conception. Highly variable and genetically determined. For human fibroblasts, typical range is 10-15 kilobases.",
          source: "PMID:24374808 (range for human fibroblasts)",
          status: "measured",
          context: "Sets the starting point for telomere shortening trajectory. This baseline is critical for calculating the absolute deficit D₂(n,t).",
          measurement: "Terminal Restriction Fragment (TRF) analysis, qPCR, Flow-FISH"
        }

      "alpha2" ->
        %{
          id: "alpha2",
          name: "α₂ (Division-Dependent Erosion Coefficient)",
          value: "50-200",
          unit: "bp per population doubling (PD)",
          description: "Average telomere loss per cell division due to the end-replication problem. DNA polymerase cannot fully replicate the 3' ends of linear chromosomes.",
          source: "PMID:24374808, PMID:30650660 (meta-analysis of cultured somatic cells)",
          status: "measured",
          context: "This is the foundational, deterministic component of telomere shortening. The exact value depends on cell type and experimental conditions.",
          measurement: "Longitudinal tracking of telomere length in synchronized cell cultures"
        }

      "beta2" ->
        %{
          id: "beta2",
          name: "β₂ (Stress-Dependent Erosion Coefficient)",
          value: "20-50",
          unit: "bp per year",
          description: "Oxidative stress-induced erosion rate. Telomeric DNA (especially G-rich 3' overhang) is highly susceptible to 8-oxoguanine formation. BER repair attempts cause replication fork collapse and stochastic loss.",
          source: "PMID:25612739, PMID:30472697 (leukocyte and post-mitotic tissue studies)",
          status: "estimated",
          context: "Provides direct mechanistic link between ROS exposure and accelerated telomere shortening independent of replication.",
          measurement: "Correlation studies between oxidative stress markers and telomere attrition rates"
        }

      "n2star" ->
        %{
          id: "n2star",
          name: "n₂* (Critical Replicative Limit / Hayflick Limit)",
          value: "40-60",
          unit: "population doublings (PD)",
          description: "Maximum number of divisions before replicative senescence in human diploid fibroblasts. Modulated by oxygen tension and other environmental factors.",
          source: "Classical limit, modulated by oxygen (PMID:11001793)",
          status: "measured",
          context: "Represents the division capacity before telomere length triggers senescence via p53/p21 and p16/pRB pathways.",
          measurement: "In vitro culturing of primary cells until growth arrest"
        }

      "tau2" ->
        %{
          id: "tau2",
          name: "τ₂ (Stochastic Shortening Timescale)",
          value: "N/A",
          unit: "days/months",
          description: "Timescale for β-component stochastic shortening. Not directly measured, requires longitudinal single-telomere tracking to quantify.",
          source: "PMID:33347069 (indicates dynamics on week scale)",
          status: "tbd",
          context: "Critical parameter for modeling stress-induced telomere erosion independent of cell division.",
          measurement: "Single-telomere tracking in live cells (technically challenging)"
        }

      _ ->
        %{
          id: param_id,
          name: "Unknown Parameter",
          value: "N/A",
          unit: "N/A",
          description: "Parameter not found in registry",
          source: "N/A",
          status: "unknown"
        }
    end
  end

  defp fetch_counter_detail(c