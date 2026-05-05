defmodule AimWeb.HomeLive do
  use AimWeb, :live_view

  def mount(_params, _session, socket), do: {:ok, socket}

  def render(assigns) do
    ~H"""
    <h1><%= t("home.heading", @locale) %></h1>
    <p><%= t("home.tagline", @locale) %></p>
    """
  end
end
