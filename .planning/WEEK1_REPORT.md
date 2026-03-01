# Week 1 Findings Report (Draft)

## Summary

- Harness status: two real implementations wired (Go + Rust).
- Idempotence: enforced and passing on seed corpus.
- Differential checks: canonical agreement confirmed on seeds.

## Inputs Tested

- Seed corpus: `corpus/seed` (DRISL1 + boundary seeds, plus CID/CAR header cases)
- Mutation runs:
  - Run A: 20,000 iterations (bounded)
  - Run B: 20,000 iterations (bounded, expanded seeds)

Bounds:
- Max input size: 256 KB
- Timeout per target: 200 ms
- Mutation classes: truncation, bit flip, append junk, duplicate region, drop chunk

## Results

- Crashes: 0
- Timeouts: 0
- Divergences: 50 (CID validity disagreements)
- Idempotence failures: 0

## CID Validity Divergence Classification (Week 1)

- Total divergences: 50
- 94% (47/50) length boundary handling
- 4% (2/50) hash_type handling
- 2% (1/50) codec handling

Interpretation: divergences are overwhelmingly concentrated in CID length boundary enforcement, indicating a systematic strictness/permissiveness difference rather than random instability.

## Notes

- CID/CAR header-only seeds now use format-aware parsing paths.
- CID fuzzing produced 50 validity disagreements: `atproto-dasl` accepted inputs that `go-dasl` rejected.
- Witnesses stored under `corpus/regressions/<hash>/` (CID format).
- Example witness: `corpus/regressions/01290db4bd1bc5edb875b4f210c2c236655791dfeb64769816041e6e4bc2b1d9/`.
- Next step: triage CID validity rules (length, codec, hash type) across implementations and file spec feedback notes.

## Classification

- No spec ambiguities discovered in Week 1.
