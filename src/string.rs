use core::{fmt::Error, str::from_utf8};
use core::fmt::{Display, Write};

use crate::hash::{djb2_hash, Hash};
pub struct SString<const C: usize>
{
    buffer: [u8;C],
}

impl<const C: usize> SString<C>
{
    pub fn new() -> Self
    {
        return Self{
            buffer: [0;C]
        }
    }

    pub fn as_str(&self) -> Result<&str, Error>
    {
        return from_utf8(&self.buffer)
        .map_err(|_| Error)
    }

    pub fn format(&mut self, args: core::fmt::Arguments) -> Result<(), Error> {
        // Clear the current buffer.
        self.buffer = [0; C];
        self.write_fmt(args)
    }
}

impl<const C: usize> Write for SString<C> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let current_len = match self.buffer.iter().position(|&c| c == 0)
        {
            Some(len) => len,
            None => return Err(Error)
        };
        let new_size = current_len + s.len();
        if current_len + s.len() > C {
            return Err(core::fmt::Error);
        }
        let current = &mut self.buffer[current_len..new_size];
        current.copy_from_slice(s.as_bytes());
        Ok(())
    }
}

impl<const C:usize> Hash for SString<C>
{
    fn hash(&self) -> usize {
        let current_len = match self.buffer.iter().position(|&c| c == 0)
        {
            Some(len) => len,
            None => self.buffer.len()
        };
        djb2_hash(&self.buffer[..current_len])
    }
}

impl<const C:usize> Display for SString<C>
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str()?)
    }
}

#[macro_export]
macro_rules! sformat {
    ($size:expr, $($arg:tt)*) => {{
        let mut s = SString::<$size>::new();
        s.format(format_args!($($arg)*)).unwrap();
        s
    }};
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_hash()
    {
        let test = sformat!(32, "Hello World!");
        let hash = test.hash();
        println!("This is the string: {}", test);
        println!("This is the hash: {}", hash);
        let test = sformat!(32, "Hello World.");
        let hash = test.hash();
        println!("This is the string: {}", test);
        println!("This is the hash: {}", hash);
    }
}
