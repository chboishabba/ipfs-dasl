# Maintainer Outreach Drafts

## go-dasl (hyphacoop/go-dasl)

Subject: CID validity divergence repros (go-dasl rejects, atproto-dasl accepts)

Body:

Hello [maintainer],

We’re running a differential DASL harness across `go-dasl` and `atproto-dasl` and have found a cluster of CID validity disagreements where `go-dasl` rejects inputs that `atproto-dasl` accepts. We have minimized witnesses and full output records for each case.

Representative repros:
- `corpus/regressions/01290db4bd1bc5edb875b4f210c2c236655791dfeb64769816041e6e4bc2b1d9/`
- `corpus/regressions/0601104d8272e465089c32a23d908c23a4733275372a3efa54a7fc06e407e010/`
- `corpus/regressions/060a41a856f1eb1df914f1b7c4032b354d1ee5125c02651266c63e8dc3d6bf50/`

We’re triaging whether this is codec/hash/digest‑length enforcement vs spec ambiguity. Would you be open to a quick review of these cases? We can package the exact inputs and a minimal runner invocation if helpful.

Thanks,
[Name]

## atproto-dasl (atproto-crates)

Subject: CID validity divergence repros (atproto-dasl accepts, go-dasl rejects)

Body:

Hello [maintainer],

We’re running a differential DASL harness across `atproto-dasl` and `go-dasl` and have found a cluster of CID validity disagreements where `atproto-dasl` accepts inputs that `go-dasl` rejects. We have minimized witnesses and full output records for each case.

Representative repros:
- `corpus/regressions/01290db4bd1bc5edb875b4f210c2c236655791dfeb64769816041e6e4bc2b1d9/`
- `corpus/regressions/0601104d8272e465089c32a23d908c23a4733275372a3efa54a7fc06e407e010/`
- `corpus/regressions/060a41a856f1eb1df914f1b7c4032b354d1ee5125c02651266c63e8dc3d6bf50/`

We’re triaging whether this reflects differing CID validity rules (codec/hash/digest length). Would you be open to a quick review of these cases? We can package the exact inputs and a minimal runner invocation if helpful.

Thanks,
[Name]
