use std::borrow::Cow;

pub trait FromInput {
    type Err;
    fn error_str(error: <Self as FromInput>::Err) -> Cow<'static, str>;
}