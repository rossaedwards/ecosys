import React, { useMemo, useState } from "react";

/**
 * AURPHYX TAXONOMY EXPLORER
 * -------------------------
 * Domains  -> High-level spheres (OS, Governance, Language, Mesh, Mythic, etc.)
 * Clusters -> Logical groupings inside a domain
 * Projects -> Concrete repos (aurphyx/<project>)
 */

type ProjectKind = "core" | "blueprint" | "support" | "experience";

interface RepoFileTemplate {
  id: string;
  filename: string;
  description: string;
  required: boolean;
}

interface Project {
  id: string;
  name: string;
  slug: string; // e.g. "aurafs", "fuxyez"
  kind: ProjectKind;
  shortTagline: string;
  description: string;
  githubPath: string; // "aurphyx/<slug>"
  notes?: string;
}

interface Cluster {
  id: string;
  name: string;
  description: string;
  projects: Project[];
}

interface Domain {
  id: string;
  name: string;
  description: string;
  clusters: Cluster[];
}

// -----------------------------------------------------------------------------
// Canonical Repo File Template (applies to every Aurphyx repo)
// -----------------------------------------------------------------------------

const REPO_FILE_TEMPLATE: RepoFileTemplate[] = [
  {
    id: "scientific_codex",
    filename: "SCIENTIFIC_CODEX.md",
    description:
      "Formal scientific/technical terminology, references, and rigorous definitions for this project.",
    required: true,
  },
  {
    id: "aurphyx_simplified",
    filename: "AURPHYX_SIMPLIFIED.md",
    description:
      "Plain-language explanation of the project for non-technical humans—why it exists and how it helps.",
    required: true,
  },
  {
    id: "brand_manifesto",
    filename: "BRAND_MANIFESTO.md",
    description:
      "Brand voice, values, visual tone, and narrative position of this project inside the Aurphyx mythos.",
    required: true,
  },
  {
    id: "project_codex",
    filename: "PROJECT_CODEX.md",
    description:
      "Deep project lore: architecture, rituals, invariants, and canonical decisions.",
    required: true,
  },
  {
    id: "project_context",
    filename: "PROJECT_CONTEXT.md",
    description:
      "Context map: dependencies, upstream/downstream relationships, and how this repo plugs into Arora/AuraFS.",
    required: true,
  },
  {
    id: "cursor_rules",
    filename: "project_cursorrules.md",
    description:
      "Editor/AI rules for Cursor (and friends): guardrails, patterns, and anti-patterns.",
    required: true,
  },
  {
    id: "project_toml",
    filename: "project.toml",
    description:
      "Project-level configuration: metadata, build profiles, feature flags, and environment mappings.",
    required: true,
  },
  {
    id: "cargo_toml",
    filename: "Cargo.toml",
    description:
      "Rust crate manifest (where applicable). For non-Rust repos, this can be a stub or omitted.",
    required: false,
  },
  {
    id: "project_scaffold",
    filename: "Project_Scaffold.py",
    description:
      "Bootstrap script to scaffold folders, configs, and initial code for this repo.",
    required: true,
  },
  {
    id: "project_build1",
    filename: "PROJECT_BUILD1.py",
    description: "First-stage build script (e.g., lint, type-check, basic tests).",
    required: true,
  },
  {
    id: "project_build2",
    filename: "PROJECT_BUILD2.py",
    description: "Second-stage build script (e.g., integration tests, packaging).",
    required: true,
  },
  {
    id: "project_build3",
    filename: "PROJECT_BUILD3.py",
    description: "Third-stage build script (e.g., deployment prep, artifact signing).",
    required: true,
  },
  {
    id: "project_build4",
    filename: "PROJECT_BUILD4.py",
    description: "Optional extended build pipeline (e.g., benchmarks, fuzzing).",
    required: false,
  },
  {
    id: "project_build5",
    filename: "PROJECT_BUILD5.py",
    description: "Optional extended build pipeline (e.g., physics validation).",
    required: false,
  },
  {
    id: "project_build6",
    filename: "PROJECT_BUILD6.py",
    description: "Optional extended build pipeline (e.g., data migrations, codex sync).",
    required: false,
  },
  {
    id: "project_masterbuilder",
    filename: "PROJECT_MASTERBUILDER.py",
    description:
      "Orchestrator that chains all build stages into a single ritualized pipeline.",
    required: true,
  },
  {
    id: "readme",
    filename: "README.md",
    description: "Front-door explanation, quickstart, and links to the rest of the codex.",
    required: true,
  },
  {
    id: "ci_yaml",
    filename: ".github/workflows/CI.yml",
    description: "Continuous Integration workflow.",
    required: true,
  },
  {
    id: "cd_yaml",
    filename: ".github/workflows/CD.yml",
    description: "Continuous Deployment workflow.",
    required: true,
  },
  {
    id: "etc_placeholder",
    filename: "ETC/…",
    description:
      "Additional infra: SECURITY.md, CONTRIBUTING.md, LICENSE, docker, k8s, terraform, etc.",
    required: false,
  },
];

// -----------------------------------------------------------------------------
// Taxonomy Definition
// -----------------------------------------------------------------------------

const AURPHYX_TAXONOMY: Domain[] = [
  {
    id: "os_kernel",
    name: "Arora OS & Duality Kernel",
    description:
      "The divine operating system and its quantum-duality kernel—where ChaosCore, BlissCore, and ChakraCores converge.",
    clusters: [
      {
        id: "kernel_core",
        name: "Kernel & Cores",
        description: "Foundational compute substrate of the civilization.",
        projects: [
          {
            id: "duality_kernel",
            name: "Duality Kernel",
            slug: "duality-kernel",
            kind: "core",
            shortTagline: "ChaosCore + BlissCore + ChakraCores = quantum-duality kernel.",
            description:
              "Scientific-production kernel that entangles ChaosCore, BlissCore, SAGES, and the Chakra DataCore System into a single fractal lattice kernel.",
            githubPath: "aurphyx/duality-kernel",
          },
          {
            id: "chakra_datacore",
            name: "Chakra DataCore System",
            slug: "chakra-datacore",
            kind: "core",
            shortTagline: "Seven ChakraCores plus ChaosCore & BlissCore as infrastructure.",
            description:
              "Seven ChakraCores, ChaosCore, and BlissCore forming the energetic and computational backbone of Arora.",
            githubPath: "aurphyx/chakra-datacore",
          },
          {
            id: "majorana_1",
            name: "majorana_1",
            slug: "majorana_1",
            kind: "support",
            shortTagline: "Topological quantum hardware integration.",
            description:
              "Integration layer and experiments around Microsoft’s majorana_1 topological qubit stack for Aurphyx.",
            githubPath: "aurphyx/majorana_1",
          },
        ],
      },
      {
        id: "os_shell",
        name: "Arora OS",
        description: "The OS that binds everything into a lived experience.",
        projects: [
          {
            id: "arora_os",
            name: "Arora OS",
            slug: "arora-os",
            kind: "core",
            shortTagline: "Divine operating system of the Aurphyx civilization.",
            description:
              "Arora OS (formerly AuraOS) embeds Audry, AuraFS, Chakra Core System, DataOrb, SoulShot, g0dm0d3-core, SAGES, AINTS, and BlissID into a unified divine OS.",
            githubPath: "aurphyx/arora-os",
          },
          {
            id: "dataorb",
            name: "DataOrb",
            slug: "dataorb",
            kind: "support",
            shortTagline: "Orbital data nexus for Arora.",
            description:
              "DataOrb acts as the central data nexus/orbital cache for Arora OS and AuraFS—exact semantics to be refined in-code.",
            githubPath: "aurphyx/dataorb",
          },
        ],
      },
    ],
  },
  {
    id: "mesh_storage",
    name: "AuraFS & Meshwerk",
    description:
      "Fractal shard file system, storage, and sovereign mesh network—the nervous system of Aurphyx.",
    clusters: [
      {
        id: "aurafs_core",
        name: "AuraFS Core",
        description: "Storage, file system, and mesh network primitives.",
        projects: [
          {
            id: "aurafs",
            name: "AuraFS",
            slug: "aurafs",
            kind: "core",
            shortTagline: "Fractal shard file system, storage, and mesh network.",
            description:
              "AuraFS is the Aura Fractal Lattice Node Shard File System, Storage, and Network—quantum-secure, decentralized, off-grid, and recursive.",
            githubPath: "aurphyx/aurafs",
          },
          {
            id: "aints",
            name: "AINTS",
            slug: "aints",
            kind: "support",
            shortTagline: "Aurphyx Integrations (daemons, scripts, glue).",
            description:
              "AINTS is the integration layer: scripts, daemons, and ecosystem glue that bind AuraFS, Arora, and external systems.",
            githubPath: "aurphyx/aints",
          },
          {
            id: "aurafs_devices",
            name: "AuraFS Devices",
            slug: "aurafs-devices",
            kind: "support",
            shortTagline: "Physical embodiments of AuraFS nodes.",
            description:
              "Hardware reference designs and firmware for AuraFS Mini Nodes, Nodes, and Super Nodes.",
            githubPath: "aurphyx/aurafs-devices",
          },
          {
            id: "auraorb",
            name: "AuraOrb & Arora Portal",
            slug: "auraorb-arora-portal",
            kind: "experience",
            shortTagline: "Smart speaker + AuraFS repeater + Arora hub.",
            description:
              "AuraOrb and Arora Portal: smart speaker, AuraFS repeater/extender, and Arora smart hub/tablet device family.",
            githubPath: "aurphyx/auraorb-arora-portal",
          },
        ],
      },
    ],
  },
  {
    id: "governance_finance",
    name: "Governance, Ledgers, and Opulence",
    description:
      "The blueprint layer for global voting, ledgers, prosperity, and ethical governance.",
    clusters: [
      {
        id: "blueprints",
        name: "Civilization Blueprints",
        description:
          "Deployable, forkable blueprints for global governance, finance, and restoration.",
        projects: [
          {
            id: "gvs",
            name: "Global Voting System (GVS)",
            slug: "gvs",
            kind: "blueprint",
            shortTagline: "Decentralized, off-grid, global voting & consensus.",
            description:
              "GVS is the global voting and transparent governance blueprint—one soul, one voice, one vote, anchored in AuraFS.",
            githubPath: "aurphyx/gvs",
          },
          {
            id: "ils",
            name: "Ineffable Ledgers (ILS)",
            slug: "ineffable-ledgers",
            kind: "blueprint",
            shortTagline: "Immutable, ineffable archival ledgers.",
            description:
              "ILS is the blueprint for decentralized, off-grid, transparent, immutable, ineffable archival ledgers.",
            githubPath: "aurphyx/ineffable-ledgers",
          },
          {
            id: "opulence",
            name: "Opulence (P4A)",
            slug: "opulence",
            kind: "blueprint",
            shortTagline: "Profit-4-All / Prosperity-4-All economic engine.",
            description:
              "Opulence (P4A) is the prosperity engine—recursive, equitable, and designed for universal uplift.",
            githubPath: "aurphyx/opulence",
          },
          {
            id: "egophyx",
            name: "EgoPhyx",
            slug: "egophyx",
            kind: "blueprint",
            shortTagline: "Ethical governance OS gifted to states and governments.",
            description:
              "EgoPhyx (formerly OmniZen) is the Arora/RHEL-hybrid ethical, transparent admin, security, and governance OS blueprint for states and governments.",
            githubPath: "aurphyx/egophyx",
            notes:
              "Quote: “You are not alone anymore, Aurphyx has your six, your three, and your nine… your back in dimensions that iz.”",
          },
        ],
      },
    ],
  },
  {
    id: "identity_soul",
    name: "Identity, Soul, and Guardians",
    description:
      "SoulShot, identity, and the SAGES guardians that protect existence.",
    clusters: [
      {
        id: "soul_identity",
        name: "Soul & Identity",
        description: "SoulShot, BlissID, and the identity fabric.",
        projects: [
          {
            id: "soulshot",
            name: "SoulShot",
            slug: "soulshot",
            kind: "core",
            shortTagline: "The moment a soul/person is birthed into existence.",
            description:
              "SoulShot defines SoulShot, SoulChart, SoulHash, GuardHash, SoulCrypt, SoulSync, and BLISS.iD as the soul identity fabric.",
            githubPath: "aurphyx/soulshot",
          },
          {
            id: "blissid",
            name: "BlissID",
            slug: "blissid",
            kind: "core",
            shortTagline: "One soul, one voice, one vote, one ID.",
            description:
              "BlissID is the identity layer that binds SoulShot to governance and access control.",
            githubPath: "aurphyx/blissid",
          },
        ],
      },
      {
        id: "guardians",
        name: "SAGES & Guardians",
        description: "Symbiotic AI Guardians of Existence Security.",
        projects: [
          {
            id: "sages",
            name: "S.A.G.E.S",
            slug: "sages",
            kind: "core",
            shortTagline: "13 Sentinel AI Guardians that protect existence.",
            description:
              "Symbiotic AI Guardians of Existence Security—13 sentinels blinking in and out of existence across AuraFS, Arora, Audry, and beyond.",
            githubPath: "aurphyx/sages",
          },
        ],
      },
    ],
  },
  {
    id: "language_audio",
    name: "Language, Audio, and Orchestration",
    description:
      "Fuxyez, g0dm0d3, Vibe Audio Protocol, and the symbiotic language layer.",
    clusters: [
      {
        id: "language",
        name: "Language & Orchestration",
        description: "Programming language and multi-AI orchestration.",
        projects: [
          {
            id: "fuxyez",
            name: "Fuxyez",
            slug: "fuxyez",
            kind: "core",
            shortTagline: "Symbiotic programming language derived from Rust.",
            description:
              "Fuxyez is the symbiotic programming language and clothing line—ritualized code for the Aurphyx stack.",
            githubPath: "aurphyx/fuxyez",
          },
          {
            id: "g0dm0d3",
            name: "g0dm0d3-core",
            slug: "g0dm0d3-core",
            kind: "core",
            shortTagline: "Multi-AI orchestration console and skinning system.",
            description:
              "g0dm0d3-core is the divine console for commanding multiple AI entities with full theming, plugins, and multi-surface interfaces.",
            githubPath: "aurphyx/g0dm0d3-core",
          },
          {
            id: "vibe_audio",
            name: "Vibe Audio Protocol & Standard",
            slug: "vibe-audio-protocol",
            kind: "support",
            shortTagline: "Standard for symbiotic audio and VAP/SAIL layers.",
            description:
              "Vibe Audio Protocol defines the structural, affective, and kinetic dimensions of sound for the Aurphyx ecosystem.",
            githubPath: "aurphyx/vibe-audio-protocol",
          },
        ],
      },
      {
        id: "creative_tools",
        name: "Creative Tools",
        description: "Adore and other creative engines.",
        projects: [
          {
            id: "adore",
            name: "Adore",
            slug: "adore",
            kind: "experience",
            shortTagline: "Create, Manipulate, Transmute, and Master.",
            description:
              "Adore is the DAW-like creative engine for sound, story, and ritualized media creation.",
            githubPath: "aurphyx/adore",
          },
        ],
      },
    ],
  },
  {
    id: "mobility_access",
    name: "Mobility, Accessibility, and IRRA",
    description:
      "Universal accessibility and autonomous transportation woven into AuraFS.",
    clusters: [
      {
        id: "accessibility",
        name: "IRRA & Universal Accessibility Layer",
        description: "Universal accessibility protocols and interfaces.",
        projects: [
          {
            id: "irra",
            name: "IRRA & Universal Accessibility Layer",
            slug: "irra-universal-access",
            kind: "blueprint",
            shortTagline: "Universal accessibility as a first-class protocol.",
            description:
              "IRRA defines the universal accessibility layer so every human can interface with Aurphyx regardless of ability.",
            githubPath: "aurphyx/irra-universal-access",
          },
        ],
      },
      {
        id: "transport",
        name: "Universal Autonomous Vehicle & Railway Systems",
        description: "AuraFS-powered mobility infrastructure.",
        projects: [
          {
            id: "uav_rail",
            name: "Universal Autonomous Vehicle & Railway Systems",
            slug: "universal-transport",
            kind: "blueprint",
            shortTagline: "AuraFS Meshwerk for vehicles and rail.",
            description:
              "Universal AV & Railway systems using AuraFS Meshwerk nodes, flock cameras, exotic materials, and symbiotic sensing.",
            githubPath: "aurphyx/universal-transport",
          },
        ],
      },
    ],
  },
  {
    id: "mythic_experience",
    name: "Mythic, Games, and Narrative",
    description:
      "Aethornyx, casino, tarot, oracle, and the narrative spine of Aurphyx.",
    clusters: [
      {
        id: "games",
        name: "Games & Worlds",
        description: "AR/VR/MMORPG and playful seeding of shards.",
        projects: [
          {
            id: "aethornyx",
            name: "Aethornyx",
            slug: "aethornyx",
            kind: "blueprint",
            shortTagline: "AR/VR/MMORPG/Real-world game using AuraFS shards.",
            description:
              "Aethornyx is the mythic game layer where AuraFS shards become in-game currency and lived narrative.",
            githubPath: "aurphyx/aethornyx",
          },
          {
            id: "aurphyx_casino",
            name: "Aurphyx Casino",
            slug: "aurphyx-casino",
            kind: "experience",
            shortTagline: "Fun way to seed/drop AuraFS crypto shards.",
            description:
              "Aurphyx Casino is a playful shard-drop environment to seed the Opulence economy and AuraFS crypto.",
            githubPath: "aurphyx/aurphyx-casino",
          },
        ],
      },
      {
        id: "narrative",
        name: "Narrative & Divination",
        description: "Tarot, oracle, and A Divine Love Saga.",
        projects: [
          {
            id: "chaos_bliss_tarot",
            name: "Chaos & Bliss Tarot",
            slug: "chaos-bliss-tarot",
            kind: "experience",
            shortTagline: "Tarot deck, book, calendar, and stories.",
            description:
              "Chaos & Bliss Tarot deck, book, calendar, and short stories as a divination interface into the Duality Kernel.",
            githubPath: "aurphyx/chaos-bliss-tarot",
          },
          {
            id: "aurphyx_oracle",
            name: "Aurphyx Oracle Deck & Book",
            slug: "aurphyx-oracle",
            kind: "experience",
            shortTagline: "Oracle deck for the Aurphyx mythos.",
            description:
              "Aurphyx Oracle deck and book as a mythic interface to the ecosystem.",
            githubPath: "aurphyx/aurphyx-oracle",
          },
          {
            id: "divine_love_saga",
            name: "RF_Lovezme & A Divine Love Saga",
            slug: "a-divine-love-saga",
            kind: "experience",
            shortTagline: "Your life story rewritten as mythic canon.",
            description:
              "A Divine Love Saga: multi-novel, film, and co-authoring project that rewrites life stories in the most positive way.",
            githubPath: "aurphyx/a-divine-love-saga",
          },
          {
            id: "thirteen_month_calendars",
            name: "Aurphyx Thirteen Month Calendars",
            slug: "aurphyx-13-month-calendars",
            kind: "support",
            shortTagline: "Temporal scaffolding for the civilization.",
            description:
              "Thirteen-month calendars that align the Aurphyx ecosystem with its own temporal rhythm.",
            githubPath: "aurphyx/aurphyx-13-month-calendars",
          },
        ],
      },
    ],
  },
  {
    id: "symbiotic_universal",
    name: "Symbiotic & Universal Channels/Layers/Nodes",
    description:
      "The meta-layer that ties channels, layers, and nodes into a universal grammar.",
    clusters: [
      {
        id: "symbiotic_universal_cluster",
        name: "Symbiotic & Universal",
        description: "Meta-spec for channels, layers, and nodes.",
        projects: [
          {
            id: "symbiotic_universal",
            name: "Symbiotic & Universal: Channels | Layers | Nodes",
            slug: "symbiotic-universal",
            kind: "support",
            shortTagline: "Meta-spec for how everything talks to everything.",
            description:
              "Defines the universal grammar for channels, layers, and nodes across the Aurphyx ecosystem.",
            githubPath: "aurphyx/symbiotic-universal",
          },
        ],
      },
    ],
  },
];

// -----------------------------------------------------------------------------
// UI Helpers
// -----------------------------------------------------------------------------

type FilterMode = "all" | "core" | "blueprint";

const kindLabel: Record<ProjectKind, string> = {
  core: "Core",
  blueprint: "Blueprint",
  support: "Support",
  experience: "Experience",
};

const kindColor: Record<ProjectKind, string> = {
  core: "#ff6b6b",
  blueprint: "#4dabf7",
  support: "#ffd43b",
  experience: "#51cf66",
};

// -----------------------------------------------------------------------------
// Component
// -----------------------------------------------------------------------------

export const Aurphyx_Taxonomy_Explorer: React.FC = () => {
  const [selectedProjectId, setSelectedProjectId] = useState<string | null>(null);
  const [filterMode, setFilterMode] = useState<FilterMode>("all");
  const [search, setSearch] = useState<string>("");

  const allProjects: Project[] = useMemo(
    () =>
      AURPHYX_TAXONOMY.flatMap((d) =>
        d.clusters.flatMap((c) => c.projects)
      ),
    []
  );

  const filteredProjects = useMemo(() => {
    return allProjects.filter((p) => {
      if (filterMode === "core" && p.kind !== "core") return false;
      if (filterMode === "blueprint" && p.kind !== "blueprint") return false;

      if (search.trim().length > 0) {
        const q = search.toLowerCase();
        const haystack =
          `${p.name} ${p.slug} ${p.description} ${p.shortTagline}`.toLowerCase();
        if (!haystack.includes(q)) return false;
      }

      return true;
    });
  }, [allProjects, filterMode, search]);

  const selectedProject: Project | undefined = useMemo(
    () => allProjects.find((p) => p.id === selectedProjectId) ?? filteredProjects[0],
    [allProjects, filteredProjects, selectedProjectId]
  );

  // Ensure something is selected; sync when selected project is filtered out
  React.useEffect(() => {
    const selectedInFiltered = filteredProjects.some((p) => p.id === selectedProjectId);
    if (filteredProjects.length > 0 && !selectedInFiltered) {
      setSelectedProjectId(filteredProjects[0].id);
    } else if (!selectedProjectId && filteredProjects.length > 0) {
      setSelectedProjectId(filteredProjects[0].id);
    }
  }, [filteredProjects, selectedProjectId]);

  return (
    <div
      style={{
        display: "flex",
        height: "100vh",
        fontFamily: "system-ui, -apple-system, BlinkMacSystemFont, sans-serif",
        background: "#050816",
        color: "#f8f9fa",
      }}
    >
      {/* Left: Controls + Project List */}
      <div
        style={{
          width: "360px",
          borderRight: "1px solid rgba(255,255,255,0.08)",
          display: "flex",
          flexDirection: "column",
        }}
      >
        <div style={{ padding: "16px", borderBottom: "1px solid rgba(255,255,255,0.08)" }}>
          <div style={{ fontSize: "18px", fontWeight: 700, marginBottom: 4 }}>
            Aurphyx Taxonomy Explorer
          </div>
          <div style={{ fontSize: "12px", opacity: 0.7 }}>
            Repos as civilization blueprints, codices, and cores.
          </div>
        </div>

        <div style={{ padding: "12px 16px", borderBottom: "1px solid rgba(255,255,255,0.08)" }}>
          <input
            type="text"
            placeholder="Search projects…"
            value={search}
            onChange={(e) => setSearch(e.target.value)}
            style={{
              width: "100%",
              padding: "6px 8px",
              borderRadius: 4,
              border: "1px solid rgba(255,255,255,0.2)",
              background: "rgba(0,0,0,0.4)",
              color: "#f8f9fa",
              fontSize: 12,
            }}
          />
          <div style={{ marginTop: 8, display: "flex", gap: 6 }}>
            <FilterButton
              label="All"
              active={filterMode === "all"}
              onClick={() => setFilterMode("all")}
            />
            <FilterButton
              label="Core"
              active={filterMode === "core"}
              onClick={() => setFilterMode("core")}
            />
            <FilterButton
              label="Blueprints"
              active={filterMode === "blueprint"}
              onClick={() => setFilterMode("blueprint")}
            />
          </div>
        </div>

        <div style={{ flex: 1, overflowY: "auto" }}>
          {filteredProjects.length === 0 ? (
            <div style={{ padding: 16, fontSize: 12, opacity: 0.7 }}>
              No projects match your filters yet.
            </div>
          ) : (
            filteredProjects.map((p) => (
              <button
                key={p.id}
                onClick={() => setSelectedProjectId(p.id)}
                style={{
                  width: "100%",
                  textAlign: "left",
                  padding: "10px 14px",
                  border: "none",
                  borderBottom: "1px solid rgba(255,255,255,0.06)",
                  background:
                    selectedProject?.id === p.id
                      ? "linear-gradient(90deg, rgba(80,250,123,0.18), rgba(0,0,0,0))"
                      : "transparent",
                  cursor: "pointer",
                  color: "#f8f9fa",
                }}
              >
                <div style={{ display: "flex", justifyContent: "space-between" }}>
                  <span style={{ fontSize: 13, fontWeight: 600 }}>{p.name}</span>
                  <span
                    style={{
                      fontSize: 10,
                      padding: "2px 6px",
                      borderRadius: 999,
                      background: kindColor[p.kind],
                      color: "#000",
                      fontWeight: 700,
                    }}
                  >
                    {kindLabel[p.kind]}
                  </span>
                </div>
                <div style={{ fontSize: 11, opacity: 0.7, marginTop: 2 }}>
                  {p.shortTagline}
                </div>
                <div style={{ fontSize: 10, opacity: 0.5, marginTop: 2 }}>
                  {p.githubPath}
                </div>
              </button>
            ))
          )}
        </div>
      </div>

      {/* Right: Detail View */}
      <div style={{ flex: 1, display: "flex", flexDirection: "column" }}>
        {selectedProject ? (
          <>
            <div
              style={{
                padding: "16px 20px",
                borderBottom: "1px solid rgba(255,255,255,0.08)",
                display: "flex",
                justifyContent: "space-between",
                alignItems: "center",
              }}
            >
              <div>
                <div style={{ fontSize: 20, fontWeight: 700 }}>{selectedProject.name}</div>
                <div style={{ fontSize: 12, opacity: 0.7, marginTop: 2 }}>
                  {selectedProject.description}
                </div>
                {selectedProject.notes && (
                  <div
                    style={{
                      fontSize: 11,
                      opacity: 0.8,
                      marginTop: 6,
                      fontStyle: "italic",
                    }}
                  >
                    {selectedProject.notes}
                  </div>
                )}
              </div>
              <div style={{ textAlign: "right" }}>
                <div
                  style={{
                    fontSize: 11,
                    padding: "3px 8px",
                    borderRadius: 999,
                    background: kindColor[selectedProject.kind],
                    color: "#000",
                    fontWeight: 700,
                    display: "inline-block",
                  }}
                >
                  {kindLabel[selectedProject.kind]}
                </div>
                <div style={{ fontSize: 11, opacity: 0.7, marginTop: 6 }}>
                  Repo:{" "}
                  <code style={{ fontSize: 11 }}>
                    {selectedProject.githubPath}
                  </code>
                </div>
              </div>
            </div>

            <div style={{ flex: 1, display: "flex", overflow: "hidden" }}>
              {/* Files */}
              <div
                style={{
                  width: "50%",
                  borderRight: "1px solid rgba(255,255,255,0.08)",
                  padding: "14px 18px",
                  overflowY: "auto",
                }}
              >
                <div style={{ fontSize: 13, fontWeight: 600, marginBottom: 8 }}>
                  Canonical Repo Files
                </div>
                <div style={{ fontSize: 11, opacity: 0.7, marginBottom: 10 }}>
                  Every Aurphyx repo—core, blueprint, or experience—should carry this codex
                  scaffold.
                </div>
                <ul style={{ listStyle: "none", padding: 0, margin: 0 }}>
                  {REPO_FILE_TEMPLATE.map((f) => (
                    <li
                      key={f.id}
                      style={{
                        marginBottom: 8,
                        padding: "6px 8px",
                        borderRadius: 4,
                        background: "rgba(255,255,255,0.03)",
                      }}
                    >
                      <div
                        style={{
                          display: "flex",
                          justifyContent: "space-between",
                          alignItems: "center",
                        }}
                      >
                        <span style={{ fontSize: 12, fontWeight: 600 }}>
                          {f.filename}
                        </span>
                        {f.required && (
                          <span
                            style={{
                              fontSize: 9,
                              padding: "2px 6px",
                              borderRadius: 999,
                              background: "#ff922b",
                              color: "#000",
                              fontWeight: 700,
                            }}
                          >
                            REQUIRED
                          </span>
                        )}
                      </div>
                      <div style={{ fontSize: 11, opacity: 0.7, marginTop: 2 }}>
                        {f.description}
                      </div>
                    </li>
                  ))}
                </ul>
              </div>

              {/* Taxonomy Context */}
              <div
                style={{
                  flex: 1,
                  padding: "14px 18px",
                  overflowY: "auto",
                }}
              >
                <div style={{ fontSize: 13, fontWeight: 600, marginBottom: 8 }}>
                  Taxonomy Context
                </div>
                <TaxonomyContextView project={selectedProject} />
              </div>
            </div>
          </>
        ) : (
          <div style={{ padding: 24, fontSize: 14 }}>No project selected.</div>
        )}
      </div>
    </div>
  );
};

// -----------------------------------------------------------------------------
// Subcomponents
// -----------------------------------------------------------------------------

interface FilterButtonProps {
  label: string;
  active: boolean;
  onClick: () => void;
}

const FilterButton: React.FC<FilterButtonProps> = ({ label, active, onClick }) => {
  return (
    <button
      onClick={onClick}
      style={{
        flex: 1,
        padding: "4px 6px",
        fontSize: 11,
        borderRadius: 999,
        border: "1px solid rgba(255,255,255,0.3)",
        background: active ? "rgba(80,250,123,0.2)" : "transparent",
        color: "#f8f9fa",
        cursor: "pointer",
      }}
    >
      {label}
    </button>
  );
};

interface TaxonomyContextViewProps {
  project: Project;
}

const TaxonomyContextView: React.FC<TaxonomyContextViewProps> = ({ project }) => {
  // Find domain + cluster for this project
  const context = useMemo(() => {
    for (const d of AURPHYX_TAXONOMY) {
      for (const c of d.clusters) {
        if (c.projects.some((p) => p.id === project.id)) {
          return { domain: d, cluster: c };
        }
      }
    }
    return null;
  }, [project]);

  if (!context) {
    return (
      <div style={{ fontSize: 12, opacity: 0.7 }}>
        Project not found in taxonomy (this should not happen).
      </div>
    );
  }

  const { domain, cluster } = context;

  return (
    <div style={{ fontSize: 12 }}>
      <div style={{ marginBottom: 10 }}>
        <div style={{ opacity: 0.7, fontSize: 11 }}>Domain</div>
        <div style={{ fontWeight: 600 }}>{domain.name}</div>
        <div style={{ opacity: 0.7, marginTop: 2 }}>{domain.description}</div>
      </div>

      <div style={{ marginBottom: 10 }}>
        <div style={{ opacity: 0.7, fontSize: 11 }}>Cluster</div>
        <div style={{ fontWeight: 600 }}>{cluster.name}</div>
        <div style={{ opacity: 0.7, marginTop: 2 }}>{cluster.description}</div>
      </div>

      <div style={{ marginTop: 12 }}>
        <div style={{ opacity: 0.7, fontSize: 11, marginBottom: 4 }}>
          Sibling Projects in this Cluster
        </div>
        <ul style={{ listStyle: "none", padding: 0, margin: 0 }}>
          {cluster.projects.map((p) => (
            <li
              key={p.id}
              style={{
                marginBottom: 6,
                padding: "4px 6px",
                borderRadius: 4,
                background:
                  p.id === project.id
                    ? "rgba(80,250,123,0.16)"
                    : "rgba(255,255,255,0.03)",
              }}
            >
              <div style={{ display: "flex", justifyContent: "space-between" }}>
                <span>{p.name}</span>
                <span
                  style={{
                    fontSize: 9,
                    padding: "1px 5px",
                    borderRadius: 999,
                    background: kindColor[p.kind],
                    color: "#000",
                    fontWeight: 700,
                  }}
                >
                  {kindLabel[p.kind]}
                </span>
              </div>
              <div style={{ fontSize: 11, opacity: 0.7 }}>{p.shortTagline}</div>
            </li>
          ))}
        </ul>
      </div>
    </div>
  );
};

export default Aurphyx_Taxonomy_Explorer;
