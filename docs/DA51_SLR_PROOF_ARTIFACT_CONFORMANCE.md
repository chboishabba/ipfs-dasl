# DA51 × SLR proof-artifact DASL conformance boundary

Date: 2026-10-17

`ipfs-dasl` remains a wire/canonicalisation conformance harness for DRISL1 DAG-CBOR, DASL CID bytes, and CAR headers. It does not own DA51 semantics, SLR PNF/proof obligations, formal evidence meaning, or publication authority.

For proof-linked ITIR artifacts the order is:

```text
formal/runtime evidence object
 -> logical ITIR artifact/shard
 -> canonical encoded bytes
 -> DASL differential conformance / idempotence witness
 -> optional CID/CAR/IPFS materialisation
```

The harness may prove or expose byte-level facts such as implementation accept/reject agreement, canonical byte equality, CID/CAR decoding consistency, and canonical->decode->canonical idempotence. Those facts are conformance evidence only.

Hard boundaries:

```text
DASL conformance != theorem proof
canonical bytes != SLR payment
CID != DA51 semantic identity
CID/CAR != logical shard identity
IPFS reachability != artifact truth
wire divergence witness != security conclusion
```

Optional proof provenance (`da51Address`, formal demand/evidence refs, statement digest) belongs to the logical artifact contract above the DASL layer and must survive canonical projections without becoming part of the selector or authority model.
