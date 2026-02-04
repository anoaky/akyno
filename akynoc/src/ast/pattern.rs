use serde::Serialize;

use crate::{ast::types::Ident, util::NodeId};

#[derive(Clone, Serialize)]
pub struct Pattern {
    pub id: NodeId,
    pub kind: PatternKind,
}

#[derive(Clone, Serialize)]
pub enum PatternKind {
    RangePattern(Ident, Range),
}

#[derive(Clone, Copy, Serialize)]
pub enum Range {
    Exclusive(i32, i32),
    Inclusive(i32, i32),
    ExclusiveInclusive(i32, i32),
    InclusiveExclusive(i32, i32),
}

impl From<(Ident, Range)> for Pattern {
    fn from((id, range): (Ident, Range)) -> Self {
        Self {
            id: NodeId::next(),
            kind: PatternKind::RangePattern(id, range),
        }
    }
}
