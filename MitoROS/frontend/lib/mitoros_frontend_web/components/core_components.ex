defmodule MitoROSFrontendWeb.CoreComponents do
  use Phoenix.Component

  alias Phoenix.LiveView.JS

  @doc """
  Renders a modal.
  """
  attr :id, :string, required: true
  attr :show, :boolean, default: false
  attr :on_cancel, JS, default: %JS{}
  slot :inner_block, required: true

  def modal(assigns) do
    ~H"""
    <div
      id={@id}
      phx-mounted={@show && show_modal(@id)}
      phx-remove={hide_modal(@id)}
      class="relative z-50 hidden"
    >
      <div id={"#{@id}-bg"} class="fixed inset-0 bg-zinc-50/90 transition-opacity" aria-hidden="true" />
      <div
        class="fixed inset-0 overflow-y-auto"
        aria-labelledby={"#{@id}-title"}
        aria-modal="true"
        role="dialog"
      >
        <div class="flex min-h-full items-center justify-center p-4">
          <.focus_wrap
            id={"#{@id}-container"}
            phx-window-keydown={JS.exec("phx-remove", to: "##{@id}")}
            phx-key="escape"
            phx-click-away={JS.exec("phx-remove", to: "##{@id}")}
            class="hidden w-full max-w-3xl rounded-2xl bg-white p-6 shadow-xl shadow-zinc-700/10 ring-1 ring-zinc-700/10 transition lg:p-8"
          >
            <div class="text-right">
              <button
                phx-click={JS.exec("phx-remove", to: "##{@id}")}
                type="button"
                class="-m-3 flex-none p-3 opacity-20 hover:opacity-40"
                aria-label={gettext("close")}
              >
                ✕
              </button>
            </div>
            <div id={"#{@id}-content"}>
              <%= render_slot(@inner_block) %>
            </div>
          </.focus_wrap>
        </div>
      </div>
    </div>
    """
  end

  @doc """
  Renders a parameter card.
  """
  attr :parameter, :map, required: true

  def parameter_card(assigns) do
    ~H"""
    <div class="rounded-lg border border-zinc-200 bg-white p-4 shadow-sm">
      <div class="flex items-center justify-between">
        <h3 class="text-lg font-semibold text-zinc-900"><%= @parameter.name %></h3>
        <span class={"rounded-full px-3 py-1 text-xs font-medium #{status_color(@parameter.status)}"}>
          <%= @parameter.status %>
        </span>
      </div>
      <p class="mt-2 text-sm text-zinc-600"><%= @parameter.description %></p>
      <div class="mt-4 grid grid-cols-2 gap-4 text-sm">
        <div>
          <span class="font-medium text-zinc-500">Value:</span>
          <span class="ml-2 font-mono text-zinc-900"><%= @parameter.value %></span>
        </div>
        <div>
          <span class="font-medium text-zinc-500">Unit:</span>
          <span class="ml-2 text-zinc-900"><%= @parameter.unit %></span>
        </div>
      </div>
      <div class="mt-4">
        <span class="font-medium text-zinc-500">Source:</span>
        <p class="mt-1 text-sm text-zinc-700"><%= @parameter.source %></p>
      </div>
    </div>
    """
  end

  @doc """
  Renders a counter registry entry.
  """
  attr :counter, :map, required: true
  attr :selected, :boolean, default: false

  def counter_registry_entry(assigns) do
    ~H"""
    <div class={"cursor-pointer rounded-lg border-2 p-4 transition-colors #{if @selected, do: "border-blue-500 bg-blue-50", else: "border-zinc-200 bg-white hover:bg-zinc-50"}"}>
      <div class="flex items-center justify-between">
        <div>
          <h4 class="font-semibold text-zinc-900">Counter #<%= @counter.id %>: <%= @counter.name %></h4>
          <p class="text-sm text-zinc-600"><%= @counter.description %></p>
        </div>
        <div class="text-right">
          <div class="text-lg font-bold text-zinc-900"><%= @counter.current_value %></div>
          <div class="text-xs text-zinc-500">Current D<sub><%= @counter.id %></sub></div>
        </div>
      </div>
      <div class="mt-4 flex items-center justify-between text-sm">
        <div>
          <span class="font-medium text-zinc-500">Weight w<sub><%= @counter.id %></sub>:</span>
          <span class="ml-2 font-mono"><%= @counter.weight %></span>
        </div>
        <div>
          <span class="font-medium text-zinc-500">Γ couplings:</span>
          <span class="ml-2">
            <%= for {target, value} <- @counter.couplings do %>
              <span class="inline-block rounded bg-zinc-100 px-2 py-1 text-xs">
                Γ<sub><%= @counter.id %>,<%= target %></sub> = <%= value %>
              </span>
            <% end %>
          </span>
        </div>
      </div>
    </div>
    """
  end

  @doc """
  Renders a sobol sensitivity visualization.
  """
  attr :data, :list, required: true
  attr :parameters, :list, required: true

  def sobol_sensitivity(assigns) do
    ~H"""
    <div class="rounded-lg border border-zinc-200 bg-white p-6">
      <h3 class="mb-4 text-lg font-semibold text-zinc-900">Sobol Sensitivity Indices</h3>
      <div class="overflow-x-auto">
        <table class="min-w-full divide-y divide-zinc-200">
          <thead>
            <tr class="bg-zinc-50">
              <th class="px-4 py-3 text-left text-xs font-medium uppercase text-zinc-500">Parameter</th>
              <th class="px-4 py-3 text-left text-xs font-medium uppercase text-zinc-500">First Order (S<sub>i</sub>)</th>
              <th class="px-4 py-3 text-left text-xs font-medium uppercase text-zinc-500">Total Order (S<sub>Ti</sub>)</th>
              <th class="px-4 py-3 text-left text-xs font-medium uppercase text-zinc-500">Influence</th>
            </tr>
          </thead>
          <tbody class="divide-y divide-zinc-200 bg-white">
            <%= for {param, idx} <- Enum.with_index(@parameters) do %>
              <tr class="hover:bg-zinc-50">
                <td class="whitespace-nowrap px-4 py-3 font-medium text-zinc-900"><%= param %></td>
                <td class="whitespace-nowrap px-4 py-3">
                  <div class="flex items-center">
                    <div class="w-32 rounded-full bg-zinc-200">
                      <div
                        class={"h-2 rounded-full #{if @data[idx].first_order > 0.3, do: "bg-red-500", else: "bg-blue-500"}"}
                        style={"width: #{min(@data[idx].first_order * 100, 100)}%"}
                      >
                      </div>
                    </div>
                    <span class="ml-2 font-mono text-sm"><%= :erlang.float_to_binary(@data[idx].first_order, decimals: 3) %></span>
                  </div>
                </td>
                <td class="whitespace-nowrap px-4 py-3">
                  <div class="flex items-center">
                    <div class="w-32 rounded-full bg-zinc-200">
                      <div
                        class={"h-2 rounded-full #{if @data[idx].total_order > 0.3, do: "bg-red-500", else: "bg-green-500"}"}
                        style={"width: #{min(@data[idx].total_order * 100, 100)}%"}
                      >
                      </div>
                    </div>
                    <span class="ml-2 font-mono text-sm"><%= :erlang.float_to_binary(@data[idx].total_order, decimals: 3) %></span>
                  </div>
                </td>
                <td class="whitespace-nowrap px-4 py-3">
                  <span class={"rounded-full px-3 py-1 text-xs font-medium #{influence_class(@data[idx].total_order)}"}>
                    <%= influence_label(@data[idx].total_order) %>
                  </span>
                </td>
              </tr>
            <% end %>
          </tbody>
        </table>
      </div>
    </div>
    """
  end

  @doc """
  Renders an HSC lineage tracking visualization.
  """
  attr :lineages, :list, required: true

  def hsc_lineage_tracking(assigns) do
    ~H"""
    <div class="rounded-lg border border-zinc-200 bg-white p-6">
      <h3 class="mb-4 text-lg font-semibold text-zinc-900">HSC Lineage Tracking</h3>
      <div class="space-y-6">
        <%= for lineage <- @lineages do %>
          <div class="border-l-4 border-blue-500 pl-4">
            <div class="flex items-center justify-between">
              <div>
                <h4 class="font-medium text-zinc-900">Lineage <%= lineage.id %></h4>
                <p class="text-sm text-zinc-600">
                  Founder heteroplasmy: <span class="font-mono"><%= lineage.founder_heteroplasmy %>%</span>
                </p>
              </div>
              <div class="text-right">
                <div class="text-2xl font-bold text-zinc-900"><%= lineage.current_cells %></div>
                <div class="text-xs text-zinc-500">cells</div>
              </div>
            </div>
            <div class="mt-4">
              <div class="mb-2 flex justify-between text-sm text-zinc-500">
                <span>Division #0</span>
                <span>Division #<%= lineage.total_divisions %></span>
              </div>
              <div class="h-4 w-full rounded-full bg-zinc-200">
                <div
                  class="h-4 rounded-full bg-gradient-to-r from-blue-400 to-purple-600"
                  style={"width: #{min(lineage.progress * 100, 100)}%"}
                >
                </div>
              </div>
              <div class="mt-2 flex justify-between text-sm">
                <span class="text-zinc-600">Heteroplasmy: <span class="font-mono"><%= lineage.initial_heteroplasmy %>%</span></span>
                <span class="text-zinc-900">Current: <span class="font-mono"><%= lineage.current_heteroplasmy %>%</span></span>
              </div>
            </div>
          </div>
        <% end %>
      </div>
    </div>
    """
  end

  @doc """
  Renders a loading spinner.
  """
  attr :size, :string, default: "md"

  def spinner(assigns) do
    size_classes = %{
      "sm" => "h-4 w-4",
      "md" => "h-8 w-8",
      "lg" => "h-12 w-12"
    }

    ~H"""
    <div class="flex items-center justify-center">
      <svg
        class={"animate-spin text-zinc-400 #{size_classes[@size]}"}
        xmlns="http://www.w3.org/2000/svg"
        fill="none"
        viewBox="0 0 24 24"
      >
        <circle
          class="opacity-25"
          cx="12"
          cy="12"
          r="10"
          stroke="currentColor"
          stroke-width="4"
        >
        </circle>
        <path
          class="opacity-75"
          fill="currentColor"
          d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
        >
        </path>
      </svg>
    </div>
    """
  end

  @doc """
  Renders an error alert.
  """
  slot :inner_block, required: true

  def error_alert(assigns) do
    ~H"""
    <div class="rounded-lg border border-red-200 bg-red-50 p-4">
      <div class="flex">
        <div class="flex-shrink-0">
          <svg class="h-5 w-5 text-red-400" viewBox="0 0 20 20" fill="currentColor">
            <path
              fill-rule="evenodd"
              d="M10 18a8 8 0 100-16 8 8 0 000 16zM8.707 7.293a1 1 0 00-1.414 1.414L8.586 10l-1.293 1.293a1 1 0 101.414 1.414L10 11.414l1.293 1.293a1 1 0 001.414-1.414L11.414 10l1.293-1.293a1 1 0 00-1.414-1.414L10 8.586 8.707 7.293z"
              clip-rule="evenodd"
            />
          </svg>
        </div>
        <div class="ml-3">
          <h3 class="text-sm font-medium text-red-800">Error</h3>
          <div class="mt-2 text-sm text-red-700">
            <%= render_slot(@inner_block) %>
          </div>
        </div>
      </div>
    </div>
    """
  end

  defp show_modal(js \\ %JS{}, id) do
    js
    |> JS.show(to: "##{id}")
    |> JS.show(
      to: "##{id}-bg",
      transition: {"transition-all transform duration-300", "opacity-0", "opacity-100"}
    )
    |> JS.show(
      to: "##{id}-container",
      transition:
        {"transition-all transform duration-300", "opacity-0 scale-95", "opacity-100 scale-100"}
    )
    |> JS.focus_first(to: "##{id}-content")
    |> JS.add_class("overflow-hidden", to: "body")
  end

  defp hide_modal(js \\ %JS{}, id) do
    js
    |> JS.hide(
      to: "##{id}-bg",
      transition: {"transition-all transform duration-300", "opacity-100", "opacity-0"}
    )
    |> JS.hide(
      to: "##{id}-container",
      transition:
        {"transition-all transform duration-300", "opacity-100 scale-100", "opacity-0 scale-95"}
    )
    |> JS.hide(to: "##{id}", transition: {"block", "block", "hidden"})
    |> JS.remove_class("overflow-hidden", to: "body")
  end

  defp status_color("TBD"), do: "bg-yellow-100 text-yellow-800"
  defp status_color("Оцениваемый"), do: "bg-blue-100 text-blue-800"
  defp status_color("Каноническое"), do: "bg-green-100 text-green-800"
  defp status_color("По умолчанию = 0"), do: "bg-zinc-100 text-zinc-800"
  defp status_color(_), do: "bg-gray-100 text-gray-800"

  defp influence_class(val) when val > 0.3, do: "bg-red-100 text-red-800"
  defp influence_class(val) when val > 0.1, do: "bg-yellow-100 text-yellow-800"
  defp influence_class(_), do: "bg-green-100 text-green-800"

  defp influence_label(val) when val > 0.3, do: "High"
  defp influence_label(val) when val > 0.1, do: "Medium"
  defp influence_label(_), do: "Low"
end