defmodule EpigeneticDriftFrontendWeb.Router do
  use EpigeneticDriftFrontendWeb, :router

  import Phoenix.LiveDashboard.Router

  pipeline :browser do
    plug :accepts, ["html"]
    plug :fetch_session
    plug :fetch_live_flash
    plug :put_root_layout, html: {EpigeneticDriftFrontendWeb.Layouts, :root}
    plug :protect_from_forgery
    plug :put_secure_browser_headers
  end

  pipeline :api do
    plug :accepts, ["json"]
  end

  scope "/", EpigeneticDriftFrontendWeb do
    pipe_through :browser

    live "/", DashboardLive, :index
    live "/counter/:id", DetailLive, :show
    live "/counter_registry", CounterRegistryLive, :index
    live "/sobol", SobolSensitivityLive, :index
    live "/lineage", HSCTrackingLive, :index
  end

  scope "/admin" do
    pipe_through :browser
    live_dashboard "/dashboard", metrics: EpigeneticDriftFrontendWeb.Telemetry
  end

  scope "/api", EpigeneticDriftFrontendWeb do
    pipe_through :api

    get "/health", HealthController, :index
  end

  if Mix.env() == :dev do
    scope "/dev" do
      pipe_through :browser

      forward "/mailbox", Plug.Swoosh.MailboxPreview
    end
  end
end