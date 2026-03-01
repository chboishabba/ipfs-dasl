# DASL Fuzz Testing Grant (RFP #2026-01)

## What This Is

A grant-funded project proposal and execution plan to build adversarial fuzz testing for DASL implementations, focused on DRISL1-serialized data and cross-implementation safety. The goal is to harden DASL parsing/encoding across multiple implementations and improve the spec via a feedback loop.

## Core Value

Make DASL implementations safe and interoperable under hostile inputs by producing a high-quality adversarial test corpus and fuzzing infrastructure.

## Requirements

### Validated

(None yet — ship to validate)

### Active

- [ ] Produce a corpus of adversarial test vectors for DASL.
- [ ] Provide fuzzing infrastructure that uses the corpus and supports regression testing.
- [ ] Primary focus on DRISL1-serialized DASL data; include CIDs and CAR in scope for the proposal.
- [ ] Test against two implementations, including at least one ATProto-based implementation.
- [ ] Detect cross-implementation divergences where the same input is interpreted differently, even when hashes match.
- [ ] Primary invariant: if two implementations accept the same input, their canonical output must match byte-for-byte.
- [ ] Establish a feedback loop with the DASL spec/editors to clarify and improve the spec as needed.
- [ ] Proposal must be clear and concise (<= 2 pages) with required sections.
- [ ] Outputs must be MIT and/or Apache-2.0 licensed.
- [ ] Proposal must emphasize measurable outputs and delivery milestones aligned to the RFP evaluation criteria.
- [ ] Harness should be an independent Rust driver that wraps implementations via CLI/FFI to avoid runtime bias.

### Out of Scope

- Building a new DASL implementation from scratch — focus is fuzzing existing implementations.
- Long-term maintenance beyond the offered 12-month maintenance grant window — separate decision.

## Context

- DASL is a strict subset of IPFS CIDs and IPLD optimized for simplicity and longevity.
- A 2025 grant produced a DASL test suite; this effort targets adversarial fuzzing to catch crashes, confusion, and cross-implementation divergences.
- The RFP highlights risk of different implementations interpreting the same DASL bytes differently even when hashes match; detecting and minimizing these mismatches is a core motivation.
- Hostile input handling and canonical agreement are high priority due to cryptographic integrity concerns.

## Constraints

- **Deadline**: Proposal due Sunday, March 15, 2026.
- **Duration**: Target project length 1-3 months (exceptions case-by-case).
- **License**: MIT and/or Apache-2.0 for project outputs.
- **Collaboration**: If contributing to a project you don’t maintain, obtain maintainer acknowledgment and link evidence.
- **Availability**: Team must be available for potential Q&A calls in early April 2026.
 - **Local stack**: Prefer implementations in JS/TS, Go, Rust, or Python (available via package managers).

## Key Decisions

| Decision | Rationale | Outcome |
|----------|-----------|---------|
| Use RFP requirements as authoritative baseline | Ensures alignment with grant expectations | — Pending |
| Keep scope focused on DRISL1 fuzzing first | Matches RFP primary focus | — Pending |
| Target two implementations, starting with installed language stacks | Faster ramp and fewer toolchain blockers | — Pending |
| Target implementations: `go-dasl` and `atproto-dasl` | Meets ATProto requirement and uses installed Go/Rust stacks | — Pending |
| Treat “installed” as package-manager availability | Confirms access without requiring local repos | — Pending |

---
*Last updated: 2026-03-01 after requirements extraction from RFP PDF*
