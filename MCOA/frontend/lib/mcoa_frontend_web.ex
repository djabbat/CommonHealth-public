defmodule MCOAFrontendWeb do
  def controller do
    quote do
      use Phoenix.Controller, namespace: MCOAFrontendWeb
      import Plug.Conn
      import MCOAFrontendWeb.Gettext
      alias MCOAFrontendWeb.Router.Helpers, as: Routes
    end
  end

  def view do
    quote do
      use Phoenix.View,
        root: "lib/mcoa_frontend_web/templates",
        namespace: MCOAFrontendWeb

      import Phoenix.Controller,
        only: [get_flash: 1, get_flash: 2, view_module: 1]

      import Phoenix.LiveView.Helpers
      import MCOAFrontendWeb.ErrorHelpers
      import MCOAFrontendWeb.Gettext
      import MCOAFrontendWeb.CoreComponents
      import MCOAFrontendWeb.Layouts

      alias MCOAFrontendWeb.Router.Helpers, as: Routes
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
      import MCOAFrontendWeb.Gettext
    end
  end

  def live_view do
    quote do
      use Phoenix.LiveView,
        layout: {MCOAFrontendWeb.Layouts, :app}

      on_mount MCOAFrontendWeb.LiveMountHooks

      import MCOAFrontendWeb.CoreComponents
      import MCOAFrontendWeb.Gettext
      alias MCOAFrontendWeb.Router.Helpers, as: Routes

      unquote(html_helpers())
    end
  end

  def live_component do
    quote do
      use Phoenix.LiveComponent

      unquote(html_helpers())
    end
  end

  defp html_helpers do
    quote do
      use Phoenix.HTML
      import Phoenix.LiveView.Helpers
      import MCOAFrontendWeb.CoreComponents
      import MCOAFrontendWeb.ErrorHelpers
      alias MCOAFrontendWeb.Router.Helpers, as: Routes
    end
  end

  @doc """
  When used, dispatch to the appropriate controller/view/etc.
  """
  defmacro __using__(which) when is_atom(which) do
    apply(__MODULE__, which, [])
  end
end