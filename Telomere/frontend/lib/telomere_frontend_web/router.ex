defmodule TelomereFrontendWeb.Router do
  use TelomereFrontendWeb, :router

  pipeline :browser do
    plug :accepts, ["html"]
    plug :fetch_session
    plug :fetch_live_flash
    plug :put_root_layout, html: {TelomereFrontendWeb.Layouts, :root}
    plug :protect_from_forgery
    plug :put_secure_browser_headers
  end

  pipeline :api do
    plug :accepts, ["json"]
  end

  scope "/", TelomereFrontendWeb do
    pipe_through :browser

    live "/", DashboardLive, :index
    live "/parameters/:parameter_id", DetailLive, :parameter
    live "/counters/:counter_id", DetailLive, :counter
  end

  if Mix.env() in [:dev, :test] do
    import Phoenix.LiveDashboard.Router

    scope "/" do
      pipe_through :browser
      live_dashboard "/dashboard", metrics: TelomereFrontendWeb.Telemetry
    end
  end
end