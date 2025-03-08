use core::hash::Hasher;



pub struct DjB2Hasher
{
    hash: usize
}

impl DjB2Hasher
{
    pub fn new() -> Self
    {
        return Self { hash: 5381 }
    }
}

impl Hasher for DjB2Hasher
{
    fn finish(&self) -> u64 {
        return self.hash as u64
    }

    fn write(&mut self, bytes: &[u8]) {
        for byte in bytes
        {
            self.hash = self.hash.wrapping_shl(5).wrapping_add(self.hash).wrapping_add(*byte as usize);
        }
    }
}
