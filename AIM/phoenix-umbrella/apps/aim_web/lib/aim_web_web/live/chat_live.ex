defmodule AimWeb.ChatLive do
  @moduledoc """
  Чат с persistence в aim_memory: каждое сообщение и ответ — отдельная запись
  в `messages` таблице, привязанная к session_id.
  """
  use AimWeb, :live_view

  def mount(_params, _session, socket) do
    {:ok, sess} = AimMemory.start_session(nil, Atom.to_string(socket.assigns.locale))
    {:ok,
     assign(socket,
       messages: [],
       input: "",
       busy?: false,
       error: nil,
       session_id: sess.id
     )}
  end

  def handle_event("update", %{"input" => v}, socket) do
    {:noreply, assign(socket, input: v)}
  end

  def handle_event("send", %{"input" => content}, socket) do
    content = String.trim(content)
    if content == "" do
      {:noreply, socket}
    else
      AimMemory.append_message(socket.assigns.session_id, "user", content)
      msgs = socket.assigns.messages ++ [%{role: "user", content: content}]

      socket =
        socket
        |> assign(messages: msgs, input: "", busy?: true, error: nil)

      send(self(), {:run_llm, msgs})
      {:noreply, socket}
    end
  end

  def handle_info({:run_llm, msgs}, socket) do
    payload = Enum.map(msgs, fn %{role: r, content: c} -> %{role: r, content: c} end)

    case AimOrchestrator.chat(payload) do
      {:ok, %{"reply" => reply} = body} ->
        AimMemory.append_message(
          socket.assigns.session_id, "assistant", reply,
          model: body["model"] || "",
          provider: to_string(body["provider"] || "")
        )
        msgs = socket.assigns.messages ++ [%{role: "assistant", content: reply}]
        {:noreply, assign(socket, messages: msgs, busy?: false)}

      {:error, reason} ->
        {:noreply, assign(socket, busy?: false, error: inspect(reason))}
    end
  end

  def render(assigns) do
    ~H"""
    <h2><%= t("chat.heading", @locale) %></h2>
    <p>session: <code><%= @session_id %></code></p>

    <ul class="messages">
      <li :for={m <- @messages} class={"msg msg-" <> m.role}>
        <strong><%= m.role %>:</strong>
        <pre><%= m.content %></pre>
      </li>
    </ul>

    <%= if @error do %>
      <div class="error">⚠ <%= @error %></div>
    <% end %>

    <form phx-submit="send" phx-change="update">
      <textarea name="input" rows="3" disabled={@busy?}><%= @input %></textarea>
      <button type="submit" disabled={@busy?}>
        <%= if @busy?, do: "…", else: "→" %>
      </button>
    </form>
    """
  end
end
