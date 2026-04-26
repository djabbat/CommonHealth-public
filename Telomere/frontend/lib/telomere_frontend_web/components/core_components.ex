defmodule TelomereFrontendWeb.CoreComponents do
  use Phoenix.Component

  @doc """
  Renders a parameter card for the dashboard.
  """
  attr :parameter, :map, required: true
  attr :id, :string, default: nil
  slot :inner_block

  def parameter_card(assigns) do
    ~H"""
    <div id={@id} class="bg-white overflow-hidden shadow rounded-lg border border-gray-200">
      <div class="px-4 py-5 sm:p-6">
        <div class="flex items-center">
          <div class="flex-shrink-0 bg-blue-500 rounded-md p-3">
            <Heroicons.calculator class="h-6 w-6 text-white" />
          </div>
          <div class="ml-5 w-0 flex-1">
            <dl>
              <dt class="text-sm font-medium text-gray-500 truncate">
                <%= @parameter.name %>
              </dt>
              <dd class="flex items-baseline">
                <div class="text-2xl font-semibold text-gray-900">
                  <%= @parameter.value %> <%= @parameter.unit %>
                </div>
                <div class="ml-2 flex items-baseline text-sm font-semibold">
                  <span class={status_color(@parameter.status)}>
                    <%= status_label(@parameter.status) %>
                  </span>
                </div>
              </dd>
            </dl>
          </div>
        </div>
        <div class="mt-4">
          <p class="text-sm text-gray-600"><%= @parameter.description %></p>
          <div class="mt-2">
            <span class="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-gray-100 text-gray-800">
              Source: <%= @parameter.source %>
            </span>
          </div>
        </div>
      </div>
    </div>
    """
  end

  @doc """
  Renders a counter registry entry.
  """
  attr :counter, :map, required: true

  def counter_registry_card(assigns) do
    ~H"""
    <div class="bg-white shadow overflow-hidden sm:rounded-lg border-l-4 border-blue-500">
      <div class="px-4 py-5 sm:px-6">
        <div class="flex items-center justify-between">
          <div>
            <h3 class="text-lg leading-6 font-medium text-gray-900">
              Counter #<%= @counter.id %>: <%= @counter.name %>
            </h3>
            <p class="mt-1 max-w-2xl text-sm text-gray-500">
              <%= @counter.description %>
            </p>
          </div>
          <div class="ml-4 flex-shrink-0">
            <span class={counter_status_class(@counter.status)}>
              <%= String.upcase(@counter.status) %>
            </span>
          </div>
        </div>
      </div>
      <div class="border-t border-gray-200 px-4 py-5 sm:p-0">
        <dl class="sm:divide-y sm:divide-gray-200">
          <div class="py-4 sm:py-5 sm:grid sm:grid-cols-3 sm:gap-4 sm:px-6">
            <dt class="text-sm font-medium text-gray-500">Kinetic Equation</dt>
            <dd class="mt-1 text-sm text-gray-900 sm:mt-0 sm:col-span-2 font-mono">
              <%= @counter.equation %>
            </dd>
          </div>
          <div class="py-4 sm:py-5 sm:grid sm:grid-cols-3 sm:gap-4 sm:px-6">
            <dt class="text-sm font-medium text-gray-500">Current State (D<sub><%= @counter.id %></sub>)</dt>
            <dd class="mt-1 text-sm text-gray-900 sm:mt-0 sm:col-span-2">
              <div class="flex items-center">
                <div class="w-full bg-gray-200 rounded-full h-2.5">
                  <div
                    class={progress_bar_color(@counter.progress)}
                    style={"width: #{@counter.progress}%"}
                  >
                  </div>
                </div>
                <span class="ml-3 text-sm font-medium text-gray-700">
                  <%= @counter.current_value %> <%= @counter.unit %>
                </span>
              </div>
            </dd>
          </div>
          <div class="py-4 sm:py-5 sm:grid sm:grid-cols-3 sm:gap-4 sm:px-6">
            <dt class="text-sm font-medium text-gray-500">Coupling Matrix (Γ)</dt>
            <dd class="mt-1 text-sm text-gray-900 sm:mt-0 sm:col-span-2">
              <div class="space-y-1">
                <p class="text-xs text-gray-500 mb-2">Applying CORRECTIONS_2026-04-22: γ_i = 0 for scaffold projects</p>
                <div class="grid grid-cols-5 gap-1">
                  <div class="text-center text-xs font-medium p-1 bg-gray-100">Counter</div>
                  <div class="text-center text-xs font-medium p-1 bg-gray-100">Γ<sub><%= @counter.id %>,1</sub></div>
                  <div class="text-center text-xs font-medium p-1 bg-gray-100">Γ<sub><%= @counter.id %>,3</sub></div>
                  <div class="text-center text-xs font-medium p-1 bg-gray-100">Γ<sub><%= @counter.id %>,4</sub></div>
                  <div class="text-center text-xs font-medium p-1 bg-gray-100">Γ<sub><%= @counter.id %>,5</sub></div>
                  <div class="text-center text-xs p-1 bg-gray-50">Value</div>
                  <div class="text-center text-xs p-1 bg-gray-50">0</div>
                  <div class="text-center text-xs p-1 bg-yellow-50">TBD</div>
                  <div class="text-center text-xs p-1 bg-gray-50">0</div>
                  <div class="text-center text-xs p-1 bg-gray-50">0</div>
                </div>
              </div>
            </dd>
          </div>
        </dl>
      </div>
    </div>
    """
  end

  @doc """
  Renders the master kinetic equation display.
  """
  attr :counter, :map, required: true

  def kinetic_equation_display(assigns) do
    ~H"""
    <div class="bg-gray-50 p-4 rounded-lg border border-gray-200">
      <h4 class="text-sm font-medium text-gray-700 mb-2">Master Kinetic Equation (Counter #<%= @counter.id %>)</h4>
      <div class="font-mono text-lg bg-white p-4 rounded border">
        D<sub><%= @counter.id %></sub>(n, t) = D<sub><%= @counter.id %>,₀</sub> + α<sub><%= @counter.id %></sub>·(n / n<sub><%= @counter.id %></sub>*) + β<sub><%= @counter.id %></sub>·(t / τ<sub><%= @counter.id %></sub>) + γ<sub><%= @counter.id %></sub>·I(others)
      </div>
      <div class="mt-3 grid grid-cols-2 gap-2 text-sm">
        <div>
          <span class="font-medium">α<sub><%= @counter.id %></sub></span>: Division-dependent erosion coefficient
        </div>
        <div>
          <span class="font-medium">β<sub><%= @counter.id %></sub></span>: Stress/time-dependent erosion coefficient
        </div>
        <div>
          <span class="font-medium">n<sub><%= @counter.id %></sub>*</span>: Critical replicative limit (Hayflick limit)
        </div>
        <div>
          <span class="font-medium">τ<sub><%= @counter.id %></sub></span>: Turnover timescale for stochastic shortening
        </div>
      </div>
    </div>
    """
  end

  defp status_color("measured"), do: "text-green-800"
  defp status_color("estimated"), do: "text-yellow-800"
  defp status_color("tbd"), do: "text-red-800"
  defp status_color(_), do: "text-gray-800"

  defp status_label("measured"), do: "Measured"
  defp status_label("estimated"), do: "Estimated"
  defp status_label("tbd"), do: "TBD"
  defp status_label(_), do: "Unknown"

  defp counter_status_class("active"), do: "inline-flex items-center px-3 py-0.5 rounded-full text-sm font-medium bg-green-100 text-green-800"
  defp counter_status_class("draft"), do: "inline-flex items-center px-3 py-0.5 rounded-full text-sm font-medium bg-yellow-100 text-yellow-800"
  defp counter_status_class("inactive"), do: "inline-flex items-center px-3 py-0.5 rounded-full text-sm font-medium bg-gray-100 text-gray-800"

  defp progress_bar_color(progress) when progress < 30, do: "bg-green-600 h-2.5 rounded-full"
  defp progress_bar_color(progress) when progress < 70, do: "bg-yellow-600 h-2.5 rounded-full"
  defp progress_bar_color(_progress), do: "bg-red-600 h-2.5 rounded-full"
end