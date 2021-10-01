use std::fmt::{ Display, Formatter, Result as FmtResult };

pub struct OpsData<'a> {
    data: &'a [ (&'a str, &'a str) ]
}

impl<'a> Display for OpsData<'a> {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        for i in self.data {
            writeln!("{}: {}", i.0, i.1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ops_data() {
        let expected = "S: Sign up\nL: Log in\n";
        let actual = OpsData {
            data: &[
                ("S", "Sign up"),
                ("L", "Log in")
            ]
        };
        assert_eq!(expected, actual);
    }
}