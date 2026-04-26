defmodule ProteostasisFrontendWeb.DetailLive do
  use ProteostasisFrontendWeb, :live_view

  alias ProteostasisFrontendWeb.BackendClient
  alias ProteostasisFrontendWeb.CoreComponents

  @impl true
  def mount(_params, _session, socket) do
    socket =
      socket
      |> assign(:page_title, "Parameter Details")
      |> assign(:current_page, :dashboard)
      |> assign(:loading, true)
      |> assign(:error, nil)
      |> assign(:parameter, nil)
      |> assign(:form, nil)
      |> assign(:validation_errors, [])

    {:ok, socket}
  end

  @impl true
  def handle_params(%{"id" => id}, _uri, socket) do
    case BackendClient.fetch_parameter(id) do
      {:ok, parameter} ->
        form = to_form(parameter, as: "parameter")
        socket =
          socket
          |> assign(:parameter, parameter)
          |> assign(:form, form)
          |> assign(:loading, false)
          |> assign(:page_title, "Parameter: #{parameter.symbol}")

        {:noreply, socket}

      {:error, error} ->
        socket =
          socket
          |> assign(:error, "Failed to load parameter: #{inspect(error)}")
          |> assign(:loading, false)

        {:noreply,
         socket
         |> put_flash(:error, "Failed to load parameter")
         |> push_navigate(to: ~p"/")}
    end
  end

  @impl true
  def handle_event("validate", %{"parameter" => params}, socket) do
    changeset =
      socket.assigns.parameter
      |> Map.merge(params)
      |> changeset()

    form = to_form(changeset)

    {:noreply, assign(socket, form: form, validation_errors: changeset.errors)}
  end

  @impl true
  def handle_event("save", %{"parameter" => params}, socket) do
    case BackendClient.update_parameter(socket.assigns.parameter.id, params) do
      {:ok, updated} ->
        form = to_form(updated, as: "parameter")

        socket =
          socket
          |> assign(:parameter, updated)
          |> assign(:form, form)
          |> assign(:validation_errors, [])
          |> put_flash(:info, "Parameter updated successfully")