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

#[ast]
pub struct Module {
    pub body: SubRange,
    pub shebang: OptionalAtomRef,
}

#[ast]
pub struct Script {
    pub body: SubRange,
    pub shebang: OptionalAtomRef,
}

// #[ast]
// pub enum ModuleItem {
//     ModuleDecl,
//     Stmt,
// }
