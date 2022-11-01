use std::fmt;

use crate::cursor::Cursor;

pub struct Class
{
    magic: u32,

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

        let magic = cursor.read_integer::<u32>();

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

struct ConstantPool
{
    count: u16,
}

impl fmt::Debug for Class
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        f.debug_struct("Class")
            .field("magic", &format!("{:x?}", self.magic))
            .field("minor", &format!("{:x?}", self.minor))
            .field("major", &format!("{:x?}", self.major))
            .field("cp", &self.cp)
            .finish()
    }
}

impl fmt::Debug for ConstantPool
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        f.debug_struct("ConstantPool")
            .field("count", &format!("{:x?}", self.count))
            .finish()
    }
}
