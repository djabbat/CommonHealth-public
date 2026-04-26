defmodule ProteostasisFrontendWeb.Router do
  use ProteostasisFrontendWeb, :router

  import Phoenix.LiveDashboard.Router

  pipeline :browser do
    plug :accepts, ["html"]
    plug :fetch_session
    plug :fetch_live_flash
    plug :put_root_layout, html: {ProteostasisFrontendWeb.Layouts, :root}
    plug :protect_from_forgery
    plug :put_secure_browser_headers
  end

  pipeline :api do
    plug :accepts, ["json"]
  end

  scope "/", ProteostasisFrontendWeb do
    pipe_through :browser

    live "/", DashboardLive, :index
    live "/detail/:id", DetailLive, :show

    if Mix.env() == :dev do
      live_dashboard "/dashboard",
        metrics: ProteostasisFrontendWeb.Telemetry,
        ecto_repos: []
    end
  end

  if Mix.env() == :dev do
    scope "/dev" do
      pipe_through :browser

      forward "/mailbox", Plug.Swoosh.MailboxPreview
    end
  end
end