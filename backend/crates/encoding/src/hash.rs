pub trait Hasher {
    fn hash_bytes(&self, data: &[u8]) -> Vec<u8>;
}

#[derive(Debug, Clone, Copy)]
pub struct Blake3Hasher;

impl Hasher for Blake3Hasher {
    fn hash_bytes(&self, data: &[u8]) -> Vec<u8> {
        hash_bytes(data)
    }
}

pub fn hash_bytes(data: &[u8]) -> Vec<u8> {
    blake3::hash(data).as_bytes().to_vec()
}

pub fn hash_with_domain(domain_tag: &str, payload: &[u8]) -> Vec<u8> {
    let mut buf = Vec::with_capacity(domain_tag.len() + payload.len());
    buf.extend_from_slice(domain_tag.as_bytes());
    buf.extend_from_slice(payload);
    hash_bytes(&buf)
}
