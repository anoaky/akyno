use std::{io::Write, rc::Rc};

use serde::Serialize;

use super::{types::Type, BoundExpr, BoundStmt};
use crate::{
    ast::{ExprKind, StmtKind},
    util::{Writable, Writer},
};

#[derive(Serialize, Clone)]
pub enum DeclKind {
    MultiVarDecl(Type, Vec<String>, Vec<Option<ExprKind>>),
    VarDecl(Type, String, Option<ExprKind>),
    StructTypeDecl(Type, String, Vec<DeclKind>),
    FunDecl(Type, String, Vec<DeclKind>),
    FunDefn {
        decl: Box<DeclKind>,
        block: Box<StmtKind>,
    },
}

#[derive(Serialize, Clone)]
pub enum BoundDecl {
    VarDecl {
        ty: Type,
        name: String,
        expr: Option<BoundExpr>,
    },
    StructTypeDecl(Type, String, Vec<Rc<BoundDecl>>),
    FunDecl {
        ty: Type,
        name: String,
        params: Vec<Rc<BoundDecl>>,
    },
    FunDefn {
        decl: Rc<BoundDecl>,
        block: Box<BoundStmt>,
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

impl BoundDecl {
    pub fn name(&self) -> String {
        match self {
            Self::VarDecl {
                ty: _,
                name,
                expr: _,
            }
            | Self::StructTypeDecl(_, name, _)
            | Self::FunDecl {
                ty: _,
                name,
                params: _,
            } => name.clone(),
            Self::FunDefn { decl, block: _ } => decl.name(),
        }
    }
}
