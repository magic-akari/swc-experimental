use crate::define_optional_index_type;

oxc_index::define_index_type! {
    pub struct Wtf8AtomId = u32;
}

define_optional_index_type!(OptionalWtf8AtomId, Wtf8AtomId);
