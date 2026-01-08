use anyhow::Result;
use serde::Serialize;

use crate::util::Writable;

#[derive(Serialize, Clone, Debug)]
pub enum Type {
    Int,
    Char,
    Void,
    Unknown,
    None,
    Pointer(Box<Type>),
    Array(usize, Box<Type>),
    Struct(String),
}

impl Writable for Type {
    fn write<T: std::io::Write>(&self, writer: &mut T) -> anyhow::Result<()> {
        use Type::*;
        let mut array_write = |t: Type| -> Result<()> {
            if let Array(s, inner) = t {
                inner.write(writer)?;
                write!(writer, "[{}]", s)?;
            } else {
                t.write(writer)?;
            }
            Ok(())
        };
        match self {
            Int | Char | Void | Unknown | None => write!(writer, "{:?}", self)?,
            Pointer(t) => {
                t.write(writer)?;
                write!(writer, "*")?;
            }
            Struct(s) => write!(writer, "struct {}", s)?,
            Array(_, _) => array_write(self.clone())?,
        };
        Ok(())
    }
}
