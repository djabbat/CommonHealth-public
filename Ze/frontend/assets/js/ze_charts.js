/**
 * ZeCharts — LiveView hook that renders charts for Ze Theory simulators.
 * Uses Chart.js loaded on demand from a CDN.
 */

const CDN = "https://cdn.jsdelivr.net/npm/chart.js@4.4.1/dist/chart.umd.min.js"
let chartJsPromise = null

function loadChartJs() {
  if (typeof window.Chart !== "undefined") return Promise.resolve(window.Chart)
  if (!chartJsPromise) {
    chartJsPromise = new Promise((resolve, reject) => {
      const s = document.createElement("script")
      s.src = CDN
      s.onload = () => resolve(window.Chart)
      s.onerror = reject
      document.head.appendChild(s)
    })
  }
  return chartJsPromise
}

function line(Chart, canvasId, label, color, { xTitle = "τ", beginAtZero = false } = {}) {
  const el = document.getElementById(canvasId)
  if (!el) return null
  return new Chart(el.getContext("2d"), {
    type: "line",
    data: { labels: [], datasets: [{ label, data: [], borderColor: color, backgroundColor: color + "22", borderWidth: 2, pointRadius: 0, tension: 0.15, fill: false }] },
    options: { responsive: true, animation: false,
      plugins: { legend: { display: true } },
      scales: { x: { title: { display: true, text: xTitle }, ticks: { maxTicksLimit: 8 } }, y: { beginAtZero } }
    }
  })
}

function setLine(chart, labels, data) {
  if (!chart) return
  chart.data.labels = labels
  chart.data.datasets[0].data = data
  chart.update()
}

function multiLine(Chart, canvasId, datasets, xTitle) {
  const el = document.getElementById(canvasId)
  if (!el) return null
  return new Chart(el.getContext("2d"), {
    type: "line",
    data: { labels: [], datasets: datasets.map(d => ({ ...d, data: [], borderWidth: 2, pointRadius: 0, tension: 0.15, fill: false })) },
    options: { responsive: true, animation: false,
      plugins: { legend: { display: true } },
      scales: { x: { title: { display: true, text: xTitle }, ticks: { maxTicksLimit: 8 } } }
    }
  })
}

function setMulti(chart, labels, arrays) {
  if (!chart) return
  chart.data.labels = labels
  arrays.forEach((arr, i) => { chart.data.datasets[i].data = arr })
  chart.update()
}

const ZeCharts = {
  mounted() {
    this.charts = {}
    this.ready = false
    this.pending = null
    loadChartJs().then((Chart) => {
      this.Chart = Chart
      this.ready = true
      if (this.pending) { this.render(this.pending); this.pending = null }
    })
    this.handleEvent("ze-data", (payload) => {
      if (!this.ready) { this.pending = payload; return }
      this.render(payload)
    })
  },

  render({ tab, data }) {
    const Chart = this.Chart
    const containerTab = this.el.dataset.tab
    if (containerTab !== tab) return  // not this container

    if (tab === "impedance") {
      this.charts.I ||= line(Chart, "chart-I", "𝓘(τ)", "#d45087")
      this.charts.t ||= line(Chart, "chart-t", "t_phys(τ) = ∫ 𝓘 dτ", "#2f4b7c")
      this.charts.C ||= line(Chart, "chart-C", "𝒞(τ) = −d𝓘/dτ", "#ff7c43")
      this.charts.K ||= line(Chart, "chart-K", "K(τ) = −𝓘", "#665191")
      const labels = data.tau.map(v => v.toFixed(1))
      setLine(this.charts.I, labels, data.i)
      setLine(this.charts.t, labels, data.t_phys)
      setLine(this.charts.C, labels, data.consciousness)
      setLine(this.charts.K, labels, data.k)
    }
    else if (tab === "chsh") {
      this.charts.sweep ||= multiLine(Chart, "chart-sweep", [
        { label: "S_QM (=2√2)", borderColor: "#808080", backgroundColor: "#80808022" },
        { label: "S_Ze (singlet-opt)", borderColor: "#d45087", backgroundColor: "#d4508722" },
        { label: "S(H) = 2√2·(1−2αH)", borderColor: "#ff7c43", backgroundColor: "#ff7c4322" },
      ], "H")
      this.charts.shift ||= line(Chart, "chart-shift", "S_Ze − S_QM", "#2f4b7c", { xTitle: "H" })
      const sweep = data.sweep
      const labels = sweep.h.map(h => h.toFixed(2))
      setMulti(this.charts.sweep, labels, [sweep.s_qm, sweep.s_ze, sweep.s_damped])
      const shift = sweep.s_ze.map((sz, i) => sz - sweep.s_qm[i])
      setLine(this.charts.shift, labels, shift)
    }
    else if (tab === "autowaves") {
      this.charts.means ||= multiLine(Chart, "chart-means", [
        { label: "⟨I⟩", borderColor: "#d45087", backgroundColor: "#d4508722" },
        { label: "⟨x⟩", borderColor: "#2f4b7c", backgroundColor: "#2f4b7c22" },
        { label: "⟨y⟩", borderColor: "#ff7c43", backgroundColor: "#ff7c4322" },
      ], "t")
      this.charts.snap ||= multiLine(Chart, "chart-snap",
        data.snapshots.map((s, i) => ({
          label: `t=${s.t.toFixed(1)}`,
          borderColor: `hsl(${(i * 60) % 360} 70% 50%)`,
          backgroundColor: "transparent"
        })),
        "x"
      )
      // if count changed, recreate snap chart
      if (this.charts.snap.data.datasets.length !== data.snapshots.length) {
        this.charts.snap.destroy()
        this.charts.snap = multiLine(Chart, "chart-snap",
          data.snapshots.map((s, i) => ({
            label: `t=${s.t.toFixed(1)}`,
            borderColor: `hsl(${(i * 60) % 360} 70% 50%)`,
            backgroundColor: "transparent"
          })),
          "x"
        )
      }
      const tlabels = data.t_axis.map(v => v.toFixed(1))
      setMulti(this.charts.means, tlabels, [data.i_mean, data.x_mean, data.y_mean])

      const nGrid = data.snapshots[0]?.i.length || 0
      const xlabels = Array.from({ length: nGrid }, (_, i) => String(i))
      setMulti(this.charts.snap, xlabels, data.snapshots.map(s => s.i))
    }
  }
}

export default ZeCharts
