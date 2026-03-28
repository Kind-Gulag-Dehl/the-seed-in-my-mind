use super::keys::{PrivateKey, PublicKey};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Signature(pub Vec<u8>);

pub trait Signer {
    fn sign(&self, key: &PrivateKey, message: &[u8]) -> Signature;
}

pub trait Verifier {
    fn verify(&self, key: &PublicKey, message: &[u8], signature: &Signature) -> bool;
}
