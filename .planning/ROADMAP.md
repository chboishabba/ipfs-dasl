# Roadmap: DASL Fuzz Testing Grant (RFP #2026-01)

## Overview

Define scope and targets, submit a compliant proposal, and execute a focused delivery plan that maximizes grant competitiveness and produces the required outputs (corpus + infrastructure + cross-impl validation + spec feedback).

## Domain Expertise

- None

## Phases

- [ ] **Phase 1: Requirements & Targets** - Confirm RFP requirements and select candidate implementations.
- [ ] **Phase 2: Proposal Draft** - Draft a competitive 2-page proposal with required sections and budget/schedule.
- [ ] **Phase 3: Delivery Plan (Grant-Ready)** - Concrete milestones, outputs, and risks that align with evaluation criteria.
- [ ] **Phase 4: Fuzzing MVP (Contingent on Award)** - Build minimal fuzzing harness and initial corpus.
- [ ] **Phase 5: Cross-Impl Validation & Spec Feedback (Contingent on Award)** - Expand corpus, run differential tests, report findings to spec editors.
- [ ] **Phase 6: Runner Implementation (Execution)** - Implement Rust driver, CLI adapters, and minimal corpus runner.

## Phase Details

### Phase 1: Requirements & Targets
**Goal**: Locked list of requirements and test targets aligned to RFP.
**Depends on**: Nothing (first phase)
**Research**: Likely (identify ATProto-based implementations and maintainers)
**Plans**: 2 plans

Plans:
- [ ] 01-01: Extract and codify RFP requirements into planning docs.
- [ ] 01-02: Select two target implementations (including one ATProto-based) prioritizing installed stacks (JS/TS, Go, Rust, Python), then contact maintainers if needed.
  - Targeted implementations: `go-dasl` and `atproto-dasl`.
  - Use package-manager availability as “installed” (no local repo required).
  - If not maintainers, open issues or reach out for review commitment and link evidence.
  - Include CIDs and CAR in proposal scope.

### Phase 2: Proposal Draft
**Goal**: Submit a compliant, competitive 2-page proposal by March 15, 2026.
**Depends on**: Phase 1
**Research**: Likely (confirm tech design and feasibility)
**Research topics**: fuzzing approaches, existing dasl-testing reuse
**Plans**: 2 plans

Plans:
- [ ] 02-01: Draft required proposal sections (overview, design, adoption, schedule/budget, qualifications, additional info).
- [ ] 02-02: Review for compliance (license, duration, maintainer approvals, Q&A availability).

### Phase 3: Delivery Plan (Grant-Ready)
**Goal**: A concrete delivery plan that maximizes grant competitiveness and de-risks execution.
**Depends on**: Phase 2
**Research**: Likely (evaluation criteria, prior award patterns)
**Plans**: 2 plans

Plans:
- [ ] 03-01: Define measurable outputs, artifacts, and acceptance criteria aligned to the RFP.
- [ ] 03-02: Define delivery milestones (Month 1-3) with risks and mitigations.

### Phase 4: Fuzzing MVP (Contingent on Award)
**Goal**: Minimal fuzzing harness + seed corpus with DRISL1 focus.
**Depends on**: Phase 3
**Research**: Likely (implementation-specific fuzz hooks)
**Research topics**: DRISL1 serialization boundaries, crash-safe harnessing
**Plans**: TBD

### Phase 5: Cross-Impl Validation & Spec Feedback (Contingent on Award)
**Goal**: Differential testing across implementations and spec feedback loop.
**Depends on**: Phase 4
**Research**: Likely (canonicalization, output normalization)
**Plans**: TBD

### Phase 6: Runner Implementation (Execution)
**Goal**: Runnable Rust driver that executes both targets and emits output records.
**Depends on**: Phase 4
**Research**: Likely (target CLI integration)
**Plans**: 2 plans

Plans:
- [ ] 06-01: Define repo layout and bootstrap Rust runner CLI.
- [ ] 06-02: Implement target adapters (Go + Rust) and emit output schema.
- [ ] 06-03: Add format-aware parsing (DRISL1/CID/CAR), idempotence checks, and bounded fuzzing.
- [ ] 06-04: Triage CID validity divergences and add regression witnesses.
  - [ ] Draft triage notes for first 5 CID witnesses in `.planning/triage/cid/`.

## Progress

| Phase | Plans Complete | Status | Completed |
|-------|----------------|--------|-----------|
| 1. Requirements & Targets | 0/2 | Not started | - |
| 2. Proposal Draft | 0/2 | Not started | - |
| 3. Delivery Plan (Grant-Ready) | 0/2 | Not started | - |
| 4. Fuzzing MVP (Contingent) | 0/TBD | Not started | - |
| 5. Cross-Impl Validation (Contingent) | 0/TBD | Not started | - |
| 6. Runner Implementation (Execution) | 0/2 | Not started | - |
