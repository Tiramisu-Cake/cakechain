# Cakechain Protocol Specification (v1)

## 0. Scope and Non-Goals

This document defines the **core protocol semantics** of a minimal account-based blockchain:
- state model,
- transaction validity and application,
- block validity and application,
- deterministic hashing and signatures.

**Non-goals (explicitly out of scope):**
- Networking, mempool, transaction propagation.
- Consensus / leader election / forks (this protocol accepts only a single linear chain).
- Execution / smart contracts / VM.
- Fee markets, gas, timestamps, difficulty, etc.

The protocol is **verify-everything**: any node must be able to validate blocks and recompute state transitions deterministically from genesis.

---

## 1. Types and Constants

### 1.1 Cryptography
- Signature scheme: **Ed25519**.
- Hash function: **SHA-256**.

### 1.2 Primitive types
- `Address`: 32 bytes, an Ed25519 public key.
- `Signature`: 64 bytes, an Ed25519 signature.
- `ChainId`: `u64`.
- `Amount`: `u64`.
- `Nonce`: `u64`.
- `BlockHeight`: `u64`.
- `Hash32`: 32 bytes.

### 1.3 Chain identifier
- `CHAIN_ID: ChainId = 1`.

`chain_id` is included in transaction signing to prevent cross-chain replay.

---

## 2. Canonical Serialization

### 2.1 General rules
All protocol-critical hashing and signing use **canonical byte serialization**:
- All `u64` are encoded as **little-endian** 8 bytes.
- `Address`, `Signature`, `Hash32` are raw byte arrays.
- No variable-length integers, no text encodings in canonical bytes.

### 2.2 Domain separation tags
To avoid cross-type collisions, all hashes and signing bytes begin with a fixed ASCII tag:

- `TX_DOMAIN_TAG = b"TXv1"`
- `STATE_DOMAIN_TAG = b"STATEv1"`
- `BLOCK_DOMAIN_TAG = b"BLOCKv1"`

Tags are included verbatim.

---

## 3. State

### 3.1 Structure
The state consists of:
- `balances: Map<Address, u64>`
- `nonces: Map<Address, u64>`

### 3.2 Missing entries
If an address is not present in a map, its value is defined as **0**:
- `balances[a] = 0` if missing,
- `nonces[a] = 0` if missing.

### 3.3 Deterministic state root
`state_root` is computed over:

TXv1 || chain_id_le || from || to || amount_le || nonce_le

The `signature` field is not included in signing bytes.

### 4.4 Signature validity
A transaction signature is valid iff:

ed25519_verify(
pubkey = from,
message = signing_bytes(tx, CHAIN_ID),
signature = signature
) == true


### 4.5 State-dependent validity
Given state `S`, a transaction `tx` is valid iff:

1. Static validity holds.
2. Signature is valid.
3. `tx.nonce == S.nonces[tx.from]`.
4. `S.balances[tx.from] >= tx.amount`.
5. `S.balances[tx.to] + tx.amount` does not overflow `u64`.

Overflow is forbidden; wrapping arithmetic is not permitted.

### 4.6 Transaction application
Applying a valid transaction yields new state `S'`:

- `balances[from] := balances[from] - amount`
- `balances[to] := balances[to] + amount`
- `nonces[from] := nonces[from] + 1`

All other entries remain unchanged.

### 4.7 Error priority
If multiple conditions fail, implementations must report errors in this order:

1. Static validity (`amount == 0`, `from == to`)
2. Invalid signature
3. Wrong nonce
4. Insufficient balance
5. Balance overflow

---

## 5. Blocks

### 5.1 Structure
A block consists of:
- `parent_hash: Hash32`
- `height: u64`
- `txs: Vec<Transaction>`
- `state_root: Hash32`

### 5.2 Block hash
Canonical block bytes:

BLOCKv1 || parent_hash || height_le || tx_count_le ||
tx_0_canonical ||
tx_1_canonical ||
... || state_root

Transaction canonical encoding:
from || to || amount_le || nonce_le || signature


Block hash:
block_hash = sha256(block_bytes)


### 5.3 Block validity
Let the node tip be `(tip_hash, tip_height)` with state `S`.

A block `B` is valid iff:
1. `B.parent_hash == tip_hash`
2. `B.height == tip_height + 1`
3. Transactions are sequentially valid and applied in order
4. The resulting state root equals `B.state_root`

### 5.4 Transaction order
Transactions are processed strictly in listed order. No reordering is permitted.

### 5.5 Empty blocks
Empty blocks are permitted and must preserve state.

---

## 6. Genesis

### 6.1 Genesis conventions
The genesis block and state are fixed constants:

- `genesis.height = 0`
- `genesis.parent_hash = [0x00; 32]`
- `genesis.txs = []`
- `genesis.state_root = state_root(genesis_state)`

`genesis_hash = block_hash(genesis_block)`.

All nodes must share identical genesis constants.

---

## 7. Chain Rule

A node maintains exactly one active chain.

- Blocks not extending the current tip are ignored.
- Forks are not accepted.

---

## 8. Protocol Invariants

For any valid chain:

1. **Determinism:** identical chains produce identical states.
2. **Nonce monotonicity:** nonces increase exactly by outgoing transactions.
3. **No negative balances:** balances never underflow.
4. **Balance preservation:** total balance is conserved (no mint/burn).

These invariants are intended targets for formal verification.

---

## 9. Conformance Testing

Implementations should provide test vectors including:
- genesis state,
- block sequences,
- expected state roots,
- expected acceptance or rejection outcomes.

This enables cross-language verification of protocol correctness.

