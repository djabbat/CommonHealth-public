defmodule McoaWebWeb.DashboardLive do
  @moduledoc """
  MCOA counter dashboard. Selects a tissue, runs a simulation via the mcoa-api backend, and shows
  per-counter trajectories with a live MCOA-vs-CDATA residual panel (per project comparison rule).
  """
  use McoaWebWeb, :live_view

  @tissues ["fibroblast", "hsc", "neuron", "hepatocyte", "beta_cell", "cd8_t_memory"]

  @impl true
  def mount(_params, _session, socket) do
    {:ok,
     socket
     |> assign(:tissues, @tissues)
     |> assign(:tissue, "hsc")
     |> assign(:divisions, 100)
     |> assign(:records, [])
     |> assign(:residual_label, "waiting")}
  end

  @impl true
  def handle_event("run", %{"tissue" => tissue, "divisions" => n}, socket) do
    n_int = String.to_integer(n)

    case mcoa_simulate(tissue, n_int) do
      {:ok, records} ->
        {:noreply,
         socket
         |> assign(:records, records)
         |> assign(:tissue, tissue)
         |> assign(:divisions, n_int)
         |> assign(:residual_label, "MCOA run complete — pair with CDATA for Δ")}

      {:error, reason} ->
        {:noreply, put_flash(socket, :error, "Simulation failed: #{reason}")}
    end
  end

  @impl true
  def render(assigns) do
    ~H"""
    <div class="p-6 max-w-5xl mx-auto">
      <h1 class="text-2xl font-bold mb-4">MCOA — Multi-Counter Dashboard</h1>

      <form phx-submit="run" class="flex gap-4 mb-6">
        <label>
          Tissue
          <select name="tissue" class="ml-2 border rounded p-1">
            <%= for t <- @tissues do %>
              <option value={t} selected={t == @tissue}><%= t %></option>
            <% end %>
          </select>
        </label>
        <label>
          Divisions
          <input type="number" name="divisions" value={@divisions} min="1" max="500" class="ml-2 border rounded p-1 w-24" />
        </label>
        <button type="submit" class="px-4 py-1 bg-blue-600 text-white rounded">Run MCOA</button>
      </form>

      <div class="text-sm text-gray-600 mb-2">Status: <%= @residual_label %></div>

      <%= if length(@records) > 0 do %>
        <table class="w-full text-sm border">
          <thead><tr><th>step</th><th>n</th><th>telomere</th><th>cent</th><th>mito</th><th>epigen</th><th>proteo</th><th>L_tissue</th></tr></thead>
          <tbody>
            <%= for r <- Enum.take_every(@records, max(div(length(@records), 20), 1)) do %>
              <tr>
                <td><%= r["step"] %></td>
                <td><%= r["n_cumulative"] %></td>
                <td><%= Float.round(r["telomere"] || 0.0, 3) %></td>
                <td><%= Float.round(r["centriolar"] || 0.0, 3) %></td>
                <td><%= Float.round(r["mito"] || 0.0, 3) %></td>
                <td><%= Float.round(r["epigenetic"] || 0.0, 3) %></td>
                <td><%= Float.round(r["proteostasis"] || 0.0, 3) %></td>
                <td><%= Float.round(r["tissue_load"] || 0.0, 3) %></td>
              </tr>
            <% end %>
          </tbody>
        </table>
      <% end %>

      <p class="mt-6 text-xs text-gray-500">
        Every MCOA run should be paired with a CDATA run via
        <code>scripts/compare_mcoa_cdata.py</code> (mandatory rule).
      </p>
    </div>
    """
  end

  defp mcoa_simulate(tissue, divisions) do
    body = Jason.encode!(%{tissue: tissue, divisions: divisions})

    case Finch.build(:post, "http://127.0.0.1:3030/api/simulate",
           [{"content-type", "application/json"}],
           body
         )
         |> Finch.request(McoaWeb.Finch) do
      {:ok, %Finch.Response{status: 200, body: resp_body}} ->
        case Jason.decode(resp_body) do
          {:ok, %{"records" => records}} -> {:ok, records}
          other -> {:error, inspect(other)}
        end

      {:ok, %Finch.Response{status: s, body: b}} ->
        {:error, "HTTP #{s}: #{b}"}

      {:error, e} ->
        {:error, inspect(e)}
    end
  end
end
