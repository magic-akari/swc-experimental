use rspack_experimental_swc_ast_macros::ast;
use swc_common::Span;

use crate::{
    Ast,
    node_id::{NodeId, OptionalAtomRef, SubRange},
};

#[ast]
pub enum Program {
    Module(Module),
    Script(Script),
}

impl Program {}

// #[ast]
// pub struct Module {
//     pub span: Span,
//     pub body: SubRange,
//     pub shebang: OptionalAtomRef,
// }
pub struct Module(NodeId);

impl Module {
    pub fn span(&self, ast: &Ast) -> Span {
        ast.nodes[self.0].span
    }

    pub fn set_span(&self, ast: &mut Ast, span: Span) {
        ast.nodes[self.0].span = span;
    }

    // FIXME: SubRange or Vec
    pub fn body(&self, ast: &Ast) -> SubRange {
        let data = unsafe { ast.nodes[self.0].data.sub_range };
        let body = unsafe { ast.extra_data[data.start].sub_range };
        body
    }

    pub fn set_body(&self, ast: &mut Ast, body: SubRange) {
        let data = unsafe { ast.nodes[self.0].data.sub_range };
        ast.extra_data[data.start].sub_range = body;
    }

    pub fn shebang(&self, ast: &Ast) -> OptionalAtomRef {
        let data = unsafe { ast.nodes[self.0].data.sub_range };
        let shebang = unsafe { ast.extra_data[data.end].optional_atom };
        shebang
    }

    pub fn set_shebang(&self, ast: &mut Ast, shebang: OptionalAtomRef) {
        let data = unsafe { ast.nodes[self.0].data.sub_range };
        ast.extra_data[data.start].optional_atom = shebang;
    }
}

#[ast]
pub struct Script {
    pub span: Span,
    pub body: SubRange,
    pub shebang: OptionalAtomRef,
}

impl Script {}

// #[ast]
// pub enum ModuleItem {
//     ModuleDecl(NodeId),
//     Stmt(NodeId),
// }
