#!/usr/bin/env python3
"""
Cakechain StateRoot reference generator (matches src/core/state.rs)

State canonical bytes:
  b"STATEv1" ||
  len(balances) as u64 LE ||
  for (addr, bal) in balances sorted by addr bytes asc:
      addr (32 bytes) || bal as u64 LE ||
  len(nonces) as u64 LE ||
  for (addr, nonce) in nonces sorted by addr bytes asc:
      addr (32 bytes) || nonce as u64 LE

StateRoot = SHA256(canonical_bytes)

Input JSON formats accepted:

1) List form (recommended):
{
  "balances": [{"addr":"<64-hex>", "amount": 100}, ...],
  "nonces":   [{"addr":"<64-hex>", "nonce":  0}, ...]
}

2) Map form (also accepted):
{
  "balances": {"<64-hex>": 100, "<64-hex>": 200},
  "nonces":   {"<64-hex>": 0}
}

Notes:
- addr must be exactly 32 bytes => 64 hex chars
- amounts/nonces must fit into u64 (0..2^64-1)
- If an address appears multiple times in list form, the last one wins (like inserting into a map).
"""

from __future__ import annotations

import argparse
import hashlib
import json
from typing import Dict, Any


TAG = b"STATEv1"
U64_MAX = (1 << 64) - 1


def u64_le(x: int) -> bytes:
    if not isinstance(x, int):
        raise TypeError(f"Expected int, got {type(x).__name__}")
    if x < 0 or x > U64_MAX:
        raise ValueError(f"Value {x} out of u64 range")
    return x.to_bytes(8, byteorder="little", signed=False)


def parse_addr_hex(s: str) -> bytes:
    if not isinstance(s, str):
        raise TypeError(f"Address must be hex string, got {type(s).__name__}")
    s2 = s.lower().removeprefix("0x")
    if len(s2) != 64:
        raise ValueError(f"Address hex must be 64 chars (32 bytes), got {len(s2)}: {s}")
    try:
        b = bytes.fromhex(s2)
    except ValueError as e:
        raise ValueError(f"Invalid hex in address: {s}") from e
    if len(b) != 32:
        raise ValueError(f"Address decoded length must be 32 bytes, got {len(b)}")
    return b


def load_section(obj: Any, value_key: str) -> Dict[bytes, int]:
    """
    Returns map: addr_bytes -> u64_value
    Accepts either list-of-objects or dict mapping.
    """
    out: Dict[bytes, int] = {}

    if obj is None:
        return out

    if isinstance(obj, dict):
        for addr_hex, val in obj.items():
            addr = parse_addr_hex(addr_hex)
            out[addr] = int(val)
        return out

    if isinstance(obj, list):
        for item in obj:
            if not isinstance(item, dict):
                raise TypeError(
                    f"List items must be objects, got {type(item).__name__}"
                )
            if "addr" not in item:
                raise ValueError(f"Missing 'addr' in item: {item}")
            if value_key not in item:
                raise ValueError(f"Missing '{value_key}' in item: {item}")
            addr = parse_addr_hex(item["addr"])
            out[addr] = int(item[value_key])  # last wins
        return out

    raise TypeError(f"Section must be list or dict, got {type(obj).__name__}")


def canonical_bytes(balances: Dict[bytes, int], nonces: Dict[bytes, int]) -> bytes:
    out = bytearray()
    out.extend(TAG)

    # balances
    out.extend(u64_le(len(balances)))
    for addr in sorted(balances.keys()):
        out.extend(addr)
        out.extend(u64_le(balances[addr]))

    # nonces
    out.extend(u64_le(len(nonces)))
    for addr in sorted(nonces.keys()):
        out.extend(addr)
        out.extend(u64_le(nonces[addr]))

    return bytes(out)


def state_root(cb: bytes) -> bytes:
    return hashlib.sha256(cb).digest()


def main() -> None:
    ap = argparse.ArgumentParser(
        description="Compute Cakechain State canonical bytes + state_root (SHA-256)."
    )
    ap.add_argument("json_path", help="Path to JSON describing state balances/nonces")
    ap.add_argument(
        "--pretty", action="store_true", help="Pretty-print JSON summary + lengths"
    )
    args = ap.parse_args()

    with open(args.json_path, "r", encoding="utf-8") as f:
        data = json.load(f)

    if not isinstance(data, dict):
        raise TypeError("Top-level JSON must be an object")

    balances = load_section(data.get("balances"), "amount")
    nonces = load_section(data.get("nonces"), "nonce")

    cb = canonical_bytes(balances, nonces)
    root = state_root(cb)

    if args.pretty:
        print("=== Parsed state ===")
        print(f"balances: {len(balances)} entries")
        print(f"nonces:   {len(nonces)} entries")
        print(f"canonical_bytes_len: {len(cb)}")
        print()

    print(f"canonical_bytes_hex: {cb.hex()}")
    print(f"state_root_hex:      {root.hex()}")


if __name__ == "__main__":
    main()
