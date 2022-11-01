#![allow(dead_code)]

use std::fmt;

use crate::cursor::Cursor;

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

            let mut pool = Vec::with_capacity(count as usize - 1);
            for _ in 0..count - 1 {
                let tag = cursor.read_integer::<u8>();

                let constant = match tag {
                    7 => {
                        let name_index = cursor.read_integer::<u16>();

                        Constant::Class { name_index }
                    }

                    9 => {
                        dbg!(&pool);
                        todo!("FieldRef")
                    }

                    10 => {
                        let class_index = cursor.read_integer::<u16>();
                        let name_and_type_index = cursor.read_integer::<u16>();

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
                        dbg!(&pool);
                        todo!("NameAndType")
                    }

                    1 => {
                        dbg!(&pool);
                        todo!("Utf8")
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

        Self {
            magic,

            minor,
            major,

            cp,
        }
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
    // NameAndType
    // Utf8
    // MethodHandle
    // MethodType
    // InvokeDynamic
}
