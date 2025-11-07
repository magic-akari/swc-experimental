oxc_index::define_index_type! {
    pub struct ExtraDataId = u32;
}

pub(crate) trait ExtraDataCompact: Sized {
    fn to_extra_data(self) -> u64;
    fn from_extra_data(raw: u64) -> Self;
}
