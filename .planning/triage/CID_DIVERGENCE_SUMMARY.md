# CID Validity Divergence Summary (First 5 Witnesses)

## Pattern Summary

- All 5 witnesses show **validity disagreements**: `atproto-dasl` accepts, `go-dasl` rejects.
- **LCP with atproto canonical bytes** is consistently **36 bytes** (CID binary length).
- **Appending one byte** to a known-valid CID is **accepted by both** implementations.
- **Trimming trailing bytes** to regain `go-dasl` acceptance requires removing 38–46 bytes (beyond the 36‑byte canonical CID length).

## Interpretation

- The canonical bytes produced by `atproto-dasl` are 36 bytes long (CID length), and the witness inputs share a 36‑byte prefix.
- The failures do **not** look like simple trailing‑byte acceptance of a valid CID, because both implementations accept a valid CID with a single trailing byte, and trimming many bytes is required to regain `go-dasl` acceptance.
- This suggests the divergence is more likely due to **CID validity rules** (e.g., hash type, codec, or digest constraints) rather than prefix‑parse behavior.

## Next Triage Questions

1. Are the failing inputs violating CID hash type or codec restrictions enforced by `go-dasl`?
2. Are they using non‑DASL CID forms that `atproto-dasl` accepts?
3. Do these inputs violate multibase identity / DAG‑CBOR tagging constraints?

## Action Items

- Add CID rule classification to triage notes (`CID/Codec`, `CID/HashType`, `CID/DigestLen`).
- Extract and decode multiformats fields from the first 5 witnesses for comparison.
- Draft spec feedback notes asking for clarification on CID validity scope for DASL.
