# Week 1 Success Criteria

All five conditions must be met to declare Week 1 complete:

1. **CI passes deterministically**
   - Seed + regression corpus passes on every push.

2. **Idempotence enforced**
   - Both implementations re-encode their own canonical bytes identically.

3. **20k mutation run executed locally**
   - Boundaries: <= 256 KB input, 200 ms timeout.
   - Outcome: zero crashes/timeouts OR at least one real divergence found.

4. **At least 1 minimized witness stored**
   - Stored under `corpus/regressions/<hash>/`.

5. **Short finding report produced**
   - 1–2 pages covering inputs tested, failures found, classification, and spec ambiguity status.
