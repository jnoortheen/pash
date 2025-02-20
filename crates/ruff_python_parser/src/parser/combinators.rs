use ruff_text_size::TextRange;

use crate::ParseErrorType;
use crate::parser::Parser;
use crate::token::TokenKind;
use anyhow::Result;

pub(super) type ParseResult<T> = Result<T, ParseErrorType>;

/// A parser combinator trait
pub(super) trait Combinator: std::fmt::Debug {
    type Output;

    fn parse(&self, parser: &mut Parser<'_>) -> ParseResult<Self::Output>;
}

/// Match a specific token kind
impl Combinator for TokenKind {
    type Output = TextRange;

    fn parse(&self, parser: &mut Parser<'_>) -> ParseResult<Self::Output> {
        if parser.at(*self) {
            let range = parser.current_token_range();
            parser.bump_any();
            Ok(range)
        } else {
            Err(ParseErrorType::ExpectedToken {
                expected: *self,
                found: parser.current_token_kind(),
            })
        }
    }
}

/// Sequence combinator - runs parsers in sequence
impl<T: Combinator, const N: usize> Combinator for [T; N] {
    type Output = [T::Output; N];

    fn parse(&self, parser: &mut Parser<'_>) -> ParseResult<Self::Output> {
        let mut results = Vec::with_capacity(N);
        for p in self {
            results.push(p.parse(parser)?);
        }
        results.try_into().map_err(|_| todo!("Handle error"))
    }
}
