use std::fmt;

use crate::cursor::Cursor;

pub struct Class
{
    magic: [u8; 4],
}

impl fmt::Debug for Class
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        f.debug_struct("Class")
            .field("magic", &format!("{:x?}", self.magic))
            .finish()
    }
}

impl Class
{
    pub fn from_bytes(bytes: &[u8]) -> Self
    {
        let mut cursor = Cursor::new(bytes);
        let magic: [_; 4] = cursor.read::<4>();

        Self { magic }
    }
}
