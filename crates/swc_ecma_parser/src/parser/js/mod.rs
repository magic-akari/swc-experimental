mod arrow;
mod class_and_fn;
mod expr;
mod grammar;
mod ident;
mod module_item;
mod object;
mod pat;
mod stmt;

use swc_experimental_allocator::boxed::Box as AstBox;
use swc_experimental_ecma_ast::*;

pub(super) struct ParamListWithInfo<'a> {
    pub params: AstBox<'a, ParamList<'a>>,
    pub is_simple: bool,
}

impl<'a> ParamListWithInfo<'a> {
    pub(super) fn new(params: AstBox<'a, ParamList<'a>>, is_simple: bool) -> Self {
        Self { params, is_simple }
    }
}

pub(crate) fn is_not_this(p: &Param<'_>) -> bool {
    let Pat::Ident(ident) = &p.pat else {
        return true;
    };

    ident.id.sym != "this"
}
