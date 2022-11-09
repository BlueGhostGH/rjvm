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
        field_refs: Box<[FieldRef]>,
        method_refs: Box<[MethodRef]>,
        strings: Box<[constant::String]>,
        name_and_types: Box<[NameAndType]>,
        utf8s: Box<[Utf8]>,
    }

    impl ConstantPool
    {
        pub(super) fn new(
            constant_pool: &[raw::Constant],
            constant_pool_count: usize,
        ) -> error::Result<Self>
        {
            let mut utf8_index_keeper = IndexKeeper::init(constant_pool_count);
            let utf8s = constant_pool
                .iter()
                .enumerate()
                .filter_map(|(original_index, constant)| {
                    if let raw::Constant::Utf8 { bytes, .. } = constant {
                        utf8_index_keeper.keep(original_index);

                        Some(bytes)
                    } else {
                        None
                    }
                })
                .cloned()
                .map(|bytes| {
                    let bytes = String::from_utf8(bytes.into_vec())
                        .map(String::into_boxed_str)
                        .map_err(error::Error::from)?;

                    Ok(Utf8 { bytes })
                })
                .collect::<error::Result<_>>()?;

            let mut class_index_keeper = IndexKeeper::init(constant_pool_count);
            let classes = constant_pool
                .iter()
                .enumerate()
                .filter_map(|(original_index, constant)| {
                    if let raw::Constant::Class { name_index } = constant {
                        class_index_keeper.keep(original_index);

                        Some(normalise_index(name_index))
                    } else {
                        None
                    }
                })
                .map(|name_index| {
                    if !(1..constant_pool_count).contains(&name_index) {
                        Err(error::Error::OutOfRangeIndex(name_index))?
                    }

                    // This will never fail as we have checked
                    // that our index is within bounds
                    let name = constant_pool.get(name_index).unwrap();
                    if !matches!(name, raw::Constant::Utf8 { .. }) {
                        Err(error::Error::UnexpectedConstantKind {
                            expected: error::ConstantKind::Utf8,
                            actual: name.into(),
                        })?
                    }

                    let name_index = utf8_index_keeper.fetch(name_index);

                    Ok(Class { name_index })
                })
                .collect::<error::Result<_>>()?;

            let strings = constant_pool
                .iter()
                .filter_map(|constant| {
                    if let raw::Constant::String { string_index } = constant {
                        Some(normalise_index(string_index))
                    } else {
                        None
                    }
                })
                .map(|string_index| {
                    if !(1..constant_pool_count).contains(&string_index) {
                        Err(error::Error::OutOfRangeIndex(string_index))?
                    }

                    // This will never fail as we have checked
                    // that our index is within bounds
                    let string = constant_pool.get(string_index).unwrap();
                    if !matches!(string, raw::Constant::Utf8 { .. }) {
                        Err(error::Error::UnexpectedConstantKind {
                            expected: error::ConstantKind::Utf8,
                            actual: string.into(),
                        })?
                    }

                    let string_index = utf8_index_keeper.fetch(string_index);

                    Ok(constant::String { string_index })
                })
                .collect::<error::Result<_>>()?;

            let mut name_and_type_index_keeper = IndexKeeper::init(constant_pool_count);
            let name_and_types = constant_pool
                .iter()
                .enumerate()
                .filter_map(|(original_index, constant)| {
                    if let raw::Constant::NameAndType {
                        name_index,
                        descriptor_index,
                    } = constant
                    {
                        name_and_type_index_keeper.keep(original_index);

                        Some((
                            normalise_index(name_index),
                            normalise_index(descriptor_index),
                        ))
                    } else {
                        None
                    }
                })
                .map(|(name_index, descriptor_index)| {
                    let bounds = 1..constant_pool_count - 1;
                    if !bounds.contains(&name_index) {
                        Err(error::Error::OutOfRangeIndex(name_index))?
                    }
                    if !bounds.contains(&descriptor_index) {
                        Err(error::Error::OutOfRangeIndex(descriptor_index))?
                    }

                    // These will never fail as we have checked
                    // that our indices are within bounds
                    let name = constant_pool.get(name_index).unwrap();
                    let descriptor = constant_pool.get(descriptor_index).unwrap();

                    if !matches!(name, raw::Constant::Utf8 { .. }) {
                        Err(error::Error::UnexpectedConstantKind {
                            expected: error::ConstantKind::Utf8,
                            actual: name.into(),
                        })?
                    }
                    if !matches!(descriptor, raw::Constant::Utf8 { .. }) {
                        Err(error::Error::UnexpectedConstantKind {
                            expected: error::ConstantKind::Utf8,
                            actual: descriptor.into(),
                        })?
                    }

                    let name_index = utf8_index_keeper.fetch(name_index);
                    let descriptor_index = utf8_index_keeper.fetch(descriptor_index);

                    Ok(NameAndType {
                        name_index,
                        descriptor_index,
                    })
                })
                .collect::<error::Result<_>>()?;

            let field_refs = constant_pool
                .iter()
                .filter_map(|constant| {
                    if let raw::Constant::FieldRef {
                        class_index,
                        name_and_type_index,
                    } = constant
                    {
                        Some((
                            normalise_index(class_index),
                            normalise_index(name_and_type_index),
                        ))
                    } else {
                        None
                    }
                })
                .map(|(class_index, name_and_type_index)| {
                    let bounds = 1..constant_pool_count - 1;
                    if !bounds.contains(&class_index) {
                        Err(error::Error::OutOfRangeIndex(class_index))?
                    }
                    if !bounds.contains(&name_and_type_index) {
                        Err(error::Error::OutOfRangeIndex(name_and_type_index))?
                    }

                    // These will never fail as we have checked
                    // that our indices are within bounds
                    let class = constant_pool.get(class_index).unwrap();
                    let name_and_type = constant_pool.get(name_and_type_index).unwrap();

                    if !matches!(class, raw::Constant::Class { .. }) {
                        Err(error::Error::UnexpectedConstantKind {
                            expected: error::ConstantKind::Class,
                            actual: class.into(),
                        })?
                    }
                    if !matches!(name_and_type, raw::Constant::NameAndType { .. }) {
                        Err(error::Error::UnexpectedConstantKind {
                            expected: error::ConstantKind::NameAndType,
                            actual: name_and_type.into(),
                        })?
                    }

                    let class_index = class_index_keeper.fetch(class_index);
                    let name_and_type_index = name_and_type_index_keeper.fetch(name_and_type_index);

                    Ok(FieldRef {
                        class_index,
                        name_and_type_index,
                    })
                })
                .collect::<error::Result<_>>()?;

            let method_refs = constant_pool
                .iter()
                .filter_map(|constant| {
                    if let raw::Constant::MethodRef {
                        class_index,
                        name_and_type_index,
                    } = constant
                    {
                        Some((
                            normalise_index(class_index),
                            normalise_index(name_and_type_index),
                        ))
                    } else {
                        None
                    }
                })
                .map(|(class_index, name_and_type_index)| {
                    let bounds = 1..constant_pool_count - 1;
                    if !bounds.contains(&class_index) {
                        Err(error::Error::OutOfRangeIndex(class_index))?
                    }
                    if !bounds.contains(&name_and_type_index) {
                        Err(error::Error::OutOfRangeIndex(name_and_type_index))?
                    }

                    // These will never fail as we have checked
                    // that our indices are within bounds
                    let class = constant_pool.get(class_index).unwrap();
                    let name_and_type = constant_pool.get(name_and_type_index).unwrap();

                    if !matches!(class, raw::Constant::Class { .. }) {
                        Err(error::Error::UnexpectedConstantKind {
                            expected: error::ConstantKind::Class,
                            actual: class.into(),
                        })?
                    }
                    if !matches!(name_and_type, raw::Constant::NameAndType { .. }) {
                        Err(error::Error::UnexpectedConstantKind {
                            expected: error::ConstantKind::NameAndType,
                            actual: name_and_type.into(),
                        })?
                    }

                    let class_index = class_index_keeper.fetch(class_index);
                    let name_and_type_index = name_and_type_index_keeper.fetch(name_and_type_index);

                    Ok(MethodRef {
                        class_index,
                        name_and_type_index,
                    })
                })
                .collect::<error::Result<_>>()?;

            Ok(ConstantPool {
                classes,
                field_refs,
                method_refs,
                strings,
                name_and_types,
                utf8s,
            })
        }
    }

    #[derive(Debug)]
    pub(super) struct Class
    {
        pub(super) name_index: usize,
    }

    #[derive(Debug)]
    pub(super) struct FieldRef
    {
        pub(super) class_index: usize,
        pub(super) name_and_type_index: usize,
    }

    #[derive(Debug)]
    pub(super) struct MethodRef
    {
        pub(super) class_index: usize,
        pub(super) name_and_type_index: usize,
    }

    pub(super) mod constant
    {
        #[derive(Debug)]
        pub(super) struct String
        {
            pub(super) string_index: usize,
        }
    }

    #[derive(Debug)]
    pub(super) struct NameAndType
    {
        pub(super) name_index: usize,
        pub(super) descriptor_index: usize,
    }

    #[derive(Debug)]
    pub(super) struct Utf8
    {
        pub(super) bytes: Box<str>,
    }

    #[derive(Debug)]
    struct IndexKeeper
    {
        indices: Vec<usize>,
        count: usize,
    }

    impl IndexKeeper
    {
        fn init(len: usize) -> Self
        {
            let indices = vec![0; len];
            let count = 0;

            IndexKeeper { indices, count }
        }

        fn keep(&mut self, original_index: usize)
        {
            self.indices[original_index] = self.count;
            self.count += 1;
        }

        fn fetch(&self, original_index: usize) -> usize
        {
            self.indices[original_index]
        }
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
