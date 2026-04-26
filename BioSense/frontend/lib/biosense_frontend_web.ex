defmodule BioSenseFrontendWeb do
  def controller do
    quote do
      use Phoenix.Controller, namespace: BioSenseFrontendWeb
      import Plug.Conn
      import BioSenseFrontendWeb.Gettext
      alias BioSenseFrontendWeb.Router.Helpers, as: Routes
    end
  end

  def live_view do
    quote do
      use Phoenix.LiveView,
        layout: {BioSenseFrontendWeb.Layouts, :app}

      unquote(html_helpers())
    end
  end

  def live_component do
    quote do
      use Phoenix.LiveComponent

      unquote(html_helpers())
    end
  end

  def html do
    quote do
      use Phoenix.Component

      import BioSenseFrontendWeb.CoreComponents
      alias BioSenseFrontendWeb.Router.Helpers, as: Routes
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
      import BioSenseFrontendWeb.Gettext
    end
  end

  defp html_helpers do
    quote do
      use Phoenix.HTML
      import BioSenseFrontendWeb.CoreComponents
      import BioSenseFrontendWeb.Gettext
      alias BioSenseFrontendWeb.Router.Helpers, as: Routes
    end
  end

  @doc """
  When used, dispatch to the appropriate controller/view/etc.
  """
  defmacro __using__(which) when is_atom(which) do
    apply(__MODULE__, which, [])
  end
end