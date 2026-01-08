use anyhow::{Result, bail};
use std::collections::VecDeque;

use crate::ast::decl::VarDecl;
use crate::ast::program::Program;
use crate::ast::types::{BaseType, Type};
use crate::lexer::Category;
use crate::{
    lexer::{Category::*, Token, Tokeniser},
    util::CompilerPass,
};

pub struct Parser {
    errors: u32,
    buffer: VecDeque<Token>,
    tokeniser: Tokeniser,
    token: Token,
    last_error_token: Option<Token>,
}

impl CompilerPass for Parser {
    fn inc_error(&mut self) {
        self.errors += 1;
    }

    fn num_errors(&self) -> u32 {
        self.errors
    }
}

impl Parser {
    pub fn with_tokeniser(mut tokeniser: Tokeniser) -> Result<Self> {
        let first_token = tokeniser.next_token()?;
        Ok(Self {
            errors: 0,
            buffer: VecDeque::new(),
            tokeniser,
            token: first_token,
            last_error_token: None,
        })
    }

    fn accept(&self, expected: Vec<Category>) -> bool {
        expected.contains(&self.token.category())
    }

    fn convert_token(&self, t: Token) -> impl Type {
        match t.category() {
            Category::Int => BaseType::INT,
            Category::Char => BaseType::CHAR,
            Category::Void => BaseType::VOID,
            _ => BaseType::UNKNOWN,
        }
    }

    fn error(&mut self, expected: Vec<Category>) {
        if self
            .last_error_token
            .clone()
            .is_some_and(|t| t == self.token)
        {
            return;
        }
        println!(
            "Parsing error: expected {:?} found {:?} at {:?}",
            expected, self.token, self.token.position
        )
    }

    fn expect(&mut self, expected: Vec<Category>) -> Result<Token> {
        let token = self.token.clone();
        for e in expected.clone() {
            if e == token.category() {
                self.next_token()?;
                return Ok(token);
            }
        }
        self.error(expected);
        Ok(token)
    }

    fn is_fun(&mut self) -> Result<bool> {
        if (!self.accept(vec![Struct, Int, Char, Void])) {
            return Ok(false);
        }

        let mut k = if self.accept(vec![Struct]) { 2 } else { 1 };

        while self.look_ahead(k)?.category() == Asterisk {
            k += 1;
        }

        Ok(self.look_ahead(k + 1)?.category() == LPar)
    }

    fn next_token(&mut self) -> Result<()> {
        if !self.buffer.is_empty() {
            self.token = self.buffer.pop_front().unwrap();
            Ok(())
        } else {
            match self.tokeniser.next_token() {
                Ok(t) => {
                    self.token = t;
                    Ok(())
                }
                Err(_) => bail!("Error reading next token"),
            }
        }
    }

    fn load_buffer(&mut self) -> Result<()> {
        match self.tokeniser.next_token() {
            Ok(t) => {
                self.buffer.push_back(t);
                Ok(())
            }
            Err(_) => bail!("Error reading next token"),
        }
    }

    fn look_ahead(&mut self, i: usize) -> Result<Token> {
        while self.buffer.len() < i {
            self.load_buffer()?;
        }

        match self.buffer.get(i) {
            None => bail!("Failed look ahead"),
            Some(t) => Ok(t.clone()),
        }
    }

    fn parse_includes(&mut self) -> Result<()> {
        if self.accept(vec![Include]) {
            self.next_token()?;
            self.expect(vec![StrLiteral])?;
            self.parse_includes()
        } else {
            Ok(())
        }
    }

    fn parse_program(&mut self) -> Result<Program> {
        self.parse_includes()?;
        let mut decls = vec![];

        while self.accept(vec![Struct, Int, Char, Void]) {}

        self.expect(vec![Eof])?;
        Ok(Program::new(decls))
    }
}
