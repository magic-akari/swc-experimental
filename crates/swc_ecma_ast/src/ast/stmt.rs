use rspack_experimental_swc_ast_macros::ast;

#[ast]
pub enum Stmt {
    Empty(EmptyStmt),
}

#[ast]
pub struct EmptyStmt {}
