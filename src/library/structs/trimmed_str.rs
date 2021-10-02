use std::ops::Deref;
use serde::{ Deserialize, Serialize };

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq, Clone)]
#[serde(transparent)]
pub struct TrimmedStr {
    s: String
}

use std::str::FromStr;
impl FromStr for TrimmedStr {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, ()> {
        let s = s.trim().to_string();
        match s.len() {
            0 => Err(()),
            _ => Ok(TrimmedStr { s })
        }
    }
}

use std::borrow::Cow;
use crate::library::input::simple::FromSimpleInput;
impl FromSimpleInput for TrimmedStr {
    fn error_str(_error: ()) -> Cow<'static, str> {
        Cow::Borrowed("Missing input")
    }
}

impl TrimmedStr {
    pub fn mutate(&mut self, f: impl FnOnce(&mut String)) {
        if &self.s != self.s.trim() {  self.s = self.s.trim().to_string()  }
        f(&mut self.s);
        self.s = self.s.trim().to_string();
    }
}

impl Deref for TrimmedStr {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.s.trim()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ops::Deref;

    fn test_base(expected: Option<&str>, initial: &str) {
        let trimmed = TrimmedStr::from_str(initial);
        assert_eq!(expected, trimmed.as_ref().map(Deref::deref).ok());
    }

    #[test]
    fn test_create() {
        test_base(
            Some("Hello world!"),
            "Hello world!"
        )
    }

    #[test]
    fn test_trim() {
        test_base(
            Some("HELLO WORLD"),
            "  HELLO WORLD      \n"
        );
    }

    #[test]
    fn test_empty() {
        test_base(
            None,
            "   \n   \n\n"
        );
    }
}