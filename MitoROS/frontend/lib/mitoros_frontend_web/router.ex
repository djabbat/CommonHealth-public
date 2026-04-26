defmodule MitoROSFrontendWeb.Router do
  use MitoROSFrontendWeb, :router

  import Phoenix.LiveDashboard.Router

  pipeline :browser do
    plug :accepts, ["html"]
    plug :fetch_session
    plug :fetch_live_flash
    plug :put_root_layout, html: {MitoROSFrontendWeb.Layouts, :root}
    plug :protect_from_forgery
    plug :put_secure_browser_headers
  end

  pipeline :api do
    plug :accepts, ["json"]
  end

  scope "/", MitoROSFrontendWeb do
    pipe_through :browser

    live "/", DashboardLive, :index
    live "/detail/:entity_id", DetailLive, :show
  end

  if Mix.env() == :dev do
    scope "/dev" do
      pipe_through :browser
      live_dashboard "/dashboard", metrics: MitoROSFrontendWeb.Telemetry
    end
  end
end