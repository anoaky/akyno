use std::{
    io::{stdout, BufWriter},
    path::Path,
};

use anyhow::Result;
use toyc::{
    ast::DeclKind,
    hir::{Binder, Scope},
    lexer::{Category, Tokeniser},
    parser::Parser,
    util::{CompilerPass, Writable, Writer},
};

const PARSER_FAIL: i32 = 245;

fn parse(tokeniser: Tokeniser) -> Result<Vec<DeclKind>> {
    let mut parser = Parser::with_tokeniser(tokeniser)?;
    let program = parser.parse()?;
    if parser.has_error() {
        std::process::exit(PARSER_FAIL);
    }
    Ok(program)
}
pub fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let mut tokeniser = Tokeniser::from_path(Path::new(&args[2]))?;

    if args[1] == "-lexer" {
        let mut t = tokeniser.next_token()?;
        while t.category() != Category::Eof {
            println!("{}", t);
            t = tokeniser.next_token()?;
        }
    } else if args[1] == "-parser" {
        let program = parse(tokeniser)?;
        let mut out = BufWriter::new(stdout());
        let mut writer = Writer::new(&mut out);
        for decl in program {
            decl.write(&mut writer, true)?;
        }
    } else if args[1] == "-sem" {
        let program = parse(tokeniser)?;
        let mut binder = Binder::new();
        let bound_ast = binder.bind(program, &mut Scope::new())?;
        println!("Name analysis successful");
    } else {
        usage();
    }
    Ok(())
}

fn usage() -> ! {
    println!("Usage: toycc <pass> <file>");
    println!("Available passes: -lexer, -parser");
    std::process::exit(-1);
}
