defmodule EpigeneticDriftFrontendWeb.CoreComponents do
  use Phoenix.Component

  alias EpigeneticDriftFrontendWeb.Components.McoaCounterCard
  alias EpigeneticDriftFrontendWeb.Components.ParameterTable
  alias EpigeneticDriftFrontendWeb.Components.SobolPlot
  alias EpigeneticDriftFrontendWeb.Components.LineageGraph

  embed_templates "core_components/*"

  attr :id, :string, required: true
  attr :title, :string, default: nil
  slot :inner_block, required: true
  slot :actions
  def card(assigns)

  attr :id, :string, required: true
  attr :label, :string, required: true
  attr :value, :any, required: true
  attr :unit, :string, default: nil
  attr :description, :string, default: nil
  attr :trend, :string, values: ["up", "down", "neutral"], default: "neutral"
  def metric_card(assigns)

  attr :id, :string, required: true
  attr :label, :string, required: true
  attr :value, :float, required: true
  attr :min, :float, required: true
  attr :max, :float, required: true
  attr :unit, :string, default: ""
  def progress_bar(assigns)

  attr :id, :string, required: true
  attr :type, :string, values: ["info", "success", "warning", "error"], default: "info"
  slot :inner_block, required: true
  def alert(assigns)

  attr :rest, :global
  slot :inner_block, required: true
  def container(assigns)

  attr :rest, :global
  def spinner(assigns)

  attr :for, :string, required: true
  attr :label, :string, required: true
  slot :inner_block, required: true
  attr :error, :list, default: []
  attr :required, :boolean, default: false
  def input(assigns)

  attr :click, :string, required: true
  attr :variant, :string, values: ["primary", "secondary", "danger", "ghost"], default: "primary"
  slot :inner_block, required: true
  attr :disabled, :boolean, default: false
  def button(assigns)

  attr :headers, :list, required: true
  attr :rows, :list, required: true
  slot :actions
  def table(assigns)

  def render("mcoa_counter_card.html", assigns), do: McoaCounterCard.render(assigns)
  def render("parameter_table.html", assigns), do: ParameterTable.render(assigns)
  def render("sobol_plot.html", assigns), do: SobolPlot.render(assigns)
  def render("lineage_graph.html", assigns), do: LineageGraph.render(assigns)
end