# Compactified Context

## Summary

The repo contains only `DASL Fuzz Testing Insights.pdf`. Extracted the Winter 2026 IPFS Implementations RFP requirements for DASL fuzz testing and codified them into planning docs.

## Key Requirements (from RFP PDF)

- Primary output: adversarial test vector corpus with fuzzing infrastructure.
- Primary focus: DRISL1-serialized DASL data; include CIDs and CAR in scope for the proposal.
- Test against multiple implementations, including at least one ATProto-based.
- Maintain a feedback loop with DASL spec/editors to clarify/improve the spec.
- Detect and minimize cross-implementation divergences where the same DASL input can be interpreted differently even when hashes match.
- Primary invariant: if two implementations accept the same input, their canonical output must match byte-for-byte.
- Proposal must emphasize measurable outputs and delivery milestones aligned to the RFP evaluation criteria.
- Proposal must be <= 2 pages with required sections.
- Outputs must be MIT and/or Apache-2.0 licensed.
- Proposal deadline: Sunday, March 15, 2026.
- Project duration target: 1-3 months.

## Planning Artifacts

- `.planning/PROJECT.md` with requirements, constraints, and context.
- `.planning/ROADMAP.md` with phases and TODOs.

## Open Questions

- Confirm package-manager availability details (module names/versions) for `go-dasl` and `atproto-dasl`.
- Should CIDs/CAR fuzzing be included in the initial scope?
- Are maintainers already contacted for any target implementations?

## Package Manager Details

- `go-dasl` Go module: `github.com/hyphacoop/go-dasl` (DASL testing page lists v0.8.0).
- `atproto-dasl` Rust crate: `atproto-dasl` (docs.rs).

## Repo Links

- `go-dasl`: https://github.com/hyphacoop/go-dasl
- `atproto-dasl`: https://tangled.org/smokesignal.events/atproto-identity-rs/tree/main/crates/atproto-dasl

## New Artifacts

- `.planning/CONTACTS.md` created to track maintainer outreach and evidence links.

## Next Actions

- Select implementations and confirm maintainer engagement if required.
- Draft proposal sections in line with RFP constraints.
