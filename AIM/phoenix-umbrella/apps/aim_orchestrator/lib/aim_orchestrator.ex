defmodule AimOrchestrator do
  @moduledoc """
  Coordinates calls across Rust services (aim-llm, aim-rag, aim-medkb,
  aim-doctor, diffdx-api, ssa-api). Replaces agents/orchestrator.py +
  agents/ensemble.py + agents/debate.py + agents/reflexion.py.

  All TODO — this is the skeleton.
  """

  alias AimOrchestrator.Upstream

  @doc "Single LLM call via aim-llm."
  def chat(messages, opts \\ []) do
    Upstream.post("llm_url", "/v1/chat", %{
      messages: messages,
      model_hint: Keyword.get(opts, :model_hint)
    })
  end

  @doc "Run the doctor pipeline (intake → diagnose → plan)."
  def diagnose(case_id) do
    Upstream.post("doctor_url", "/v1/diagnose", %{case_id: case_id})
  end
end
