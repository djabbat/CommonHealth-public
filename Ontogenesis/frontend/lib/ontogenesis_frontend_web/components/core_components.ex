defmodule OntogenesisFrontendWeb.CoreComponents do
  use Phoenix.Component

  @doc """
  Renders a button.
  """
  attr :type, :string, default: nil
  attr :class, :string, default: nil
  attr :rest, :global, include: ~w(disabled form name value)

  slot :inner_block, required: true

  def button(assigns) do
    ~H"""
    <button
      type={@type}
      class={[
        "phx-submit-loading:opacity-75 rounded-lg bg-zinc-900 hover:bg-zinc-700 py-2 px-3",
        "text-sm font-semibold leading-6 text-white active:text-white/80",
        @class
      ]}
      {@rest}
    >
      <%= render_slot(@inner_block) %>
    </button>
    """
  end

  @doc """
  Renders a card.
  """
  attr :class, :string, default: nil
  slot :inner_block, required: true

  def card(assigns) do
    ~H"""
    <div class={["bg-white overflow-hidden shadow rounded-lg divide-y divide-gray-200", @class]}>
      <%= render_slot(@inner_block) %>
    </div>
    """
  end

  @doc """
  Renders a badge with status color.
  """
  attr :status, :string, required: true
  slot :inner_block, required: true

  def badge(assigns) do
    color_class = case @status do
      "Measured" -> "bg-green-100 text-green-800"
      "Estimated" -> "bg-yellow-100 text-yellow-800"
      "TBD" -> "bg-gray-100 text-gray-800"
      _ -> "bg-gray-100 text-gray-800"
    end

    ~H"""
    <span class={"inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium #{color_class}"}>
      <%= render_slot(@inner_block) %>
    </span>
    """
  end

  @doc """
  Renders a progress bar.
  """
  attr :value, :integer, required: true
  attr :max, :integer, default: 100
  attr :label, :string, default: nil

  def progress_bar(assigns) do
    percentage = Float.round(@value / @max * 100, 1)
    
    ~H"""
    <div>
      <div class="flex justify-between mb-1">
        <span class="text-sm font-medium text-gray-700"><%= @label %></span>
        <span class="text-sm font-medium text-gray-700"><%= percentage %>%</span>
      </div>
      <div class="w-full bg-gray-200 rounded-full h-2.5">
        <div class="bg-blue-600 h-2.5 rounded-full" style={"width: #{percentage}%"}></div>
      </div>
    </div>
    """
  end

  @doc """
  Renders a loading spinner.
  """
  def spinner(assigns) do
    ~H"""
    <div class="flex justify-center items-center">
      <div class="animate-spin rounded-full h-8 w-8 border-b-2 border-gray-900"></div>
    </div>
    """
  end

  @doc """
  Renders an error alert.
  """
  attr :title, :string, required: true
  slot :inner_block, required: true

  def error_alert(assigns) do
    ~H"""
    <div class="rounded-md bg-red-50 p-4">
      <div class="flex">
        <div class="flex-shrink-0">
          <Heroicons.exclamation_triangle class="h-5 w-5 text-red-400" />
        </div>
        <div class="ml-3">
          <h3 class="text-sm font-medium text-red-800"><%= @title %></h3>
          <div class="mt-2 text-sm text-red-700">
            <%= render_slot(@inner_block) %>
          </div>
        </div>
      </div>
    </div>
    """
  end

  @doc """
  Renders a phase indicator.
  """
  attr :phase, :string, required: true
  attr :age_range, :string, required: true

  def phase_indicator(assigns) do
    color_class = case @phase do
      "I" -> "bg-blue-100 text-blue-800 border-blue-300"
      "II" -> "bg-green-100 text-green-800 border-green-300"
      "III" -> "bg-yellow-100 text-yellow-800 border-yellow-300"
      "IV" -> "bg-orange-100 text-orange-800 border-orange-300"
      "V" -> "bg-red-100 text-red-800 border-red-300"
      _ -> "bg-gray-100 text-gray-800 border-gray-300"
    end

    ~H"""
    <div class={"inline-flex flex-col items-center px-4 py-2 border-2 rounded-lg #{color_class}"}>
      <span class="text-lg font-bold">Phase <%= @phase %></span>
      <span class="text-sm"><%= @age_range %></span>
    </div>
    """
  end
end