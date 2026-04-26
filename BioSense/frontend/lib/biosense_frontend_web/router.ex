defmodule BioSenseFrontendWeb.Router do
  use BioSenseFrontendWeb, :router

  pipeline :browser do
    plug :accepts, ["html"]
    plug :fetch_session
    plug :fetch_live_flash
    plug :put_root_layout, html: {BioSenseFrontendWeb.Layouts, :root}
    plug :protect_from_forgery
    plug :put_secure_browser_headers
  end

  pipeline :api do
    plug :accepts, ["json"]
  end

  scope "/", BioSenseFrontendWeb do
    pipe_through :browser

    live "/", DashboardLive, :index
    live "/datasets/:id", DetailLive, :dataset
    live "/parameters/:id", DetailLive, :parameter
    live "/knowledge/:id", DetailLive, :knowledge
    live "/counters", DetailLive, :counters
    live "/sensitivity", DetailLive, :sensitivity
  end

  if Mix.env() == :dev do
    scope "/dev" do
      pipe_through :browser
      forward "/mailbox", Plug.Swoosh.MailboxPreview
    end
  end
end