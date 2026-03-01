# Output Schema and Differential Procedure

## Output Schema (per run)

Each run produces a record with per-implementation results and comparison status.

- `run_id`: unique run identifier
- `case_id`: corpus ID (or generated case ID)
- `format`: `drisl1` | `cid` | `car`
- `input_hash`: hash of input bytes
- `results`: array of per-implementation results
  - `impl_id`
  - `accepted` (bool)
  - `error_class` (string)
  - `canonical_bytes` (hex if accepted)
  - `time_ms`
- `invariant_status`: `pass` | `fail` | `not_applicable`
- `idempotence_status`: `pass` | `fail` | `not_applicable`
- `divergence_notes`: optional short note

## Differential Run Procedure

1. Load corpus entry or generated case.
2. Execute input against each target adapter.
3. Record per-implementation results (accept/reject, errors, canonical bytes).
4. If two or more implementations accept:
   - Compare canonical bytes byte-for-byte.
   - Set `invariant_status = fail` if any mismatch.
5. For each accepting implementation:
   - Re-run on canonical bytes and compare output.
   - Set `idempotence_status = fail` if any mismatch.
6. If invariant fails, idempotence fails, or crash occurs:
   - Record divergence notes.
   - Add minimized reproducer to corpus (category `divergence` or `crash`).
7. Emit run record to JSON/CSV output.

## Notes

- `invariant_status = not_applicable` when fewer than two implementations accept.
- `idempotence_status = not_applicable` when no implementations accept.
- All divergence failures must result in a corpus update and spec feedback note.
