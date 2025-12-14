#[derive(Ord, PartialEq, PartialOrd, Eq, Clone, Copy, Hash, Debug)]
pub struct Address([u8; 32]);

impl From<PublicKey> for Address {
    fn from(pk: PublicKey) -> Self {
        Address(pk.0)
    }
}

impl Address {
    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}

pub struct PublicKey([u8; 32]);

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct BlockHash(pub [u8; 32]);

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct BlockHeight(pub u64);
