use rspack_experimental_swc_ast_macros::ast;

#[ast]
pub enum Decl {
    Class(ClassDecl),
}

#[ast]
pub struct ClassDecl {
    ident: TypedNode<Ident>,
    declare: bool,
    // class: Box<Class>,
}
