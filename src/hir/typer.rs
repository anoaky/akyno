use anyhow::{bail, Result};

use crate::util::{CompilerPass, NodeRef};

use super::Scope;

pub struct Typer<'a> {
    struct_decls: Scope<'a, NodeRef>,
    fun_decls: Scope<'a, NodeRef>,
    errors: u32,
}

impl<'a> CompilerPass for Typer<'a> {
    fn inc_error(&mut self) {
        self.errors += 1;
    }
    fn num_errors(&self) -> u32 {
        self.errors
    }
}

impl<'a> Typer<'a> {
    pub fn new() -> Self {
        Self {
            struct_decls: Scope::new(),
            fun_decls: Scope::new(),
            errors: 0,
        }
    }

    fn error(&mut self, err: String) -> Result<()> {
        self.inc_error();
        bail!("{}", err);
    }
}
