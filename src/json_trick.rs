use std::borrow::Cow;

pub struct Prepared<SliceType>(pub SliceType);

pub(crate) fn prepare_str_with_tag(tag: &str, input: &mut Cow<str>) {
    todo!()
}

pub(crate) fn prepare_slice_with_tag(tag: &str, input: &mut Cow<[u8]>) {
    todo!()
}
