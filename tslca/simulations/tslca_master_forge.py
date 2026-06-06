"""
tslca_master_forge.py

Orchestrates the Three-Squared-Lattice Cognitive Architecture (TSLCA):
- wires HIF, activation, propagation, stability, continuity, and Balance Manifold
- exposes a single entrypoint for manuscript-grade simulations.
"""

from .hif_field_engine import HIFFieldEngine
from .tsl_activation_engine import TSLActivationEngine
from .tsl_propagation_engine import TSLPropagationEngine
from .tsl_stability_engine import TSLStabilityEngine
from .tsl_continuity_engine import TSLContinuityEngine
from .balance_manifold_engine import BalanceManifoldEngine
from .unified_simulation_engine import UnifiedSimulationEngine


class TSLMasterForge:
    """
    High-level forge that assembles all engines into a coherent simulation stack.
    """

    def __init__(self, config: dict | None = None) -> None:
        self.config = config or {}
        self.hif = HIFFieldEngine(self.config.get("hif", {}))
        self.activation = TSLActivationEngine(self.config.get("activation", {}))
        self.propagation = TSLPropagationEngine(self.config.get("propagation", {}))
        self.stability = TSLStabilityEngine(self.config.get("stability", {}))
        self.continuity = TSLContinuityEngine(self.config.get("continuity", {}))
        self.balance = BalanceManifoldEngine(self.config.get("balance", {}))

        self.sim = UnifiedSimulationEngine(
            hif=self.hif,
            activation=self.activation,
            propagation=self.propagation,
            stability=self.stability,
            continuity=self.continuity,
            balance=self.balance,
            config=self.config.get("simulation", {}),
        )

    def run_lattice_epoch(self, steps: int = 100) -> dict:
        """
        Run a full lattice epoch: activation → propagation → stability → continuity.
        Returns a summary dict suitable for figure generation and logging.
        """
        return self.sim.run(steps=steps)

    def export_manuscript_data(self, out_dir: str) -> None:
        """
        Export data products (HIF fields, stability indices, continuity traces)
        for LaTeX figures and tables.
        """
        self.sim.export(out_dir=out_dir)
