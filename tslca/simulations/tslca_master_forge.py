"""
tslca_master_forge.py

Orchestrates the Three-Squared-Lattice Cognitive Architecture (TSLCA):
- wires HIF, activation, propagation, stability, continuity, and Balance Manifold
- exposes a single entrypoint for manuscript-grade simulations.

Enhancements (v2):
- TSLForgeConfig dataclass for type-safe config injection
- Structured logging with epoch telemetry (epoch_id, elapsed, steps)
- validate() dry-run to verify all engine I/O contracts before full epochs
- run_lattice_epoch() returns _meta block for figure labeling
- sweep() for parameter grid searches across engine configs
- save_checkpoint() / load_checkpoint() for fault-tolerant long runs
- reset_engine() / reset_all() for hot-swapping individual engines
- probe_balance_manifold() for pre/post-run manifold health diagnostics
- CLI entrypoint: python tslca_master_forge.py --config cfg.json --steps N --out ./out
"""

from __future__ import annotations

import argparse
import json
import logging
import pickle
import time
import uuid
from copy import deepcopy
from dataclasses import dataclass, field
from itertools import product
from pathlib import Path
from typing import Any

from .hif_field_engine import HIFFieldEngine
from .tsl_activation_engine import TSLActivationEngine
from .tsl_propagation_engine import TSLPropagationEngine
from .tsl_stability_engine import TSLStabilityEngine
from .tsl_continuity_engine import TSLContinuityEngine
from .balance_manifold_engine import BalanceManifoldEngine
from .unified_simulation_engine import UnifiedSimulationEngine

logging.basicConfig(
    level=logging.INFO,
    format="[TSLForge] %(asctime)s %(levelname)s %(message)s",
    datefmt="%Y-%m-%dT%H:%M:%S",
)
logger = logging.getLogger(__name__)


# ---------------------------------------------------------------------------
# Engine contract error
# ---------------------------------------------------------------------------

class EngineContractError(RuntimeError):
    """Raised when engine I/O shapes or types are incompatible."""


# ---------------------------------------------------------------------------
# Typed config dataclass
# ---------------------------------------------------------------------------

@dataclass
class TSLForgeConfig:
    """
    Typed, IDE-friendly configuration for TSLMasterForge.
    Each field maps to the config dict accepted by its engine.
    """
    hif: dict[str, Any] = field(default_factory=dict)
    activation: dict[str, Any] = field(default_factory=dict)
    propagation: dict[str, Any] = field(default_factory=dict)
    stability: dict[str, Any] = field(default_factory=dict)
    continuity: dict[str, Any] = field(default_factory=dict)
    balance: dict[str, Any] = field(default_factory=dict)
    simulation: dict[str, Any] = field(default_factory=dict)

    @classmethod
    def from_dict(cls, d: dict) -> "TSLForgeConfig":
        return cls(
            hif=d.get("hif", {}),
            activation=d.get("activation", {}),
            propagation=d.get("propagation", {}),
            stability=d.get("stability", {}),
            continuity=d.get("continuity", {}),
            balance=d.get("balance", {}),
            simulation=d.get("simulation", {}),
        )

    @classmethod
    def from_json(cls, path: str | Path) -> "TSLForgeConfig":
        with open(path) as f:
            return cls.from_dict(json.load(f))

    def to_dict(self) -> dict:
        return {
            "hif": self.hif,
            "activation": self.activation,
            "propagation": self.propagation,
            "stability": self.stability,
            "continuity": self.continuity,
            "balance": self.balance,
            "simulation": self.simulation,
        }


# ---------------------------------------------------------------------------
# Master Forge
# ---------------------------------------------------------------------------

class TSLMasterForge:
    """
    High-level forge that assembles all TSLCA engines into a coherent
    simulation stack.

    Accepts either a TSLForgeConfig dataclass or a raw dict (legacy).
    """

    _ENGINE_NAMES = ("hif", "activation", "propagation", "stability", "continuity", "balance")

    def __init__(self, config: TSLForgeConfig | dict | None = None) -> None:
        if isinstance(config, dict):
            config = TSLForgeConfig.from_dict(config)
        self.config: TSLForgeConfig = config or TSLForgeConfig()
        self._build_engines()

    # ------------------------------------------------------------------
    # Internal engine construction
    # ------------------------------------------------------------------

    def _build_engines(self) -> None:
        self.hif = HIFFieldEngine(self.config.hif)
        self.activation = TSLActivationEngine(self.config.activation)
        self.propagation = TSLPropagationEngine(self.config.propagation)
        self.stability = TSLStabilityEngine(self.config.stability)
        self.continuity = TSLContinuityEngine(self.config.continuity)
        self.balance = BalanceManifoldEngine(self.config.balance)
        self.sim = UnifiedSimulationEngine(
            hif=self.hif,
            activation=self.activation,
            propagation=self.propagation,
            stability=self.stability,
            continuity=self.continuity,
            balance=self.balance,
            config=self.config.simulation,
        )
        logger.info("All engines initialized.")

    # ------------------------------------------------------------------
    # Engine lifecycle
    # ------------------------------------------------------------------

    _ENGINE_CLASS_MAP = {
        "hif": (HIFFieldEngine, "hif"),
        "activation": (TSLActivationEngine, "activation"),
        "propagation": (TSLPropagationEngine, "propagation"),
        "stability": (TSLStabilityEngine, "stability"),
        "continuity": (TSLContinuityEngine, "continuity"),
        "balance": (BalanceManifoldEngine, "balance"),
    }

    def reset_engine(self, engine_name: str, config: dict | None = None) -> None:
        """
        Hot-swap a single engine without tearing down the whole forge.

        Args:
            engine_name: One of 'hif', 'activation', 'propagation',
                         'stability', 'continuity', 'balance'.
            config: Optional new config dict for the engine. If None, the
                    engine is reinitialized from the existing forge config.
        """
        if engine_name not in self._ENGINE_CLASS_MAP:
            raise ValueError(f"Unknown engine: {engine_name!r}. "
                             f"Valid: {list(self._ENGINE_CLASS_MAP)}")
        cls, cfg_attr = self._ENGINE_CLASS_MAP[engine_name]
        if config is not None:
            setattr(self.config, cfg_attr, config)
        new_engine = cls(getattr(self.config, cfg_attr))
        setattr(self, engine_name, new_engine)
        # Rewire sim with updated engine reference
        self._rebuild_sim()
        logger.info(f"Engine '{engine_name}' hot-swapped.")

    def reset_all(self) -> None:
        """Full forge reset — reinitializes all engines from current config."""
        self._build_engines()
        logger.info("Full forge reset complete.")

    def _rebuild_sim(self) -> None:
        self.sim = UnifiedSimulationEngine(
            hif=self.hif,
            activation=self.activation,
            propagation=self.propagation,
            stability=self.stability,
            continuity=self.continuity,
            balance=self.balance,
            config=self.config.simulation,
        )

    # ------------------------------------------------------------------
    # Validation
    # ------------------------------------------------------------------

    def validate(self) -> bool:
        """
        Dry-run 1 step to verify all engine I/O contracts are satisfied.

        Returns True on success. Raises EngineContractError on shape/type
        mismatch so issues surface before a long run.
        """
        logger.info("Running validation dry-run (1 step)...")
        try:
            result = self.sim.run(steps=1)
        except Exception as exc:
            raise EngineContractError(
                f"Engine contract violation during 1-step dry-run: {exc}"
            ) from exc
        if not isinstance(result, dict):
            raise EngineContractError(
                f"UnifiedSimulationEngine.run() must return dict; got {type(result)}"
            )
        logger.info("Validation passed.")
        return True

    # ------------------------------------------------------------------
    # Core simulation
    # ------------------------------------------------------------------

    def run_lattice_epoch(
        self,
        steps: int = 100,
        epoch_id: str | None = None,
        validate_first: bool = False,
    ) -> dict:
        """
        Run a full lattice epoch: activation → propagation → stability → continuity.

        Args:
            steps: Number of simulation steps.
            epoch_id: Optional human-readable label (auto-generated if None).
            validate_first: If True, run validate() before the epoch.

        Returns:
            Summary dict with simulation results plus a '_meta' block
            containing epoch_id, steps, elapsed_s — suitable for figure
            generation and LaTeX table labeling.
        """
        epoch_id = epoch_id or str(uuid.uuid4())[:8]
        if validate_first:
            self.validate()
        manifold_pre = self.probe_balance_manifold()
        if manifold_pre.get("equilibrium_distance", 0.0) > 1.0:
            logger.warning(
                f"[epoch={epoch_id}] Balance Manifold equilibrium_distance="
                f"{manifold_pre['equilibrium_distance']:.4f} > 1.0 — "
                "consider resetting before run."
            )
        t0 = time.perf_counter()
        result = self.sim.run(steps=steps)
        elapsed = time.perf_counter() - t0
        result["_meta"] = {
            "epoch_id": epoch_id,
            "steps": steps,
            "elapsed_s": round(elapsed, 6),
            "manifold_pre": manifold_pre,
        }
        logger.info(f"epoch={epoch_id} steps={steps} elapsed={elapsed:.3f}s")
        return result

    # ------------------------------------------------------------------
    # Parameter sweep
    # ------------------------------------------------------------------

    def sweep(
        self,
        param_grid: dict[str, list],
        steps: int = 100,
        reset_between: bool = True,
    ) -> list[dict]:
        """
        Run lattice epochs over a Cartesian grid of config overrides.

        Args:
            param_grid: Dot-notation engine param overrides mapped to lists
                        of values. Example:
                        {"hif.coupling_strength": [0.1, 0.5, 1.0],
                         "activation.threshold": [0.3, 0.7]}
                        Generates 3×2 = 6 runs.
            steps: Steps per epoch.
            reset_between: If True, reset_all() between runs to prevent
                           state contamination.

        Returns:
            List of result dicts, each tagged with the param combination
            used — one entry per grid point.
        """
        keys = list(param_grid.keys())
        values = list(param_grid.values())
        results = []
        for combo in product(*values):
            params = dict(zip(keys, combo))
            # Apply overrides
            for dot_key, val in params.items():
                engine_name, param_name = dot_key.split(".", 1)
                engine_cfg = deepcopy(getattr(self.config, engine_name, {}))
                engine_cfg[param_name] = val
                setattr(self.config, engine_name, engine_cfg)
            if reset_between:
                self.reset_all()
            epoch_id = "_".join(f"{k.split('.')[-1]}={v}" for k, v in params.items())
            result = self.run_lattice_epoch(steps=steps, epoch_id=epoch_id)
            result["_sweep_params"] = params
            results.append(result)
            logger.info(f"Sweep point complete: {params}")
        logger.info(f"Sweep complete: {len(results)} runs.")
        return results

    # ------------------------------------------------------------------
    # Checkpoint / resume
    # ------------------------------------------------------------------

    def save_checkpoint(self, path: str | Path) -> None:
        """
        Serialize forge state (config + engine states) to a pickle checkpoint.
        Stored in the provided path; parent directories are created if needed.
        """
        path = Path(path)
        path.parent.mkdir(parents=True, exist_ok=True)
        state = {
            "config": self.config.to_dict(),
            "sim_state": self.sim.get_state() if hasattr(self.sim, "get_state") else None,
        }
        with open(path, "wb") as f:
            pickle.dump(state, f)
        logger.info(f"Checkpoint saved → {path}")

    def load_checkpoint(self, path: str | Path) -> None:
        """
        Restore forge state from a pickle checkpoint.
        Rebuilds all engines from the serialized config, then restores
        simulation state if available.
        """
        path = Path(path)
        with open(path, "rb") as f:
            state = pickle.load(f)
        self.config = TSLForgeConfig.from_dict(state["config"])
        self._build_engines()
        if state.get("sim_state") is not None and hasattr(self.sim, "set_state"):
            self.sim.set_state(state["sim_state"])
        logger.info(f"Checkpoint loaded ← {path}")

    # ------------------------------------------------------------------
    # Balance Manifold health probe
    # ------------------------------------------------------------------

    def probe_balance_manifold(self) -> dict:
        """
        Query current Balance Manifold state.

        Returns a dict with at minimum:
            symmetry_index       — float, 1.0 = perfect symmetry
            tension_gradient     — float, magnitude of field tension
            equilibrium_distance — float, distance from equilibrium basin

        If BalanceManifoldEngine does not implement get_state_report(),
        returns a placeholder dict so the rest of the pipeline is not blocked.
        """
        if hasattr(self.balance, "get_state_report"):
            return self.balance.get_state_report()
        logger.warning(
            "BalanceManifoldEngine.get_state_report() not implemented — "
            "returning placeholder."
        )
        return {
            "symmetry_index": None,
            "tension_gradient": None,
            "equilibrium_distance": 0.0,
        }

    # ------------------------------------------------------------------
    # Export
    # ------------------------------------------------------------------

    def export_manuscript_data(self, out_dir: str | Path) -> None:
        """
        Export data products (HIF fields, stability indices, continuity traces)
        for LaTeX figures and tables.

        Args:
            out_dir: Output directory path. Created if it does not exist.
        """
        out_dir = Path(out_dir)
        out_dir.mkdir(parents=True, exist_ok=True)
        self.sim.export(out_dir=str(out_dir))
        logger.info(f"Manuscript data exported → {out_dir}")


# ---------------------------------------------------------------------------
# CLI entrypoint
# ---------------------------------------------------------------------------

if __name__ == "__main__":
    parser = argparse.ArgumentParser(
        description="TSLCA Master Forge — manuscript-grade simulation CLI",
        formatter_class=argparse.ArgumentDefaultsHelpFormatter,
    )
    parser.add_argument("--config", type=str, default=None,
                        help="Path to JSON config file (TSLForgeConfig)")
    parser.add_argument("--steps", type=int, default=100,
                        help="Number of simulation steps per epoch")
    parser.add_argument("--out", type=str, default="./output",
                        help="Output directory for manuscript data export")
    parser.add_argument("--epoch-id", type=str, default=None,
                        help="Optional human-readable epoch label")
    parser.add_argument("--validate", action="store_true",
                        help="Run I/O contract validation before epoch")
    parser.add_argument("--checkpoint-save", type=str, default=None,
                        help="Save checkpoint to this path after run")
    parser.add_argument("--checkpoint-load", type=str, default=None,
                        help="Load checkpoint from this path before run")
    parser.add_argument("--probe-manifold", action="store_true",
                        help="Print Balance Manifold health report and exit")
    args = parser.parse_args()

    cfg = TSLForgeConfig.from_json(args.config) if args.config else TSLForgeConfig()
    forge = TSLMasterForge(config=cfg)

    if args.checkpoint_load:
        forge.load_checkpoint(args.checkpoint_load)

    if args.probe_manifold:
        report = forge.probe_balance_manifold()
        print(json.dumps(report, indent=2))
        raise SystemExit(0)

    result = forge.run_lattice_epoch(
        steps=args.steps,
        epoch_id=args.epoch_id,
        validate_first=args.validate,
    )
    forge.export_manuscript_data(out_dir=args.out)

    if args.checkpoint_save:
        forge.save_checkpoint(args.checkpoint_save)

    print(json.dumps(result, indent=2, default=str))
