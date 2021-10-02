use std::borrow::Cow;
use crate::library::input::InputManager;
use std::fmt::Display;

pub trait FromFormInput: Display + Sized {
    type Err;

    fn from_input(input: &mut InputManager) -> Option<Result<Self, Self::Err>>;

    fn error_str(error: Self::Err) -> Cow<'static, str>;
}