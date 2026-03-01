# Harness Architecture (MVP)

## Components

1. **Input Loader**
   - Loads corpus entries (DRISL1 + CID/CAR) and generated cases.
   - Emits raw bytes and metadata to the runner.

2. **Target Adapter Interface**
   - Standard interface for invoking `go-dasl` and `atproto-dasl`.
   - Converts raw input bytes into per-implementation results.
   - Implemented in a separate Rust driver to avoid bias toward any one implementation runtime.

3. **Runner**
   - Rust core runner orchestrates targets (CLI-wrapped Go/Rust/JS as needed).
   - Aggregates results into the output schema.

4. **Output Reporter**
   - Writes JSON/CSV output per run.
   - Performs primary invariant checks and flags divergences.

5. **Corpus Updater**
   - Adds minimized repros and divergence cases back into corpus.

## Data Flow

Input Loader → Runner → Target Adapters → Output Reporter → Corpus Updater

## Target Adapter Contract

**Input:**
- `bytes`: raw input bytes (DRISL1/CID/CAR)
- `case_id`: corpus ID (optional for generated cases)

**Output:**
- `impl_id`: implementation identifier
- `accepted`: boolean
- `error_class`: string (e.g., parse_error, overflow, panic, timeout)
- `canonical_bytes`: bytes (if accepted)
- `canonical_hash`: optional (if used by impl)
- `time_ms`: execution time

**Requirements:**
- Always return structured output (no silent failures).
- If accepted, must return canonical output bytes for byte-for-byte comparison.

## Notes

- Architecture is implementation-agnostic; adapters can be CLI-driven or library-driven.
- Output schema must support primary invariant checks and regression reporting.
