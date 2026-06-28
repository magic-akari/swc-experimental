//! Cover grammar helpers for destructuring assignment.

use crate::{
    PResult,
    error::SyntaxError,
    input::Tokens,
    lexer::Token,
    parser::{Parser, js::pat::PatType},
};
use swc_experimental_allocator::boxed::Box;
use swc_experimental_ecma_ast::*;

impl<'a, I: Tokens<'a>> Parser<'a, I> {
    pub(super) fn cover_expr_as_assign_target(
        &mut self,
        expr: Expr<'a>,
    ) -> PResult<AssignTarget<'a>> {
        Ok(match expr {
            Expr::Object(object) => {
                let object = self
                    .cover_object_lit_as_object_pat(PatType::AssignPat, Box::into_inner(object))?;
                AssignTarget::Pat(self.ast.allocator.boxed(AssignTargetPat::Object(object)))
            }
            Expr::Array(array) => {
                let span = array.span();
                let array = self.cover_array_lit_as_array_pat(
                    PatType::AssignPat,
                    Box::into_inner(array),
                    span,
                )?;
                AssignTarget::Pat(self.ast.allocator.boxed(AssignTargetPat::Array(array)))
            }
            Expr::Ident(ident) => {
                let expr = Expr::Ident(ident);
                self.check_assign_target(&expr, true);
                let Expr::Ident(ident) = expr else {
                    unreachable!();
                };
                AssignTarget::Simple(
                    self.ast
                        .allocator
                        .boxed(SimpleAssignTarget::Ident(self.ast.box_binding_ident(ident))),
                )
            }
            Expr::Invalid(invalid) => AssignTarget::Simple(
                self.ast
                    .allocator
                    .boxed(SimpleAssignTarget::Invalid(invalid)),
            ),
            expr => {
                self.check_assign_target(&expr, true);
                match AssignTarget::try_from_expr(expr, self.ast.allocator) {
                    Ok(target) => target,
                    Err(expr) => {
                        syntax_error!(self, expr.span(), SyntaxError::InvalidAssignTarget)
                    }
                }
            }
        })
    }

    // Destructuring-target conversion is large and comparatively rare. Keeping
    // it out of line avoids inlining this branch into hot expression parsing.
    #[inline(never)]
    pub(super) fn cover_object_lit_as_object_pat(
        &mut self,
        pat_ty: PatType,
        object: ObjectLit<'a>,
    ) -> PResult<Box<'a, ObjectPat<'a>>> {
        let object_span = object.span();
        let props = object.props;
        let len = props.len();

        let mut obj_props = self.vec();
        let mut rest = None;
        for (idx, prop) in props.into_iter().enumerate() {
            let span = prop.span();
            match prop {
                PropOrSpread::Prop(prop) => {
                    let prop = match Box::into_inner(prop) {
                        Prop::Shorthand(id) => {
                            let id = Box::into_inner(id);
                            let span = id.span();
                            let binding_ident = self.ast.box_binding_ident(self.boxed(id));
                            self.ast
                                .object_pat_prop_assign_pat_prop(span, binding_ident, None)
                        }
                        Prop::KeyValue(kv_prop) => {
                            let kv_prop = Box::into_inner(kv_prop);
                            let pat = self.reparse_expr_as_pat(pat_ty.element(), kv_prop.value)?;
                            self.ast
                                .object_pat_prop_key_value_pat_prop(kv_prop.key, pat)
                        }
                        Prop::Assign(assign_prop) => {
                            let assign_prop = Box::into_inner(assign_prop);
                            let id = assign_prop.key;
                            let key = self.ast.box_binding_ident(id);
                            let value = assign_prop.value;
                            self.ast
                                .object_pat_prop_assign_pat_prop(span, key, Some(value))
                        }
                        _ => syntax_error!(self, span, SyntaxError::InvalidPat),
                    };

                    obj_props.push(prop);
                }
                PropOrSpread::Spread(spread) => {
                    let spread = Box::into_inner(spread);
                    let dot_3_token = spread.dot3_token;
                    let expr = spread.expr;
                    if idx != len - 1 {
                        self.emit_err(span, SyntaxError::NonLastRestParam)
                    } else if let Some(trailing_comma) =
                        self.state().trailing_commas.get(&object_span.start)
                    {
                        self.emit_err(*trailing_comma, SyntaxError::CommaAfterRestElement);
                    };

                    let element_pat_ty = pat_ty.element();
                    let pat = if let PatType::BindingElement = element_pat_ty {
                        if let Expr::Ident(i) = expr {
                            Pat::Ident(self.ast.box_binding_ident(i))
                        } else {
                            self.emit_err(span, SyntaxError::DotsWithoutIdentifier);
                            self.ast.pat_invalid()
                        }
                    } else {
                        self.reparse_expr_as_pat(element_pat_ty, expr)?
                    };
                    if let Pat::Assign(_) = pat {
                        self.emit_err(span, SyntaxError::TS1048)
                    };
                    rest = Some(self.ast.box_rest_pat(span, dot_3_token, pat));
                }
                #[cfg(swc_ast_unknown)]
                _ => unreachable!(),
            }
        }

        Ok(self.ast.box_object_pat(object_span, obj_props, rest, false))
    }

    // See `cover_object_lit_as_object_pat` for why this stays out of line.
    #[inline(never)]
    pub(super) fn cover_array_lit_as_array_pat(
        &mut self,
        pat_ty: PatType,
        array: ArrayLit<'a>,
        span: Span,
    ) -> PResult<Box<'a, ArrayPat<'a>>> {
        let mut exprs = array.elems;
        if exprs.is_empty() {
            return Ok(self.ast.box_array_pat(span, self.vec(), None, false));
        }
        let count_of_trailing_comma = exprs.iter().rev().take_while(|e| e.is_none()).count();
        let len = exprs.len();
        let mut params = self.vec();
        let mut rest = None;
        let idx_of_rest_not_allowed = if count_of_trailing_comma == 0 {
            len - 1
        } else {
            len - count_of_trailing_comma
        };

        let after = exprs.split_off(idx_of_rest_not_allowed);
        for expr in exprs {
            match expr {
                Some(expr_or_spread) => {
                    let expr_or_spread = Box::into_inner(expr_or_spread);
                    match expr_or_spread.spread {
                        Some(_) => {
                            self.emit_err(expr_or_spread.expr.span(), SyntaxError::NonLastRestParam)
                        }
                        None => {
                            let expr = expr_or_spread.expr;
                            params.push(Some(self.reparse_expr_as_pat(pat_ty.element(), expr)?))
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
            match expr {
                Some(expr_or_spread) => {
                    let expr_or_spread = Box::into_inner(expr_or_spread);
                    let spread = expr_or_spread.spread;
                    let expr = expr_or_spread.expr;
                    match spread {
                        Some(spread) => {
                            if let Expr::Assign(_) = expr {
                                self.emit_err(expr.span(), SyntaxError::TS1048);
                            };
                            if let Some(trailing_comma) =
                                self.state().trailing_commas.get(&span.start)
                            {
                                self.emit_err(*trailing_comma, SyntaxError::CommaAfterRestElement);
                            }
                            let expr_span = expr.span();
                            let spread_span = spread;

                            let pat = self.reparse_expr_as_pat(pat_ty.element(), expr)?;
                            rest = Some(self.ast.box_rest_pat(
                                Span::new(spread_span.start, expr_span.end),
                                spread_span,
                                pat,
                            ));
                        }
                        None => {
                            params.push(Some(self.reparse_expr_as_pat(pat_ty.element(), expr)?));
                        }
                    }
                }
                None => params.push(None),
            }
        }

        Ok(self.ast.box_array_pat(span, params, rest, false))
    }
}
