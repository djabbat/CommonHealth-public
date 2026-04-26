defmodule BioSenseFrontendWeb.CoreComponents do
  use Phoenix.Component

  alias Phoenix.LiveView.JS

  @doc """
  Renders a modal.

  ## Examples

      <.modal id="confirm-modal">
        This is a modal.
      </.modal>
  """
  attr :id, :string, required: true
  attr :show, :boolean, default: false
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
        aria-describedby={"#{@id}-description"}
        role="dialog"
        aria-modal="true"
      >
        <div class="flex min-h-full items-center justify-center">
          <div class="w-full max-w-3xl p-4 sm:p-6 lg:p-8">
            <.focus_wrap
              id={"#{@id}-container"}
              phx-mounted={@show && show_modal(@id)}
              phx-window-keydown={hide_modal(@id)}
              phx-key="escape"
              phx-click-away={hide_modal(@id)}
              class="hidden relative rounded-2xl bg-white p-6 shadow-lg shadow-zinc-700/10 ring-1 ring-zinc-700/10 transition"
            >
              <div class="absolute top-6 right-6">
                <button
                  phx-click={hide_modal(@id)}
                  type="button"
                  class="-m-3 flex-none p-3 opacity-20 hover:opacity-40"
                  aria-label={gettext("Close")}
                >
                  <.icon name="hero-x-mark-solid" class="h-5 w-5 stroke-current" />
                </button>
              </div>
              <div id={"#{@id}-content"}>
                <%= render_slot(@inner_block) %>
              </div>
            </.focus_wrap>
          </div>
        </div>
      </div>
    </div>
    """
  end

  @doc """
  Renders a loading spinner.
  """
  attr :class, :string, default: nil

  def spinner(assigns) do
    ~H"""
    <div class={["animate-spin rounded-full border-2 border-zinc-300 border-t-zinc-900", @class]}>
      <span class="sr-only">Loading...</span>
    </div>
    """
  end

  @doc """
  Renders a badge with status color.
  """
  attr :status, :string, required: true
  slot :inner_block

  def status_badge(assigns) do
    colors = %{
      "validated" => "bg-green-100 text-green-800",
      "planned" => "bg-blue-100 text-blue-800",
      "testing" => "bg-yellow-100 text-yellow-800",
      "retracted" => "bg-red-100 text-red-800",
      "null" => "bg-gray-100 text-gray-800"
    }

    ~H"""
    <span class={"inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium #{colors[@status]}"}>
      <%= render_slot(@inner_block) %>
    </span>
    """
  end

  @doc """
  Renders a card with header and content.
  """
  attr :title, :string, required: true
  slot :inner_block, required: true

  def card(assigns) do
    ~H"""
    <div class="bg-white overflow-hidden shadow rounded-lg divide-y divide-gray-200">
      <div class="px-4 py-5 sm:px-6">
        <h3 class="text-lg font-medium text-gray-900"><%= @title %></h3>
      </div>
      <div class="px-4 py-5 sm:p-6">
        <%= render_slot(@inner_block) %>
      </div>
    </div>
    """
  end

  @doc """
  Renders a sensor data stream visualization.
  """
  attr :data, :list, default: []
  attr :title, :string, required: true
  attr :unit, :string, default: ""

  def sensor_stream(assigns) do
    ~H"""
    <div class="bg-white p-4 rounded-lg shadow">
      <h4 class="text-sm font-medium text-gray-900 mb-2"><%= @title %></h4>
      <div class="h-48 bg-gray-50 rounded overflow-hidden">
        <svg class="w-full h-full" viewBox="0 0 400 200" preserveAspectRatio="none">
          <polyline
            points={points(@data)}
            fill="none"
            stroke="#3b82f6"
            stroke-width="2"
          />
        </svg>
      </div>
      <div class="mt-2 text-xs text-gray-500">Unit: <%= @unit %></div>
    </div>
    """
  end

  defp points(data) do
    data
    |> Enum.with_index()
    |> Enum.map(fn {value, idx} -> "#{idx * 10},#{200 - value * 2}" end)
    |> Enum.join(" ")
  end

  defp hide_modal(js \\ %JS{}, id) do
    js
    |> JS.hide(to: "##{id}", transition: "fade-out")
    |> JS.hide(to: "##{id}-bg", transition: "fade-out")
    |> JS.remove_class("overflow-hidden", to: "body")
  end

  defp show_modal(js \\ %JS{}, id) do
    js
    |> JS.show(to: "##{id}-bg", transition: "fade-in")
    |> JS.show(to: "##{id}", transition: "fade-in-scale")
    |> JS.add_class("overflow-hidden", to: "body")
    |> JS.focus_first(to: "##{id}-content")
  end
end