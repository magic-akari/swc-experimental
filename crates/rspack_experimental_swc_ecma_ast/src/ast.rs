use std::marker::PhantomData;

use crate::node_id::SubRange;

pub(crate) mod ident;
pub(crate) mod lit;
pub(crate) mod module;
pub(crate) mod module_decl;
pub(crate) mod stmt;

pub struct TypedSubRange<T> {
    sub_range: SubRange,
    _phantom: PhantomData<T>,
}

impl<T> From<SubRange> for TypedSubRange<T> {
    fn from(sub_range: SubRange) -> Self {
        Self {
            sub_range,
            _phantom: PhantomData::default(),
        }
    }
}

impl<T> From<TypedSubRange<T>> for SubRange {
    fn from(value: TypedSubRange<T>) -> Self {
        value.sub_range
    }
}
