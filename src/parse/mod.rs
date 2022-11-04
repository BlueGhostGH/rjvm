use std::fmt;

mod cursor;
pub mod raw;

use raw::ClassFile;

#[derive(Debug)]
pub struct Class
{
    magic: Magic,
    version: Version,

    constant_pool: constant_pool::ConstantPool,
}

impl Class
{
    pub(crate) fn parse_class_file(class_file: ClassFile) -> error::Result<Self>
    {
        let magic = Magic(class_file.magic);

        let version = Version(class_file.major, class_file.minor);

        let constant_pool = constant_pool::ConstantPool::new(
            &class_file.constant_pool,
            class_file.constant_pool_count as usize,
        )?;

        Ok(Class {
            magic,
            version,

            constant_pool,
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

struct Version(u16, u16);

impl fmt::Debug for Version
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        write!(f, "{}.{}", self.0, self.1)
    }
}

mod constant_pool
{
    use crate::raw;

    fn normalise_index(index: &u16) -> usize
    {
        (*index) as usize - 1
    }

    #[derive(Debug)]
    pub(super) struct ConstantPool
    {
        classes: Box<[Class]>,
        name_and_types: Box<[NameAndType]>,
    }

    impl ConstantPool
    {
        pub(super) fn new(
            constant_pool: &[raw::Constant],
            constant_pool_count: usize,
        ) -> error::Result<Self>
        {
            let classes = constant_pool
                .iter()
                .filter_map(|constant| {
                    if let raw::Constant::Class { name_index } = constant {
                        Some(normalise_index(name_index))
                    } else {
                        None
                    }
                })
                .map(|name_index| {
                    if !(1..constant_pool_count).contains(&name_index) {
                        Err(error::Error::OutOfRangeIndex(name_index))?
                    } else {
                        // This will never fail as we have checked
                        // that our index is within bounds
                        let name = constant_pool.get(name_index).unwrap();

                        let name = if let raw::Constant::Utf8 { bytes, .. } = name {
                            String::from_utf8(bytes.clone().into_vec())
                                .map(String::into_boxed_str)
                                .map_err(error::Error::from)
                        } else {
                            Err(error::Error::UnexpectedConstantKind {
                                expected: error::ConstantKind::Utf8,
                                actual: name.into(),
                            })
                        }?;

                        Ok(Class { name })
                    }
                })
                .collect::<error::Result<_>>()?;

            let name_and_types = constant_pool
                .iter()
                .filter_map(|constant| {
                    if let raw::Constant::NameAndType {
                        name_index,
                        descriptor_index,
                    } = constant
                    {
                        Some((
                            normalise_index(name_index),
                            normalise_index(descriptor_index),
                        ))
                    } else {
                        None
                    }
                })
                .map(|(name_index, descriptor_index)| {
                    match 1..constant_pool_count - 1 {
                        range if !range.contains(&name_index) => {
                            Err(error::Error::OutOfRangeIndex(name_index))?
                        }
                        range if !range.contains(&descriptor_index) => {
                            Err(error::Error::OutOfRangeIndex(descriptor_index))?
                        }
                        _ => {
                            // These will never fail as we have checked
                            // that our indices are within bounds
                            let name = constant_pool.get(name_index).unwrap();
                            let descriptor = constant_pool.get(descriptor_index).unwrap();

                            let name = if let raw::Constant::Utf8 { bytes, .. } = name {
                                String::from_utf8(bytes.clone().into_vec())
                                    .map(String::into_boxed_str)
                                    .map_err(error::Error::from)
                            } else {
                                Err(error::Error::UnexpectedConstantKind {
                                    expected: error::ConstantKind::Utf8,
                                    actual: name.into(),
                                })
                            }?;
                            let descriptor = if let raw::Constant::Utf8 { bytes, .. } = descriptor {
                                String::from_utf8(bytes.clone().into_vec())
                                    .map(String::into_boxed_str)
                                    .map_err(error::Error::from)
                            } else {
                                Err(error::Error::UnexpectedConstantKind {
                                    expected: error::ConstantKind::Utf8,
                                    actual: descriptor.into(),
                                })
                            }?;

                            Ok(NameAndType { name, descriptor })
                        }
                    }
                })
                .collect::<error::Result<_>>()?;

            Ok(ConstantPool {
                classes,
                name_and_types,
            })
        }
    }

    #[derive(Debug)]
    pub(super) struct Class
    {
        pub(super) name: Box<str>,
    }

    #[derive(Debug)]
    pub(super) struct NameAndType
    {
        pub(super) name: Box<str>,
        pub(super) descriptor: Box<str>,
    }

    pub mod error
    {
        use std::{error, fmt, result, str, string};

        use crate::raw;

        pub(super) type Result<T> = result::Result<T, Error>;

        #[derive(Debug)]
        pub enum ConstantKind
        {
            Class,
            FieldRef,
            MethodRef,
            String,
            NameAndType,
            Utf8,
        }

        impl From<&raw::Constant> for ConstantKind
        {
            fn from(value: &raw::Constant) -> Self
            {
                match value {
                    raw::Constant::Class { .. } => ConstantKind::Class,
                    raw::Constant::FieldRef { .. } => ConstantKind::FieldRef,
                    raw::Constant::MethodRef { .. } => ConstantKind::MethodRef,
                    raw::Constant::String { .. } => ConstantKind::String,
                    raw::Constant::NameAndType { .. } => ConstantKind::NameAndType,
                    raw::Constant::Utf8 { .. } => ConstantKind::Utf8,
                }
            }
        }

        #[derive(Debug)]
        pub enum Error
        {
            OutOfRangeIndex(usize),
            UnexpectedConstantKind
            {
                expected: ConstantKind,
                actual: ConstantKind,
            },
            Utf8(str::Utf8Error),
        }

        impl fmt::Display for Error
        {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
            {
                match self {
                    Error::OutOfRangeIndex(index) => {
                        write!(f, "out of range constant pool index {index}")
                    }
                    Error::UnexpectedConstantKind { expected, actual } => {
                        write!(
                            f,
                            "expected {} constant, but instead got a {} constant",
                            format!("{expected:?}").to_ascii_lowercase(),
                            format!("{actual:?}").to_ascii_lowercase()
                        )
                    }
                    Error::Utf8(utf8_err) => write!(f, "{utf8_err}"),
                }
            }
        }

        impl error::Error for Error
        {
            fn source(&self) -> Option<&(dyn error::Error + 'static)>
            {
                match self {
                    Error::Utf8(utf8_err) => Some(utf8_err),
                    _ => None,
                }
            }
        }

        impl From<string::FromUtf8Error> for Error
        {
            fn from(utf8_err: string::FromUtf8Error) -> Self
            {
                Error::Utf8(utf8_err.utf8_error())
            }
        }
    }
}

pub mod error
{
    use std::{error, fmt, result};

    use crate::parse::constant_pool;

    pub type Result<T> = result::Result<T, Error>;

    #[derive(Debug)]
    pub enum Error
    {
        ConstantPool(constant_pool::error::Error),
    }

    impl fmt::Display for Error
    {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
        {
            match self {
                Error::ConstantPool(constant_pool_err) => {
                    write!(f, "{constant_pool_err}")
                }
            }
        }
    }

    impl error::Error for Error
    {
        fn source(&self) -> Option<&(dyn error::Error + 'static)>
        {
            match self {
                Error::ConstantPool(constant_pool_err) => Some(constant_pool_err),
                #[allow(unreachable_patterns)]
                _ => None,
            }
        }
    }

    impl From<constant_pool::error::Error> for Error
    {
        fn from(constant_pool_err: constant_pool::error::Error) -> Self
        {
            Error::ConstantPool(constant_pool_err)
        }
    }
}
