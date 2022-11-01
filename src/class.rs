use std::fmt;

use crate::cursor::Cursor;

pub struct Class
{
    magic: u32,

    minor: u16,
    major: u16,
}

impl Class
{
    #[must_use]
    pub fn from_bytes(bytes: &[u8]) -> Self
    {
        let mut cursor = Cursor::new(bytes);

        let magic = cursor.read_integer::<u32>();

        let minor = cursor.read_integer::<u16>();
        let major = cursor.read_integer::<u16>();

        Self {
            magic,

            minor,
            major,
        }
    }
}

impl fmt::Debug for Class
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        f.debug_struct("Class")
            .field("magic", &format!("{:x?}", self.magic))
            .field("minor", &format!("{:x?}", self.minor))
            .field("major", &format!("{:x?}", self.major))
            .finish()
    }
}
