pub trait Codec<T> {
    fn encode(value: &T) -> Vec<u8>;
    fn decode(bytes: &[u8]) -> Result<T, String>;
}
