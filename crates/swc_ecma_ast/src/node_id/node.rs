use crate::define_optional_index_type;

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

oxc_index::define_index_type! {
    pub struct NodeId = u32;
}

define_optional_index_type!(OptionalNodeId, NodeId);
