/// Make a FnOnce that doesn't take any arguments.
///
/// This actually doesn't change the value, but the returned value
/// can definitely only be called once.
pub(crate) fn make_fnonce<T>(f: impl FnOnce() -> T) -> impl FnOnce() -> T {
    f
}

/// Erase `&Result` to a dyn error.
pub(crate) fn erase_error_ref<'error, T, E: std::error::Error + 'error>(
    result: &Result<T, E>,
) -> Option<&(dyn std::error::Error + 'error)> {
    result
        .as_ref()
        .err()
        .map(|x| -> &(dyn std::error::Error + 'error) { x })
}
