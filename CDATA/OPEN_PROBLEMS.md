# CDATA — Open Problems and Falsification Tests

**Date:** 2026‑04‑25 (updated)  
**Status:** Current for Longevity Impetus submission (Sobol paradox resolved).

[Same structure as v5.2 – with the following updates:]

### Problem OP3: Dominance of the epigenetic parameter in Sobol analysis

**2026‑04‑25 Status: RESOLVED (theoretically) – awaiting Cell‑DT v4.0 validation.**

The coupling model (`ep_age(t) = ep_rate_base × t + k_ep × ∫D dτ`) has been derived analytically and shown to eliminate the paradox in the simplified analytic model. The full ODE implementation (Cell‑DT v4.0) will be completed and validated in Aim 2 of this proposal (see TEAM_AND_BUDGET.md). The test criteria for FT3.1 remain, but the expected outcome (Confirmation) is now supported by the analytical result.

**Test FT3.1 (update):** Full ODE Sobol with coupling implemented. Expected: S1(alpha) > 0.3, S1(ep_rate_base) < 0.15.

### Problem OP1, OP2, OP4, OP5, OP6 – unchanged.

---

## Pre‑registration Plan and Sample Size Calculation

[Updated in EVIDENCE.md §4 and §6. The plan includes formal power analysis for FT1.1 and FT1.2, and a risk matrix with six rows.]
