defmodule EpigeneticDriftFrontendWeb do
  @moduledoc """
  The entrypoint for defining your web interface, such
  as controllers, views, channels, and so on.
  """

  def controller do
    quote do
      use Phoenix.Controller, namespace: EpigeneticDriftFrontendWeb

      import Plug.Conn
      import EpigeneticDriftFrontendWeb.Gettext
      alias EpigeneticDriftFrontendWeb.Router.Helpers, as: Routes
    end
  end

  def view do
    quote do
      use Phoenix.View,
        root: "lib/epigeneticdrift_frontend_web/templates",
        namespace: EpigeneticDriftFrontendWeb

      import Phoenix.Controller,
        only: [get_flash: 1, get_flash: 2, view_module: 1]

      import Phoenix.LiveView.Helpers
      import EpigeneticDriftFrontendWeb.ErrorHelpers
      import EpigeneticDriftFrontendWeb.Gettext
      import EpigeneticDriftFrontendWeb.CoreComponents
      import EpigeneticDriftFrontendWeb.Layouts

      alias EpigeneticDriftFrontendWeb.Router.Helpers, as: Routes
    end
  end

  def live_view do
    quote do
      use Phoenix.LiveView,
        layout: {EpigeneticDriftFrontendWeb.Layouts, :app}

      import EpigeneticDriftFrontendWeb.CoreComponents
      import EpigeneticDriftFrontendWeb.LiveHelpers

      on_mount {EpigeneticDriftFrontendWeb.LiveAuth, :ensure_mounted}

      @doc """
      Renders a component inside the `EpigeneticDriftFrontendWeb.CoreComponents` module.

      The `EpigeneticDriftFrontendWeb.CoreComponents` module is generated
      by running `mix phx.gen.core_components`.
      """
      attr :rest, :global, doc: "the arbitrary HTML attributes to add to the component"
      def core_component(assigns), do: EpigeneticDriftFrontendWeb.CoreComponents.core_component(assigns)
    end
  end

  def live_component do
    quote do
      use Phoenix.LiveComponent

      import EpigeneticDriftFrontendWeb.CoreComponents
      import EpigeneticDriftFrontendWeb.LiveHelpers
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
      import EpigeneticDriftFrontendWeb.Gettext
    end
  end

  @doc """
  When used, dispatch to the appropriate controller/view/etc.
  """
  defmacro __using__(which) when is_atom(which) do
    apply(__MODULE__, which, [])
  end
end