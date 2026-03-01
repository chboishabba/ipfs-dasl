# Spec Feedback Note: CID Length Enforcement

## Case Summary

- **Case ID**: 01290db4bd1bc5edb875b4f210c2c236655791dfeb64769816041e6e4bc2b1d9
- **Format**: `cid`
- **Category**: divergence (validity boundary)
- **Inputs**: `corpus/regressions/01290db4bd1bc5edb875b4f210c2c236655791dfeb64769816041e6e4bc2b1d9/`

## Observed Behavior

- **go-dasl**: reject (`decode_error`)
- **atproto-dasl**: accept; canonical bytes = 36‑byte CID

## Invariant Impact

- **Invariant Status**: not_applicable (validity disagreement)
- **Details**: Input length is not 36 bytes; `atproto-dasl` accepts while `go-dasl` rejects.

## Proposed Clarification

> Clarify whether DASL CID parsing requires the input byte slice to be exactly 36 bytes long (CIDv1 + codec + hash type + 32‑byte digest), and whether any non‑exact lengths must be rejected.

## Evidence Links

- Output record: `corpus/regressions/01290db4bd1bc5edb875b4f210c2c236655791dfeb64769816041e6e4bc2b1d9/results.json`
- Corpus entry: `corpus/regressions/01290db4bd1bc5edb875b4f210c2c236655791dfeb64769816041e6e4bc2b1d9/input.bin`
