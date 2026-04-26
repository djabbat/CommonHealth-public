defmodule Mitorosweb.MixProject do
  use Mix.Project

  def project do
    [
      app: :mito_ros_web,
      version: "0.1.0",
      elixir: "~> 1.14",
      description: "MCOA Counter #3 (Mitochondrial ROS & mtDNA Damage Counter) — Phoenix LiveView dashboard",
      deps: deps()
    ]
  end

  def application do
    [
      mod: {Mitorosweb.Application, []},
      extra_applications: [:logger, :runtime_tools]
    ]
  end

  defp deps do
    [
      {:phoenix, "~> 1.7"},
      {:phoenix_live_view, "~> 0.20"},
      {:plug_cowboy, "~> 2.6"},
      {:jason, "~> 1.4"}
    ]
  end
end
