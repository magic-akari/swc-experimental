use rspack_experimental_swc_ast_macros::ast;

use crate::ast::Lit;

#[ast]
pub enum Expr {
    Lit(Lit),
}

#[ast]
pub struct ObjectLit {}

#[ast]
pub struct ClassExpr {}

#[ast]
pub struct FnExpr {}
