# Target CLI Contract

Each target adapter must implement a CLI that:

- Reads raw input bytes from `stdin`
- Writes a single JSON object to `stdout`
- Exits with code 0 on success (even for reject outcomes)

## JSON Output Fields

- `impl_id`: string
- `accepted`: boolean
- `error_class`: string (use "none" when accepted)
- `canonical_bytes`: optional hex string (required when accepted)
- `time_ms`: integer (execution time)

## Notes

- Do not print non-JSON content to stdout.
- Any stderr output is treated as diagnostic only.
- The runner sets `DASL_FORMAT` to `drisl1`, `cid`, or `car` to signal how input bytes should be interpreted.
