use std::io::Write;

use serde::Serialize;

use crate::{
    ast::{Literal, Type},
    util::{Writable, Writer},
};

#[derive(Serialize, Clone)]
pub enum ExprKind {
    InvalidExpr,
    Literal(Literal),
    VarExpr(String),
    BinOp(Box<ExprKind>, OpKind, Box<ExprKind>),
    Assign(Box<ExprKind>, Box<ExprKind>),
    FunCallExpr(Box<ExprKind>, Vec<ExprKind>),
    TypecastExpr(Type, Box<ExprKind>),
    RefExpr(Box<ExprKind>),
    DerefExpr(Box<ExprKind>),
}

#[derive(Serialize, Clone, Copy)]
pub enum OpKind {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Gt,
    Ge,
    Lt,
    Le,
    Eq,
    Ne,
    And,
    Or,
}

impl Writable for OpKind {
    fn write<T: std::io::Write>(&self, writer: &mut Writer<'_, T>, _: bool) -> anyhow::Result<()> {
        use OpKind::*;
        let s = match self {
            Add => "+",
            Sub => "-",
            Mul => "*",
            Div => "/",
            Mod => "%",
            Gt => ">",
            Ge => ">=",
            Lt => "<",
            Le => "<=",
            Eq => "==",
            Ne => "!=",
            And => "&&",
            Or => "||",
        };
        write!(writer, "{}", s)?;
        Ok(())
    }
}

impl Writable for ExprKind {
    fn write<T: std::io::Write>(
        &self,
        writer: &mut Writer<'_, T>,
        eol: bool,
    ) -> anyhow::Result<()> {
        match self {
            Self::InvalidExpr => write!(writer, "Invalid expression")?,
            Self::Literal(l) => l.write(writer, false)?,
            Self::VarExpr(s) => write!(writer, "{}", s)?,
            Self::BinOp(lhs, op, rhs) => {
                write!(writer, "(")?;
                lhs.write(writer, false)?;
                write!(writer, " ")?;
                op.write(writer, false)?;
                write!(writer, " ")?;
                rhs.write(writer, false)?;
                write!(writer, ")")?;
            }
            Self::Assign(lhs, rhs) => {
                lhs.write(writer, false)?;
                write!(writer, " = ")?;
                rhs.write(writer, false)?;
            }
            Self::FunCallExpr(name, args) => {
                name.write(writer, false)?;
                write!(writer, "(")?;
                let mut delim = "";
                for arg in args {
                    write!(writer, "{}", delim)?;
                    delim = ", ";
                    arg.write(writer, false)?;
                }
                write!(writer, ")")?;
            }
            Self::TypecastExpr(t, expr) => {
                write!(writer, "(")?;
                t.write(writer, false)?;
                write!(writer, ")")?;
                expr.write(writer, false)?;
            }
            Self::RefExpr(expr) => {
                write!(writer, "&")?;
                expr.write(writer, false)?;
            }
            Self::DerefExpr(expr) => {
                write!(writer, "*")?;
                expr.write(writer, false)?;
            }
        };
        if eol {
            writeln!(writer, ";")?;
        }
        Ok(())
    }
}
