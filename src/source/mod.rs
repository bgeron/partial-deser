use std::borrow::Borrow;

use serde::Deserializer;

/// Represents the source of a data value that can be repeatedly deserialized.
/// For instance, serde_json on a borrowed string.
pub trait Source<'de> {
    /// Stack storage for the deserializer.
    type DeserializerStorage;

    type Deserializer<'storage>: Deserializer<'de, Error = Self::Error>
    where
        'de: 'storage;

    /// The error type cannot depend on `'storage`.
    type Error;

    /// Recreate a deserializer for this source.
    ///
    /// Every deserializer created from a source should behave exactly the
    /// same.
    ///
    /// If end of file happens in a map in between the key and the value, then
    /// the first go at partially deserializing will fail, and we have to recreate
    /// a new deserializer for the same source.
    fn recreate_deserializer_storage(&mut self) -> Self::DeserializerStorage;

    /// Will be called exactly once per [`Self::DeserializerStorage`]. The argument
    /// is guaranteed to be `Some`.
    ///
    /// Typically returns either `storage.take()` or `&mut storage`.
    fn use_deserializer_from_storage<'storage>(
        storage: &'storage mut Option<Self::DeserializerStorage>,
    ) -> Self::Deserializer<'storage>;
}

/// Use [`serde_json::from_str`].
#[cfg(feature = "serde_json")]
pub struct JsonStr<T: Borrow<str>>(T);

/// Use [`serde_json::from_slice`].
#[cfg(feature = "serde_json")]
pub struct JsonBytes<T: Borrow<[u8]>>(T);

#[cfg(feature = "serde_json")]
impl<'de, T: Borrow<str>> Source<'de> for &'de JsonStr<T> {
    type DeserializerStorage = serde_json::Deserializer<serde_json::de::StrRead<'de>>;
    type Deserializer<'storage>
        = &'storage mut serde_json::Deserializer<serde_json::de::StrRead<'de>>
    where
        'de: 'storage;
    type Error = serde_json::Error;

    // type Deserializer<'storage> =
    //     &'storage mut serde_json::Deserializer<serde_json::de::StrRead<'de>>;

    fn recreate_deserializer_storage<'a>(&'a mut self) -> Self::DeserializerStorage {
        serde_json::Deserializer::from_str(self.0.borrow())
    }

    fn use_deserializer_from_storage<'storage>(
        storage: &'storage mut Option<serde_json::Deserializer<serde_json::de::StrRead<'de>>>,
    ) -> &'storage mut serde_json::Deserializer<serde_json::de::StrRead<'de>> {
        storage
            .as_mut()
            .expect("use_deserializer_from_storage only called on Some")
    }
}

#[cfg(feature = "serde_json")]
impl<'de, T: Borrow<[u8]>> Source<'de> for &'de JsonBytes<T> {
    type DeserializerStorage = serde_json::Deserializer<serde_json::de::SliceRead<'de>>;
    type Deserializer<'storage>
        = &'storage mut serde_json::Deserializer<serde_json::de::SliceRead<'de>>
    where
        'de: 'storage;
    type Error = serde_json::Error;

    fn recreate_deserializer_storage<'a>(&'a mut self) -> Self::DeserializerStorage {
        serde_json::Deserializer::from_slice(self.0.borrow())
    }

    fn use_deserializer_from_storage<'storage>(
        storage: &'storage mut Option<serde_json::Deserializer<serde_json::de::SliceRead<'de>>>,
    ) -> &'storage mut serde_json::Deserializer<serde_json::de::SliceRead<'de>> {
        storage
            .as_mut()
            .expect("use_deserializer_from_storage only called on Some")
    }
}
