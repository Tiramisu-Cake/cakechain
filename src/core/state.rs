use crate::core::basics::Address;
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;

#[derive(Clone, Debug, Default)]
pub struct State {
    pub balances: BTreeMap<Address, u64>,
    pub nonces: BTreeMap<Address, u64>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StateRoot(pub [u8; 32]);

impl State {
    pub fn canonical_bytes(&self) -> Vec<u8> {
        let mut out = Vec::new();

        out.extend_from_slice(b"STATEv1");

        out.extend_from_slice(&(self.balances.len() as u64).to_le_bytes());
        for (addr, bal) in self.balances.iter() {
            out.extend_from_slice(addr.as_bytes());
            out.extend_from_slice(&bal.to_le_bytes());
        }

        out.extend_from_slice(&(self.nonces.len() as u64).to_le_bytes());
        for (addr, nonce) in self.nonces.iter() {
            out.extend_from_slice(addr.as_bytes());
            out.extend_from_slice(&nonce.to_le_bytes());
        }

        out
    }

    /// StateRoot = SHA256(canonical_bytes(state))
    pub fn state_root(&self) -> StateRoot {
        let bytes = self.canonical_bytes();
        let digest = Sha256::digest(bytes);

        let mut out = [0u8; 32];
        out.copy_from_slice(&digest);
        StateRoot(out)
    }

    pub fn balance_of(&self, addr: &Address) -> Option<&u64> {
        self.balances.get(addr)
    }

    pub fn nonce_of(&self, addr: &Address) -> Option<&u64> {
        self.nonces.get(addr)
    }
}

#[cfg(test)]
mod tests {
    use super::Address;
    use super::State;

    const EXPECTED_CANONICAL_BYTES_HEX: &str = "53544154457631020000000000000000000000000000000000000000000000000000000000000000000000000000010a00000000000000ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff1400000000000000010000000000000000000000000000000000000000000000000000000000000000000000000000010000000000000000";

    const EXPECTED_STATE_ROOT_HEX: &str =
        "ce8eb714576293f084a4f3ab758db36931136c2184c801f13bb90893bd02dbed";

    fn bytes_to_hex(bytes: &[u8]) -> String {
        const HEX: &[u8; 16] = b"0123456789abcdef";
        let mut s = String::with_capacity(bytes.len() * 2);
        for &b in bytes {
            s.push(HEX[(b >> 4) as usize] as char);
            s.push(HEX[(b & 0x0f) as usize] as char);
        }
        s
    }

    fn hex_to_bytes(hex: &str) -> Vec<u8> {
        fn val(c: u8) -> u8 {
            match c {
                b'0'..=b'9' => c - b'0',
                b'a'..=b'f' => c - b'a' + 10,
                b'A'..=b'F' => c - b'A' + 10,
                _ => panic!("Invalid hex char: {c}"),
            }
        }

        let h = hex.as_bytes();
        assert!(h.len() % 2 == 0, "Hex length must be even");
        let mut out = Vec::with_capacity(h.len() / 2);
        for i in (0..h.len()).step_by(2) {
            let hi = val(h[i]);
            let lo = val(h[i + 1]);
            out.push((hi << 4) | lo);
        }
        out
    }

    #[test]
    fn test_state_root_vector_1() {
        // addr_01 = 0x00..00 01
        let mut addr_01 = [0u8; 32];
        addr_01[31] = 1;

        // addr_ff = 0xff..ff
        let addr_ff = [0xffu8; 32];
        let mut s = State::default();

        // reverse order as in BTreeMap
        s.balances.insert(Address(addr_ff), 20);
        s.balances.insert(Address(addr_01), 10);

        s.nonces.insert(Address(addr_01), 0);

        let cb = s.canonical_bytes();
        assert_eq!(cb.len(), 143, "canonical_bytes_len mismatch");

        let cb_hex = bytes_to_hex(&cb);
        assert_eq!(
            cb_hex, EXPECTED_CANONICAL_BYTES_HEX,
            "canonical_bytes hex mismatch"
        );

        let root = s.state_root();
        let root_hex = bytes_to_hex(&root.0);
        assert_eq!(root_hex, EXPECTED_STATE_ROOT_HEX, "state_root hex mismatch");

        // Optional: sanity-check that the root matches hashing of the expected bytes.
        let expected_bytes = hex_to_bytes(EXPECTED_CANONICAL_BYTES_HEX);
        assert_eq!(
            cb, expected_bytes,
            "canonical_bytes differ from decoded expected bytes"
        );
    }
}
