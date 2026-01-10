use std::io::Write;

use serde::Serialize;

use super::types::Type;
use crate::{
    ast::{Ast, ExprKind, StmtKind},
    util::{Writable, Writer},
};

#[derive(Serialize, Clone)]
pub enum DeclKind {
    MultiVarDecl(Type, Vec<String>, Vec<Option<ExprKind>>),
    VarDecl(Type, String, Option<ExprKind>),
    StructTypeDecl(Type, String, Vec<DeclKind>),
    FunDecl(Type, String, Vec<DeclKind>),
    FunDefn {
        decl: Box<Ast>,
        block: Box<StmtKind>,
    },
}

impl Writable for DeclKind {
    fn write<T: std::io::Write>(
        &self,
        writer: &mut Writer<'_, T>,
        eol: bool,
    ) -> anyhow::Result<()> {
        match self {
            DeclKind::MultiVarDecl(t, s, e) => {
                t.write(writer, false)?;
                for i in 0..s.len() {
                    let id = &s[i];
                    let expr = &e[i];
                    write!(writer, " {}", id)?;
                    if let Some(expr) = expr {
                        write!(writer, " = ")?;
                        expr.write(writer, false)?;
                    }
                    if i < s.len() - 1 {
                        write!(writer, ",")?;
                    }
                }
            }
            DeclKind::VarDecl(t, s, e) => {
                t.write(writer, false)?;
                write!(writer, " {}", s)?;
                if let Some(e) = e {
                    write!(writer, " = ")?;
                    e.write(writer, false)?;
                }
            }
            DeclKind::StructTypeDecl(_, name, fields) => {
                writeln!(writer, "struct {} {{", name)?;
                for field in fields {
                    write!(writer, "\t")?;
                    field.write(writer, true)?;
                }
                write!(writer, "}}")?;
            }
            DeclKind::FunDecl(t, name, params) => {
                t.write(writer, false)?;
                write!(writer, " {}(", name)?;
                let mut delim = "";
                for param in params {
                    write!(writer, "{}", delim)?;
                    param.write(writer, false)?;
                    delim = ", ";
                }
                write!(writer, ")")?;
            }
            DeclKind::FunDefn { decl, block } => {
                decl.write(writer, false)?;
                write!(writer, " ")?;
                block.write(writer, false)?;
            }
        }
        if eol {
            writeln!(writer, ";")?;
        }
        Ok(())
    }
}
