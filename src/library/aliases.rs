use std::io::Error as IoError;
pub type LineInputResult = Result<String, Option<IoError>>;