use crate::Span;
use swc_experimental_allocator::boxed::Box;
use swc_experimental_allocator::vec::Vec;
use swc_experimental_ast_macros::ast;

use crate::ast::{BlockStmt, Decorator, Expr, Pat};

#[ast]
#[derive(Debug)]
pub struct Function<'a> {
    pub span: Span,
    pub params: Box<'a, ParamList<'a>>,
    pub decorators: Vec<'a, Decorator<'a>>,
    pub body: Box<'a, BlockStmt<'a>>,
    pub is_generator: bool,
    pub is_async: bool,
    // pub type_params: Option<Box<TsTypeParamDecl>>,
    // pub return_type: Option<Box<TsTypeAnn>>,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParamListKind {
    Formal,
    Unique,
    Arrow,
    Signature,
}

#[ast]
#[derive(Debug)]
pub struct ParamList<'a> {
    pub span: Span,
    pub kind: ParamListKind,
    pub items: Vec<'a, Param<'a>>,
    pub rest: Option<Box<'a, ParamRest<'a>>>,
}

#[ast]
#[derive(Debug)]
pub struct Param<'a> {
    pub span: Span,
    pub decorators: Vec<'a, Decorator<'a>>,
    pub pat: Pat<'a>,
    pub initializer: Option<Expr<'a>>,
    pub optional: bool,
}

#[ast]
#[derive(Debug)]
pub struct ParamRest<'a> {
    pub span: Span,
    pub decorators: Vec<'a, Decorator<'a>>,
    pub arg: Pat<'a>,
}
