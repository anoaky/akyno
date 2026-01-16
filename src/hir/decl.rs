use type_equalities::TypeEq;

use crate::ast::Type;

pub trait DeclKind {}

pub enum VarDeclType {}
impl DeclKind for VarDeclType {}

pub enum Decl<T: DeclKind> {
    VarDecl(TypeEq<T, VarDeclType>, Type, String),
}
