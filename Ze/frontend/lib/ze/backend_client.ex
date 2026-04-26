defmodule Ze.BackendClient do
  @moduledoc "Клиент к Rust backend ze_backend (127.0.0.1:4001 по умолчанию)."

  @default_base "http://127.0.0.1:4001"

  def base_url, do: System.get_env("ZE_BACKEND_URL") || @default_base

  def impedance(scenario, horizon) do
    get("/api/impedance", scenario: scenario, horizon: horizon)
  end

  def chsh(h, alpha, delta) do
    get("/api/chsh", h: h, alpha: alpha, delta: delta)
  end

  def autowaves(n, steps, snapshot_every) do
    get("/api/autowaves", n: n, steps: steps, snapshot_every: snapshot_every)
  end

  defp get(path, params) do
    url = base_url() <> path

    case Req.get(url,
           params: params,
           connect_options: [timeout: 2_000],
           retry: false,
           receive_timeout: 20_000
         ) do
      {:ok, %Req.Response{status: 200, body: body}} -> {:ok, body}
      {:ok, %Req.Response{status: s, body: b}} -> {:error, {:http, s, b}}
      {:error, reason} -> {:error, reason}
    end
  end
end
