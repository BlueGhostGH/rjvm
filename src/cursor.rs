use std::mem;

#[derive(Debug)]
pub struct Cursor<'a>
{
    bytes: &'a [u8],
}

impl<'a> Cursor<'a>
{
    pub const fn new(bytes: &'a [u8]) -> Self
    {
        Self { bytes }
    }

    pub fn read_integer<I>(&mut self) -> I
    where
        I: Integer,
        [(); I::SIZE]:,
    {
        I::from_be_bytes(self.read::<{ I::SIZE }>())
    }

    fn read<const C: usize>(&mut self) -> [u8; C]
    {
        assert!(C <= self.bytes.len());

        let bytes = self.bytes[..C]
            .try_into()
            .expect("Failed to turn bytes slice ref to bytes array");

        self.bytes = &self.bytes[C..];
        bytes
    }
}

pub trait Integer: Sized
{
    const SIZE: usize = mem::size_of::<Self>();

    fn from_be_bytes(bytes: [u8; Self::SIZE]) -> Self;
}

impl Integer for u8
{
    fn from_be_bytes(bytes: [u8; Self::SIZE]) -> Self
    {
        Self::from_be_bytes(bytes)
    }
}
impl Integer for u16
{
    fn from_be_bytes(bytes: [u8; Self::SIZE]) -> Self
    {
        Self::from_be_bytes(bytes)
    }
}
impl Integer for u32
{
    fn from_be_bytes(bytes: [u8; Self::SIZE]) -> Self
    {
        Self::from_be_bytes(bytes)
    }
}
