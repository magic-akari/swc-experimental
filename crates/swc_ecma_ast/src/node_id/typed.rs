use std::marker::PhantomData;

use crate::node_id::SubRange;

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
