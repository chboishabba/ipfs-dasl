# Spec Feedback Note: CID Validity Split (go-dasl rejects, atproto-dasl accepts)

## Case Summary

- **Case ID**: CID divergence cluster (5 representatives)
- **Format**: `cid`
- **Category**: divergence (validity boundary)
- **Inputs**: see `corpus/regressions/` directories:
  - `01290db4bd1bc5edb875b4f210c2c236655791dfeb64769816041e6e4bc2b1d9/`
  - `0601104d8272e465089c32a23d908c23a4733275372a3efa54a7fc06e407e010/`
  - `060a41a856f1eb1df914f1b7c4032b354d1ee5125c02651266c63e8dc3d6bf50/`
  - `096ac1942fe10445eea25fdfd4dc8909d1a3cd32c0a9e345c6f7118ddbee4ef1/`
  - `0cb6fa1baae1233b32d4b85f35853b95c12a65e3b1cf9de2e4b9ecbc2a458ed8/`

## Observed Behavior

- **go-dasl**: reject (`decode_error`)
- **atproto-dasl**: accept; canonical bytes = 36‑byte CID

## Invariant Impact

- **Invariant Status**: not_applicable (validity disagreement)
- **Details**: One implementation accepts mutated CID bytes while the other rejects.

## Proposed Clarification

> Clarify the exact validity rules for DASL CID byte parsing (permitted codecs, hash types, digest length, and whether non‑DASL CIDs are allowed). Explicitly define whether parsers must reject any CID not conforming to DASL‑specific constraints.

## Evidence Links

- Output records: `corpus/regressions/<hash>/results.json`
- Corpus entries: `corpus/regressions/<hash>/input.bin`
- Minimal repros: stored under the same directories
