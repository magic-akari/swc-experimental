use crate::{Ast, define_optional_index_type};

pub trait GetNodeId {
    fn node_id(&self) -> NodeId;
}

pub trait GetOptionalNodeId {
    fn optional_node_id(&self) -> OptionalNodeId;
}

impl<T: GetNodeId> GetOptionalNodeId for T {
    fn optional_node_id(&self) -> OptionalNodeId {
        self.node_id().into()
    }
}

// TODO: Make it unsafe
pub trait FromNodeId {
    fn from_node_id(id: NodeId, ast: &Ast) -> Self;
}

oxc_index::define_index_type! {
    pub struct NodeId = u32;
}

define_optional_index_type!(OptionalNodeId, NodeId);
