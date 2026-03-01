# Cross-Implementation Validation Workflow

## End-to-End Flow

1. **Select inputs** from corpus or generator.
2. **Run targets** (`go-dasl`, `atproto-dasl`) via adapters.
3. **Record outputs** in the output schema.
4. **Evaluate invariant** (byte-for-byte canonical outputs match if both accept).
5. **Classify outcome** (triage category).
6. **Persist artifacts** (output record + repro in corpus).
7. **Regression integration** (ensure future runs re-test).

## Triage Categories and Actions

1. **Crash / Panic / OOM**
   - Action: minimize input; add to corpus with `crash` category; open maintainer issue.

2. **Parse Error (Rejected)**
   - Action: record rejection; if other impl accepts, treat as potential divergence and create spec feedback note.

3. **Canonical Mismatch (Invariant Fail)**
   - Action: minimize input; add to corpus with `divergence` category; create spec feedback note.

4. **Spec Ambiguity**
   - Action: document ambiguity with proposed clarification; include cross-impl evidence.

## Artifacts

- Output records per run (`OUTPUT_SCHEMA.md`)
- Corpus entries with IDs and categories (`CORPUS_SPEC.md`)
- Spec feedback notes (`SPEC_FEEDBACK.md`)
