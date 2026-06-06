# =============================================================
#  Aurphyx Canon Map — Cross‑Module Structural Topology
#  Version: 1.0.0
#  Status: ACTIVE — Expandable after page creation
# =============================================================

canon_map:
  version: 1.0.0
  description: >
    High‑level structural map of the Aurphyx Canon. This file defines
    the topological relationships, dependency flows, recursion layers,
    and cross‑volume substrate bindings across all modules.

  # -----------------------------------------------------------
  #  ROOT LAYER — MASTER CANON
  # -----------------------------------------------------------
  root:
    MASTER.yaml:
      role: root_index
      exports:
        - MASTER_INDEX.md
        - CANON_MAP.yaml
        - SCIENTIFIC_TERMINOLOGY.md
        - TECHNICAL_TERMINOLOGY.md
        - MYTHOLOGY_TERMINOLOGY.md
        - PHYSICS.yaml
        - AURAFS_SHARD_TAXONOMY.md
        - AURAFS_CROSS_VOLUME_SUBSTRATE_SPEC.md
        - AURAFS_NINEFOLD_DUALITY_KERNEL_SPEC.md
      depends_on: []
      provides:
        - canonical_structure
        - cross_module_registry
        - dependency_graph

  # -----------------------------------------------------------
  #  TERMINOLOGY LAYER
  # -----------------------------------------------------------
  terminology:
    SCIENTIFIC_TERMINOLOGY.md:
      role: scientific_lexicon
      depends_on: []
      provides:
        - physics_terms
        - mathematical_terms
        - substrate_terms

    TECHNICAL_TERMINOLOGY.md:
      role: engineering_lexicon
      depends_on: []
      provides:
        - systems_terms
        - protocol_terms
        - architecture_terms

    MYTHOLOGY_TERMINOLOGY.md:
      role: mythic_lexicon
      depends_on: []
      provides:
        - symbolic_terms
        - narrative_terms
        - archetypal_terms

  # -----------------------------------------------------------
  #  PHYSICS LAYER
  # -----------------------------------------------------------
  physics:
    PHYSICS.yaml:
      role: physics_core
      depends_on:
        - SCIENTIFIC_TERMINOLOGY.md
      provides:
        - locked_constants
        - fractal_geometry
        - psk_governor
        - floquet_drive
        - rg_flow
        - substrate_physics

  # -----------------------------------------------------------
  #  AURAFS LAYER
  # -----------------------------------------------------------
  aurafs:
    AURAFS_SHARD_TAXONOMY.md:
      role: shard_classification
      depends_on:
        - TECHNICAL_TERMINOLOGY.md
        - MYTHOLOGY_TERMINOLOGY.md
      provides:
        - shard_types
        - shard_roles
        - ninefold_alignment

    AURAFS_CROSS_VOLUME_SUBSTRATE_SPEC.md:
      role: substrate_spec
      depends_on:
        - PHYSICS.yaml
        - SCIENTIFIC_TERMINOLOGY.md
      provides:
        - cross_volume_rules
        - substrate_interfaces
        - FTQC_TSLCA_TVFD_bindings

    AURAFS_NINEFOLD_DUALITY_KERNEL_SPEC.md:
      role: duality_kernel
      depends_on:
        - AURAFS_SHARD_TAXONOMY.md
        - MYTHOLOGY_TERMINOLOGY.md
        - PHYSICS.yaml
      provides:
        - chaoscore
        - blisscore
        - duality_equations
        - ninefold_kernel_logic

  # -----------------------------------------------------------
  #  CROSS‑MODULE FLOWS
  # -----------------------------------------------------------
  flows:
    - name: terminology_to_physics
      from: [SCIENTIFIC_TERMINOLOGY.md]
      to: [PHYSICS.yaml]
      type: semantic_dependency

    - name: physics_to_substrate
      from: [PHYSICS.yaml]
      to: [AURAFS_CROSS_VOLUME_SUBSTRATE_SPEC.md]
      type: physical_constraint

    - name: mythology_to_shards
      from: [MYTHOLOGY_TERMINOLOGY.md]
      to: [AURAFS_SHARD_TAXONOMY.md]
      type: symbolic_alignment

    - name: shards_to_duality_kernel
      from: [AURAFS_SHARD_TAXONOMY.md]
      to: [AURAFS_NINEFOLD_DUALITY_KERNEL_SPEC.md]
      type: structural_dependency

    - name: physics_to_duality_kernel
      from: [PHYSICS.yaml]
      to: [AURAFS_NINEFOLD_DUALITY_KERNEL_SPEC.md]
      type: mathematical_dependency

  # -----------------------------------------------------------
  #  FRACTAL RECURSION LAYER
  # -----------------------------------------------------------
  recursion:
    depth: 3
    rule: >
      Each module may recursively expand into submodules following the
      Balance Canon Recursion Law: M → {M.core, M.syntax, M.semantics, M.bindings}.
    applies_to:
      - MASTER.yaml
      - PHYSICS.yaml
      - AURAFS_CROSS_VOLUME_SUBSTRATE_SPEC.md
      - AURAFS_NINEFOLD_DUALITY_KERNEL_SPEC.md

  # -----------------------------------------------------------
  #  EXPORTS
  # -----------------------------------------------------------
  exports:
    - MASTER.yaml
    - MASTER_INDEX.md
    - CANON_MAP.yaml
    - PHYSICS.yaml
    - AURAFS_SHARD_TAXONOMY.md
    - AURAFS_CROSS_VOLUME_SUBSTRATE_SPEC.md
    - AURAFS_NINEFOLD_DUALITY_KERNEL_SPEC.md
    - SCIENTIFIC_TERMINOLOGY.md
    - TECHNICAL_TERMINOLOGY.md
    - MYTHOLOGY_TERMINOLOGY.md
