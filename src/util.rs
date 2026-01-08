use std::io::Write;

use anyhow::Result;

pub trait CompilerPass {
    fn num_errors(&self) -> u32;
    fn inc_error(&mut self);
    fn has_error(&self) -> bool {
        self.num_errors() > 0
    }
}

pub trait Writable {
    fn write<T: Write>(&self, writer: &mut T) -> Result<()>;
}
