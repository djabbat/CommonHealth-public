defmodule EpigeneticDriftFrontend.MixProject do
  use Mix.Project

  def project do
    [
      app: :epigeneticdrift_frontend,
      version: "0.1.0",
      elixir: "~> 1.16",
      elixirc_paths: elixirc_paths(Mix.env()),
      start_permanent: Mix.env() == :prod,
      aliases: aliases(),
      deps: deps(),
      dialyzer: [plt_add_apps: [:mix]]
    ]
  end

  def application do
    [
      mod: {EpigeneticDriftFrontend.Application, []},
      extra_applications: [:logger, :runtime_tools, :os_mon]
    ]
  end

  defp elixirc_paths(:test), do: ["lib", "test/support"]
  defp elixirc_paths(_), do: ["lib"]

  defp deps do
    [
      {:phoenix, "~> 1.7.7"},
      {:phoenix_live_view, "~> 0.19.0"},
      {:phoenix_html, "~> 3.3"},
      {:phoenix_live_reload, "~> 1.2", only: :dev},
      {:phoenix_live_dashboard, "~> 0.8.0"},
      {:telemetry_metrics, "~> 0.6"},
      {:telemetry_poller, "~> 1.0"},
      {:jason, "~> 1.4"},
      {:dns_cluster, "~> 0.1.1"},
      {:plug_cowboy, "~> 2.5"},
      {:req, "~> 0.3.0"},
      {:nimble_parsec, "~> 1.0"},
      {:decimal, "~> 2.0"},
      {:phoenix_ecto, "~> 4.4"},
      {:remote_ip, "~> 1.0"},
      {:sentry, "~> 9.0"},
      {:credo, "~> 1.7", only: [:dev, :test], runtime: false},
      {:dialyxir, "~> 1.3", only: [:dev], runtime: false}
    ]
  end

  defp aliases do
    [
      setup: ["deps.get"],
      "assets.deploy": ["cmd npm run deploy --prefix assets"],
      test: ["test --no-start"]
    ]
  end
end