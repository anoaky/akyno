use std::any::Any;

use crate::ast::ASTNode;

pub trait Type: ASTNode {
    fn equals(&self, other: Box<dyn Any>) -> bool;
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum BaseType {
    INT,
    CHAR,
    VOID,
    UNKNOWN,
    NONE,
}

impl ASTNode for BaseType {
    fn type_name(&self) -> &'static str {
        "BaseType"
    }
    fn children(&self) -> Vec<&dyn ASTNode> {
        vec![]
    }
}

impl Type for BaseType {
    fn equals(&self, other: Box<dyn Any>) -> bool {
        if let Ok(other) = other.downcast::<BaseType>() {
            *self == *other.as_ref()
        } else {
            false
        }
    }
}

pub struct ArrayType {
    pub ty: Box<dyn Type>,
    pub len: usize,
}

impl ASTNode for ArrayType {
    fn type_name(&self) -> &'static str {
        "ArrayType"
    }
    fn children(&self) -> Vec<&dyn ASTNode> {
        vec![self.ty.as_ref()]
    }
}

impl Type for ArrayType {
    fn equals(&self, other: Box<dyn Any>) -> bool {
        if let Ok(other) = other.downcast::<ArrayType>() {
            self.len == other.len && self.ty.equals(other.ty)
        } else {
            false
        }
    }
}

impl ArrayType {
    pub fn new(ty: Box<dyn Type>, len: usize) -> Self {
        Self { ty, len }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_base_type_equals() {
        let t1 = Box::new(BaseType::INT);
        let t2 = Box::new(BaseType::INT);

        assert!(t1.equals(t2));

        let t3 = Box::new(BaseType::CHAR);
        assert!(!t1.equals(t3));
    }

    #[test]
    fn check_array_type_equals() {
        let t1 = Box::new(BaseType::INT);
        let t2 = Box::new(BaseType::INT);
        let at1 = Box::new(ArrayType::new(t1, 2));
        let at2 = Box::new(ArrayType::new(t2, 2));

        assert!(at1.equals(at2));
    }
}
