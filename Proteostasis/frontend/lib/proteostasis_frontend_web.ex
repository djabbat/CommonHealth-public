defmodule ProteostasisFrontendWeb do
  @moduledoc """
  The entrypoint for defining your web interface, such
  as controllers, views, live views and components.
  """

  def controller do
    quote do
      use Phoenix.Controller, namespace: ProteostasisFrontendWeb

      import Plug.Conn
      import ProteostasisFrontendWeb.Gettext
      alias ProteostasisFrontendWeb.Router.Helpers, as: Routes
    end
  end

  def view do
    quote do
      use Phoenix.View,
        root: "lib/proteostasis_frontend_web/templates",
        namespace: ProteostasisFrontendWeb

      import Phoenix.Controller,
        only: [get_flash: 1, get_flash: 2, view_module: 1, view_template: 1]

      import ProteostasisFrontendWeb.ErrorHelpers
      import ProteostasisFrontendWeb.Gettext
      import ProteostasisFrontendWeb.CoreComponents
      import ProteostasisFrontendWeb.Layouts

      alias ProteostasisFrontendWeb.Router.Helpers, as: Routes
    end
  end

  def live_view do
    quote do
      use Phoenix.LiveView,
        layout: {ProteostasisFrontendWeb.Layouts, :app}

      import ProteostasisFrontendWeb.CoreComponents
      import ProteostasisFrontendWeb.Gettext

      alias ProteostasisFrontendWeb.Router.Helpers, as: Routes

      on_mount ProteostasisFrontendWeb.LiveAuth
    end
  end

  def live_component do
    quote do
      use Phoenix.LiveComponent

      import ProteostasisFrontendWeb.CoreComponents
      import ProteostasisFrontendWeb.Gettext

      alias ProteostasisFrontendWeb.Router.Helpers, as: Routes
    end
  end

  def component do
    quote do
      use Phoenix.Component

      import ProteostasisFrontendWeb.CoreComponents
      import ProteostasisFrontendWeb.Gettext

      alias ProteostasisFrontendWeb.Router.Helpers, as: Routes
    end
  end

  def router do
    quote do
      use Phoenix.Router

      import Plug.Conn
      import Phoenix.Controller
      import Phoenix.LiveView.Router
    end
  end

  def channel do
    quote do
      use Phoenix.Channel
      import ProteostasisFrontendWeb.Gettext
    end
  end

  @doc """
  When used, dispatch to the appropriate controller/view/etc.
  """
  defmacro __using__(which) when is_atom(which) do
    apply(__MODULE__, which, [])
  end
end

defmodule ProteostasisFrontendWeb.LiveAuth do
  import Phoenix.LiveView

  def on_mount(:default, _params, _session, socket) do
    {:cont, socket}
  end
end