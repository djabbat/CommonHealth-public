defmodule OntogenesisFrontendWeb.Router do
  use OntogenesisFrontendWeb, :router

  import Phoenix.LiveDashboard.Router

  pipeline :browser do
    plug :accepts, ["html"]
    plug :fetch_session
    plug :fetch_live_flash
    plug :put_root_layout, html: {OntogenesisFrontendWeb.Layouts, :root}
    plug :protect_from_forgery
    plug :put_secure_browser_headers
  end

  pipeline :api do
    plug :accepts, ["json"]
  end

  scope "/", OntogenesisFrontendWeb do
    pipe_through :browser

    live "/", DashboardLive, :index
    live "/detail/:id", DetailLive, :show

    live_dashboard "/dashboard", metrics: OntogenesisFrontendWeb.Telemetry, ecto_repos: []
  end

  if Mix.env() == :dev do
    scope "/dev" do
      pipe_through :browser

      forward "/mailbox", Plug.Swoosh.MailboxPreview
    end
  end
end