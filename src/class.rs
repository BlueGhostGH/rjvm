use std::fmt;

use crate::cursor::Cursor;

pub struct Class
{
    magic: [u8; 4],

    minor: [u8; 2],
    major: [u8; 2],
}

impl Class
{
    pub fn from_bytes(bytes: &[u8]) -> Self
    {
        let mut cursor = Cursor::new(bytes);

        let magic = cursor.read::<4>();

        let minor = cursor.read::<2>();
        let major = cursor.read::<2>();

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
