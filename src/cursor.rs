use std::mem;

pub use self::error::{Error, Result};

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

    pub fn read_integer<I>(&mut self) -> Result<I>
    where
        I: Integer,
        [(); I::SIZE]:,
    {
        self.read::<{ I::SIZE }>().map(I::from_be_bytes)
    }

    pub fn read_bytes(&mut self, count: usize) -> Result<Box<[u8]>>
    {
        if count > self.bytes.len() {
            Err(Error::ReadPastEnd {
                tried: count,
                left: self.bytes.len(),
            })
        } else {
            let bytes = &self.bytes[..count];
            self.bytes = &self.bytes[count..];

            Ok(bytes.into())
        }
    }

    fn read<const C: usize>(&mut self) -> Result<[u8; C]>
    {
        if C > self.bytes.len() {
            Err(Error::ReadPastEnd {
                tried: C,
                left: self.bytes.len(),
            })
        } else {
            let bytes = &self.bytes[..C];
            self.bytes = &self.bytes[C..];

            // This will never fail as we have sliced off
            // exactly the length of our array from the
            // inner bytes ref slice
            Ok(bytes.try_into().unwrap())
        }
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

mod error
{
    use std::{error, fmt, result};

    pub type Result<T> = result::Result<T, Error>;

    #[derive(Debug)]
    pub enum Error
    {
        ReadPastEnd
        {
            tried: usize, left: usize
        },
    }

    impl fmt::Display for Error
    {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
        {
            match self {
                Error::ReadPastEnd { tried, left } => {
                    write!(f, "tried reading {tried} bytes when only {left} are left")
                }
            }
        }
    }

    impl error::Error for Error
    {
        fn source(&self) -> Option<&(dyn error::Error + 'static)>
        {
            match self {
                _ => None,
            }
        }
    }
}
