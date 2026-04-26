defmodule HAPFrontendWeb.CoreComponents do
  use Phoenix.Component

  @doc """
  Renders a modal.

  ## Examples

      <.modal id="confirm-modal">
        This is a modal.
        <:confirm>OK</:confirm>
        <:cancel>Cancel</:cancel>
      </.modal>
  """
  attr :id, :string, required: true
  attr :show, :boolean, default: false
  slot :inner_block, required: true
  slot :confirm
  slot :cancel

  def modal(assigns) do
    ~H"""
    <div
      id={@id}
      phx-remove={hide_modal(@id)}
      class="phx-modal fade-in"
      phx-mounted={show_modal(@id)}
      phx-click-away={JS.hide(to: "##{@id}")}
    >
      <div class="phx-modal-content" phx-click="close-modal" phx-window-keydown={JS.hide(to: "##{@id}")} phx-key="escape">
        <%= render_slot(@inner_block) %>
        <div class="mt-6 flex justify-end gap-3">
          <%= for confirm <- @confirm do %>
            <button class="px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700">
              <%= render_slot(confirm) %>
            </button>
          <% end %>
          <%= for cancel <- @cancel do %>
            <button class="px-4 py-2 bg-gray-200 text-gray-800 rounded-lg hover:bg-gray-300">
              <%= render_slot(cancel) %>
            </button>
          <% end %>
        </div>
      </div>
    </div>
    """
  end

  @doc """
  Renders a table.
  """
  attr :rows, :list, required: true
  attr :columns, :list, required: true

  def table(assigns) do
    ~H"""
    <div class="overflow-x-auto rounded-lg border border-gray-200">
      <table class="min-w-full divide-y divide-gray-200">
        <thead class="bg-gray-50">
          <tr>
            <%= for column <- @columns do %>
              <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                <%= column[:label] %>
              </th>
            <% end %>
          </tr>
        </thead>
        <tbody class="bg-white divide-y divide-gray-200">
          <%= for row <- @rows do %>
            <tr class="hover:bg-gray-50">
              <%= for column <- @columns do %>
                <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-900">
                  <%= render_column(row, column) %>
                </td>
              <% end %>
            </tr>
          <% end %>
        </tbody>
      </table>
    </div>
    """
  end

  defp render_column(row, column) do
    case column[:render] do
      nil -> Map.get(row, column[:key])
      fun when is_function(fun) -> fun.(row)
    end
  end

  @doc """
  Renders a badge with status color.
  """
  attr :status, :string, required: true
  slot :inner_block, required: true

  def status_badge(assigns) do
    colors = %{
      "Measured" => "bg-green-100 text-green-800",
      "Estimated" => "bg-yellow-100 text-yellow-800",
      "TBD" => "bg-gray-100 text-gray-800",
      "Canonical" => "bg-purple-100 text-purple-800"
    }

    color_class = colors[@status] || "bg-gray-100 text-gray-800"

    ~H"""
    <span class={"inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium #{color_class}"}>
      <%= render_slot(@inner_block) %>
    </span>
    """
  end

  @doc """
  Renders a loading spinner.
  """
  def spinner(assigns) do
    ~H"""
    <div class="flex justify-center items-center p-8">
      <div class="animate-spin rounded-full h-12 w-12 border-b-2 border-blue-600"></div>
    </div>
    """
  end

  @doc """
  Renders an error alert.
  """
  attr :message, :string, required: true

  def error_alert(assigns) do
    ~H"""
    <div class="rounded-md bg-red-50 p-4">
      <div class="flex">
        <div class="flex-shrink-0">
          <svg class="h-5 w-5 text-red-400" viewBox="0 0 20 20" fill="currentColor">
            <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zM8.707 7.293a1 1 0 00-1.414 1.414L8.586 10l-1.293 1.293a1 1 0 101.414 1.414L10 11.414l1.293 1.293a1 1 0 001.414-1.414L11.414 10l1.293-1.293a1 1 0 00-1.414-1.414L10 8.586 8.707 7.293z" clip-rule="evenodd"/>
          </svg>
        </div>
        <div class="ml-3">
          <h3 class="text-sm font-medium text-red-800">Error</h3>
          <div class="mt-2 text-sm text-red-700">
            <p><%= @message %></p>
          </div>
        </div>
      </div>
    </div>
    """
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
  end
end