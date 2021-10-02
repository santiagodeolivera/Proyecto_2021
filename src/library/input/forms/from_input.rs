use std::borrow::Cow;
use crate::library::input::InputManager;

pub trait FromFormInput: Sized {
    type Err;
    fn from_input(input: &mut InputManager) -> Option<Result<Self, Self::Err>>;

    fn error_str(error: Self::Err) -> Cow<'static, str>;
}