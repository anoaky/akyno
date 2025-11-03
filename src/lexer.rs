use core::fmt;

#[derive(Clone, Copy, Debug)]
pub enum Category {
    Identifier,
    Assign,
    LBrace,
    RBrace,
    LPar,
    RPar,
    LBrack,
    RBrack,
    Semi,
    Comma,
    Int,
    Void,
    Char,
    If,
    Else,
    While,
    Return,
    Struct,
    Sizeof,
    Continue,
    Break,
    Include,
    CharLiteral,
    StrLiteral,
    IntLiteral,
    LogAnd,
    LogOr,
    Eq,
    Ne,
    Gt,
    Lt,
    Le,
    Ge,
    Plus,
    Minus,
    Asterisk,
    Div,
    Rem,
    And,
    Dot,
    Eof,
    Invalid,
}

#[derive(Clone)]
pub struct Token {
    category: Category,
    data: String,
    position: (u8, u8), // line, col
}

impl Token {
    pub fn new(category: Category, data: Option<String>, line: u8, col: u8) -> Self {
        match data {
            Some(s) => Self {
                category,
                data: s,
                position: (line, col),
            },
            None => Self {
                category,
                data: "".to_string(),
                position: (line, col),
            },
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.data.is_empty() {
            write!(f, "{:?}", self.category)
        } else {
            write!(f, "{:?}({})", self.category, self.data)
        }
    }
}
