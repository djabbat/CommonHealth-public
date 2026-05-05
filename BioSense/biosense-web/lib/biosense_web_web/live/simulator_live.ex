defmodule BiosenseWebWeb.SimulatorLive do
  use BiosenseWebWeb, :live_view

  alias BiosenseWeb.BackendClient

  @impl true
  def mount(_params, _session, socket) do
    backend_status =
      case BackendClient.healthz() do
        {:ok, %{"status" => "ok", "version" => v}} -> "ok v#{v}"
        _ -> "unreachable"
      end

    socket =
      socket
      |> assign(:page_title, "Simulator")
      |> assign(:v_eeg, 0.50)
      |> assign(:v_hrv, 0.45)
      |> assign(:v_resp, 0.45)
      |> assign(:v_sleep, 0.46)
      |> assign(:d, 0.30)
      |> assign(:age, 70.0)
      |> assign(:sex_male, false)
      |> assign(:chi_now, 0.55)
      |> assign(:chi_7d, 0.61)
      |> assign(:loading, false)
      |> assign(:error, nil)
      |> assign(:backend_status, backend_status)
      |> assign(:chi, nil)
      |> assign(:bridge, nil)
      |> assign(:risk, nil)
      |> assign(:bridge_curve, nil)
      |> recompute_async()

    {:ok, socket}
  end

  @impl true
  def handle_event("update", %{"_target" => [field]} = params, socket) do
    case Map.fetch(params, field) do
      {:ok, raw} -> handle_field(field, raw, socket)
      :error -> {:noreply, socket}
    end
  end

  def handle_event("update", _, socket), do: {:noreply, socket}

  defp handle_field("sex_male", val, socket) do
    sex = val == "true" or val == "on" or val == "1"
    {:noreply, socket |> assign(:sex_male, sex) |> recompute_async()}
  end

  defp handle_field(field, raw, socket) when is_binary(raw) do
    case Float.parse(raw) do
      {v, _} ->
        atom = String.to_existing_atom(field)
        {:noreply, socket |> assign(atom, v) |> recompute_async()}
      :error -> {:noreply, assign(socket, :error, "invalid number for #{field}")}
    end
  end

  @impl true
  def handle_info({:results, %{} = data}, socket) do
    {:noreply, socket
     |> assign(:loading, false)
     |> assign(:chi, data[:chi])
     |> assign(:bridge, data[:bridge])
     |> assign(:risk, data[:risk])
     |> assign(:bridge_curve, data[:bridge_curve])
     |> assign(:error, data[:error])}
  end

  def handle_info({:DOWN, _, :process, _, _}, socket), do: {:noreply, socket}

  defp recompute_async(socket) do
    me = self()
    a = socket.assigns
    Task.start(fn ->
      send(me, {:results, compute(a.v_eeg, a.v_hrv, a.v_resp, a.v_sleep, a.d,
                                  a.age, a.sex_male, a.chi_now, a.chi_7d)})
    end)
    assign(socket, :loading, true)
  end

  defp compute(v_eeg, v_hrv, v_resp, v_sleep, d, age, sex_male, chi_now, chi_7d) do
    chi = case BackendClient.chi_ze(v_eeg, v_hrv, v_resp, v_sleep) do
      {:ok, p} -> p
      {:error, e} -> %{error: e}
    end

    bridge = case BackendClient.bridge(d) do
      {:ok, p} -> p
      {:error, e} -> %{error: e}
    end

    sex = if sex_male, do: "M", else: "F"
    risk = case BackendClient.exacerbation(age, sex, chi_now, chi_7d) do
      {:ok, p} -> p
      {:error, e} -> %{error: e}
    end

    bridge_curve =
      0..40
      |> Enum.map(fn k -> k / 40.0 end)
      |> Enum.map(fn d ->
        case BackendClient.bridge(d) do
          {:ok, %{"chi_ze" => chi}} -> {d, chi}
          _ -> {d, 0.0}
        end
      end)

    %{chi: chi, bridge: bridge, risk: risk, bridge_curve: bridge_curve, error: nil}
  end

  defp backend_class("ok" <> _), do: "badge-ok"
  defp backend_class(_), do: "badge-warn"

  @impl true
  def render(assigns) do
    ~H"""
    <details class="essence" open>
      <summary><strong>ℹ BioSense · wearable χ_Ze biomarker</strong> — what this dashboard does (click to collapse)</summary>
      <div class="essence-body">
        <p>
          <strong>The dashboard below</strong> reads sample EEG / HRV / respiration / sleep
          velocities and computes the χ_Ze aging-activity biomarker continuously. Each modality's
          velocity slider feeds the variational principle
          <code>F = E − T·S − λ·I_pred</code>; the χ_Ze fixed point falls out at
          <code>v* = 0.45631</code>. Sensitivity range: <code>[0.32, 0.58]</code> for
          <code>k_λ ∈ [0.5, 2.0]</code>.
        </p>
        <p>
          <strong>Empirical validation (2026).</strong> A swept-v* search on All-of-Us N=500
          returned <code>v*_optimal = 0.451 (95% CI 0.443–0.459)</code> — consistent with the
          theoretical prediction. Confirmatory pre-registered N≥2000 trial pending EIC funding.
          See the parent theory at <a href="https://ze.longevity.ge">ze.longevity.ge</a>.
        </p>
        <p>
          <strong>Honest disclosure.</strong> The multimodal weights (0.30 · 0.30 · 0.20 · 0.20
          for EEG · HRV · respiration · sleep) are <em>post-hoc</em> pilot fits, not theory-fixed.
          They will be re-derived under the pre-registered protocol before the confirmatory trial.
        </p>
        <p>
          <strong>Privacy.</strong> Raw signals never leave the device; only the scalar χ_Ze is
          exported. The on-device estimator is open-source (Python/NumPy reference + mobile WebUI).
          For federated cohort studies see <a href="https://fclc.longevity.ge">FCLC</a>.
        </p>
        <p>
          <strong>How to use.</strong> Drag the modality velocity sliders to feed synthetic input;
          watch χ_Ze converge or diverge from v*. Real measurement requires connecting a wearable
          device to the BioSense backend.
        </p>
        <p>
          <strong>For:</strong> wearable-device engineers · sleep scientists ·
          exacerbation-prediction clinicians · AoU / UK Biobank reusers · participants in
          <a href="https://mcoa.longevity.ge">MCOA</a> counter-weight calibration.
        </p>
      </div>
    </details>

    <div class="card">
      <h2>Modality velocities (input)</h2>
      <p style="font-size:13px;color:#52525b;margin:0 0 8px 0;">
        Each slider sets the observed Ze velocity for one modality. Theoretical fixed point: <span class="formula">v* = 0.45631</span>
      </p>
      <form phx-change="update" phx-debounce="200">
        <div class="row">
          <div class="control">
            <label>v(EEG): <span class="val">{Float.round(@v_eeg, 4)}</span></label>
            <input type="range" name="v_eeg" min="-1" max="1" step="0.01" value={@v_eeg} />
          </div>
          <div class="control">
            <label>v(HRV): <span class="val">{Float.round(@v_hrv, 4)}</span></label>
            <input type="range" name="v_hrv" min="-1" max="1" step="0.01" value={@v_hrv} />
          </div>
          <div class="control">
            <label>v(resp): <span class="val">{Float.round(@v_resp, 4)}</span></label>
            <input type="range" name="v_resp" min="-1" max="1" step="0.01" value={@v_resp} />
          </div>
          <div class="control">
            <label>v(sleep): <span class="val">{Float.round(@v_sleep, 4)}</span></label>
            <input type="range" name="v_sleep" min="-1" max="1" step="0.01" value={@v_sleep} />
          </div>
        </div>

        <h2 style="margin-top: 18px;">Bridge / risk inputs</h2>
        <div class="row">
          <div class="control">
            <label>D (centriolar damage): <span class="val">{Float.round(@d, 3)}</span></label>
            <input type="range" name="d" min="0" max="1.0" step="0.01" value={@d} />
          </div>
          <div class="control">
            <label>age (years): <span class="val">{round(@age)}</span></label>
            <input type="range" name="age" min="20" max="100" step="1" value={@age} />
          </div>
          <div class="control">
            <label>χ_Ze now: <span class="val">{Float.round(@chi_now, 3)}</span></label>
            <input type="range" name="chi_now" min="0" max="1" step="0.01" value={@chi_now} />
          </div>
          <div class="control">
            <label>χ_Ze (7d ago): <span class="val">{Float.round(@chi_7d, 3)}</span></label>
            <input type="range" name="chi_7d" min="0" max="1" step="0.01" value={@chi_7d} />
          </div>
        </div>

        <div style="margin-top: 8px;">
          <span class={"badge " <> backend_class(@backend_status)}>backend: {@backend_status}</span>
          <%= if @loading do %>
            <span class="badge">computing…</span>
          <% end %>
          <%= if @error do %>
            <span class="err">{@error}</span>
          <% end %>
        </div>
      </form>
    </div>

    <div class="card">
      <h2>χ_Ze composite</h2>
      <%= if is_map(@chi) and Map.has_key?(@chi, "composite") do %>
        <div class="row-stats" style="margin-top: 8px;">
          <div class="stat"><div class="k">composite</div><div class="v">{Float.round(@chi["composite"], 4)}</div></div>
          <div class="stat"><div class="k">EEG</div><div class="v">{Float.round(@chi["per_modality"]["eeg"], 3)}</div></div>
          <div class="stat"><div class="k">HRV</div><div class="v">{Float.round(@chi["per_modality"]["hrv"], 3)}</div></div>
          <div class="stat"><div class="k">resp</div><div class="v">{Float.round(@chi["per_modality"]["resp"], 3)}</div></div>
          <div class="stat"><div class="k">sleep</div><div class="v">{Float.round(@chi["per_modality"]["sleep"], 3)}</div></div>
          <div class="stat"><div class="k">v*</div><div class="v">{Float.round(@chi["v_star"], 5)}</div></div>
        </div>
      <% end %>
    </div>

    <div class="row">
      <div class="card">
        <h2>CDATA bridge</h2>
        <p class="formula">A(D) = a + b·D + c·D² ; χ_Ze = g₀ − g₁·A</p>
        <%= if is_map(@bridge) and Map.has_key?(@bridge, "a") do %>
          <div class="row-stats" style="margin-top: 8px;">
            <div class="stat"><div class="k">A(D)</div><div class="v">{Float.round(@bridge["a"], 4)}</div></div>
            <div class="stat"><div class="k">χ_Ze(D)</div><div class="v">{Float.round(@bridge["chi_ze"], 4)}</div></div>
          </div>
        <% end %>
        {bridge_plot(@bridge_curve)}
      </div>
      <div class="card">
        <h2>30-day exacerbation risk</h2>
        <p class="formula">P = σ(β₀ + β_age·age + β_sex·sex + β_chi·χ_now + β_dchi·Δχ)</p>
        <%= if is_map(@risk) and Map.has_key?(@risk, "risk_30d") do %>
          <div class="row-stats" style="margin-top: 8px;">
            <div class="stat"><div class="k">P_30d</div><div class="v">{Float.round(@risk["risk_30d"], 4)}</div></div>
            <div class="stat"><div class="k">logit</div><div class="v">{Float.round(@risk["logit"], 3)}</div></div>
          </div>
        <% end %>
        <p style="font-size: 12px; color: #71717a; margin-top: 12px;">
          ⚠ Research-grade output. Not medical advice. Apply your own clinical decision rule.
        </p>
      </div>
    </div>
    """
  end

  defp bridge_plot(nil) do
    assigns = %{}
    ~H[<p class="err">no data</p>]
  end

  defp bridge_plot([]) do
    assigns = %{}
    ~H[<p class="err">no data</p>]
  end

  defp bridge_plot(points) do
    {w, h} = {500, 200}
    pl = 40; pr = 16; pt = 16; pb = 30
    inner_w = w - pl - pr
    inner_h = h - pt - pb

    pts =
      points
      |> Enum.map(fn {d, chi} ->
        x = pl + d * inner_w
        y = pt + (1.0 - chi) * inner_h
        "#{Float.round(x, 1)},#{Float.round(y, 1)}"
      end)
      |> Enum.join(" ")

    assigns = %{w: w, h: h, pl: pl, pr: pr, pt: pt, pb: pb, pts: pts}

    ~H"""
    <svg class="plot" viewBox={"0 0 #{@w} #{@h}"} preserveAspectRatio="xMidYMid meet">
      <line class="axis" x1={@pl} y1={@pt} x2={@pl} y2={@h - @pb} />
      <line class="axis" x1={@pl} y1={@h - @pb} x2={@w - @pr} y2={@h - @pb} />
      <text x={@pl - 4} y={@pt + 4} text-anchor="end">1</text>
      <text x={@pl - 4} y={@h - @pb} text-anchor="end">0</text>
      <text x={@pl} y={@h - 8}>D=0</text>
      <text x={@w - @pr} y={@h - 8} text-anchor="end">D=1</text>
      <polyline class="curve" points={@pts} fill="none" />
    </svg>
    """
  end
end
