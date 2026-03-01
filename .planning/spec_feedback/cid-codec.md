# Spec Feedback Note: CID Codec Enforcement

## Case Summary

- **Case ID**: 46b91fa4eebce6b3dbe50bb69ef1a653bc9777f3b3958157ba4a57e05e17842e
- **Format**: `cid`
- **Category**: divergence (validity boundary)
- **Inputs**: `corpus/regressions/46b91fa4eebce6b3dbe50bb69ef1a653bc9777f3b3958157ba4a57e05e17842e/`

## Observed Behavior

- **go-dasl**: reject (`decode_error`)
- **atproto-dasl**: accept; canonical bytes = 36‑byte CID

## Invariant Impact

- **Invariant Status**: not_applicable (validity disagreement)
- **Details**: Input uses a codec outside the go‑dasl allowed set (raw/dag‑cbor). `atproto-dasl` accepts while `go-dasl` rejects.

## Proposed Clarification

> Clarify the allowed codecs for DASL CID byte parsing and whether non‑DASL codecs must be rejected.

## Evidence Links

- Output record: `corpus/regressions/46b91fa4eebce6b3dbe50bb69ef1a653bc9777f3b3958157ba4a57e05e17842e/results.json`
- Corpus entry: `corpus/regressions/46b91fa4eebce6b3dbe50bb69ef1a653bc9777f3b3958157ba4a57e05e17842e/input.bin`
