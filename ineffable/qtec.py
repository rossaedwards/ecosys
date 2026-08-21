from qiskit import QuantumCircuit, Aer, execute
from qiskit.quantum_info import Statevector, partial_trace, entropy
from qiskit.visualization import plot_histogram
import numpy as np
import matplotlib.pyplot as plt

class QuantumTransactionEngineCore:
    def __init__(self, n_qubits=5):
        self.n_qubits = n_qubits
        self.circuit = QuantumCircuit(n_qubits, n_qubits)  # measured qubits

    def prepare_wallet_superposition(self, qubit_index):
        """Put the wallet qubit into superposition (representing quantum funds)."""
        self.circuit.h(qubit_index)

    def entangle_wallets(self, control, target):
        """Entangle wallets to form a quantum transaction channel."""
        self.circuit.cx(control, target)

    def encode_transaction_phase(self, qubit_index, theta):
        """Phase encoding for transaction metadata or flags."""
        self.circuit.rz(theta, qubit_index)

    def measure_all(self):
        """Measure all qubits to collapse the quantum state."""
        self.circuit.measure(range(self.n_qubits), range(self.n_qubits))

    def run_simulation(self, shots=1024):
        """Simulate the circuit and return measurement counts."""
        backend = Aer.get_backend('qasm_simulator')
        job = execute(self.circuit, backend, shots=shots)
        result = job.result()
        return result.get_counts()

    def compute_entanglement_entropy(self):
        """Compute the entanglement entropy between halves of the system."""
        backend = Aer.get_backend('statevector_simulator')
        job = execute(self.circuit.remove_final_measurements(inplace=False), backend)
        statevec = job.result().get_statevector()
        # Partial trace and entropy for splitting qubits in half
        subsys = partial_trace(statevec, list(range(self.n_qubits//2, self.n_qubits)))
        return entropy(subsys)

    def visualize_results(self, counts):
        """Display a histogram of measurement outcomes."""
        plot_histogram(counts)
        plt.show()

# Example usage
qtec = QuantumTransactionEngineCore(n_qubits=5)
qtec.prepare_wallet_superposition(0)
qtec.entangle_wallets(0, 1)
qtec.entangle_wallets(1, 2)
qtec.encode_transaction_phase(2, np.pi/4)  # example phase
qtec.measure_all()

counts = qtec.run_simulation(shots=1024)
print("QTEC Transaction Results:\n", counts)

entropy = qtec.compute_entanglement_entropy()
print(f"Estimated Entanglement Entropy: {entropy:.4f}")

qtec.visualize_results(counts)