use swc_common::Span;

use crate::node_id::AtomRef;

pub struct Ident {
    pub span: Span,
    pub sym: AtomRef,
    pub optional: bool,
}

pub struct BindingIdent {}
