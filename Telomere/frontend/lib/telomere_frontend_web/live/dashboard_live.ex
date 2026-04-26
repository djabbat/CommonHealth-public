defmodule TelomereFrontendWeb.DashboardLive do
  use TelomereFrontendWeb, :live_view

  alias TelomereFrontendWeb.Clients.BackendClient
  alias TelomereFrontendWeb.CoreComponents

  @impl true
  def mount(_params, _session, socket) do
    if connected?(socket), do: schedule_refresh()

    {:ok,
     socket
     |> assign(page_title: "Telomere Counter Dashboard")
     |> assign_initial_data()}
  end

  @impl true
  def handle_params(_params, _uri, socket) do
    {:noreply, socket}
  end

  @impl true
  def handle_event("refresh", _, socket) do
    {:noreply,
     socket
     |> assign_initial_data()
     |> put_flash(:info, "Dashboard refreshed")}
  end

  @impl true
  def handle_info(:refresh, socket) do
    schedule_refresh()
    {:noreply, assign_initial_data(socket)}
  end

  @impl true
  def render(assigns) do
    ~H"""
    <div>
      <div class="mb-8">
        <div class="md:flex md:items-center md:justify-between">
          <div class="min-w-0 flex-1">
            <h2 class="text-2xl font-bold leading-7 text-gray-900 sm:truncate sm:text-3xl sm:tracking-tight">
              Multi-Counter Architecture of Organismal Aging (MCOA)
            </h2>
            <div class="mt-1 flex flex-col sm:mt-0 sm:flex-row sm:flex-wrap sm:space-x-6">
              <div class="mt-2 flex items-center text-sm text-gray-500">
                <Heroicons.information_circle class="mr-1.5 h-5 w-5 flex-shrink-0 text-gray-400" />
                Telomere Shortening Counter #2 – Formal Kinetic Framework
              </div>
            </div>
          </div>
          <div class="mt-4 flex md:ml-4 md:mt-0">
            <button
              type="button"
              phx-click="refresh"
              class="ml-3 inline-flex items-center rounded-md bg-blue-600 px-3 py-2 text-sm font-semibold text-white shadow-sm hover:bg-blue-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-blue-600"
            >
              <Heroicons.arrow_path class="-ml-0.5 mr-1.5 h-5 w-5" />
              Refresh
            </button>
          </div>
        </div>
      </div>

      <.live_component
        module={TelomereFrontendWeb.DashboardLive.BackendStatus}
        id="backend-status"
        status={@backend_status}
      />

      <div class="grid grid-cols-1 gap-6 mb-8">
        <.counter_registry_card
          :for={counter <- @mcoa_counters}
          counter={counter}
        />
      </div>

      <div class="mb-8">
        <h3 class="text-lg font-medium text-gray-900 mb-4">Counter #2 – Kinetic Equation</h3>
        <.kinetic_equation_display counter={Enum.find(@mcoa_counters, &(&1.id == 2))} />
      </div>

      <div>
        <h3 class="text-lg font-medium text-gray-900 mb-4">Canonical Parameters (from PARAMETERS.md)</h3>
        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
          <.parameter_card
            :for={parameter <- @telomere_parameters}
            id={"parameter-#{parameter.id}"}
            parameter={parameter}
          />
        </div>
      </div>

      <div class="mt-8 p-4 bg-yellow-50 border border-yellow-200 rounded-lg">
        <div class="flex">
          <Heroicons.exclamation_triangle class="h-5 w-5 text-yellow-400" />
          <div class="ml-3">
            <h3 class="text-sm font-medium text-yellow-800">Applying CORRECTIONS_2026-04-22</h3>
            <div class="mt-2 text-sm text-yellow-700">
              <ul class="list-disc pl-5 space-y-1">
                <li>Coupling parameters γ_i = 0 for all scaffold projects</li>
                <li>No χ_Ze validated biomarker claims for BioSense</li>
                <li>For Ze displays: v = N_S/(N-1)</li>
                <li>Coupling matrix entries cannot be derived from "MCOA Test 2"</li>
              </ul>
            </div>
          </div>
        </div>
      </div>
    </div>
    """
  end

  defp assign_initial_data(socket) do
    backend_status = BackendClient.health_check()

    socket
    |> assign(
      backend_status: case backend_status do
        {:ok, %{status: 200}} -> "Connected"
        _ -> "Disconnected"
      end,
      telomere_parameters: fetch_telomere_parameters(),
      mcoa_counters: fetch_mcoa_counters()
    )
  end

  defp fetch_telomere_parameters do
    [
      %{
        id: "D20",
        name: "D₂,₀ (Baseline Length)",
        value: "10-15",
        unit: "kb",
        description: "Baseline telomere length at birth (fibroblasts)",
        source: "PMID:24374808",
        status: "measured"
      },
      %{
        id: "alpha2",
        name: "α₂ (Division-Dependent Erosion)",
        value: "50-200",
        unit: "bp/PD",
        description: "Telomere loss per population doubling (end-replication problem)",
        source: "PMID:24374808, PMID:30650660",
        status: "measured"
      },
      %{
        id: "beta2",
        name: "β₂ (Stress-Dependent Erosion)",
        value: "20-50",
        unit: "bp/year",
        description: "Oxidative damage-induced erosion rate",
        source: "PMID:25612739, PMID:30472697",
        status: "estimated"
      },
      %{
        id: "n2star",
        name: "n₂* (Hayflick Limit)",
        value: "40-60",
        unit: "PD",
        description: "Critical replicative limit for human diploid fibroblasts",
        source: "Classical limit, modulated by oxygen (PMID:11001793)",
        status: "measured"
      },
      %{
        id: "tau2",
        name: "τ₂ (Stochastic Timescale)",
        value: "N/A",
        unit: "days/months",
        description: "Timescale for stochastic shortening - requires single-telomere tracking",
        source: "PMID:33347069",
        status: "tbd"
      }
    ]
  end

  defp fetch_mcoa_counters do
    [
      %{
        id: 1,
        name: "Centriolar Aberrations",
        description: "Centriole duplication errors and ciliary dysfunction",
        status: "active",
        equation: "D₁ = f(centriole errors, cilia function)",
        current_value: "Pending",
        unit: "error units",
        progress: 25
      },
      %{
        id: 2,
        name: "Telomere Shortening",
        description: "Progressive loss of telomeric DNA repeats at chromosome ends",
        status: "active",
        equation: "D₂(n,t) = D₂,₀ + α₂·(n/n₂*) + β₂·(t/τ₂) + γ₂·I(others)",
        current_value: "8.2 ± 1.3",
        unit: "kb",
        progress: 65
      },
      %{
        id: 3,
        name: "MitoROS Accumulation",
        description: "Mitochondrial ROS production and antioxidant defense imbalance",
        status: "draft",
        equation: "D₃ = ∫(ROS_production - ROS_scavenging)dt",
        current_value: "Pending",
        unit: "ROS units",
        progress: 15
      },
      %{
        id: 4,
        name: "Epigenetic Drift",
        description: "Cumulative changes in DNA methylation and histone modifications",
        status: "draft",
        equation: "D₄ = Σ|Δmethylation| + histone_mod_changes",
        current_value: "Pending",
        unit: "drift units",
        progress: 10
      },
      %{
        id: 5,
        name: "Proteostasis Collapse",
        description: "Failure of protein quality control systems",
        status: "inactive",
        equation: "D₅ = f(aggregates, chaperone capacity, UPS function)",
        current_value: "Pending",
        unit: "collapse units",
        progress: 5
      }
    ]
  end

  defp schedule_refresh do
    Process.send_after(self(), :refresh, 30_000)
  end

  defmodule BackendStatus do
    use TelomereFrontendWeb, :live_component

    def render(assigns) do
      ~H"""
      <div class="mb-6">
        <div class={[
          "rounded-md p-4",
          if(@status == "Connected",
            do: "bg-green-50 border border-green-200",
            else: "bg-red-50 border border-red-200"
          )
        ]}>
          <div class="flex">
            <div class="flex-shrink-0">
              <Heroicons
                :if={@status == "Connected"}
                name="check_circle"
                class="h-5 w-5 text-green-400"
              />
              <Heroicons
                :if={@status != "Connected"}
                name="exclamation_circle"
                class="h-5 w-5 text-red-400"
              />
            </div>
            <div class="ml-3">
              <h3 class={[
                "text-sm font-medium",
                if(@status == "Connected",
                  do: "text-green-800",
                  else: "text-red-800"
                )
              ]}>
                Backend Status: <%= @status %>
              </h3>
              <div class={[
                "mt-2 text-sm",
                if(@status == "Connected",
                  do: "text-green-700",
                  else: "text-red-700"
                )
              ]}>
                <p>
                  <%= if @status == "Connected" do %>
                    Connected to Rust backend at <%= System.get_env("BACKEND_URL", "http://localhost:3005") %>
                  <% else %>
                    Backend connection failed. Displaying static data from PARAMETERS.md.
                  <% end %>
                </p>
              </div>
            </div>
          </div>
        </div>
      </div>
      """
    end
  end
end