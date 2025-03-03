use std::collections::HashMap;
use std::vec;

use ruff_python_ast::name::Name;
use ruff_python_ast::{self as ast, DictItem, Expr, ExprContext, ExprDict};
use ruff_text_size::{Ranged, TextRange, TextSize};

use crate::ParseErrorType;
use crate::{
    parser::{Parser, ParserProgress},
    token::TokenKind,
};

use super::combinators::{Combinator as _, ParseResult};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum RedirectType {
    Write,
    Append,
    Read,
}

impl Parser<'_> {
    /// Parses a subprocess expression.
    /// This includes various forms of subprocess capture like `$(...)`, `$[...]`, `!(...)`, and `![...]`.
    pub(super) fn parse_subprocs(&mut self, method: impl Into<Name>, closing: TokenKind) -> Expr {
        let start = self.node_start();

        let mut cmd = self
            .xonsh_attr("cmd", None)
            .call(self.parse_cmd_group(closing), self.node_range(start));
        while self.at(TokenKind::Vbar) {
            let pipe_start = self.node_start();
            self.bump_any();
            cmd = cmd
                .attr("pipe", self.node_range(pipe_start))
                .call(self.parse_cmd_group(closing), self.node_range(pipe_start));
        }
        cmd.attr(method, self.node_range(start))
            .call_empty(self.node_range(start))
    }

    /// Parses a subprocess expression like `ls tmp-dir` with ![]
    pub(super) fn parse_bare_proc(&mut self) -> ast::Stmt {
        let start = self.node_start();
        let expr = self.parse_subprocs("hide", TokenKind::Newline);
        ast::Stmt::Expr(ast::StmtExpr {
            range: self.node_range(start),
            value: Box::new(expr),
        })
    }

    fn parse_cmd_group(&mut self, closing: TokenKind) -> ast::Arguments {
        const REDIR_NAMES: &[&str] = &["o", "out", "e", "err", "a", "all"];
        let start = self.node_start();
        let mut cmds = Vec::new();
        let mut keywords = Vec::new();
        let mut redirects = HashMap::new();
        let mut progress = ParserProgress::default();

        loop {
            match self.current_token_kind() {
                tk if tk == closing => {
                    self.bump_any();
                    break;
                }
                TokenKind::Vbar => break,
                TokenKind::Int | TokenKind::Amper
                    if matches!(
                        self.peek(),
                        TokenKind::Greater | TokenKind::RightShift | TokenKind::Less
                    ) =>
                {
                    self.parse_redirection_with_src(
                        closing,
                        &mut redirects,
                        Some(self.current_token_kind()),
                    );
                }
                TokenKind::Name
                    if matches!(
                        self.peek(),
                        TokenKind::Greater | TokenKind::RightShift | TokenKind::Less
                    ) && REDIR_NAMES.contains(&&self.source[self.current_token_range()]) =>
                {
                    self.parse_redirection_with_src(
                        closing,
                        &mut redirects,
                        Some(self.current_token_kind()),
                    );
                }
                TokenKind::RightShift | TokenKind::Greater | TokenKind::Less => {
                    self.parse_redirection_with_src(closing, &mut redirects, None);
                }
                TokenKind::Amper if self.peek() == closing => {
                    keywords.push(ast::Keyword {
                        arg: Some(self.to_identifier("bg")),
                        value: self.literal_true(),
                        range: self.current_token_range(),
                    });
                    self.bump_any(); // skip `&`
                    self.bump(closing); // skip `)`
                    break;
                }
                _ => cmds.push(self.parse_proc_arg(&mut progress, closing)),
            }
        }

        for (typ, redirects) in redirects {
            let kw_name = match typ {
                RedirectType::Write => "writes",
                RedirectType::Append => "appends",
                RedirectType::Read => "reads",
            };
            if let Some(first) = redirects.first() {
                if let Some(last) = redirects.last() {
                    let range = TextRange::new(first.range().start(), last.range().end());
                    let expr = Expr::from(ExprDict {
                        range,
                        items: redirects,
                    });
                    keywords.push(ast::Keyword {
                        arg: Some(self.to_identifier(kw_name)),
                        value: expr,
                        range: self.node_range(start),
                    });
                }
            }
        }

        ast::Arguments {
            range: self.node_range(start),
            args: cmds.into_boxed_slice(),
            keywords: keywords.into_boxed_slice(),
        }
    }

    /// Parses arguments in a subprocess expression.
    fn parse_proc_arg(&mut self, progress: &mut ParserProgress, closing: TokenKind) -> Expr {
        progress.assert_progressing(self);
        let kind = self.current_token_kind();

        match kind {
            TokenKind::At => self.parse_decorator_or_interpolation(),
            TokenKind::Lbrace => self.parse_decorator_or_interpolation(),
            TokenKind::String
            | TokenKind::FStringStart
            | TokenKind::Lpar
            | TokenKind::Dollar
            | TokenKind::DollarLParen
            | TokenKind::AtDollarLParen => self.parse_atom().expr,
            tk if tk.is_proc_op() => {
                let range = self.current_token_range();
                self.bump_any();
                self.to_string_literal(range)
            }
            _ => self.parse_proc_single(closing),
        }
    }
    fn parse_proc_single(&mut self, closing: TokenKind) -> Expr {
        let start = self.node_start();
        let mut offset = self.node_end();
        let mut nesting = 0;
        self.bump_any();

        while !matches!(self.current_token_kind(), tk if
            tk.is_proc_op() ||
            offset != self.node_start() ||
            (tk == closing && nesting == 0)
        ) {
            if self.current_token_kind() == TokenKind::Lpar {
                nesting += 1;
            }
            offset = self.node_end();
            self.bump_any();
        }

        self.to_string_literal(TextRange::new(start, offset))
    }
    fn parse_redirect_type(&mut self) -> RedirectType {
        let typ = match self.current_token_kind() {
            TokenKind::Greater => RedirectType::Write,
            TokenKind::RightShift => RedirectType::Append,
            TokenKind::Less => RedirectType::Read,
            _ => unreachable!(),
        };
        self.bump_any(); // skip the `>`
        typ
    }
    fn parse_redirection_with_src(
        &mut self,
        closing: TokenKind,
        redirects: &mut HashMap<RedirectType, Vec<DictItem>>,
        src: Option<TokenKind>,
    ) {
        let src = if let Some(src) = src {
            match src {
                TokenKind::Int => self.parse_atom().expr,
                TokenKind::Name => {
                    let name_expr = self.parse_name();
                    self.to_string_literal(name_expr.range())
                }
                TokenKind::Amper => {
                    let val = TokenKind::Amper.parse(self).unwrap();
                    self.to_string_literal(val.range())
                }
                _ => unreachable!(),
            }
        } else {
            string_literal(self.current_token_range(), String::new())
        };

        let typ = self.parse_redirect_type();
        let dest = self.parse_proc_single(closing);
        redirects.entry(typ).or_default().push(DictItem {
            key: Some(src),
            value: dest,
        });
    }
    pub(super) fn parse_decorator_or_interpolation(&mut self) -> Expr {
        if self.at(TokenKind::Lbrace) {
            self.bump_any();
            let expr = self.parse_slice(TokenKind::Rbrace);
            self.bump(TokenKind::Rbrace);
            return expr;
        }
        self.bump_any(); // skip the `@`
        match self.current_token_kind() {
            TokenKind::Lpar => {
                let expr = self.parse_atom().expr;
                let range = expr.range();
                self.xonsh_attr("list_of_strs_or_callables", None)
                    .call0(vec![expr], range)
            }
            TokenKind::Name if self.peek() == TokenKind::String => {
                let start = self.node_start();
                let name = Expr::from(self.parse_name());
                let string = self.parse_strings();
                let range = self.node_range(start);
                self.xonsh_attr("Pattern", None)
                    .call0(vec![string], range)
                    .attr("invoke", range)
                    .call0(vec![name], range)
                    .star(self.node_range(self.node_start()))
            }
            _ => unreachable!("Expected to parse a name and a string"),
        }
    }

    // #[inline]
    // fn take_while(
    //     &mut self,
    //     mut f: impl FnMut(TokenKind, i32) -> bool,
    //     closing: TokenKind,
    // ) -> TextSize {
    //     let mut nesting = 0;
    //     let mut range = self.current_token_range();
    //     let is_opening = match closing {
    //         TokenKind::Rsqb => TokenKind::is_open_square,
    //         _ => TokenKind::is_open_paren,
    //     };

    //     while f(self.current_token_kind(), nesting) {
    //         if is_opening(&self.current_token_kind()) {
    //             nesting += 1;
    //         }
    //         if self.current_token_kind() == closing {
    //             if nesting == 0 {
    //                 break;
    //             }
    //             nesting -= 1;
    //         }
    //         range = self.current_token_range();
    //         self.bump_any();
    //     }
    //     range.end()
    // }

    /// Creates a xonsh attribute expression.
    fn xonsh_attr(&mut self, name: impl Into<Name>, range: Option<TextRange>) -> Expr {
        let xonsh = self.expr_name("ox");
        xonsh.attr(name, range.unwrap_or(self.current_token_range()))
    }
    fn to_identifier(&self, name: impl Into<Name>) -> ast::Identifier {
        Expr::identifier(name, self.current_token_range())
    }
    fn expr_name(&self, name: impl AsRef<str>) -> Expr {
        let val = ast::ExprName {
            range: self.current_token_range(),
            id: Name::new(name),
            ctx: ExprContext::Load,
        };
        Expr::Name(val)
    }
    fn to_string_literal(&self, range: TextRange) -> Expr {
        let value = self.source[range].to_string();
        string_literal(range, value)
    }

    pub(super) fn parse_env_name(&mut self) -> ParseResult<Expr> {
        // Match $ followed by a name
        let dollar = TokenKind::Dollar.parse(self)?;
        let attr = self.xonsh_attr("env", Some(dollar));
        let start = self.node_start();
        let name = TokenKind::Name.parse(self)?;
        let slice = self.to_string_literal(name);
        let ast = ast::ExprSubscript {
            value: Box::new(attr),
            slice: Box::new(slice),
            ctx: ExprContext::Load,
            range: self.node_range(start),
        };
        Ok(Expr::Subscript(ast))
    }
    pub(super) fn parse_env_expr(&mut self) -> ParseResult<Expr> {
        let dollar = TokenKind::DollarLBrace.parse(self)?;
        let attr = self.xonsh_attr("env", Some(dollar));

        // Slice range doesn't include the `[` token.
        let slice_start = self.node_start();

        if self.eat(TokenKind::Rbrace) {
            // Create an error when receiving an empty slice to parse, e.g. `x[]`
            return Err(ParseErrorType::OtherError(format!(
                "Expected an Environment variable name or expression, got {}",
                self.current_token_kind()
            )));
        }
        let slice = self.parse_slice(TokenKind::Rbrace);

        self.bump(TokenKind::Rbrace);

        let ast = ast::ExprSubscript {
            value: Box::new(attr),
            slice: Box::new(slice),
            ctx: ExprContext::Load,
            range: self.node_range(slice_start),
        };
        Ok(Expr::Subscript(ast))
    }
    pub(super) fn parse_special_strings(&mut self, expr: Expr, start: TextSize) -> Expr {
        if let Expr::StringLiteral(s) = &expr {
            if s.value.is_path() {
                return self
                    .xonsh_attr("path", None)
                    .call0(vec![expr], self.node_range(start));
            } else if s.value.is_regex() {
                return self
                    .xonsh_attr("Pattern", None)
                    .call0(vec![expr], self.node_range(start))
                    .attr("regex", self.node_range(start))
                    .call_empty(self.node_range(start));
            }
        }
        expr
    }

    fn literal_true(&self) -> Expr {
        Expr::BooleanLiteral(ast::ExprBooleanLiteral {
            value: true,
            range: self.node_range(self.node_start()),
        })
    }

    pub(super) fn parse_help_expr(&mut self, lhs: Expr, start: TextSize) -> Expr {
        self.bump_any();

        let range = self.node_range(start);
        let method = if self.at(TokenKind::Question) {
            self.bump_any();
            "superhelp"
        } else {
            "help"
        };
        let args = vec![lhs];
        self.xonsh_attr(method, None).call0(args, range)
    }
}

// fn dedent(input: &str, min_indent: usize) -> String {
//     let lines: Vec<&str> = input.lines().collect();

//     lines
//         .iter()
//         .map(|line| {
//             if line.trim().is_empty() {
//                 (*line).to_string() // Preserve empty lines
//             } else {
//                 line.chars().skip(min_indent).collect()
//             }
//         })
//         .collect::<Vec<String>>()
//         .join("\n")
// }

fn string_literal(range: TextRange, value: String) -> Expr {
    let literal = ast::StringLiteral {
        value: value.into_boxed_str(),
        range,
        flags: ast::StringLiteralFlags::default(),
    };

    Expr::from(ast::ExprStringLiteral {
        value: ast::StringLiteralValue::single(literal),
        range,
    })
}
