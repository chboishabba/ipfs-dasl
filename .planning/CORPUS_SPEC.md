# Corpus Specification

## Entry Schema

Each corpus entry is a single test case with the following fields:

- `id`: unique identifier (string)
- `category`: one of `crash`, `confusion`, `divergence`, `boundary`, `cid`, `car`
- `description`: short human-readable summary
- `bytes`: raw input bytes (base64 or hex)
- `format`: `drisl1` | `cid` | `car`
- `expected`: notes on expected behavior (accept/reject or invariant focus)
- `source`: `fuzzer` | `manual` | `imported`
- `minimized`: boolean

## Seed Corpus Plan

Minimum initial coverage targets:

- **DRISL1**
  - 10 crash/parse-error cases
  - 10 confusion/ambiguity cases
  - 10 divergence-trigger candidates
- **CID**
  - 5 boundary/invalid CIDs
  - 5 valid but unusual encodings
- **CAR**
  - 5 boundary/invalid CAR files
  - 5 minimal valid CARs

## Notes

- Each entry must include `format` to separate DRISL1 vs CID/CAR handling.
- Divergence cases should include a brief note on the invariant violation.
