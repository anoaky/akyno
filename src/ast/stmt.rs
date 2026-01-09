use std::io::Write;

use serde::Serialize;

use crate::{
    ast::{DeclKind, ExprKind},
    util::{Writable, Writer},
};

#[derive(Serialize, Clone)]
pub enum StmtKind {
    Block {
        stmts: Vec<StmtKind>,
    },
    While {
        expr: Box<ExprKind>,
        stmt: Box<StmtKind>,
    },
    If {
        expr: Box<ExprKind>,
        then: Box<StmtKind>,
        els: Option<Box<StmtKind>>,
    },
    Decl(DeclKind),
    Return(Option<Box<ExprKind>>),
    ExprStmt(Box<ExprKind>),
    Break,
    Continue,
}

impl Writable for StmtKind {
    fn write<T: std::io::Write>(&self, writer: &mut Writer<'_, T>) -> anyhow::Result<()> {
        match self {
            Self::Block { stmts } => {
                writeln!(writer, "{{")?;
                writer.inctabs();
                for stmt in stmts {
                    writer.tabs()?;
                    stmt.write(writer)?;
                }
                writer.dectabs();
                writer.tabs()?;
                writeln!(writer, "}}")?;
            }
            Self::While { expr, stmt } => {
                write!(writer, "while (")?;
                expr.write(writer)?;
                write!(writer, ") ")?;
                stmt.write(writer)?;
            }
            Self::If { expr, then, els } => {
                write!(writer, "if (")?;
                expr.write(writer)?;
                write!(writer, ") ")?;
                then.write(writer)?;
                if let Some(els) = els {
                    write!(writer, "\n else ")?;
                    els.write(writer)?;
                }
            }
            Self::Decl(decl) => decl.write(writer)?,
            Self::Return(rv) => {
                write!(writer, "return")?;
                if let Some(rv) = rv {
                    write!(writer, " ")?;
                    rv.write(writer)?;
                }
            }
            Self::ExprStmt(expr) => {
                expr.write(writer)?;
                writeln!(writer, ";")?;
            }
            Self::Break => write!(writer, "break")?,
            Self::Continue => write!(writer, "continue")?,
        }
        Ok(())
    }
}
