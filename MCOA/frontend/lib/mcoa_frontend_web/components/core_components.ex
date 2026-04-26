defmodule MCOAFrontendWeb.CoreComponents do
  use Phoenix.Component

  attr :type, :string, default: "info"
  attr :class, :string, default: ""
  attr :rest, :global

  slot :inner_block, required: true

  def alert(assigns) do
    ~H"""
    <div
      class={[
        "rounded-lg p-4 mb-4",
        @type == "error" && "bg-red-50 text-red-800 border border-red-200",
        @type == "warning" && "bg-yellow-50 text-yellow-800 border border-yellow-200",
        @type == "info" && "bg-blue-50 text-blue-800 border border-blue-200",
        @type == "success" && "bg-green-50 text-green-800 border border-green-200",
        @class
      ]}
      {@rest}
    >
      <%= render_slot(@inner_block) %>
    </div>
    """
  end

  attr :title, :string, required: true
  slot :inner_block, required: true

  def card(assigns) do
    ~H"""
    <div class="bg-white shadow rounded-lg overflow-hidden">
      <div class="px-6 py-4 border-b border-gray-200">
        <h3 class="text-lg font-semibold text-gray-900"><%= @title %></h3>
      </div>
      <div class="px-6 py-4">
        <%= render_slot(@inner_block) %>
      </div>
    </div>
    """
  end

  attr :headers, :list, required: true
  slot :rows, required: true

  def table(assigns) do
    ~H"""
    <div class="overflow-x-auto">
      <table class="min-w-full divide-y divide-gray-200">
        <thead class="bg-gray-50">
          <tr>
            <th :for={header <- @headers} scope="col" class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
              <%= header %>
            </th>
          </tr>
        </thead>
        <tbody class="bg-white divide-y divide-gray-200">
          <%= for row <- @rows do %>
            <tr class="hover:bg-gray-50">
              <%= render_slot(row) %>
            </tr>
          <% end %>
        </tbody>
      </table>
    </div>
    """
  end

  attr :value, :float, required: true
  attr :max, :float, default: 1.0
  attr :label, :string, default: ""

  def progress_bar(assigns) do
    percentage = Float.round(@value / @max * 100, 1)

    ~H"""
    <div class="w-full">
      <div class="flex justify-between mb-1">
        <span class="text-sm font-medium text-gray-700"><%= @label %></span>
        <span class="text-sm font-medium text-gray-700"><%= percentage %>%</span>
      </div>
      <div class="w-full bg-gray-200 rounded-full h-2.5">
        <div
          class={[
            "h-2.5 rounded-full",
            percentage > 80 && "bg-red-600",
            percentage > 60 && percentage <= 80 && "bg-yellow-500",
            percentage <= 60 && "bg-green-600"
          ]}
          style={"width: #{percentage}%"}
        >
        </div>
      </div>
    </div>
    """
  end

  attr :id, :string, required: true
  attr :label, :string, required: true
  attr :type, :string, default: "text"
  attr :value, :any
  attr :field, :any
  attr :errors, :list
  attr :rest, :global

  def input(assigns) do
    ~H"""
    <div class="mb-4">
      <label for={@id} class="block text-sm font-medium text-gray-700 mb-1">
        <%= @label %>
      </label>
      <input
        type={@type}
        id={@id}
        name={@id}
        value={Phoenix.HTML.Form.normalize_value(@type, @value)}
        class={[
          "block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500 sm:text-sm",
          @errors && "border-red-300 focus:border-red-500 focus:ring-red-500"
        ]}
        {@rest}
      />
      <%= if @errors do %>
        <p class="mt-1 text-sm text-red-600">
          <%= Enum.join(@errors, ", ") %>
        </p>
      <% end %>
    </div>
    """
  end
end