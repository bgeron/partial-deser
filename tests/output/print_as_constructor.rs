use std::fmt::{Debug, Display};

/// Wrapper around a type to make it print in the way to construct
/// it for Rust, both as Debug and Display.
///
/// Byte strings will be construct with `b""` syntax.
///
/// It also implements [`PartialEq`] from the left side with what's inside.
#[derive(Clone, Copy)]
pub(crate) struct PrintAsConstructor<T: ?Sized>(pub T);

impl<T> PrintAsConstructor<T> {
    pub fn map<U>(self, f: impl FnOnce(T) -> U) -> PrintAsConstructor<U> {
        PrintAsConstructor(f(self.0))
    }
}

impl<T, U> PartialEq<U> for PrintAsConstructor<T>
where
    T: PartialEq<U>,
{
    fn eq(&self, other: &U) -> bool {
        &self.0 == other
    }
}

impl<T> Debug for PrintAsConstructor<T>
where
    PrintAsConstructor<T>: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

impl<T> Display for PrintAsConstructor<&T>
where
    PrintAsConstructor<T>: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "&{}", PrintAsConstructor(self.0))
    }
}

impl Display for PrintAsConstructor<str> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", &self.0)
    }
}

impl Display for PrintAsConstructor<String> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}.to_string()", self.0)
    }
}

impl Display for PrintAsConstructor<&'_ [u8]> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, r#"b"{:?}""#, self.0.escape_ascii())
    }
}

impl<T> Display for PrintAsConstructor<Option<T>>
where
    PrintAsConstructor<T>: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.0 {
            Some(inner) => write!(f, "Some({})", PrintAsConstructor(inner)),
            None => write!(f, "None"),
        }
    }
}

impl<T, E> Display for PrintAsConstructor<Result<T, E>>
where
    PrintAsConstructor<T>: Display,
    PrintAsConstructor<E>: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.0 {
            Ok(value) => write!(f, "Ok({})", PrintAsConstructor(value)),
            Err(err) => write!(f, "Err({})", PrintAsConstructor(err)),
        }
    }
}

impl<T> Display for PrintAsConstructor<Vec<T>>
where
    PrintAsConstructor<T>: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}]",
            self.0
                .iter()
                .map(PrintAsConstructor)
                .map(|inner| inner.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}

impl Display for PrintAsConstructor<()> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "()")
    }
}

impl<T1> Display for PrintAsConstructor<(T1,)>
where
    PrintAsConstructor<T1>: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},)", PrintAsConstructor(&self.0 .0))
    }
}

impl<T1, T2> Display for PrintAsConstructor<(T1, T2)>
where
    PrintAsConstructor<T1>: Display,
    PrintAsConstructor<T2>: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({}, {})",
            PrintAsConstructor(&self.0 .0),
            PrintAsConstructor(&self.0 .1)
        )
    }
}

impl<T1, T2, T3> Display for PrintAsConstructor<(T1, T2, T3)>
where
    PrintAsConstructor<T1>: Display,
    PrintAsConstructor<T2>: Display,
    PrintAsConstructor<T3>: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({}, {}, {})",
            PrintAsConstructor(&self.0 .0),
            PrintAsConstructor(&self.0 .1),
            PrintAsConstructor(&self.0 .2)
        )
    }
}

impl Display for PrintAsConstructor<serde_json::Value> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "::serde_json::json!({})", self.0)
    }
}

impl Display for PrintAsConstructor<bool> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Display for PrintAsConstructor<u8> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Display for PrintAsConstructor<u16> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Display for PrintAsConstructor<u32> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Display for PrintAsConstructor<u64> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Display for PrintAsConstructor<i8> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Display for PrintAsConstructor<i16> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Display for PrintAsConstructor<i32> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Display for PrintAsConstructor<i64> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Display for PrintAsConstructor<f32> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Display for PrintAsConstructor<f64> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
