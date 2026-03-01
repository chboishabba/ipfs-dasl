# Spec Feedback Note: CID Hash Type Enforcement

## Case Summary

- **Case ID**: 36bbde8b0861c911272f2d8e877c0016f395e80ddd549cbc3da2b241c2d8f5de
- **Format**: `cid`
- **Category**: divergence (validity boundary)
- **Inputs**: `corpus/regressions/36bbde8b0861c911272f2d8e877c0016f395e80ddd549cbc3da2b241c2d8f5de/`

## Observed Behavior

- **go-dasl**: reject (`decode_error`)
- **atproto-dasl**: accept; canonical bytes = 36‑byte CID

## Invariant Impact

- **Invariant Status**: not_applicable (validity disagreement)
- **Details**: Input uses a hash type outside the go‑dasl allowed set (sha2‑256/blake3). `atproto-dasl` accepts while `go-dasl` rejects.

## Proposed Clarification

> Clarify the allowed hash types for DASL CID byte parsing and whether non‑DASL hash types must be rejected.

## Evidence Links

- Output record: `corpus/regressions/36bbde8b0861c911272f2d8e877c0016f395e80ddd549cbc3da2b241c2d8f5de/results.json`
- Corpus entry: `corpus/regressions/36bbde8b0861c911272f2d8e877c0016f395e80ddd549cbc3da2b241c2d8f5de/input.bin`
