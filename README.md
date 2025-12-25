# Cakechain

Cakechain is a **minimal, specification-driven blockchain protocol** designed to explore core ledger semantics without implementation noise.

The project deliberately focuses on **what the protocol is**, not on how a network is built around it.

---

## Goals

- Define a **small, precise, and unambiguous blockchain protocol**
- Make **state transitions fully deterministic**
- Ensure that **any two correct implementations produce identical results**
- Separate **protocol semantics** from networking, consensus, and execution
- Enable **formal reasoning and verification** of core invariants

---

## What This Is

- Account-based ledger
- Single linear chain (no forks)
- One transaction type: value transfer
- Explicit state model (balances + nonces)
- Canonical serialization for hashing and signing
- Replay protection via `chain_id`
- Genesis fixed and hard-coded
- “Verify everything” trust model

This repository treats the protocol as a **state transition system**:

stateₙ₊₁ = apply(blockₙ₊₁, stateₙ)

---

## What This Is *Not*

By design, Cakechain does **not** include:

- Networking or peer discovery
- Mempool or transaction propagation rules
- Consensus algorithms or fork choice
- Validators, staking, PoW/PoS
- Smart contracts or execution engine
- Fees, gas, timestamps, or difficulty

These concerns are intentionally excluded to keep the protocol **minimal, analyzable, and verifiable**.

---

## Specification-First Design

The authoritative definition of the protocol lives in the formal specification:

📄 **[`spec/protocol-spec.md`](spec/protocol-spec.md)**

That document defines:
- all data structures,
- canonical byte formats,
- validity rules,
- state transition functions,
- protocol invariants.

**The specification is the source of truth.**  
Implementations must conform to it, not vice versa.

---

## Implementations

- **Rust**: reference implementation of the protocol rules
- (TODO) **Haskell**: independent re-implementation to validate spec completeness
- (TODO) **Lean**: formal model and proofs of core invariants

Multiple implementations over the same spec are used to detect ambiguity and underspecification.

---

## Formal Methods

Cakechain is explicitly designed to be amenable to formal reasoning.

Target properties include:
- determinism of state transitions,
- nonce monotonicity,
- balance preservation,
- absence of underflow/overflow,
- replay resistance.

Formalization is planned using **Lean**, directly reflecting the protocol specification.

---

## Project Philosophy

- Minimal surface area
- No hidden behavior
- No implicit assumptions
- No “magic”
- Everything explicitly specified
- Everything verifiable

This is a protocol, not a product.

---

## License

MIT

