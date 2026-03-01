# Proposed Repo Layout (Runner)

```
differential-runner/
  runner/               # Rust runner core
  targets/
    go_dasl/            # Go target adapter wrapper
    rust_atproto_dasl/  # Rust target adapter wrapper
  corpus/               # Seed corpus and minimized repros
  schemas/              # Output schema and corpus spec
  scripts/              # Helper scripts (build/run)
```

Notes:
- `runner/` hosts the Rust CLI and core types.
- `targets/` contains adapter wrappers; each writes normalized outputs.
- `corpus/` and `schemas/` are independent of any target implementation.
