defmodule AimMemory.Application do
  @moduledoc false
  use Application

  @impl true
  def start(_type, _args) do
    children = [
      AimMemory.Repo
    ]

    Supervisor.start_link(children, strategy: :one_for_one, name: AimMemory.Supervisor)
  end
end
