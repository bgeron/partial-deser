/// Represents the source of a data value that can be repeatedly deserialized.
/// For instance, serde_json on a borrowed string.
pub trait Source<'a> {
    type Deserializer: serde::Deserializer<'a>;

    /// Recreate a deserializer for this source.
    ///
    /// Every deserializer created from a source should behave exactly the
    /// same.
    ///
    /// If end of file happens in a map in between the key and the value, then
    /// the first go at partially deserializing will fail, and we have to recreate
    /// a new deserializer for the same source.
    fn recreate_deserializer(&mut self) -> Self::Deserializer;
}
