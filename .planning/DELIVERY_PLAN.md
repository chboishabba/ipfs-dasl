# Delivery Plan: DASL Fuzz Testing (RFP #2026-01)

## Deliverables

1. **Adversarial Test Corpus (DRISL1 + CID/CAR)**
   - A versioned corpus of adversarial test vectors that includes DRISL1-serialized DASL inputs and CID/CAR cases.
   - Includes minimized crashers, confusion cases, and divergence triggers.

2. **Fuzzing Harness + Regression Runner**
   - A harness that executes decoders (and secondarily encoders) for `go-dasl` and `atproto-dasl` against the corpus and generated cases.
   - A regression runner that replays corpus cases and records outcomes.
   - Format-aware parsing paths for `drisl1`, `cid`, and `car`.

3. **Differential Testing Output Schema**
   - A stable output format (JSON or CSV) that records per-implementation accept/reject outcomes, error classes, and canonical output bytes.

4. **Spec Feedback Loop Artifacts**
   - Issue templates or reports capturing divergences with minimal repro inputs, plus proposed spec clarifications.
   - CID/CAR divergence triage notes and categorized repros.

## Acceptance Criteria

1. **Adversarial Test Corpus**
   - Corpus includes DRISL1 cases plus CID and CAR cases.
   - Each corpus entry has a unique ID, input bytes, and a short description of the failure mode.
   - At least one minimized reproducer exists for each divergence class discovered.

2. **Fuzzing Harness + Regression Runner**
   - Harness can run both targets (`go-dasl`, `atproto-dasl`) on the same inputs.
   - Regression runner produces deterministic output for the same corpus version.
   - Runs are resumable and support time/iteration limits.

3. **Differential Testing Output Schema**
   - Output format records for each input: implementation ID, accept/reject, error class, and canonical output bytes if accepted.
   - Enables direct comparison of canonical outputs across implementations.

4. **Primary Invariant (Core Requirement)**
   - If two implementations accept the same input, their canonical output must match byte-for-byte.
   - Any violation is recorded as a divergence with a minimized reproducer in the corpus.

5. **Spec Feedback Loop Artifacts**
   - Each divergence has an associated spec feedback note that states the ambiguous behavior and proposes clarification.
   - CID validity divergences are classified by codec/hash/digest-length rules.

## Notes

- Scope includes DRISL1 plus CID/CAR as per RFP optional inclusion.
- Targets are `go-dasl` and `atproto-dasl`.

## Milestones

**Month 1: Harness + Seed Corpus**
- Harness runs both targets with DRISL1 + CID/CAR inputs.
- Seed corpus with documented failure modes and unique IDs.
- Output schema (JSON/CSV) defined and emitted by runs.

**Month 2: Differential Testing + Minimization**
- Differential runs across both implementations with recorded canonical outputs.
- Minimization pipeline produces reduced repros for divergences.
- Regression runner validates previously found cases.

**Month 3: Spec Feedback + Final Artifacts**
- Spec feedback notes for each divergence class with minimal repros.
- Final corpus, harness, and docs published under MIT/Apache-2.0.
- Summary report of divergence classes and resolution status.

## Risks & Mitigations

- **Risk**: Implementation behavior differs in undocumented ways that are hard to classify.  
  **Mitigation**: Capture per-impl outputs and errors; prioritize cases that violate the primary invariant for feedback loop.
- **Risk**: Maintainer availability slows triage or fixes.  
  **Mitigation**: Provide minimal repros and clear evidence; proceed with spec clarification and regression corpus regardless of fix timing.
- **Risk**: CID/CAR cases expand scope beyond capacity.  
  **Mitigation**: Bound CID/CAR coverage to representative cases and keep DRISL1 as the primary focus.
