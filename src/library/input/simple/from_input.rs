use std::str::FromStr;
use std::borrow::Cow;

pub trait FromSimpleInput: FromStr {
    fn error_str(error: <Self as FromStr>::Err) -> Cow<'static, str>;
}