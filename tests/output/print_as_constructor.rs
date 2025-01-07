use std::fmt::{Debug, Display};

pub(crate) mod prelude {
    //! Functions necessary to use the resulting Rust code
    pub(crate) use bstr::B;
    pub(crate) use serde_json::json;

    pub(crate) use crate::common::ComparisonLine::*;
}

/// Wrapper around a type to make it print in the way to construct
/// it for Rust, both as Debug and Display.
///
/// Byte strings will be construct with `b""` syntax.
///
/// It also implements [`PartialEq`] from the left side with what's inside.
#[derive(Clone, Copy)]
pub(crate) struct PrintAsConstructor<T: ?Sized>(pub T);

impl<T> From<T> for PrintAsConstructor<T> {
    fn from(inner: T) -> Self {
        PrintAsConstructor(inner)
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

impl<T: FmtConstructor + ?Sized> Debug for PrintAsConstructor<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        FmtConstructor::fmt(&self.0, f)
    }
}

impl<T: FmtConstructor + ?Sized> Display for PrintAsConstructor<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        FmtConstructor::fmt(&self.0, f)
    }
}

pub(crate) struct PrintAsConstructorByRef<'a, T: ?Sized>(pub(crate) &'a T);

impl<T: FmtConstructor + ?Sized> Display for PrintAsConstructorByRef<'_, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        FmtConstructor::fmt(self.0, f)
    }
}

pub trait FmtConstructor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;
}

impl<T: FmtConstructor + NotSpecialCasedRef + ?Sized> FmtConstructor for &T {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "&{}", PrintAsConstructorByRef(self))
    }
}

trait NotSpecialCasedRef {}

impl FmtConstructor for &str {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", &self)
    }
}

impl FmtConstructor for str {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", &self)
    }
}

impl FmtConstructor for String {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}.to_string()", self)
    }
}

impl FmtConstructor for &'_ [u8] {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, r#"B(b"{}")"#, self.escape_ascii())
    }
}

impl<T: FmtConstructor> FmtConstructor for Option<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Some(inner) => write!(f, "Some({})", PrintAsConstructorByRef(inner)),
            None => write!(f, "None"),
        }
    }
}

impl<T: FmtConstructor, E: FmtConstructor> FmtConstructor for Result<T, E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Ok(value) => write!(f, "Ok({})", PrintAsConstructorByRef(value)),
            Err(err) => write!(f, "Err({})", PrintAsConstructorByRef(err)),
        }
    }
}

impl<T: FmtConstructor> FmtConstructor for Vec<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "vec![{}]",
            self.iter()
                .map(PrintAsConstructorByRef)
                .map(|inner| inner.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}

impl FmtConstructor for () {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "()")
    }
}

impl<T1> FmtConstructor for (T1,)
where
    T1: FmtConstructor + ?Sized,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},)", PrintAsConstructorByRef(&self.0))
    }
}

impl<T1, T2> FmtConstructor for (T1, T2)
where
    T1: FmtConstructor,
    T2: FmtConstructor,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", PrintAsConstructorByRef(&self.0), PrintAsConstructorByRef(&self.1))
    }
}

impl<T1, T2, T3> FmtConstructor for (T1, T2, T3)
where
    T1: FmtConstructor,
    T2: FmtConstructor,
    T3: FmtConstructor,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({}, {}, {})",
            PrintAsConstructorByRef(&self.0),
            PrintAsConstructorByRef(&self.1),
            PrintAsConstructorByRef(&self.2)
        )
    }
}

impl FmtConstructor for serde_json::Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "json!({})", self)
    }
}

impl FmtConstructor for bool {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl FmtConstructor for u8 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl FmtConstructor for u16 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl FmtConstructor for u32 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl FmtConstructor for u64 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl FmtConstructor for i8 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl FmtConstructor for i16 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl FmtConstructor for i32 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl FmtConstructor for i64 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl FmtConstructor for f32 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl FmtConstructor for f64 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}
