#![allow(dead_code, incomplete_features)]
#![feature(generic_const_exprs)]

use parse::raw;

pub mod parse;

pub fn parse_raw_class_file(source: &[u8]) -> raw::error::Result<raw::ClassFile>
{
    raw::ClassFile::parse_bytes(source)
}

pub fn parse_class(class_file: raw::ClassFile) -> parse::error::Result<parse::Class>
{
    parse::Class::parse_class_file(class_file)
}

pub fn parse(source: &[u8]) -> error::Result<parse::Class>
{
    let class_file = parse::raw::ClassFile::parse_bytes(source)?;
    let class = parse::Class::parse_class_file(class_file)?;

    Ok(class)
}

pub mod error
{
    use std::{error, fmt, result};

    use crate::parse;

    pub type Result<T> = result::Result<T, Error>;

    #[derive(Debug)]
    pub enum Error
    {
        ParseRaw(parse::raw::error::Error),
        Parse(parse::error::Error),
    }

    impl fmt::Display for Error
    {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
        {
            match self {
                Error::ParseRaw(parse_raw_err) => {
                    write!(f, "{parse_raw_err}")
                }
                Error::Parse(parse_err) => {
                    write!(f, "{parse_err}")
                }
            }
        }
    }

    impl error::Error for Error
    {
        fn source(&self) -> Option<&(dyn error::Error + 'static)>
        {
            match self {
                Error::ParseRaw(parse_raw_err) => Some(parse_raw_err),
                Error::Parse(parse_err) => Some(parse_err),
                #[allow(unreachable_patterns)]
                _ => None,
            }
        }
    }

    impl From<parse::raw::error::Error> for Error
    {
        fn from(parse_raw_err: parse::raw::error::Error) -> Self
        {
            Error::ParseRaw(parse_raw_err)
        }
    }

    impl From<parse::error::Error> for Error
    {
        fn from(parse_err: parse::error::Error) -> Self
        {
            Error::Parse(parse_err)
        }
    }
}
