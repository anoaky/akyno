use std::{io::Write, rc::Rc};

use serde::Serialize;

use crate::{
    ast::{DeclKind, ExprKind},
    util::{Writable, Writer},
};

use super::{decl::BoundDecl, expr::BoundExpr};

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

#[derive(Serialize, Clone)]
pub enum BoundStmt {
    Block(Vec<BoundStmt>),
    While(Box<BoundExpr>, Box<BoundStmt>),
    If(Box<BoundExpr>, Box<BoundStmt>, Option<Box<BoundStmt>>),
    Decl(Rc<BoundDecl>),
    Return(Option<Box<BoundExpr>>),
    ExprStmt(Box<BoundExpr>),
    Break,
    Continue,
}

impl Writable for StmtKind {
    fn write<T: std::io::Write>(
        &self,
        writer: &mut Writer<'_, T>,
        eol: bool,
    ) -> anyhow::Result<()> {
        match self {
            Self::Block { stmts } => {
                writeln!(writer, "{{")?;
                writer.inctabs();
                for stmt in stmts {
                    writer.tabs()?;
                    stmt.write(writer, true)?;
                }
                writer.dectabs();
                writer.tabs()?;
                write!(writer, "}}")?;
            }
            Self::While { expr, stmt } => {
                write!(writer, "while (")?;
                expr.write(writer, false)?;
                write!(writer, ") ")?;
                stmt.write(writer, false)?;
            }
            Self::If { expr, then, els } => {
                write!(writer, "if (")?;
                expr.write(writer, false)?;
                write!(writer, ") ")?;
                then.write(writer, !eol && els.is_none())?;
                if let Some(els) = els {
                    write!(writer, " else ")?;
                    els.write(writer, !eol)?;
                }
            }
            Self::Decl(decl) => decl.write(writer, false)?,
            Self::Return(rv) => {
                write!(writer, "return")?;
                if let Some(rv) = rv {
                    write!(writer, " ")?;
                    rv.write(writer, true)?;
                }
            }
            Self::ExprStmt(expr) => {
                expr.write(writer, !eol)?;
            }
            Self::Break => write!(writer, "break")?,
            Self::Continue => write!(writer, "continue")?,
        }
        if eol {
            writeln!(writer, ";")?;
        }
        Ok(())
    }
}
