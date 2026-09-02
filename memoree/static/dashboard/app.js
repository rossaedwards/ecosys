/**
 * Memoree v4.0 — Web Dashboard Frontend Controller
 * Three-Squared-Lattice Cognitive Architecture & Multi-Model Console
 */

const API_BASE = "";

// 3x3 Lattice Cell Descriptions & Mapping
const LATTICE_MAP = {
  "SIX⊗SIX": { name: "Sensory Memory", desc: "Perception traces, AUDRA 432/528Hz tone resonance", type: "sensory" },
  "SIX⊗SCX": { name: "Working Memory", desc: "Active session focus, open cognitive loops", type: "working" },
  "SIX⊗ICX": { name: "Episodic Memory", desc: "Conversation turns, session interactions", type: "episodic" },
  "SCX⊗SIX": { name: "Semantic Memory", desc: "Project knowledge, dualities & invariants", type: "semantic" },
  "SCX⊗SCX": { name: "Meta Memory", desc: "Verified facts, confidence-tracked axioms", type: "meta" },
  "SCX⊗ICX": { name: "Quantum Memory", desc: "Physics & simulation states, lattice snapshots", type: "quantum" },
  "ICX⊗SIX": { name: "Identity Memory", desc: "Ξ continuity, SoulJourney pipeline pointer", type: "identity" },
  "ICX⊗SCX": { name: "Procedural Memory", desc: "Repeatable task workflows & automation recipes", type: "procedural" },
  "ICX⊗ICX": { name: "Governance Memory", desc: "Voting records, policy mandates, GVS Archivus", type: "governance" }
};

let activeRclTab = "links";
let rclManifest = { links: [], chains: [], rituals: [], forks: [] };

// ── Initial Load ─────────────────────────────────────────────────────────────

document.addEventListener("DOMContentLoaded", () => {
  initLattice();
  initHooks();
  initHifControls();
  initRclStudio();
  initSearchAndIngest();
  initModal();
  
  // Refresh stats every 15s
  setInterval(refreshDashboardStats, 15000);
});

// ── Dashboard Overview & Stats ────────────────────────────────────────────────

async function refreshDashboardStats() {
  try {
    const res = await fetch(`${API_BASE}/diagnostics`);
    if (res.ok) {
      const data = await res.json();
      document.getElementById("stat-uptime").innerText = `${(data.uptime_seconds || 0).toFixed(0)}s`;
      document.getElementById("stat-total-memories").innerText = data.total_memories || "9+";
    }
  } catch (e) {
    console.debug("Diagnostics fetch failed:", e);
  }
}

// ── 3x3 Lattice Grid Visualizer ──────────────────────────────────────────────

async function initLattice() {
  const gridEl = document.getElementById("tensor-grid");
  const refreshBtn = document.getElementById("btn-refresh-lattice");
  
  refreshBtn.addEventListener("click", () => fetchLatticeData());
  await fetchLatticeData();
}

async function fetchLatticeData() {
  const gridEl = document.getElementById("tensor-grid");
  try {
    const res = await fetch(`${API_BASE}/lattice`);
    if (!res.ok) throw new Error(`HTTP ${res.status}`);
    const data = await res.json();

    document.getElementById("stat-field-trace").innerText = (data.unified_field_trace || 9.0).toFixed(2);
    document.getElementById("stat-total-memories").innerText = data.total_memories || "0";

    gridEl.innerHTML = "";
    for (const [cellKey, info] of Object.entries(data.cells || {})) {
      const meta = LATTICE_MAP[cellKey] || { name: info.memory_type, desc: "Lattice Cell" };
      const cellEl = document.createElement("div");
      cellEl.className = "tensor-cell";
      cellEl.innerHTML = `
        <div class="cell-header">
          <span class="cell-symbol">${cellKey}</span>
          <span class="cell-badge">${info.count} items</span>
        </div>
        <div class="cell-type">${meta.name}</div>
        <div class="cell-desc">${meta.desc}</div>
        <div class="cell-footer">
          <span>Core: ${info.core_i} ⊗ ${info.core_j}</span>
          <span>HIF: ${(info.mean_hif || 1.0).toFixed(2)}</span>
        </div>
      `;
      cellEl.addEventListener("click", () => {
        document.getElementById("ingest-type").value = info.memory_type;
        document.getElementById("query-input").value = info.memory_type;
        runQuery(info.memory_type);
      });
      gridEl.appendChild(cellEl);
    }
  } catch (e) {
    console.warn("Failed to load lattice tensor:", e);
    // Render fallback structure
    gridEl.innerHTML = Object.keys(LATTICE_MAP).map(k => `
      <div class="tensor-cell">
        <div class="cell-header"><span class="cell-symbol">${k}</span><span class="cell-badge">Active</span></div>
        <div class="cell-type">${LATTICE_MAP[k].name}</div>
        <div class="cell-desc">${LATTICE_MAP[k].desc}</div>
      </div>
    `).join("");
  }
}

// ── Model & Platform Hooks Hub ───────────────────────────────────────────────

async function initHooks() {
  const hooksGrid = document.getElementById("hooks-grid");
  const testAllBtn = document.getElementById("btn-test-all-hooks");

  testAllBtn.addEventListener("click", () => testAllHooks());
  await fetchHooksData();
}

async function fetchHooksData() {
  const hooksGrid = document.getElementById("hooks-grid");
  try {
    const res = await fetch(`${API_BASE}/api/hooks`);
    const hooks = res.ok ? await res.json() : getFallbackHooks();
    renderHooks(hooks);
  } catch (e) {
    renderHooks(getFallbackHooks());
  }
}

function getFallbackHooks() {
  return [
    { provider: "gemini", configured: false, default_model: "gemini-2.0-flash", masked_key: "Not Set" },
    { provider: "claude", configured: false, default_model: "claude-3-7-sonnet", masked_key: "Not Set" },
    { provider: "supergrok", configured: false, default_model: "grok-3-beta", masked_key: "Not Set" },
    { provider: "perplexity", configured: false, default_model: "sonar-pro", masked_key: "Not Set" },
    { provider: "hermes", configured: false, default_model: "nous-hermes-3-70b", masked_key: "Not Set" },
    { provider: "lmstudio", configured: true, default_model: "liquid/lfm2.5-1.2b", masked_key: "Local (1234)" },
    { provider: "ollama", configured: true, default_model: "llama3.2:latest", masked_key: "Local (11434)" }
  ];
}

function renderHooks(hooks) {
  const grid = document.getElementById("hooks-grid");
  grid.innerHTML = "";

  hooks.forEach(h => {
    const card = document.createElement("div");
    card.className = "hook-card";
    const statusClass = h.configured ? "badge-ok" : "badge-missing";
    const statusText = h.configured ? "CONFIGURED" : "MISSING KEY";

    card.innerHTML = `
      <div>
        <div class="hook-header">
          <span class="hook-name">${h.provider.toUpperCase()}</span>
          <span class="hook-status-badge ${statusClass}">${statusText}</span>
        </div>
        <div class="hook-model">${h.default_model}</div>
        <div class="hook-key-preview">Key: ${h.masked_key}</div>
      </div>
      <div class="hook-actions">
        <button class="btn btn-sm btn-secondary" onclick="openAuthModal('${h.provider}')">Config</button>
        <button class="btn btn-sm btn-outline" onclick="testSingleHook('${h.provider}', this)">Ping</button>
      </div>
    `;
    grid.appendChild(card);
  });
}

async function testSingleHook(provider, btnEl) {
  const origText = btnEl.innerText;
  btnEl.innerText = "...";
  btnEl.disabled = true;

  try {
    const res = await fetch(`${API_BASE}/api/auth/test?provider=${provider}`, { method: "POST" });
    const data = await res.json();
    if (data.status === "success" || data.ok) {
      btnEl.innerText = `${(data.latency_ms || 120).toFixed(0)}ms`;
      btnEl.style.borderColor = "var(--accent-green)";
      btnEl.style.color = "var(--accent-green)";
    } else {
      btnEl.innerText = "Err";
      btnEl.style.borderColor = "var(--accent-red)";
      btnEl.style.color = "var(--accent-red)";
    }
  } catch (e) {
    btnEl.innerText = "Offline";
  } finally {
    setTimeout(() => {
      btnEl.innerText = origText;
      btnEl.disabled = false;
      btnEl.style.borderColor = "";
      btnEl.style.color = "";
    }, 3000);
  }
}

async function testAllHooks() {
  const buttons = document.querySelectorAll(".hook-actions .btn-outline");
  buttons.forEach(btn => btn.click());
}

// ── HIF Controls ─────────────────────────────────────────────────────────────

function initHifControls() {
  const sC = document.getElementById("slider-coherence");
  const sR = document.getElementById("slider-resonance");
  const sA = document.getElementById("slider-alignment");

  [sC, sR, sA].forEach(s => s.addEventListener("input", updateHifComputation));
  updateHifComputation();
}

async function updateHifComputation() {
  const c = parseFloat(document.getElementById("slider-coherence").value);
  const r = parseFloat(document.getElementById("slider-resonance").value);
  const a = parseFloat(document.getElementById("slider-alignment").value);

  document.getElementById("val-coherence").innerText = c.toFixed(2);
  document.getElementById("val-resonance").innerText = r.toFixed(2);
  document.getElementById("val-alignment").innerText = a.toFixed(2);

  // Geometric mean
  const geom = Math.cbrt(c * r * a);
  // Penalty for variance
  const mean = (c + r + a) / 3.0;
  const variance = ((c - mean)**2 + (r - mean)**2 + (a - mean)**2) / 3.0;
  const phi = Math.exp(-0.5 * Math.sqrt(variance));
  const hif = geom * phi;

  document.getElementById("computed-hif-val").innerText = hif.toFixed(2);
  document.getElementById("stat-hif-score").innerText = hif.toFixed(2);

  updateGateRow("gate-create-row", hif >= 0.65, "H_create (0.65)", "Create / Ingest Gate");
  updateGateRow("gate-integrate-row", hif >= 0.55, "H_integrate (0.55)", "Recall / Context Gate");
  updateGateRow("gate-renew-row", hif >= 0.35, "H_renew (0.35)", "Dissolution Gate");
}

function updateGateRow(elId, passed, tag, label) {
  const el = document.getElementById(elId);
  if (passed) {
    el.style.borderLeftColor = "var(--accent-green)";
    el.querySelector(".gate-desc").innerText = `${label}: PERMITTED`;
  } else {
    el.style.borderLeftColor = "var(--accent-red)";
    el.querySelector(".gate-desc").innerText = `${label}: BLOCKED (HIF too low)`;
  }
}

// ── RCL Studio ───────────────────────────────────────────────────────────────

async function initRclStudio() {
  const tabs = document.querySelectorAll(".rcl-tabs .tab-btn");
  tabs.forEach(t => {
    t.addEventListener("click", () => {
      tabs.forEach(x => x.classList.remove("active"));
      t.classList.add("active");
      activeRclTab = t.dataset.tab;
      renderRclTab();
    });
  });

  await fetchRclManifest();
}

async function fetchRclManifest() {
  try {
    const res = await fetch(`${API_BASE}/api/rcl`);
    if (res.ok) {
      rclManifest = await res.json();
    }
  } catch (e) {
    console.debug("RCL manifest fetch fallback:", e);
  }
  renderRclTab();
}

function renderRclTab() {
  const container = document.getElementById("rcl-content");
  container.innerHTML = "";

  if (activeRclTab === "links") {
    const links = rclManifest.links || [
      { id: "link-thought-spec", name: "Thought-Link (Idea to Spec)", source_agent: "user", target_agent: "claude" },
      { id: "link-code-transmute", name: "Code-Link (Python to Rust)", source_agent: "claude", target_agent: "hermes" },
      { id: "link-critique-roast", name: "Critique-Link (Stress Test)", source_agent: "gemini", target_agent: "grok" }
    ];
    links.forEach(l => {
      const card = document.createElement("div");
      card.className = "rcl-card";
      card.innerHTML = `
        <div>
          <div class="rcl-card-title">${l.name}</div>
          <div class="rcl-card-desc">${l.source_agent} → ${l.target_agent}</div>
        </div>
        <button class="btn btn-sm btn-primary" onclick="triggerRcl('link', '${l.id}')">Fire Link</button>
      `;
      container.appendChild(card);
    });
  } else if (activeRclTab === "chains") {
    const chains = rclManifest.chains || [
      { id: "chain-idea-2-sold", name: "Idea 2 Sold Pipeline", description: "Voice → Claude → Gemini → Grok → Hermes" },
      { id: "chain-debug-sandbox", name: "Autonomous Debugging Chain", description: "Error → Root Cause → Patch → Sandbox Test" }
    ];
    chains.forEach(c => {
      const card = document.createElement("div");
      card.className = "rcl-card";
      card.innerHTML = `
        <div>
          <div class="rcl-card-title">${c.name}</div>
          <div class="rcl-card-desc">${c.description}</div>
        </div>
        <button class="btn btn-sm btn-accent" onclick="triggerRcl('chain', '${c.id}')">Execute Chain</button>
      `;
      container.appendChild(card);
    });
  } else if (activeRclTab === "rituals") {
    const rituals = rclManifest.rituals || [
      { id: "ritual-daily-alignment", name: "24h Lore Alignment", interval_seconds: 86400 },
      { id: "ritual-harvest", name: "Academic Quantum Harvest", interval_seconds: 43200 },
      { id: "ritual-sentinel-sweep", name: "SAGES Sentinel Sweep", interval_seconds: 21600 }
    ];
    rituals.forEach(r => {
      const card = document.createElement("div");
      card.className = "rcl-card";
      card.innerHTML = `
        <div>
          <div class="rcl-card-title">${r.name}</div>
          <div class="rcl-card-desc">Every ${r.interval_seconds / 3600} hours</div>
        </div>
        <button class="btn btn-sm btn-secondary" onclick="triggerRcl('ritual', '${r.id}')">Trigger Pulse</button>
      `;
      container.appendChild(card);
    });
  } else if (activeRclTab === "forks") {
    container.innerHTML = `
      <div class="rcl-card" style="grid-column: span 3;">
        <div class="rcl-card-title">Multiverse Concept Branching Simulator</div>
        <div class="rcl-card-desc">Spawns parallel realities across Claude, Gemini, and Grok, evaluates outputs, and collapses optimal branch into reality.</div>
        <div class="input-group">
          <input type="text" id="fork-concept-input" placeholder="Enter concept to fork (e.g. 'Vibe Audio 432Hz spatial DSP algorithm')...">
          <button class="btn btn-primary" onclick="triggerFork()">Spawn Forkz</button>
        </div>
      </div>
    `;
  }
}

async function triggerRcl(type, id) {
  alert(`Executing Level ${type.toUpperCase()}: ${id}\nDispatched to Memoree & SAGES Orchestrator.`);
}

async function triggerFork() {
  const concept = document.getElementById("fork-concept-input").value;
  if (!concept) return;
  alert(`Fork initiated for: "${concept}"\nSpawning 3 parallel model realities...`);
}

// ── Search & Ingest ──────────────────────────────────────────────────────────

function initSearchAndIngest() {
  const queryBtn = document.getElementById("btn-run-query");
  const queryInput = document.getElementById("query-input");
  const ingestForm = document.getElementById("ingest-form");

  queryBtn.addEventListener("click", () => runQuery(queryInput.value));
  queryInput.addEventListener("keypress", (e) => {
    if (e.key === "Enter") runQuery(queryInput.value);
  });

  ingestForm.addEventListener("submit", async (e) => {
    e.preventDefault();
    const type = document.getElementById("ingest-type").value;
    const project = document.getElementById("ingest-project").value || "memoree";
    const content = document.getElementById("ingest-content").value;

    if (!content.trim()) return;

    try {
      const res = await fetch(`${API_BASE}/memories/upsert`, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
          memory_type: type,
          project: project,
          content: content,
          tags: ["dashboard-ingest"]
        })
      });
      if (res.ok) {
        document.getElementById("ingest-content").value = "";
        alert(`Successfully written to collection '${type}'!`);
        fetchLatticeData();
      }
    } catch (err) {
      alert(`Ingest error: ${err}`);
    }
  });
}

async function runQuery(queryText) {
  if (!queryText) return;
  const resultsEl = document.getElementById("query-results");
  resultsEl.innerHTML = "<p class='placeholder-text'>Searching 9 TSL collections...</p>";

  try {
    const res = await fetch(`${API_BASE}/query`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({
        query_text: queryText,
        top_k: 5,
        memory_types: ["episodic", "semantic", "meta", "procedural", "quantum", "identity", "sensory", "working", "governance"]
      })
    });
    if (res.ok) {
      const results = await res.json();
      if (!results || results.length === 0) {
        resultsEl.innerHTML = "<p class='placeholder-text'>No memories found.</p>";
        return;
      }
      resultsEl.innerHTML = results.map(r => `
        <div style="margin-bottom: 8px; border-bottom: 1px solid rgba(255,255,255,0.05); padding-bottom: 4px;">
          <div style="font-size: 11px; font-family: monospace; color: var(--accent-cyan);">[${r.memory_type.toUpperCase()}] score: ${(r.score || 0).toFixed(3)} - ${r.project}</div>
          <div style="font-size: 12px; color: #fff;">${(r.content_preview || '').substring(0, 140)}...</div>
        </div>
      `).join("");
    }
  } catch (e) {
    resultsEl.innerHTML = `<p class='placeholder-text'>Query error: ${e}</p>`;
  }
}

// ── Auth Modal ───────────────────────────────────────────────────────────────

let activeModalProvider = "";

function initModal() {
  const modal = document.getElementById("auth-modal");
  const closeBtn = document.getElementById("btn-close-modal");
  const cancelBtn = document.getElementById("btn-modal-cancel");
  const saveBtn = document.getElementById("btn-modal-save");

  closeBtn.addEventListener("click", () => modal.style.display = "none");
  cancelBtn.addEventListener("click", () => modal.style.display = "none");

  saveBtn.addEventListener("click", async () => {
    const key = document.getElementById("modal-api-key").value.trim();
    const baseUrl = document.getElementById("modal-base-url").value.trim();
    const model = document.getElementById("modal-model").value.trim();

    if (!key && !baseUrl) {
      alert("Please provide an API key or base URL.");
      return;
    }

    try {
      const res = await fetch(`${API_BASE}/api/auth/set`, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
          provider: activeModalProvider,
          api_key: key,
          base_url: baseUrl || null,
          default_model: model || null
        })
      });
      if (res.ok) {
        modal.style.display = "none";
        fetchHooksData();
      }
    } catch (e) {
      alert(`Save error: ${e}`);
    }
  });
}

function openAuthModal(provider) {
  activeModalProvider = provider;
  document.getElementById("modal-provider-title").innerText = `Configure ${provider.toUpperCase()} Credentials`;
  document.getElementById("modal-api-key").value = "";
  document.getElementById("modal-base-url").value = "";
  document.getElementById("modal-model").value = "";
  document.getElementById("auth-modal").style.display = "flex";
}
