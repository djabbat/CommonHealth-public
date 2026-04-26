defmodule Epigeneticweb.MixProject do
  use Mix.Project

  def project do
    [
      app: :epigenetic_web,
      version: "0.1.0",
      elixir: "~> 1.14",
      description: "MCOA Counter #4 (Epigenetic Drift Counter) — Phoenix LiveView dashboard",
      deps: deps()
    ]
  end

  def application do
    [
      mod: {Epigeneticweb.Application, []},
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
