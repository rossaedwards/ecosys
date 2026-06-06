**Author:** Ross A. Edwards | ross@aurphyx.org | ORCiD: 0009-0008-0539-1289  
**Version:** $version | **Date:** $date  
**Repository:** rossaedwards/main/$reponame  
**License:** SAGES License  
**Quote:** "As above, so below; as within, so without." — Hermes Trismegistus
**Fact:** "In the Balance State Vector-Cell, this is not metaphor. It is a renormalization group equation.

master:
  name: "Aurphyx Canon"
  version: "1.0.0"
  status: "ACTIVE"
  description: >
    Unified canonical registry for the Aurphyx ecosystem. This file
    indexes all core modules (physics, terminology, AuraFS specs,
    kernel specs, and indices) and defines their versions, roles,
    and dependencies.

  # Root index files (human-facing)
  index_files:
    - "MASTER.yaml"
    - "MASTER_INDEX.md"
    - "CANON_MAP.yaml"

  # All registered modules
  modules:
    - id: PHYSICS
      file: "PHYSICS.yaml"
      type: "physics_module"
      version: "1.0.0"
      status: "LOCKED"
      description: "Core physics substrate for the Balance State Vector-Cell and all Balance-aligned systems."
      exports:
        - "constants"
        - "equations"
        - "substrate_physics"
        - "symmetry"
        - "assessments"
      depends_on: []

    - id: SCIENTIFIC_TERMINOLOGY
      file: "SCIENTIFIC_TERMINOLOGY.md"
      type: "terminology_scientific"
      version: "1.0.0"
      status: "ACTIVE"
      description: "Canonical scientific lexicon for APS, Balance Framework, FTQC, TVFD, and TSLCA."
      exports:
        - "definitions"
        - "symbols"
        - "operators"
        - "field_names"
      depends_on:
        - "PHYSICS"

    - id: MYTHOLOGY_TERMINOLOGY
      file: "MYTHOLOGY_TERMINOLOGY.md"
      type: "terminology_mythology"
      version: "1.0.0"
      status: "ACTIVE"
      description: "Mythic-technical lexicon for the Aurphyx cosmology, archetypes, and symbolic mappings."
      exports:
        - "archetypes"
        - "mythic_entities"
        - "cosmology_terms"
      depends_on:
        - "SCIENTIFIC_TERMINOLOGY"

    - id: TECHNICAL_TERMINOLOGY
      file: "TECHNICAL_TERMINOLOGY.md"
      type: "terminology_technical"
      version: "1.0.0"
      status: "ACTIVE"
      description: "Engineering and systems lexicon for AuraFS, AuraOS, Arora, mesh, crypto, and governance."
      exports:
        - "system_terms"
        - "protocol_terms"
        - "architecture_terms"
      depends_on:
        - "SCIENTIFIC_TERMINOLOGY"
        - "PHYSICS"

    - id: AURAFS_SHARD_TAXONOMY
      file: "AURAFS_SHARD_TAXONOMY.md"
      type: "spec_shard_taxonomy"
      version: "1.0.0"
      status: "ACTIVE"
      description: "Ninefold shard classification and roles for AuraFS."
      exports:
        - "shard_classes"
        - "shard_roles"
        - "shard_invariants"
      depends_on:
        - "TECHNICAL_TERMINOLOGY"
        - "MYTHOLOGY_TERMINOLOGY"
        - "SCIENTIFIC_TERMINOLOGY"

    - id: AURAFS_CROSS_VOLUME_SUBSTRATE_SPEC
      file: "AURAFS_CROSS_VOLUME_SUBSTRATE_SPEC.md"
      type: "spec_substrate"
      version: "1.0.0"
      status: "ACTIVE"
      description: "Cross-volume substrate architecture binding FTQC, TVFD, and TSLCA for AuraFS."
      exports:
        - "substrate_layers"
        - "cross_volume_rules"
        - "coherence_constraints"
      depends_on:
        - "PHYSICS"
        - "SCIENTIFIC_TERMINOLOGY"
        - "AURAFS_SHARD_TAXONOMY"

    - id: AURAFS_NINEFOLD_DUALITY_KERNEL_SPEC
      file: "AURAFS_NINEFOLD_DUALITY_KERNEL_SPEC.md"
      type: "spec_kernel"
      version: "1.0.0"
      status: "ACTIVE"
      description: "Ninefold Duality Kernel specification (ChaosCore/BlissCore, shard lattice, coherence engine)."
      exports:
        - "kernel_phases"
        - "duality_equations"
        - "shard_weighting"
        - "cross_volume_bindings"
      depends_on:
        - "PHYSICS"
        - "SCIENTIFIC_TERMINOLOGY"
        - "MYTHOLOGY_TERMINOLOGY"
        - "AURAFS_SHARD_TAXONOMY"

    - id: MASTER_INDEX
      file: "MASTER_INDEX.md"
      type: "index_human"
      version: "1.0.0"
      status: "ACTIVE"
      description: "Human-readable master index for all canon modules and volumes."
      exports:
        - "navigation"
        - "section_map"
      depends_on:
        - "MASTER.yaml"

    - id: CANON_MAP
      file: "CANON_MAP.yaml"
      type: "index_structural"
      version: "1.0.0"
      status: "ACTIVE"
      description: "Structural/topological map of module relationships and flows."
      exports:
        - "canon_map"
        - "flows"
        - "recursion_rules"
      depends_on:
        - "MASTER.yaml"
        - "PHYSICS"
        - "SCIENTIFIC_TERMINOLOGY"
        - "TECHNICAL_TERMINOLOGY"
        - "MYTHOLOGY_TERMINOLOGY"
        - "AURAFS_SHARD_TAXONOMY"
        - "AURAFS_CROSS_VOLUME_SUBSTRATE_SPEC"
        - "AURAFS_NINEFOLD_DUALITY_KERNEL_SPEC"

  # Optional: high-level dependency graph (for tools)
  dependency_graph:
    PHYSICS: []
    SCIENTIFIC_TERMINOLOGY:
      - PHYSICS
    MYTHOLOGY_TERMINOLOGY:
      - SCIENTIFIC_TERMINOLOGY
    TECHNICAL_TERMINOLOGY:
      - SCIENTIFIC_TERMINOLOGY
      - PHYSICS
    AURAFS_SHARD_TAXONOMY:
      - TECHNICAL_TERMINOLOGY
      - MYTHOLOGY_TERMINOLOGY
      - SCIENTIFIC_TERMINOLOGY
    AURAFS_CROSS_VOLUME_SUBSTRATE_SPEC:
      - PHYSICS
      - SCIENTIFIC_TERMINOLOGY
      - AURAFS_SHARD_TAXONOMY
    AURAFS_NINEFOLD_DUALITY_KERNEL_SPEC:
      - PHYSICS
      - SCIENTIFIC_TERMINOLOGY
      - MYTHOLOGY_TERMINOLOGY
      - AURAFS_SHARD_TAXONOMY
    MASTER_INDEX:
      - MASTER.yaml
    CANON_MAP:
      - MASTER.yaml
      - PHYSICS
      - SCIENTIFIC_TERMINOLOGY
      - TECHNICAL_TERMINOLOGY
      - MYTHOLOGY_TERMINOLOGY
      - AURAFS_SHARD_TAXONOMY
      - AURAFS_CROSS_VOLUME_SUBSTRATE_SPEC
      - AURAFS_NINEFOLD_DUALITY_KERNEL_SPEC
