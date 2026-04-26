defmodule MCOAFrontendWeb.Router do
  use MCOAFrontendWeb, :router

  import Phoenix.LiveDashboard.Router

  pipeline :browser do
    plug :accepts, ["html"]
    plug :fetch_session
    plug :fetch_live_flash
    plug :put_root_layout, html: {MCOAFrontendWeb.Layouts, :root}
    plug :protect_from_forgery
    plug :put_secure_browser_headers
  end

  pipeline :api do
    plug :accepts, ["json"]
  end

  scope "/", MCOAFrontendWeb do
    pipe_through :browser

    live "/", DashboardLive, :index
    live "/counter/:counter_id", DetailLive, :show
    live "/counter-registry", DashboardLive, :registry
  end

  if Mix.env() == :dev do
    scope "/dev" do
      pipe_through :browser

      live_dashboard "/dashboard",
        metrics: MCOAFrontendWeb.Telemetry,
        ecto_repos: []
    end
  end
end