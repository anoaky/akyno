use serde::Serialize;

use crate::{ast::Ast, util::Writable};

#[derive(Serialize, Clone)]
pub enum StmtKind {
    Block {
        decls: Vec<Ast>,
        stmts: Vec<Ast>,
    },
    While {
        expr: Box<Ast>,
        stmt: Box<Ast>,
    },
    If {
        expr: Box<Ast>,
        then: Box<Ast>,
        els: Option<Box<Ast>>,
    },
    Return(Option<Box<Ast>>),
    ExprStmt(Box<Ast>),
    Break,
    Continue,
}

impl Writable for StmtKind {
    fn write<T: std::io::Write>(&self, writer: &mut T) -> anyhow::Result<()> {
        match self {
            Self::Block { decls, stmts } => {
                writeln!(writer, "{{")?;
                for decl in decls {
                    write!(writer, "\t")?;
                    decl.write(writer)?;
                }
                for stmt in stmts {
                    write!(writer, "\t")?;
                    stmt.write(writer)?;
                }
                writeln!(writer, "}}")?;
            }
            Self::While { expr, stmt } => {
                write!(writer, "while (")?;
                expr.write(writer)?;
                writeln!(writer, ") ")?;
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
