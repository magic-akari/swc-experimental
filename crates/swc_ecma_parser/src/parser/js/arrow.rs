use crate::{
    Context, PResult,
    error::SyntaxError,
    input::Tokens,
    lexer::Token,
    parser::{Parser, js::ParamListWithInfo},
};
use swc_experimental_ecma_ast::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum ParenArrowLookahead {
    NotArrow,
    DefiniteArrow,
    Ambiguous,
}

impl<'a, I: Tokens<'a>> Parser<'a, I> {
    pub(super) fn try_parse_definite_parenthesized_arrow_expr(
        &mut self,
        expr_start: u32,
        async_span: Option<Span>,
        can_be_arrow: bool,
    ) -> PResult<Option<Expr<'a>>> {
        if self.lookahead_parenthesized_arrow(async_span.is_some())
            != ParenArrowLookahead::DefiniteArrow
        {
            return Ok(None);
        }

        expect!(self, Token::LParen);
        let params = self.do_outside_of_context(Context::WillExpectColonForCond, |p| {
            p.do_inside_of_context(Context::InParameters, |p| {
                p.do_outside_of_context(Context::InFunction, |p| {
                    if async_span.is_some() {
                        p.do_inside_of_context(Context::InAsync, |p| {
                            p.parse_param_list_with_info(ParamListKind::Arrow)
                        })
                    } else {
                        p.do_outside_of_context(Context::InAsync, |p| {
                            p.parse_param_list_with_info(ParamListKind::Arrow)
                        })
                    }
                })
            })
        })?;
        expect!(self, Token::RParen);

        if self.input().had_line_break_before_cur() {
            syntax_error!(
                self,
                self.span(expr_start),
                SyntaxError::LineBreakBeforeArrow
            );
        }

        if !can_be_arrow {
            syntax_error!(self, self.span(expr_start), SyntaxError::ArrowNotAllowed);
        }
        expect!(self, Token::Arrow);

        self.parse_arrow_expr_body_with_params(expr_start, async_span.is_some(), params)
    }

    fn parse_arrow_expr_body_with_params(
        &mut self,
        expr_start: u32,
        is_async: bool,
        params: ParamListWithInfo<'a>,
    ) -> PResult<Option<Expr<'a>>> {
        if self.ctx().contains(Context::Strict) {
            self.param_list_is_valid_argument_in_strict(&params.params);
        }

        let body: BlockStmtOrExpr =
            self.parse_fn_block_or_expr_body(is_async, false, true, params.is_simple)?;

        let arrow_expr = self
            .ast
            .arrow_expr(self.span(expr_start), params.params, body, is_async);
        if arrow_expr.body.is_block_stmt() && self.input().cur().is_bin_op() {
            self.emit_err(self.input().cur_span(), SyntaxError::TS1005);
            let errorred_expr =
                self.parse_bin_op_recursively(Expr::Arrow(self.boxed(arrow_expr)), 0)?;

            if !self.is_general_semi() {
                self.emit_err(self.input().cur_span(), SyntaxError::TS1005);
            }

            return Ok(Some(errorred_expr));
        }

        Ok(Some(Expr::Arrow(self.boxed(arrow_expr))))
    }

    fn lookahead_parenthesized_arrow(&mut self, is_async: bool) -> ParenArrowLookahead {
        debug_assert!(self.input().is(Token::LParen));
        let checkpoint = self.checkpoint_save();
        let result = self.lookahead_parenthesized_arrow_worker(is_async);
        self.checkpoint_load(checkpoint);
        result
    }

    fn lookahead_parenthesized_arrow_worker(&mut self, is_async: bool) -> ParenArrowLookahead {
        self.bump();

        match self.input().cur() {
            Token::RParen => {
                self.bump();
                if self.input().is(Token::Arrow) {
                    ParenArrowLookahead::DefiniteArrow
                } else {
                    ParenArrowLookahead::NotArrow
                }
            }
            Token::LBrace | Token::LBracket => ParenArrowLookahead::Ambiguous,
            Token::DotDotDot => self.lookahead_rest_arrow_head(is_async),
            token if self.is_simple_arrow_param_token(token, is_async) => {
                self.bump();
                self.lookahead_simple_arrow_params_tail(is_async)
            }
            token if Self::is_literal_token(token) => ParenArrowLookahead::NotArrow,
            _ => ParenArrowLookahead::Ambiguous,
        }
    }

    fn lookahead_rest_arrow_head(&mut self, is_async: bool) -> ParenArrowLookahead {
        self.bump();
        let token = self.input().cur();
        if !self.is_simple_arrow_param_token(token, is_async) {
            return if Self::is_literal_token(token) {
                ParenArrowLookahead::NotArrow
            } else {
                ParenArrowLookahead::Ambiguous
            };
        }

        self.bump();
        if !self.input().is(Token::RParen) {
            return ParenArrowLookahead::Ambiguous;
        }
        self.bump();
        if self.input().is(Token::Arrow) {
            ParenArrowLookahead::DefiniteArrow
        } else {
            ParenArrowLookahead::NotArrow
        }
    }

    fn lookahead_simple_arrow_params_tail(&mut self, is_async: bool) -> ParenArrowLookahead {
        loop {
            match self.input().cur() {
                Token::RParen => {
                    self.bump();
                    return if self.input().is(Token::Arrow) {
                        ParenArrowLookahead::DefiniteArrow
                    } else {
                        ParenArrowLookahead::NotArrow
                    };
                }
                Token::Comma => {
                    self.bump();
                    if self.input().is(Token::RParen) {
                        self.bump();
                        return if self.input().is(Token::Arrow) {
                            ParenArrowLookahead::DefiniteArrow
                        } else {
                            ParenArrowLookahead::NotArrow
                        };
                    }

                    let token = self.input().cur();
                    if token == Token::DotDotDot {
                        return self.lookahead_rest_arrow_head(is_async);
                    }
                    if !self.is_simple_arrow_param_token(token, is_async) {
                        return if Self::is_literal_token(token) {
                            ParenArrowLookahead::NotArrow
                        } else {
                            ParenArrowLookahead::Ambiguous
                        };
                    }
                    self.bump();
                }
                Token::Eq | Token::Colon | Token::QuestionMark => {
                    return ParenArrowLookahead::Ambiguous;
                }
                _ => return ParenArrowLookahead::NotArrow,
            }
        }
    }

    fn is_simple_arrow_param_token(&self, token: Token, is_async: bool) -> bool {
        token.is_word()
            && !matches!(token, Token::Null | Token::True | Token::False)
            && !(is_async && token == Token::Await)
            && !token.is_reserved(self.ctx())
    }

    fn is_literal_token(token: Token) -> bool {
        matches!(
            token,
            Token::Null | Token::True | Token::False | Token::Num | Token::BigInt | Token::Str
        )
    }
}
