use chumsky::{
    extra::Err,
    input::{Input, Stream, ValueInput},
    prelude::*,
    span::SimpleSpan,
    Parser,
};

use crate::{
    ast::{
        exprs::{Expr, ExprKind, Literal},
        patterns::PatternKind,
    },
    lexer::{lex, SourceFile, Token},
};

pub fn token_stream<'tok, 'src: 'tok>(
    src: &'src SourceFile,
) -> impl ValueInput<'tok, Span = SimpleSpan, Token = Token<'src>> {
    let token_iter = lex(src);
    Stream::from_iter(token_iter).map((0..src.source.len()).into(), |(t, s)| (t, s))
}

pub fn parser<'tok, 'src: 'tok, I>() -> impl Parser<'tok, I, Vec<Expr>, Err<Rich<'tok, Token<'src>>>>
where
    I: ValueInput<'tok, Span = SimpleSpan, Token = Token<'src>>,
{
    let pattern = select! {
        Token::Identifier(s) => ExprKind::Pattern(PatternKind::Single(s.into()).into()).into(),
    };
    let literal = select! {
        Token::IntLiteral(i) => Literal::Int(i.parse::<u32>().unwrap()).into()
    };
    choice((pattern, literal)).repeated().collect::<Vec<Expr>>()
}

pub fn print_errors<'tok, 'src: 'tok>(
    source: &ariadne::Source,
    errs: Vec<Rich<'tok, Token<'src>>>,
) {
    for err in errs {
        let reason = err.reason().clone().map_token(|t| t.to_string());
        ariadne::Report::build(ariadne::ReportKind::Error, ((), err.span().into_range()))
            .with_config(ariadne::Config::new().with_index_type(ariadne::IndexType::Byte))
            .with_code(245)
            .with_message(err.to_string())
            .with_label(
                ariadne::Label::new(((), err.span().into_range()))
                    .with_message(reason.to_string())
                    .with_color(ariadne::Color::Red),
            )
            .finish()
            .eprint(source)
            .unwrap();
    }
}
