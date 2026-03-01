# DASL Differential Runner

A neutral, Rust‑based differential harness for DASL implementations. It runs multiple targets on the same input bytes, compares accept/reject outcomes, enforces canonical byte equality, and checks idempotence (canonical → decode → canonical).

## Current Targets

- `go-dasl` (Go adapter)
- `atproto-dasl` (Rust adapter)

Adapters are CLI wrappers that read bytes from stdin and emit a single JSON object to stdout.

## Formats

The runner sets `DASL_FORMAT` for each input:

- `drisl1` — DRISL1 DAG‑CBOR
- `cid` — DASL CID bytes
- `car` — CAR v1 header bytes

Format is inferred from filename prefix:

- `cid_*.bin` → `cid`
- `car_*.bin` → `car`
- otherwise → `drisl1`

## Usage

Run a single file:

```bash
cargo run --quiet --manifest-path runner/Cargo.toml run path/to/input.bin
```

Run a directory (JSONL report appended to `reports/report.jsonl`):

```bash
cargo run --quiet --manifest-path runner/Cargo.toml run corpus/seed
```

Run bounded fuzzing (20k iterations, 10 min cap):

```bash
cargo run --quiet --manifest-path runner/Cargo.toml fuzz corpus/seed
```

## Witnesses and Regressions

Any divergence or idempotence failure is written to:

```
corpus/regressions/<hash>/
  input.bin
  results.json
  canon/<impl>.bin
```

## CI

GitHub Actions runs deterministic conformance on the seed and regression corpora:

- `.github/workflows/conformance.yml`

## Notes

- Canonical bytes are hex‑encoded in JSON output.
- CAR handling is currently implemented for `atproto-dasl` (header only) and marked `unsupported_format` in `go-dasl` until a Go CAR path is wired.
