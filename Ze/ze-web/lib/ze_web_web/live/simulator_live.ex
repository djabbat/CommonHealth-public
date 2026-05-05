defmodule ZeWebWeb.SimulatorLive do
  use ZeWebWeb, :live_view

  alias ZeWeb.BackendClient

  @debounce_ms 200

  @impl true
  def mount(_params, _session, socket) do
    backend_status =
      case BackendClient.healthz() do
        {:ok, %{"status" => "ok", "version" => v}} -> "ok v#{v}"
        {:ok, _} -> "ok"
        {:error, _} -> "unreachable"
      end

    socket =
      socket
      |> assign(:page_title, "Simulator")
      |> assign(:delta, 0.10)
      |> assign(:beta, 1.0)
      |> assign(:i, 0.5)
      |> assign(:c0, 1.0)
      |> assign(:tau_max, 5.0)
      |> assign(:loading, false)
      |> assign(:error, nil)
      |> assign(:backend_status, backend_status)
      |> assign(:chsh, nil)
      |> assign(:decay_curve, nil)
      |> assign(:qfi_sweep, nil)
      |> recompute_async()

    {:ok, socket}
  end

  @impl true
  def handle_event("update", %{"_target" => [field]} = params, socket) do
    raw = Map.get(params, field)

    case parse_float(raw) do
      {:ok, value} ->
        socket =
          socket
          |> assign(String.to_existing_atom(field), value)
          |> recompute_async()

        {:noreply, socket}

      :error ->
        {:noreply, assign(socket, :error, "invalid number for #{field}")}
    end
  end

  def handle_event("update", _params, socket), do: {:noreply, socket}

  @impl true
  def handle_info({:results, %{} = data}, socket) do
    {:noreply,
     socket
     |> assign(:loading, false)
     |> assign(:chsh, data[:chsh])
     |> assign(:decay_curve, data[:decay])
     |> assign(:qfi_sweep, data[:qfi])
     |> assign(:error, data[:error])}
  end

  def handle_info({:DOWN, _ref, :process, _pid, _reason}, socket), do: {:noreply, socket}

  defp recompute_async(socket) do
    me = self()
    delta = socket.assigns.delta
    beta = socket.assigns.beta
    i = socket.assigns.i
    c0 = socket.assigns.c0
    tau_max = socket.assigns.tau_max

    Task.start(fn ->
      send(me, {:results, compute(delta, beta, i, c0, tau_max)})
    end)

    assign(socket, :loading, true)
  end

  defp compute(delta, beta, i, c0, tau_max) do
    chsh =
      case BackendClient.chsh(delta, n: 64) do
        {:ok, payload} -> payload
        {:error, e} -> %{error: e}
      end

    tau_grid =
      0..40
      |> Enum.map(fn k -> tau_max * k / 40 end)
      |> Enum.filter(fn t -> beta * i * t < 1.0 end)

    decay =
      case BackendClient.correlation(c0, beta, i, tau_grid) do
        {:ok, %{"c" => c_values}} -> Enum.zip(tau_grid, c_values)
        {:error, _} -> []
      end

    i_grid = log_grid(0.01, 1.0, 25)

    qfi =
      case BackendClient.qfi_sweep(c0, beta, i_grid) do
        {:ok, %{"f_q" => fq, "dtau_dt_abs" => dtau}} -> Enum.zip(dtau, fq)
        {:error, _} -> []
      end

    %{chsh: chsh, decay: decay, qfi: qfi, error: nil}
  end

  defp log_grid(lo, hi, n) do
    log_lo = :math.log(lo)
    log_hi = :math.log(hi)

    Enum.map(0..(n - 1), fn k ->
      :math.exp(log_lo + (log_hi - log_lo) * k / (n - 1))
    end)
  end

  defp parse_float(""), do: :error

  defp parse_float(s) when is_binary(s) do
    case Float.parse(s) do
      {v, _} -> {:ok, v}
      :error -> :error
    end
  end

  defp parse_float(_), do: :error

  @impl true
  def render(assigns) do
    ~H"""
    <details class="essence" open>
      <summary><strong>ℹ Ze Theory · entropic-geometric ansatz</strong> — what this simulator computes (click to collapse)</summary>
      <div class="essence-body">
        <p>
          <strong>The interactive widget below</strong> simulates the central law of Ze Theory:
          <code>dτ_Ze/dt = −α · I(Z)</code>, where <code>I(Z)</code> is the Kullback–Leibler
          divergence between actual and modelled state (the <em>impedance</em>). From this single
          ansatz the simulator <em>derives mathematically</em>:
        </p>
        <ul>
          <li><code>E_Ze(a,b) = −a·b + δ · [(a·b)² − 1/3]</code> — quadratic CHSH deformation</li>
          <li><code>S_Ze = 2√2 + δ · 1.7478</code> — optimal CHSH parameter under deformation</li>
          <li><code>C(τ) = C₀ · exp(−β·I·τ)</code> — correlation decay</li>
          <li><code>F_Q ≥ 8·C₀·(β·I·τ)²·(1 − β·I·τ)</code> — QFI lower bound (Lemma E)</li>
          <li><code>F_Q,max ∝ |dτ_Ze/dt|</code> — Theorem 1 (leading order at optimal τ)</li>
          <li><code>v* = 0.45631</code> at <code>k_λ = 1</code> — the universal aging fixed point</li>
        </ul>
        <p>
          <strong>Why it exists.</strong> Aging research has decoupled "information" (epigenetic
          clocks, biomarkers) from "thermodynamics" (entropy production, dissipation). Ze Theory
          unifies them via a single quantity: prediction error. A system that predicts itself well
          burns less time; a system whose model decays burns more. The χ_Ze fixed point is what
          falls out of the variational principle <code>F = E − T·S − λ·I_pred</code>.
        </p>
        <p>
          <strong>Status.</strong> Internal manuscript, not peer-reviewed (Tkemaladze 2026,
          <em>Longevity Horizon</em> 2(5), DOI <a href="https://doi.org/10.65649/xf5vp867">10.65649/xf5vp867</a>).
          Mathematical derivations passing CI; biological extension is hypothesis-stage —
          <a href="https://biosense.longevity.ge">BioSense</a> empirically confirms the v*
          fixed point on All-of-Us N=500 (95% CI 0.443–0.459).
        </p>
        <p>
          <strong>How to use.</strong> Drag the sliders below to watch the CHSH deformation,
          decay curve, and QFI sweep update in real time. The simulation runs server-side; only
          the slider values are transmitted. For the full derivation see <a href="/about">/about</a>.
        </p>
        <p>
          <strong>For:</strong> theorists · physicists checking the CHSH/LGI/QFI derivation ·
          readers cross-validating the v* fixed point · ecosystem partners linking Ze to
          <a href="https://mcoa.longevity.ge">MCOA</a> counters.
        </p>
      </div>
    </details>

    <div class="card">
      <h2>Parameters</h2>
      <form phx-change="update" phx-debounce="200">
        <div class="row">
          <div class="control">
            <label>δ (CHSH deformation): <span class="val">{Float.round(@delta, 4)}</span></label>
            <input type="range" name="delta" min="0" max="0.5" step="0.005" value={@delta} />
          </div>
          <div class="control">
            <label>I (impedance): <span class="val">{Float.round(@i, 4)}</span></label>
            <input type="range" name="i" min="0" max="1.0" step="0.005" value={@i} />
          </div>
          <div class="control">
            <label>β (decay coefficient): <span class="val">{Float.round(@beta, 4)}</span></label>
            <input type="range" name="beta" min="0.1" max="3.0" step="0.05" value={@beta} />
          </div>
          <div class="control">
            <label>C₀ (correlation amplitude): <span class="val">{Float.round(@c0, 4)}</span></label>
            <input type="range" name="c0" min="0.1" max="2.0" step="0.05" value={@c0} />
          </div>
          <div class="control">
            <label>τ_max (plot range): <span class="val">{Float.round(@tau_max, 2)}</span></label>
            <input type="range" name="tau_max" min="1.0" max="10.0" step="0.5" value={@tau_max} />
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
      <h2>CHSH</h2>
      <p class="formula">E_Ze(a,b) = −a·b + δ·[(a·b)² − 1/3]</p>
      <%= if is_map(@chsh) and Map.has_key?(@chsh, "s_ze") do %>
        <div class="row-stats" style="margin-top: 12px;">
          <div class="stat">
            <div class="k">S_QM (Tsirelson)</div>
            <div class="v">{Float.round(@chsh["s_qm"], 4)}</div>
          </div>
          <div class="stat">
            <div class="k">S_Ze (numerical opt)</div>
            <div class="v">{Float.round(@chsh["s_ze"], 4)}</div>
          </div>
          <div class="stat">
            <div class="k">S_Ze predicted (linear)</div>
            <div class="v">{Float.round(@chsh["predicted_linear"], 4)}</div>
          </div>
          <div class="stat">
            <div class="k">δ × 1.7478</div>
            <div class="v">{Float.round(@chsh["delta"] * @chsh["deformation_const"], 4)}</div>
          </div>
        </div>
        <%= if @chsh["warning"] do %>
          <p style="margin-top: 8px;">
            <span class="badge badge-warn">{@chsh["warning"]}</span>
          </p>
        <% end %>
      <% end %>
    </div>

    <div class="row">
      <div class="card">
        <h2>Correlation decay C(τ) = C₀·exp(−β·I·τ)</h2>
        {decay_plot(@decay_curve, @c0)}
      </div>
      <div class="card">
        <h2>QFI sweep — F_Q,max vs |dτ_Ze/dt|</h2>
        {qfi_plot(@qfi_sweep)}
      </div>
    </div>
    """
  end

  defp backend_class("ok" <> _), do: "badge-ok"
  defp backend_class(_), do: "badge-warn"

  defp decay_plot(nil, _) do
    assigns = %{}
    ~H[<p class="err">no data</p>]
  end

  defp decay_plot([], _) do
    assigns = %{}
    ~H[<p class="err">no data (β·I·τ may be out of regime)</p>]
  end

  defp decay_plot(points, c0) do
    {w, h} = {500, 200}
    pl = 40
    pr = 16
    pt = 16
    pb = 30
    inner_w = w - pl - pr
    inner_h = h - pt - pb
    {tau_min, tau_max} = {0.0, points |> Enum.map(&elem(&1, 0)) |> Enum.max(fn -> 1.0 end)}
    {y_min, y_max} = {0.0, max(c0, 1.0)}

    pts =
      points
      |> Enum.map(fn {tau, c} ->
        x = pl + (tau - tau_min) / max(tau_max - tau_min, 1.0e-9) * inner_w
        y = pt + (1.0 - (c - y_min) / max(y_max - y_min, 1.0e-9)) * inner_h
        "#{Float.round(x, 1)},#{Float.round(y, 1)}"
      end)
      |> Enum.join(" ")

    assigns = %{w: w, h: h, pl: pl, pr: pr, pt: pt, pb: pb, pts: pts, tau_max: tau_max, y_max: y_max}

    ~H"""
    <svg class="plot" viewBox={"0 0 #{@w} #{@h}"} preserveAspectRatio="xMidYMid meet">
      <line class="axis" x1={@pl} y1={@pt} x2={@pl} y2={@h - @pb} />
      <line class="axis" x1={@pl} y1={@h - @pb} x2={@w - @pr} y2={@h - @pb} />
      <text x={@pl - 4} y={@pt + 4} text-anchor="end">{Float.round(@y_max, 2)}</text>
      <text x={@pl - 4} y={@h - @pb} text-anchor="end">0</text>
      <text x={@pl} y={@h - 8}>0</text>
      <text x={@w - @pr} y={@h - 8} text-anchor="end">τ = {Float.round(@tau_max, 2)}</text>
      <polyline class="curve" points={@pts} fill="none" />
    </svg>
    """
  end

  defp qfi_plot(nil) do
    assigns = %{}
    ~H[<p class="err">no data</p>]
  end

  defp qfi_plot([]) do
    assigns = %{}
    ~H[<p class="err">no data</p>]
  end

  defp qfi_plot(points) do
    {w, h} = {500, 200}
    pl = 40
    pr = 16
    pt = 16
    pb = 30
    inner_w = w - pl - pr
    inner_h = h - pt - pb

    xs = Enum.map(points, &elem(&1, 0))
    ys = Enum.map(points, &elem(&1, 1))
    {x_min, x_max} = {Enum.min(xs), Enum.max(xs)}
    {y_min, y_max} = {0.0, max(Enum.max(ys), 1.0e-6)}

    pts =
      points
      |> Enum.map(fn {x, y} ->
        sx = pl + (x - x_min) / max(x_max - x_min, 1.0e-9) * inner_w
        sy = pt + (1.0 - (y - y_min) / max(y_max - y_min, 1.0e-9)) * inner_h
        "#{Float.round(sx, 1)},#{Float.round(sy, 1)}"
      end)
      |> Enum.join(" ")

    assigns = %{w: w, h: h, pl: pl, pr: pr, pt: pt, pb: pb, pts: pts, x_max: x_max, y_max: y_max}

    ~H"""
    <svg class="plot" viewBox={"0 0 #{@w} #{@h}"} preserveAspectRatio="xMidYMid meet">
      <line class="axis" x1={@pl} y1={@pt} x2={@pl} y2={@h - @pb} />
      <line class="axis" x1={@pl} y1={@h - @pb} x2={@w - @pr} y2={@h - @pb} />
      <text x={@pl - 4} y={@pt + 4} text-anchor="end">{Float.round(@y_max, 3)}</text>
      <text x={@pl - 4} y={@h - @pb} text-anchor="end">0</text>
      <text x={@pl} y={@h - 8}>I→0</text>
      <text x={@w - @pr} y={@h - 8} text-anchor="end">I = {Float.round(@x_max, 2)}</text>
      <polyline class="curve curve-alt" points={@pts} fill="none" />
    </svg>
    """
  end
end
