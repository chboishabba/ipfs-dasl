# Proposal Draft: DASL Fuzz Testing (RFP #2026-01)

## Project Overview

This project will harden DASL implementations against hostile inputs by producing a curated corpus of adversarial test vectors and a fuzzing harness focused on DRISL1-serialized data, with CIDs and CAR included in scope. A core focus is detecting cross-implementation divergences where the same input is interpreted differently (even when hashes match). We will compare outputs across multiple implementations, including an ATProto-based implementation, and feed findings back to the DASL spec editors to clarify ambiguous or divergent behaviors.

**Primary targets:** `go-dasl` (Go module `github.com/hyphacoop/go-dasl`) and `atproto-dasl` (Rust crate `atproto-dasl` on docs.rs).

## Technical Design

- Build a fuzzing harness that ingests DRISL1-serialized inputs plus CID/CAR cases and exercises decoders (and secondarily encoders).
- Maintain a corpus of minimized adversarial test vectors (crashers, confusion cases, and divergence triggers) with unique IDs and failure-mode notes.
- Run differential testing across the two target implementations and record accept/reject outcomes, error classes, and canonical output bytes to detect mismatches across implementations.
- Enforce the primary invariant: if two implementations accept the same input, their canonical output must match byte-for-byte.
- Provide a stable output schema (JSON/CSV) for per-implementation results and comparisons.
- Add regression tests to lock in fixes and prevent reintroductions.
- Integrate a feedback loop with DASL spec editors when divergence indicates specification ambiguity.

## User Feedback and Adoption Plan

- Coordinate with maintainers of the two target implementations.
- Publish corpus and harness under MIT/Apache-2.0 for reuse.
- Provide clear instructions for running the harness and adding new test cases.

## Schedule and Budget

- **Month 1**: Harness runs both targets with DRISL1 + CID/CAR; seed corpus with IDs/failure modes; output schema emitted.
- **Month 2**: Differential testing + minimization pipeline; regression runner validates discovered cases.
- **Month 3**: Spec feedback loop; final corpus + harness + docs published; summary report of divergence classes.

**Budget:** TBD (target grant size $5,000–$25,000).

## Qualifications of Team

- TBD (insert relevant OSS experience and maintainership).

## Additional Information

- Outputs licensed under MIT and/or Apache-2.0.
- If not a maintainer of target projects, maintainer acknowledgements and evidence links will be included.
- Team availability for April 2026 Q&A calls: TBD.
