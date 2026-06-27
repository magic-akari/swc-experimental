//! 13.3.3 Destructuring Binding Patterns

use crate::{
    Context, PResult,
    error::SyntaxError,
    input::Tokens,
    lexer::Token,
    parser::{Parser, js::expr::AssignTargetOrSpread, util::ExprExt},
};
use swc_experimental_allocator::boxed::Box;
use swc_experimental_allocator::vec::Vec;
use swc_experimental_ecma_ast::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum PatType {
    BindingPat,
    BindingElement,
    /// AssignmentPattern
    AssignPat,
    AssignElement,
}

impl PatType {
    fn element(self) -> Self {
        match self {
            PatType::BindingPat | PatType::BindingElement => PatType::BindingElement,
            PatType::AssignPat | PatType::AssignElement => PatType::AssignElement,
        }
    }
}

impl<'a, I: Tokens<'a>> Parser<'a, I> {
    pub fn parse_pat(&mut self) -> PResult<Pat<'a>> {
        self.parse_binding_pat_or_ident(false)
    }

    /// argument of arrow is pattern, although idents in pattern is already
    /// checked if is a keyword, it should also be checked if is arguments or
    /// eval
    fn pat_is_valid_argument_in_strict(&mut self, pat: &Pat) {
        debug_assert!(self.ctx().contains(Context::Strict));
        match pat {
            Pat::Ident(i) => {
                if i.is_reserved_in_strict_bind() {
                    self.emit_strict_mode_err(i.span(), SyntaxError::EvalAndArgumentsInStrict)
                }
            }
            Pat::Array(arr) => {
                for pat in arr.elems.iter().flatten() {
                    self.pat_is_valid_argument_in_strict(pat);
                }
            }
            Pat::Rest(r) => self.pat_is_valid_argument_in_strict(&r.arg),
            Pat::Object(obj) => {
                for prop in obj.props.iter() {
                    match prop {
                        ObjectPatProp::KeyValue(key_value) => {
                            self.pat_is_valid_argument_in_strict(&key_value.value)
                        }
                        ObjectPatProp::Rest(rest) => {
                            self.pat_is_valid_argument_in_strict(&rest.arg)
                        }
                        ObjectPatProp::Assign(assign) => {
                            let key = &assign.key;
                            if key.is_reserved_in_strict_bind() {
                                self.emit_strict_mode_err(
                                    key.span(),
                                    SyntaxError::EvalAndArgumentsInStrict,
                                )
                            }
                        }
                        #[cfg(swc_ast_unknown)]
                        _ => unreachable!(),
                    }
                }
            }
            Pat::Assign(a) => self.pat_is_valid_argument_in_strict(&a.left),
            Pat::Invalid(_) | Pat::Expr(_) => (),
            #[cfg(swc_ast_unknown)]
            _ => unreachable!(),
        }
    }

    /// This does not return 'rest' pattern because non-last parameter cannot be
    /// rest.
    pub(super) fn reparse_expr_as_pat(
        &mut self,
        pat_ty: PatType,
        expr: Expr<'a>,
    ) -> PResult<Pat<'a>> {
        if let Expr::Invalid(i) = expr {
            return Ok(Pat::Invalid(i));
        }
        if pat_ty == PatType::AssignPat {
            match expr {
                Expr::Object(..) | Expr::Array(..) => {
                    // It is a Syntax Error if LeftHandSideExpression is either
                    // an ObjectLiteral or an ArrayLiteral
                    // and LeftHandSideExpression cannot
                    // be reparsed as an AssignmentPattern.
                }
                _ => {
                    self.check_assign_target(&expr, true);
                }
            }
        }
        self.reparse_expr_as_pat_inner(pat_ty, expr)
    }

    fn reparse_expr_as_pat_inner(&mut self, pat_ty: PatType, expr: Expr<'a>) -> PResult<Pat<'a>> {
        // In dts, we do not reparse.
        debug_assert!(!self.input().syntax().dts());
        let span = expr.span();
        if pat_ty == PatType::AssignPat {
            match expr {
                Expr::Object(..) | Expr::Array(..) => {
                    // It is a Syntax Error if LeftHandSideExpression is either
                    // an ObjectLiteral or an ArrayLiteral
                    // and LeftHandSideExpression cannot
                    // be reparsed as an AssignmentPattern.
                }

                _ => match expr {
                    // It is a Syntax Error if the LeftHandSideExpression is
                    // CoverParenthesizedExpressionAndArrowParameterList:(Expression) and
                    // Expression derives a phrase that would produce a Syntax Error according
                    // to these rules if that phrase were substituted for
                    // LeftHandSideExpression. This rule is recursively applied.
                    Expr::Paren(..) => {
                        return Ok(Pat::Expr(self.boxed(expr)));
                    }
                    Expr::Ident(i) => {
                        let i = self.ast.box_binding_ident(i);
                        return Ok(Pat::Ident(i));
                    }
                    _ => {
                        return Ok(Pat::Expr(self.boxed(expr)));
                    }
                },
            }
        }

        // AssignmentElement:
        //      DestructuringAssignmentTarget Initializer[+In]?
        //
        // DestructuringAssignmentTarget:
        //      LeftHandSideExpression
        if pat_ty == PatType::AssignElement {
            match expr {
                Expr::Array(..) | Expr::Object(..) => {}
                Expr::Member(..)
                | Expr::SuperProp(..)
                | Expr::Call(..)
                | Expr::New(..)
                | Expr::Lit(..)
                | Expr::Ident(..)
                | Expr::Fn(..)
                | Expr::Class(..)
                | Expr::Paren(..)
                // | Expr::TsAs(..) 
                | Expr::Tpl(..)=> {
                    if !expr.is_valid_simple_assignment_target(self.ctx().contains(Context::Strict))
                    {
                        self.emit_err(span, SyntaxError::NotSimpleAssign)
                    }
                    match expr {
                        Expr::Ident(i) => {
                            let i = self.ast.box_binding_ident(i);
                            return Ok(Pat::Ident(i))
                        },
                        _ => {
                            return Ok(Pat::Expr(self.boxed(expr)));
                        }
                    }
                }
                // It's special because of optional initializer
                Expr::Assign(..) => {}
                _ => self.emit_err(span, SyntaxError::InvalidPat),
            }
        }

        match expr {
            Expr::Paren(..) => {
                self.emit_err(span, SyntaxError::InvalidPat);
                Ok(self.ast.pat_invalid())
            }
            Expr::Assign(assign_expr) => {
                let assign_expr = Box::into_inner(assign_expr);
                let span = assign_expr.span();
                if assign_expr.op != AssignOp::Assign {
                    self.emit_err(span, SyntaxError::InvalidPat);
                    return Ok(self.ast.pat_invalid());
                }

                let left = match assign_expr.left {
                    AssignTarget::Simple(left) => {
                        let left = match Box::into_inner(left) {
                            SimpleAssignTarget::Ident(binding_ident) => {
                                let binding_ident = Box::into_inner(binding_ident);
                                let sym = binding_ident.id;
                                Expr::Ident(sym)
                            }
                            SimpleAssignTarget::Member(member_expr) => Expr::Member(member_expr),
                            SimpleAssignTarget::SuperProp(super_prop_expr) => {
                                Expr::SuperProp(super_prop_expr)
                            }
                            SimpleAssignTarget::Paren(paren_expr) => Expr::Paren(paren_expr),
                            SimpleAssignTarget::OptChain(opt_chain_expr) => {
                                Expr::OptChain(opt_chain_expr)
                            }
                            SimpleAssignTarget::Invalid(invalid) => Expr::Invalid(invalid),
                        };
                        self.reparse_expr_as_pat(pat_ty, left)?
                    }
                    AssignTarget::Pat(pat) => match Box::into_inner(pat) {
                        AssignTargetPat::Array(array_pat) => Pat::Array(array_pat),
                        AssignTargetPat::Object(object_pat) => Pat::Object(object_pat),
                        AssignTargetPat::Invalid(invalid) => Pat::Invalid(invalid),
                    },
                    #[cfg(swc_ast_unknown)]
                    _ => unreachable!(),
                };
                let right = assign_expr.right;
                Ok(self.ast.pat_assign_pat(span, left, right))
            }
            Expr::Object(object) => {
                let object = Box::into_inner(object);
                // {}
                let object_span = object.span();
                let props = object.props;
                let len = props.len();

                let props = self.collect_vec(|p, obj_props| {
                    for (idx, prop) in props.into_iter().enumerate() {
                        let span = prop.span();
                        let prop = match prop {
                            PropOrSpread::Prop(prop) => match Box::into_inner(prop) {
                                Prop::Shorthand(id) => {
                                    let id = Box::into_inner(id);
                                    let span = id.span();
                                    let binding_ident = p.ast.box_binding_ident(p.boxed(id));
                                    p.ast
                                        .object_pat_prop_assign_pat_prop(span, binding_ident, None)
                                }
                                Prop::KeyValue(kv_prop) => {
                                    let kv_prop = Box::into_inner(kv_prop);
                                    let pat =
                                        p.reparse_expr_as_pat(pat_ty.element(), kv_prop.value)?;
                                    p.ast.object_pat_prop_key_value_pat_prop(kv_prop.key, pat)
                                }
                                Prop::Assign(assign_prop) => {
                                    let assign_prop = Box::into_inner(assign_prop);
                                    let id = assign_prop.key;
                                    let key = p.ast.box_binding_ident(id);
                                    let value = assign_prop.value;
                                    p.ast
                                        .object_pat_prop_assign_pat_prop(span, key, Some(value))
                                }
                                _ => syntax_error!(p, span, SyntaxError::InvalidPat),
                            },

                            PropOrSpread::Spread(spread) => {
                                let spread = Box::into_inner(spread);
                                let dot_3_token = spread.dot3_token;
                                let expr = spread.expr;
                                if idx != len - 1 {
                                    p.emit_err(span, SyntaxError::NonLastRestParam)
                                } else if let Some(trailing_comma) =
                                    p.state().trailing_commas.get(&object_span.start)
                                {
                                    p.emit_err(*trailing_comma, SyntaxError::CommaAfterRestElement);
                                };

                                let element_pat_ty = pat_ty.element();
                                let pat = if let PatType::BindingElement = element_pat_ty {
                                    if let Expr::Ident(i) = expr {
                                        Pat::Ident(p.ast.box_binding_ident(i))
                                    } else {
                                        p.emit_err(span, SyntaxError::DotsWithoutIdentifier);
                                        p.ast.pat_invalid()
                                    }
                                } else {
                                    p.reparse_expr_as_pat(element_pat_ty, expr)?
                                };
                                if let Pat::Assign(_) = pat {
                                    p.emit_err(span, SyntaxError::TS1048)
                                };
                                p.ast.object_pat_prop_rest_pat(span, dot_3_token, pat)
                            }
                            #[cfg(swc_ast_unknown)]
                            _ => unreachable!(),
                        };

                        obj_props.push(prop);
                    }

                    Ok(())
                })?;

                Ok(self.ast.pat_object_pat(object_span, props, false))
            }
            Expr::Ident(ident) => Ok(Pat::Ident(self.ast.box_binding_ident(ident))),
            Expr::Array(array) => {
                let array = Box::into_inner(array);
                let mut exprs = array.elems;
                if exprs.is_empty() {
                    return Ok(self.ast.pat_array_pat(span, self.vec(), false));
                }
                // Trailing comma may exist. We should remove those commas.
                let count_of_trailing_comma =
                    exprs.iter().rev().take_while(|e| e.is_none()).count();
                let len = exprs.len();
                let mut params = self.vec();
                // Comma or other pattern cannot follow a rest pattern.
                let idx_of_rest_not_allowed = if count_of_trailing_comma == 0 {
                    len - 1
                } else {
                    // last element is comma, so rest is not allowed for every pattern element.
                    len - count_of_trailing_comma
                };

                let after = exprs.split_off(idx_of_rest_not_allowed);
                for expr in exprs {
                    match expr {
                        Some(expr_or_spread) => {
                            let expr_or_spread = Box::into_inner(expr_or_spread);
                            match expr_or_spread.spread {
                                Some(_) => self.emit_err(
                                    expr_or_spread.expr.span(),
                                    SyntaxError::NonLastRestParam,
                                ),
                                None => {
                                    let expr = expr_or_spread.expr;
                                    params.push(Some(
                                        self.reparse_expr_as_pat(pat_ty.element(), expr)?,
                                    ))
                                }
                            }
                        }
                        None => params.push(None),
                    }
                }

                let exprs = after;
                if count_of_trailing_comma == 0 {
                    let mut exprs = exprs;
                    let expr = exprs.remove(0);
                    let last = match expr {
                        // Rest
                        Some(expr_or_spread) => {
                            let expr_or_spread = Box::into_inner(expr_or_spread);
                            let spread = expr_or_spread.spread;
                            let expr = expr_or_spread.expr;
                            match spread {
                                Some(spread) => {
                                    // TODO: is BindingPat correct?
                                    if let Expr::Assign(_) = expr {
                                        self.emit_err(expr.span(), SyntaxError::TS1048);
                                    };
                                    if let Some(trailing_comma) =
                                        self.state().trailing_commas.get(&span.start)
                                    {
                                        self.emit_err(
                                            *trailing_comma,
                                            SyntaxError::CommaAfterRestElement,
                                        );
                                    }
                                    let expr_span = expr.span();
                                    let spread_span = spread;

                                    Some(self.reparse_expr_as_pat(pat_ty.element(), expr).map(
                                        |pat| self.ast.pat_rest_pat(expr_span, spread_span, pat),
                                    )?)
                                }
                                None => {
                                    // TODO: is BindingPat correct?
                                    Some(self.reparse_expr_as_pat(pat_ty.element(), expr)?)
                                }
                            }
                        }
                        // TODO: syntax error if last element is ellison and ...rest exists.
                        None => None,
                    };
                    params.push(last);
                }

                Ok(self.ast.pat_array_pat(span, params, false))
            }

            // Invalid patterns.
            // Note that assignment expression with '=' is valid, and handled above.
            Expr::Lit(..) => {
                self.emit_err(span, SyntaxError::InvalidPat);
                Ok(self.ast.pat_invalid())
            }

            Expr::Yield(..) if self.ctx().contains(Context::InGenerator) => {
                self.emit_err(span, SyntaxError::InvalidPat);
                Ok(self.ast.pat_invalid())
            }

            _ => {
                self.emit_err(span, SyntaxError::InvalidPat);

                Ok(self.ast.pat_invalid())
            }
        }
    }

    pub(super) fn parse_binding_element(&mut self) -> PResult<Pat<'a>> {
        trace_cur!(self, parse_binding_element);

        let start = self.cur_pos();
        let left = self.parse_binding_pat_or_ident(false)?;

        if self.input_mut().eat(Token::Eq) {
            let right = self.allow_in_expr(Self::parse_assignment_expr)?;

            if self.ctx().contains(Context::InDeclare) {
                self.emit_err(self.span(start), SyntaxError::TS2371);
            }

            return Ok(self.ast.pat_assign_pat(self.span(start), left, right));
        }

        Ok(left)
    }

    pub(crate) fn parse_binding_pat_or_ident(&mut self, disallow_let: bool) -> PResult<Pat<'a>> {
        trace_cur!(self, parse_binding_pat_or_ident);

        let cur = self.input().cur();
        if cur.is_word() {
            let ident = self.parse_binding_ident(disallow_let)?;
            Ok(Pat::Ident(self.ast.box_binding_ident(self.boxed(ident))))
        } else if cur == Token::LBracket {
            self.parse_array_binding_pat()
        } else if cur == Token::LBrace {
            self.parse_object_pat()
        } else if cur == Token::Error {
            let err = self.input_mut().expect_error_token_and_bump();
            Err(err)
        } else {
            unexpected!(self, "yield, an identifier, [ or {")
        }
    }

    fn parse_array_binding_pat(&mut self) -> PResult<Pat<'a>> {
        let start = self.cur_pos();

        self.assert_and_bump(Token::LBracket);

        let mut elems = self.vec();

        let mut rest_span = Span::default();

        while !self.input().is(Token::RBracket) {
            if self.input_mut().eat(Token::Comma) {
                elems.push(None);
                continue;
            }

            if !rest_span.is_dummy() {
                self.emit_err(rest_span, SyntaxError::NonLastRestParam);
            }

            let start = self.cur_pos();

            let mut is_rest = false;
            if self.input_mut().eat(Token::DotDotDot) {
                is_rest = true;
                let dot3_token = self.span(start);

                let pat = self.parse_binding_pat_or_ident(false)?;
                rest_span = self.span(start);

                let pat = self.ast.pat_rest_pat(rest_span, dot3_token, pat);
                elems.push(Some(pat));
            } else {
                elems.push(Some(self.parse_binding_element()?));
            }

            if !self.input().is(Token::RBracket) {
                expect!(self, Token::Comma);
                if is_rest && self.input().is(Token::RBracket) {
                    self.emit_err(self.input().prev_span(), SyntaxError::CommaAfterRestElement);
                }
            }
        }

        expect!(self, Token::RBracket);
        let optional = (self.input().syntax().dts() || self.ctx().contains(Context::InDeclare))
            && self.input_mut().eat(Token::QuestionMark);

        Ok(self.ast.pat_array_pat(self.span(start), elems, optional))
    }

    /// spec: 'FormalParameter'
    ///
    /// babel: `parseAssignableListItem`
    fn parse_formal_param_pat(&mut self) -> PResult<(Pat<'a>, Option<Expr<'a>>)> {
        let start = self.cur_pos();

        // let has_modifier = self.eat_any_ts_modifier()?;

        let pat = self.parse_binding_pat_or_ident(false)?;
        let opt = false;

        // if self.input().syntax().typescript() {
        //     if self.input_mut().eat(Token::QuestionMark) {
        //         match pat {
        //             Pat::Ident(BindingIdent {
        //                 id:
        //                     Ident {
        //                         ref mut optional, ..
        //                     },
        //                 ..
        //             })
        //             | Pat::Array(ArrayPat {
        //                 ref mut optional, ..
        //             })
        //             | Pat::Object(ObjectPat {
        //                 ref mut optional, ..
        //             }) => {
        //                 *optional = true;
        //                 opt = true;
        //             }
        //             _ if self.input().syntax().dts() || self.ctx().contains(Context::InDeclare) => {
        //             }
        //             _ => {
        //                 syntax_error!(
        //                     self,
        //                     self.input().prev_span(),
        //                     SyntaxError::TsBindingPatCannotBeOptional
        //                 );
        //             }
        //         }
        //     }
        // }

        let pat = if self.input_mut().eat(Token::Eq) {
            // `=` cannot follow optional parameter.
            if opt {
                self.emit_err(pat.span(), SyntaxError::TS1015);
            }

            let right = self.allow_in_expr(Self::parse_assignment_expr)?;
            if self.ctx().contains(Context::InDeclare) {
                self.emit_err(self.span(start), SyntaxError::TS2371);
            }

            return Ok((pat, Some(right)));
        } else {
            pat
        };

        // if has_modifier {
        //     self.emit_err(self.span(start), SyntaxError::TS2369);
        //     return Ok(pat);
        // }

        Ok((pat, None))
    }

    fn make_param_from_pat(
        &mut self,
        span: Span,
        decorators: Vec<'a, Decorator<'a>>,
        pat: Pat<'a>,
    ) -> Param<'a> {
        if let Pat::Assign(assign) = pat {
            let assign = Box::into_inner(assign);
            self.ast
                .param(span, decorators, assign.left, Some(assign.right))
        } else {
            self.ast.param(span, decorators, pat, None)
        }
    }

    fn parse_formal_param(
        &mut self,
        param_start: u32,
        decorators: Vec<'a, Decorator<'a>>,
    ) -> PResult<Param<'a>> {
        // let (accessibility, is_override, readonly) = if self.input().syntax().typescript() {
        //     let accessibility = self.parse_access_modifier()?;
        //     (
        //         accessibility,
        //         self.parse_ts_modifier(&[Token::Override], false)?.is_some(),
        //         self.parse_ts_modifier(&[Token::Readonly], false)?.is_some(),
        //     )
        // } else {
        //     (None, false, false)
        // };
        // if accessibility.is_some() || is_override || readonly {
        //     let param = match self.parse_formal_param_pat()? {
        //         Pat::Ident(i) => TsParamPropParam::Ident(i),
        //         Pat::Assign(a) => TsParamPropParam::Assign(a),
        //         node => syntax_error!(self, node.span(), SyntaxError::TsInvalidParamPropPat),
        //     };
        // }
        let (pat, initializer) = self.parse_formal_param_pat()?;
        Ok(self
            .ast
            .param(self.span(param_start), decorators, pat, initializer))
    }

    pub(crate) fn parse_param_list(
        &mut self,
        kind: ParamListKind,
    ) -> PResult<Box<'a, ParamList<'a>>> {
        let prev_span = self.input().prev_span();
        let start = prev_span.start;
        let mut end = prev_span.end;
        let mut items = self.vec();
        let mut rest = None;
        let mut rest_span = Span::default();

        while !self.input().is(Token::RParen) {
            if !rest_span.is_dummy() {
                self.emit_err(rest_span, SyntaxError::TS1014);
            }

            let param_start = self.cur_pos();
            let decorators = self.parse_decorators(false)?;
            let pat_start = self.cur_pos();

            let mut is_rest = false;
            if self.input_mut().eat(Token::DotDotDot) {
                is_rest = true;
                let dot3_token = self.span(pat_start);

                let mut pat = self.parse_binding_pat_or_ident(false)?;

                if self.input_mut().eat(Token::Eq) {
                    let right = self.parse_assignment_expr()?;
                    self.emit_err(pat.span(), SyntaxError::TS1048);
                    pat = self.ast.pat_assign_pat(self.span(pat_start), pat, right);
                }

                rest_span = self.span(pat_start);

                if self.syntax().typescript() && self.input_mut().eat(Token::QuestionMark) {
                    self.emit_err(self.input().prev_span(), SyntaxError::TS1047);
                    //
                }

                let param_rest = self.ast.box_param_rest(
                    Span::new(dot3_token.start, pat.span().end),
                    decorators,
                    pat,
                );
                end = param_rest.span.end;
                rest = Some(param_rest);
            } else {
                let param = self.parse_formal_param(param_start, decorators)?;
                end = param.span.end;
                items.push(param);
            }

            if !self.input().is(Token::RParen) {
                expect!(self, Token::Comma);
                if self.input().is(Token::RParen) && is_rest {
                    self.emit_err(self.input().prev_span(), SyntaxError::CommaAfterRestElement);
                }
            }
        }

        Ok(self
            .ast
            .box_param_list(Span::new(start, end), kind, items, rest))
    }

    pub(crate) fn parse_constructor_params(&mut self) -> PResult<Box<'a, ParamList<'a>>> {
        self.parse_param_list(ParamListKind::Formal)
    }

    pub(crate) fn parse_formal_params(&mut self) -> PResult<Box<'a, ParamList<'a>>> {
        self.parse_param_list(ParamListKind::Formal)
    }

    pub(crate) fn parse_unique_formal_params(&mut self) -> PResult<Box<'a, ParamList<'a>>> {
        // FIXME: This is wrong
        self.parse_param_list(ParamListKind::Unique)
    }

    pub(super) fn parse_paren_items_as_params(
        &mut self,
        mut exprs: Vec<'a, AssignTargetOrSpread<'a>>,
        trailing_comma: Option<Span>,
    ) -> PResult<Box<'a, ParamList<'a>>> {
        let pat_ty = PatType::BindingPat;

        let len = exprs.len();
        if len == 0 {
            let span = self.input().prev_span();
            return Ok(self.ast.box_param_list(
                Span::new(span.start, span.start),
                ParamListKind::Arrow,
                self.vec(),
                None,
            ));
        }

        let start = exprs
            .first()
            .map_or_else(|| self.cur_pos(), |expr| expr.span().start);
        let mut params = self.vec();
        let mut rest = None;

        for expr in exprs.drain(..len - 1) {
            match expr {
                AssignTargetOrSpread::ExprOrSpread(expr_or_spread) => {
                    let span = expr_or_spread.span();
                    match expr_or_spread.spread {
                        Some(_) => self.emit_err(span, SyntaxError::TS1014),
                        None => {
                            let pat = self.reparse_expr_as_pat(pat_ty, expr_or_spread.expr)?;
                            let decorators = self.vec();
                            let param = self.make_param_from_pat(span, decorators, pat);
                            params.push(param);
                        }
                    }
                }
                AssignTargetOrSpread::Pat(Pat::Rest(..)) => {
                    self.emit_err(expr.span(), SyntaxError::TS1014)
                }
                AssignTargetOrSpread::Pat(pat) => {
                    let span = pat.span();
                    let decorators = self.vec();
                    let param = self.make_param_from_pat(span, decorators, pat);
                    params.push(param);
                }
            }
        }

        debug_assert_eq!(exprs.len(), 1);
        let expr = exprs.pop().unwrap();
        let outer_expr_span = expr.span();
        let end = outer_expr_span.end;
        match expr {
            // Rest
            AssignTargetOrSpread::ExprOrSpread(expr_or_spread) => match expr_or_spread.spread {
                Some(dot3_token) => {
                    let expr = expr_or_spread.expr;
                    if let Expr::Assign(_) = expr {
                        self.emit_err(outer_expr_span, SyntaxError::TS1048)
                    };
                    if let Some(trailing_comma) = trailing_comma {
                        self.emit_err(trailing_comma, SyntaxError::CommaAfterRestElement);
                    }

                    let expr_span = expr.span();
                    let dot3_span = dot3_token;
                    let pat = self.reparse_expr_as_pat(pat_ty, expr)?;
                    rest = Some(self.ast.box_param_rest(
                        Span::new(dot3_span.start, expr_span.end),
                        self.vec(),
                        pat,
                    ));
                }
                None => {
                    let span = expr_or_spread.span();
                    let pat = self.reparse_expr_as_pat(pat_ty, expr_or_spread.expr)?;
                    let decorators = self.vec();
                    let param = self.make_param_from_pat(span, decorators, pat);
                    params.push(param);
                }
            },
            AssignTargetOrSpread::Pat(pat) => {
                if let Some(trailing_comma) = trailing_comma
                    && let Pat::Rest(..) = pat
                {
                    self.emit_err(trailing_comma, SyntaxError::CommaAfterRestElement);
                }
                let span = pat.span();
                let decorators = self.vec();
                let param = self.make_param_from_pat(span, decorators, pat);
                params.push(param);
            }
        }

        if self.ctx().contains(Context::Strict) {
            for param in params.iter() {
                self.pat_is_valid_argument_in_strict(&param.pat)
            }
            if let Some(rest) = &rest {
                self.pat_is_valid_argument_in_strict(&rest.arg)
            }
        }
        Ok(self
            .ast
            .box_param_list(Span::new(start, end), ParamListKind::Arrow, params, rest))
    }
}
