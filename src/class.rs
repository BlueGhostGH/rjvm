use std::fmt;

use crate::cursor::Cursor;

struct Magic(u32);

#[derive(Debug)]
pub struct Class
{
    magic: Magic,

    minor: u16,
    major: u16,

    cp: ConstantPool,
}

impl Class
{
    #[must_use]
    pub fn from_bytes(bytes: &[u8]) -> Self
    {
        let mut cursor = Cursor::new(bytes);

        let magic = Magic(cursor.read_integer::<u32>());

        let minor = cursor.read_integer::<u16>();
        let major = cursor.read_integer::<u16>();

        let cp = {
            let count = cursor.read_integer::<u16>();
            ConstantPool { count }
        };

        Self {
            magic,

            minor,
            major,

            cp,
        }
    }
}

#[derive(Debug)]
struct ConstantPool
{
    count: u16,
}

impl fmt::Debug for Magic
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        write!(f, "{:x?}", self.0)
    }
}
