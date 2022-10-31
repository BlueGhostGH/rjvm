use std::fmt;

#[derive(Debug)]
pub struct Cursor<'a>
{
    bytes: &'a [u8],
}

impl<'a> Cursor<'a>
{
    pub fn new(bytes: &'a [u8]) -> Self
    {
        Self { bytes }
    }

    pub fn read<const C: usize>(&mut self) -> [u8; C]
    {
        assert!(C <= self.bytes.len());

        let bytes = self.bytes[..C].try_into().unwrap();

        self.bytes = &self.bytes[C..];
        bytes
    }
}
