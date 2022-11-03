#![allow(dead_code, incomplete_features)]
#![feature(generic_const_exprs)]

use parse::raw;

pub mod parse;

pub fn parse_raw_class_file(source: &[u8]) -> raw::error::Result<raw::ClassFile>
{
    raw::ClassFile::parse_bytes(source)
}
