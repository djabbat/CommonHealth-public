# DESIGN — ImagingControl

## Architecture
PyMMCore-Plus daemon → 2× sCMOS via USB3 → ROI extraction → AICoordinator (Python+Rust) → command queue → device drivers.

## Latency budget
< 200 ms frame-to-decision-to-action for adaptive imaging.

## Replication strategy
Software stack: docker-free Python venv + Rust bins; reproducible via requirements.txt + Cargo.lock.

---
