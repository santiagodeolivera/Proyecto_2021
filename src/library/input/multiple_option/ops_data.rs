use std::fmt::{ Display, Formatter, Result as FmtResult };
use std::convert::From;

// Used in the context of when the program asks for multiple-option data to the user,
// it represents the options of which that data can be.
pub struct OpsData<'a> {
    pub data: &'a [ (&'a str, &'a str) ]
}

impl<'a> From<&'a [ (&'a str, &'a str) ]> for OpsData<'a> {
    fn from(data: &'a [ (&'a str, &'a str) ]) -> Self {
        OpsData { data }
    }
}

impl<'a> Display for OpsData<'a> {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        for i in self.data {
            writeln!(f, "{}: {}", i.0, i.1)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ops_data() {
        let expected = if cfg!(windows) {
            "S: Sign up\r\nL: Log in\r\n"
        } else {
            "S: Sign up\nL: Log in\n"
        };

        let actual = OpsData {
            data: &[
                ("S", "Sign up"),
                ("L", "Log in")
            ]
        };
        assert_eq!(expected, actual.to_string());
    }
}