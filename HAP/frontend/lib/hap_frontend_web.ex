defmodule HAPFrontendWeb do
  def controller do
    quote do
      use Phoenix.Controller, namespace: HAPFrontendWeb
      import Plug.Conn
      import HAPFrontendWeb.Gettext
      alias HAPFrontendWeb.Router.Helpers, as: Routes
    end
  end

  def html do
    quote do
      use Phoenix.Component
      import Phoenix.Controller
      import Phoenix.LiveView.Helpers
      alias HAPFrontendWeb.Router.Helpers, as: Routes
      import HAPFrontendWeb.CoreComponents
      import HAPFrontendWeb.Layouts
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
      import HAPFrontendWeb.Gettext
    end
  end

  def live_view do
    quote do
      use Phoenix.LiveView,
        layout: {HAPFrontendWeb.Layouts, :app}

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
      import Phoenix.HTML
      import Phoenix.LiveView.Helpers
      import HAPFrontendWeb.CoreComponents
      import HAPFrontendWeb.Layouts
      alias HAPFrontendWeb.Router.Helpers, as: Routes
    end
  end

  @doc """
  When used, dispatch to the appropriate controller/view/etc.
  """
  defmacro __using__(which) when is_atom(which) do
    apply(__MODULE__, which, [])
  end
end