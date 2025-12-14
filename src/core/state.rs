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
