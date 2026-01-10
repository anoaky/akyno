use std::io::Write;

use anyhow::Result;
use serde::Serialize;

use crate::util::{Writable, Writer};

use super::Type;

#[derive(Serialize, Clone)]
pub enum Literal {
    Int(i32),
    Char(char),
    Str(String),
    Sizeof(Type),
}

impl Writable for Literal {
    fn write<T: Write>(&self, writer: &mut Writer<'_, T>, _: bool) -> Result<()> {
        match self {
            Literal::Int(i) => write!(writer, "{}", i),
            Literal::Char(c) => write!(writer, "'{}'", c),
            Literal::Str(s) => write!(writer, "\"{}\"", s),
            Literal::Sizeof(t) => {
                write!(writer, "sizeof(")?;
                t.write(writer, false)?;
                write!(writer, ")")
            }
        }?;
        Ok(())
    }
}
