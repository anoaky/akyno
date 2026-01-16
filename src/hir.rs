mod bind;
mod scope;
mod typer;

pub use scope::Scope;

use std::rc::Rc;

pub use bind::Binder;
use serde::Serialize;
pub use typer::Typer;

use crate::{
    ast::{Literal, Operator, Type},
    util::{NodeId, NodeRef},
};

#[derive(Serialize, Clone)]
pub enum HIRKind {
    Invalid,
    VarDecl(Type, String, Option<NodeRef>),
    StructTypeDecl(Type, String, Vec<NodeRef>),
    FunDecl(Type, String, Vec<NodeRef>),
    FunDefn(NodeRef, NodeRef),

    Literal(Literal),
    VarExpr(NodeRef),
    BinOp(NodeRef, Operator, NodeRef),
    Assign(NodeRef, NodeRef),
    FunCallExpr(NodeRef, Vec<NodeRef>),
    TypecastExpr(Type, NodeRef),
    RefExpr(NodeRef),
    DerefExpr(NodeRef),
    FieldAccessExpr(NodeRef, String),
    ArrayAccessExpr(NodeRef, NodeRef),

    Block(Vec<NodeRef>),
    While(NodeRef, NodeRef),
    If(NodeRef, NodeRef, Option<NodeRef>),
    Return(Option<NodeRef>),
    Break,
    Continue,
}

#[derive(Serialize, Clone)]
pub struct UntypedHir {
    id: NodeId,
    kind: HIRKind,
}

impl UntypedHir {
    fn new(kind: HIRKind) -> Self {
        Self {
            kind,
            ..Self::default()
        }
    }
}

impl Default for UntypedHir {
    fn default() -> Self {
        Self {
            id: NodeId::next(),
            kind: HIRKind::Invalid,
        }
    }
}

#[derive(Serialize, Clone)]
pub struct HirPool<T>(Vec<T>);

impl<T> HirPool<T> {
    pub fn new() -> Self {
        Self(vec![])
    }

    pub fn add(&mut self, node: T) -> NodeRef {
        let new_ref = self.0.len();
        self.0.push(node);
        NodeRef(new_ref)
    }

    pub fn get(&self, node_ref: NodeRef) -> Option<&T> {
        self.0.get(node_ref.0)
    }
}

#[derive(Serialize, Clone)]
pub enum HIRDeclKind {
    VarDecl(Type, String, Option<HIRExprKind>),
    StructTypeDecl(Type, String, Vec<Rc<HIRDeclKind>>),
    FunDecl(Type, String, Vec<Rc<HIRDeclKind>>),
    FunDefn(Rc<HIRDeclKind>, Box<HIRStmtKind>),
}

#[derive(Serialize, Clone)]
pub enum HIRExprKind {
    Invalid,
    Literal(Type, Literal), // potentially redundant ?
    VarExpr(NodeRef),
    BinOp(NodeRef, Operator, NodeRef),
    Assign(NodeRef, NodeRef),
    FunCallExpr(NodeRef, Vec<NodeRef>),
    TypecastExpr(Type, NodeRef),
    RefExpr(NodeRef),
    DerefExpr(NodeRef),
    FieldAccessExpr(NodeRef, String),
    ArrayAccessExpr(NodeRef, NodeRef),
}

#[derive(Serialize, Clone)]
pub enum HIRStmtKind {
    Block(Vec<HIRStmtKind>),
    While(Box<HIRExprKind>, Box<HIRStmtKind>),
    If(Box<HIRExprKind>, Box<HIRStmtKind>, Option<Box<HIRStmtKind>>),
    Decl(Rc<HIRDeclKind>),
    Return(Option<Box<HIRExprKind>>),
    ExprStmt(HIRExprKind),
    Break,
    Continue,
}

#[derive(Serialize, Clone)]
pub struct HIRDecl {
    id: NodeId,
    kind: HIRDeclKind,
}

#[derive(Serialize, Clone)]
pub struct UntypedExpr {
    id: NodeId,
    kind: HIRExprKind,
}

#[derive(Serialize, Clone)]
pub struct TypedExpr {
    id: NodeId,
    ty: Type,
    kind: HIRExprKind,
}

#[derive(Serialize, Clone)]
pub struct HIRStmt {
    id: NodeId,
    kind: HIRStmtKind,
}
