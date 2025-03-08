const PRIME:usize = 16777619;


pub fn fnv1a_hash(data: &[u8]) -> usize
{
    let mut hash: usize = 2166136261;
    for byte in data
    {
        hash ^= *byte as usize;
        hash = hash.wrapping_mul(PRIME);
    }
    return hash;
}

pub trait Hash
{
    fn hash(&self) -> usize;
}
