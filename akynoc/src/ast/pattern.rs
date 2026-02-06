use serde::Serialize;

use crate::{
    ast::{exprs::Expr, types::Ident},
    util::NodeId,
};

#[derive(Clone, Serialize)]
pub struct Pattern {
    pub id: NodeId,
    pub kind: PatternKind,
}

#[derive(Clone, Serialize)]
pub enum PatternKind {
    RangePattern(Ident, Expr, Expr),
}

impl From<PatternKind> for Pattern {
    fn from(value: PatternKind) -> Self {
        Self {
            id: NodeId::next(),
            kind: value,
        }
    }
}
