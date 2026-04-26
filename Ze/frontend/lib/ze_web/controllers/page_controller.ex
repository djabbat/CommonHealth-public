defmodule ZeWeb.PageController do
  use ZeWeb, :controller

  def home(conn, _params) do
    render(conn, :home)
  end
end
