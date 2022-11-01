#![allow(dead_code, incomplete_features)]
#![feature(generic_const_exprs)]

mod cursor;

use std::fmt;

use self::cursor::Cursor;
pub use self::error::{Error, Result};

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
    pub fn parse_bytes(bytes: &[u8]) -> Result<Self>
    {
        let mut cursor = Cursor::new(bytes);

        let magic = Magic(cursor.read_integer::<u32>()?);

        let minor = cursor.read_integer::<u16>()?;
        let major = cursor.read_integer::<u16>()?;

        let cp = {
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
                        dbg!(&pool);
                        todo!("FieldRef")
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
                        dbg!(&pool);
                        todo!("String")
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

                    _ => panic!("Unexpected constant tag"),
                };

                pool.push(constant);
            }

            ConstantPool { pool }
        };

        Ok(Self {
            magic,

            minor,
            major,

            cp,
        })
    }
}

struct Magic(u32);

impl fmt::Debug for Magic
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        write!(f, "{:x?}", self.0)
    }
}

#[derive(Debug)]
struct ConstantPool
{
    pool: Vec<Constant>,
}

#[derive(Debug)]
enum Constant
{
    Class
    {
        name_index: u16
    },
    // Field
    MethodRef
    {
        class_index: u16,
        name_and_type_index: u16,
    },
    // InterfaceMethodRef
    // String
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

mod error
{
    use std::{error, fmt, result};

    use crate::cursor;

    pub type Result<T> = result::Result<T, Error>;

    #[derive(Debug)]
    pub enum Error
    {
        Cursor(cursor::Error),
        ReadPastEnd
        {
            tried: usize,
            left: usize,
        },
    }

    impl fmt::Display for Error
    {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
        {
            match self {
                Error::Cursor(cursor_err) => write!(f, "{cursor_err}"),
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
