use std::marker::PhantomData;

use crate::node_id::ExtraDataId;

#[derive(Debug, Clone, Copy, Hash)]
pub struct SubRange {
    pub start: ExtraDataId,
    pub end: ExtraDataId,
}

oxc_index::define_index_type! {
    pub struct BigIntId = u32;
}

pub struct TypedSubRange<T> {
    inner: SubRange,
    _phantom: PhantomData<T>,
}

impl<T> From<TypedSubRange<T>> for SubRange {
    fn from(value: TypedSubRange<T>) -> Self {
        value.inner
    }
}

impl SubRange {
    pub(crate) unsafe fn cast_to_typed<T>(self) -> TypedSubRange<T> {
        TypedSubRange {
            inner: self,
            _phantom: PhantomData::default(),
        }
    }
}
