defmodule HAPFrontendWeb.Router do
  use HAPFrontendWeb, :router

  pipeline :browser do
    plug :accepts, ["html"]
    plug :fetch_session
    plug :fetch_live_flash
    plug :put_root_layout, html: {HAPFrontendWeb.Layouts, :root}
    plug :protect_from_forgery
    plug :put_secure_browser_headers
  end

  pipeline :api do
    plug :accepts, ["json"]
  end

  scope "/", HAPFrontendWeb do
    pipe_through :browser

    live "/", DashboardLive, :index
    live "/concept", DetailLive, :concept
    live "/parameters", DetailLive, :parameters
    live "/knowledge", DetailLive, :knowledge
  end

  if Mix.env() == :dev do
    scope "/dev" do
      pipe_through :browser

      forward "/mailbox", Plug.Swoosh.MailboxPreview
    end
  end
end