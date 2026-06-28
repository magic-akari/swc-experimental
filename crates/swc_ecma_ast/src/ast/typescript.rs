use crate::Span;
use swc_experimental_allocator::boxed::Box;
use swc_experimental_ast_macros::ast;

#[ast]
#[derive(Debug)]
pub struct TSThisParameter<'a> {
    pub span: Span,
    pub this_span: Span,
    pub type_annotation: Option<Box<'a, TSTypeAnnotation>>,
}

#[ast]
#[derive(Debug)]
pub struct TSTypeAnnotation {
    pub span: Span,
}
