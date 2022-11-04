use crate::parse::cursor::Cursor;

use self::error::{Error, Result};

#[derive(Debug)]
pub struct ClassFile
{
    pub(super) magic: u32,

    pub(super) minor: u16,
    pub(super) major: u16,

    pub(super) constant_pool_count: u16,
    pub(super) constant_pool: Box<[Constant]>,
}

impl ClassFile
{
    pub(crate) fn parse_bytes(bytes: &[u8]) -> Result<Self>
    {
        let mut cursor = Cursor::new(bytes);

        let magic = cursor.read_integer::<u32>()?;

        let minor = cursor.read_integer::<u16>()?;
        let major = cursor.read_integer::<u16>()?;

        let (constant_pool_count, constant_pool) = {
            let count = cursor.read_integer::<u16>()?;

            let mut pool = Vec::with_capacity(count as usize - 1);
            for _ in 0..count - 1 {
                let tag = cursor.read_integer::<u8>()?;

                let constant = match tag {
                    7 => {
                        let name_index = cursor.read_integer::<u16>()?;

                        Constant::Class { name_index }
                    }

                    9 => {
                        let class_index = cursor.read_integer::<u16>()?;
                        let name_and_type_index = cursor.read_integer::<u16>()?;

                        Constant::FieldRef {
                            class_index,
                            name_and_type_index,
                        }
                    }

                    10 => {
                        let class_index = cursor.read_integer::<u16>()?;
                        let name_and_type_index = cursor.read_integer::<u16>()?;

                        Constant::MethodRef {
                            class_index,
                            name_and_type_index,
                        }
                    }

                    11 => {
                        dbg!(&pool);
                        todo!("InterfaceMethodRef")
                    }

                    8 => {
                        let string_index = cursor.read_integer::<u16>()?;

                        Constant::String { string_index }
                    }

                    3 => {
                        dbg!(&pool);
                        todo!("Integer")
                    }

                    4 => {
                        dbg!(&pool);
                        todo!("Float")
                    }

                    5 => {
                        dbg!(&pool);
                        todo!("Long")
                    }

                    6 => {
                        dbg!(&pool);
                        todo!("Double")
                    }

                    12 => {
                        let name_index = cursor.read_integer::<u16>()?;
                        let descriptor_index = cursor.read_integer::<u16>()?;

                        Constant::NameAndType {
                            name_index,
                            descriptor_index,
                        }
                    }

                    1 => {
                        let length = cursor.read_integer::<u16>()?;
                        let bytes = cursor.read_bytes(length as usize)?;

                        Constant::Utf8 { length, bytes }
                    }

                    15 => {
                        dbg!(&pool);
                        todo!("MethodHandle")
                    }

                    16 => {
                        dbg!(&pool);
                        todo!("MethodType")
                    }

                    18 => {
                        dbg!(&pool);
                        todo!("InvokeDynamic")
                    }

                    tag => Err(Error::UnexpectedConstantTag(tag))?,
                };

                pool.push(constant);
            }

            let pool = pool.into_boxed_slice();

            (count, pool)
        };

        Ok(Self {
            magic,

            minor,
            major,

            constant_pool_count,
            constant_pool,
        })
    }
}

#[derive(Debug)]
pub(super) enum Constant
{
    Class
    {
        name_index: u16
    },

    FieldRef
    {
        class_index: u16,
        name_and_type_index: u16,
    },

    MethodRef
    {
        class_index: u16,
        name_and_type_index: u16,
    },
    // InterfaceMethodRef
    String
    {
        string_index: u16
    },
    // Integer
    // Float
    // Long
    // Double
    NameAndType
    {
        name_index: u16,
        descriptor_index: u16,
    },

    Utf8
    {
        length: u16, bytes: Box<[u8]>
    },
    // MethodHandle
    // MethodType
    // InvokeDynamic
}

pub mod error
{
    use std::{error, fmt, result};

    use crate::parse::cursor;

    pub type Result<T> = result::Result<T, Error>;

    #[derive(Debug)]
    pub enum Error
    {
        Cursor(cursor::Error),
        UnexpectedConstantTag(u8),
    }

    impl fmt::Display for Error
    {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
        {
            match self {
                Error::Cursor(cursor_err) => write!(f, "{cursor_err}"),
                Error::UnexpectedConstantTag(tag) => {
                    write!(f, "unexpected constant tag {tag}")
                }
            }
        }
    }

    impl error::Error for Error
    {
        fn source(&self) -> Option<&(dyn error::Error + 'static)>
        {
            match self {
                Error::Cursor(cursor_err) => Some(cursor_err),
                _ => None,
            }
        }
    }

    impl From<cursor::Error> for Error
    {
        fn from(cursor_err: cursor::Error) -> Self
        {
            Error::Cursor(cursor_err)
        }
    }
}
