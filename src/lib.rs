#![allow(dead_code, incomplete_features)]
#![feature(generic_const_exprs)]

mod cursor;
pub mod parse;

pub fn parse_class_file(source: &[u8]) -> parse::error::Result<parse::ClassFile>
{
    parse::ClassFile::parse_bytes(source)
}
