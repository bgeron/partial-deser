#![cfg_attr(not(feature = "serde_json"), allow(dead_code))]

/// Bytes and string types, which for serde_json may suffer from trailing data
/// that wasn't present in the input.
pub(crate) trait StringLike {
    /// Length in bytes
    fn len(&self) -> usize;
    fn ends_with_string(&self, string: &str) -> bool;
    fn truncate_to_bytes(&mut self, target_len: usize);
}

impl StringLike for &str {
    fn len(&self) -> usize {
        (*self).len()
    }

    fn ends_with_string(&self, string: &str) -> bool {
        self.ends_with(string)
    }

    fn truncate_to_bytes(&mut self, target_len: usize) {
        *self = &self[..target_len];
    }
}

impl StringLike for &[u8] {
    fn len(&self) -> usize {
        (*self).len()
    }

    fn ends_with_string(&self, string: &str) -> bool {
        self.ends_with(string.as_bytes())
    }

    fn truncate_to_bytes(&mut self, target_len: usize) {
        *self = &self[..target_len];
    }
}

impl StringLike for String {
    fn len(&self) -> usize {
        self.len()
    }

    fn ends_with_string(&self, string: &str) -> bool {
        self.ends_with(string)
    }

    fn truncate_to_bytes(&mut self, target_len: usize) {
        self.truncate(target_len);
    }
}

impl StringLike for Vec<u8> {
    fn len(&self) -> usize {
        self.len()
    }

    fn ends_with_string(&self, string: &str) -> bool {
        self.ends_with(string.as_bytes())
    }

    fn truncate_to_bytes(&mut self, target_len: usize) {
        self.truncate(target_len);
    }
}

#[cfg(test)]
impl StringLike for std::borrow::Cow<'_, str> {
    fn len(&self) -> usize {
        (**self).len()
    }

    fn ends_with_string(&self, string: &str) -> bool {
        self.ends_with(string)
    }

    fn truncate_to_bytes(&mut self, target_len: usize) {
        match self {
            std::borrow::Cow::Borrowed(slice) => {
                slice.truncate_to_bytes(target_len);
            }
            std::borrow::Cow::Owned(string) => {
                string.truncate_to_bytes(target_len);
            }
        }
    }
}
