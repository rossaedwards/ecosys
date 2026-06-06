
# Integration Testing Guide
## Quantum Environmental Isolation Modules with AuraOS

### 1. Overview
This guide provides comprehensive testing procedures for integrating environmental isolation modules (acoustic and EMI) with AuraOS and quantum transaction engines.

---

### 2. Test Environment Setup

#### Hardware Requirements
- Vibration sensors (piezoelectric accelerometers)
- EMI sensors (vector network analyzers or field probes)
- Active cancellation actuators (piezoelectric mounts, EM coils)
- FPGA or microcontroller for real-time control
- Test quantum system or simulator

#### Software Requirements
- AuraOS kernel with isolation module drivers
- Python 3.9+ with numpy, scipy, requests
- Qiskit for quantum simulation
- Testing framework (pytest)
- Docker for containerized testing

---

### 3. Unit Tests

#### Test 1: Sensor Data Acquisition
**Objective:** Verify sensor APIs return valid data.

```python
import requests

def test_sensor_data_retrieval():
    response = requests.get('https://auraos.local/api/isolation/sensor/data')
    assert response.status_code == 200
    data = response.json()
    assert 'vibration_level' in data
    assert 'emi_magnitude' in data
    assert isinstance(data['vibration_level'], float)
```

#### Test 2: Noise Cancellation Control
**Objective:** Verify active noise cancellation accepts parameters.

```python
def test_noise_cancellation_control():
    payload = {
        'frequency': 1000,
        'attenuation_db': 20,
        'phase_shift': 180
    }
    response = requests.post(
        'https://auraos.local/api/isolation/control/noise-cancel',
        json=payload
    )
    assert response.status_code == 200
```

#### Test 3: EMI Cancellation Control
**Objective:** Verify EMI cancellation parameter setting.

```python
def test_emi_cancellation_control():
    payload = {
        'frequency': 500000000,
        'amplitude': 0.1,
        'phase_shift': 90
    }
    response = requests.post(
        'https://auraos.local/api/isolation/control/emi-cancel',
        json=payload
    )
    assert response.status_code == 200
```

---

### 4. Integration Tests

#### Test 4: End-to-End Noise Suppression
**Objective:** Measure noise reduction with active cancellation.

**Procedure:**
1. Generate known vibration pattern
2. Enable active cancellation
3. Measure residual vibration
4. Verify reduction > 15 dB

```python
import numpy as np

def test_end_to_end_noise_reduction():
    # Baseline measurement
    baseline_noise = measure_vibration_rms()

    # Enable cancellation
    set_noise_cancellation(freq=1000, atten=20, phase=180)

    # Wait for stabilization
    time.sleep(2)

    # Measure with cancellation
    active_noise = measure_vibration_rms()

    reduction_db = 20 * np.log10(baseline_noise / active_noise)
    assert reduction_db > 15  # At least 15 dB reduction
```

#### Test 5: System Status Monitoring
**Objective:** Verify status endpoint reports correctly.

```python
def test_status_monitoring():
    response = requests.get('https://auraos.local/api/isolation/status')
    assert response.status_code == 200
    status = response.json()
    assert status['acoustic_status'] in ['active', 'inactive', 'error']
    assert status['emi_status'] in ['active', 'inactive', 'error']
```

---

### 5. Performance Tests

#### Test 6: Latency Measurement
**Objective:** Measure API response time for control commands.

```python
import time

def test_api_latency():
    start = time.time()
    response = requests.post(
        'https://auraos.local/api/isolation/control/noise-cancel',
        json={'frequency': 1000, 'attenuation_db': 20, 'phase_shift': 180}
    )
    latency = time.time() - start
    assert latency < 0.1  # Must respond within 100ms
```

#### Test 7: Sustained Load Test
**Objective:** Verify system stability under continuous operation.

```python
def test_sustained_load():
    for i in range(1000):
        set_noise_cancellation(freq=1000+i, atten=20, phase=180)
        if i % 100 == 0:
            status = get_system_status()
            assert status['acoustic_status'] == 'active'
```

---

### 6. Quantum Integration Tests

#### Test 8: Quantum Circuit Execution with Isolation
**Objective:** Verify quantum operations under environmental control.

```python
from qiskit import QuantumCircuit, Aer, execute

def test_quantum_with_isolation():
    # Enable isolation
    set_noise_cancellation(freq=1000, atten=25, phase=180)
    set_emi_cancellation(freq=500e6, amp=0.1, phase=90)

    # Create quantum circuit
    qc = QuantumCircuit(2, 2)
    qc.h(0)
    qc.cx(0, 1)
    qc.measure([0, 1], [0, 1])

    # Execute
    backend = Aer.get_backend('qasm_simulator')
    job = execute(qc, backend, shots=1024)
    result = job.result()
    counts = result.get_counts()

    # Verify expected Bell state distribution
    assert abs(counts.get('00', 0) - 512) < 100
    assert abs(counts.get('11', 0) - 512) < 100
```

---

### 7. Error Handling Tests

#### Test 9: Invalid Parameter Handling
**Objective:** Verify API rejects invalid inputs.

```python
def test_invalid_parameters():
    invalid_payload = {
        'frequency': -1000,  # Negative frequency
        'attenuation_db': 20,
        'phase_shift': 180
    }
    response = requests.post(
        'https://auraos.local/api/isolation/control/noise-cancel',
        json=invalid_payload
    )
    assert response.status_code == 400
```

#### Test 10: Sensor Failure Recovery
**Objective:** Test system behavior when sensors fail.

```python
def test_sensor_failure_recovery():
    # Simulate sensor failure
    simulate_sensor_disconnect('vibration')

    status = get_system_status()
    assert 'sensor_error' in status['error_logs']

    # Verify failsafe activation
    assert status['acoustic_status'] == 'failsafe'
```

---

### 8. Continuous Integration Setup

#### Docker Test Environment
```dockerfile
FROM python:3.9-slim

WORKDIR /app
COPY requirements.txt .
RUN pip install -r requirements.txt

COPY tests/ ./tests/
COPY api_client.py .

CMD ["pytest", "tests/", "-v"]
```

#### requirements.txt
```
pytest==7.4.0
requests==2.31.0
numpy==1.24.3
scipy==1.11.1
qiskit==0.43.0
```

#### Run Tests
```bash
docker build -t isolation-tests .
docker run isolation-tests
```

---

### 9. Test Reporting

Generate test reports with coverage:
```bash
pytest tests/ --cov=api_client --cov-report=html
```

View results in `htmlcov/index.html`
