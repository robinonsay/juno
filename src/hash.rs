

pub fn djb2_hash(data: &[u8]) -> usize
{
    let mut hash: usize = 5381;
    for byte in data
    {
        hash = hash.wrapping_shl(5).wrapping_add(hash).wrapping_add(*byte as usize);
    }
    return hash;
}

pub trait Hash
{
    fn hash(&self) -> usize;
}
