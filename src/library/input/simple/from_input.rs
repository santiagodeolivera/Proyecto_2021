use std::borrow::Cow;

pub trait FromSimpleInput: Sized {
    type Err;
    fn from_input(s: &str) -> Result<Self, Self::Err>;
    fn error_str(_error: Self::Err) -> Cow<'static, str> {
        Cow::Borrowed("Invalid input")
    }
}