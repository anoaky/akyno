use erased_serde::serialize_trait_object;
use serde::Serialize;

use crate::ast::{
    ASTNode,
    types::{StructType, Type},
};

pub trait Decl: ASTNode {
    fn ty(&self) -> &dyn Type;
    fn name(&self) -> &str;
}
serialize_trait_object!(Decl);

#[derive(Serialize)]
pub struct VarDecl {
    /*
    TODO: stack information needs to go here
    */
    ty: Box<dyn Type>,
    name: String,
}

impl ASTNode for VarDecl {
    fn type_name(&self) -> &'static str {
        "VarDecl"
    }
    fn children(&self) -> Vec<&dyn ASTNode> {
        vec![self.ty.as_ref()]
    }
}

impl Decl for VarDecl {
    fn ty(&self) -> &dyn Type {
        self.ty.as_ref()
    }

    fn name(&self) -> &str {
        &self.name
    }
}

impl VarDecl {
    pub fn new(ty: Box<dyn Type>, name: String) -> Self {
        Self { ty, name }
    }
}

#[derive(Serialize)]
pub struct StructTypeDecl {
    ty: Box<StructType>,
    fields: Vec<VarDecl>,
}

impl ASTNode for StructTypeDecl {
    fn type_name(&self) -> &'static str {
        "StructTypeDecl"
    }
    fn children(&self) -> Vec<&dyn ASTNode> {
        let mut v = vec![];
        for field in &self.fields {
            v.push(field as &dyn ASTNode);
        }
        v
    }
}

impl Decl for StructTypeDecl {
    fn ty(&self) -> &dyn Type {
        self.ty.as_ref()
    }
    fn name(&self) -> &str {
        &self.ty.name
    }
}

impl StructTypeDecl {
    pub fn new(ty: Box<StructType>) -> Self {
        Self { ty, fields: vec![] }
    }

    pub fn get_field(&self, name: String) -> Option<&VarDecl> {
        self.fields.iter().find(|&field| field.name == name)
    }

    pub fn add_var_decl(&mut self, vd: VarDecl) {
        self.fields.push(vd);
    }
}
