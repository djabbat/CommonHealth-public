defmodule TelomereFrontendWeb do
  @moduledoc """
  The entrypoint for defining your web interface.
  """

  def controller do
    quote do
      use Phoenix.Controller, namespace: TelomereFrontendWeb
      import Plug.Conn
      import TelomereFrontendWeb.Gettext
      alias TelomereFrontendWeb.Router.Helpers, as: Routes
    end
  end

  def view do
    quote do
      use Phoenix.View,
        root: "lib/telomere_frontend_web/templates",
        namespace: TelomereFrontendWeb

      import Phoenix.Controller,
        only: [get_flash: 1, get_flash: 2, view_module: 1, view_template: 1]

      unquote(view_helpers())
    end
  end

  def live_view do
    quote do
      use Phoenix.LiveView,
        layout: {TelomereFrontendWeb.Layouts, :app}

      unquote(view_helpers())
    end
  end

  def live_component do
    quote do
      use Phoenix.LiveComponent

      unquote(view_helpers())
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
      import TelomereFrontendWeb.Gettext
    end
  end

  defp view_helpers do
    quote do
      use Phoenix.HTML

      import Phoenix.LiveView.Helpers
      import Phoenix.View

      import TelomereFrontendWeb.ErrorHelpers
      import TelomereFrontendWeb.Gettext
      import TelomereFrontendWeb.CoreComponents
      alias TelomereFrontendWeb.Router.Helpers, as: Routes
    end
  end

  @doc """
  When used, dispatch to the appropriate controller/view/etc.
  """
  defmacro __using__(which) when is_atom(which) do
    apply(__MODULE__, which, [])
  end
end