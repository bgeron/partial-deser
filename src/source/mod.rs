use serde::Deserializer;

#[cfg(feature = "serde_json")]
mod json;
#[cfg(feature = "serde_yaml")]
mod yaml;

#[cfg(feature = "serde_json")]
pub use json::{JsonBytes, JsonStr};
#[cfg(feature = "serde_yaml")]
pub use yaml::{YamlBytes, YamlStr};

/// Represents the source of a data value that can be repeatedly deserialized.
/// For instance, serde_json on a borrowed string.
pub trait Source<'de> {
    /// Stack storage for the deserializer.
    type DeserializerStorage;

    type Deserializer<'storage>: Deserializer<'de, Error = Self::Error>
    where
        'de: 'storage;

    /// The error type cannot depend on `'storage`.
    type Error: serde::de::Error;

    /// Recreate a deserializer for this source.
    ///
    /// Every deserializer created from a source should behave exactly the
    /// same.
    ///
    /// If end of file happens in a map in between the key and the value, then
    /// our first go at deserializing will fail, and we have to recreate
    /// a new deserializer for the same source.
    fn recreate_deserializer_storage(&mut self) -> Self::DeserializerStorage;

    /// Will be called exactly once per [`Self::DeserializerStorage`]. The argument
    /// is guaranteed to be `Some`.
    ///
    /// Typically returns either `storage.take()` or `&mut storage`.
    fn use_deserializer_from_storage(
        storage: &mut Option<Self::DeserializerStorage>,
    ) -> Self::Deserializer<'_>;
}
