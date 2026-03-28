use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct KeyId(pub Uuid);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PublicKey(pub Vec<u8>);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrivateKey(pub Vec<u8>);
