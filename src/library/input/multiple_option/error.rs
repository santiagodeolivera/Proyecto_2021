use std::io::Error as IoError;
use std::str::FromStr;

pub type MultipleOptionResult<T> = Result<T, MultipleOptionError<<T as FromStr>::Err>>;

// This enum represents the result of obtaining an input line and parsing it into an object
pub enum MultipleOptionError<T> {

    // End Of Input error
    EOI,

    // I/O error
    IoErr(IoError),

    // Conversion error
    ToStrErr(T),

}
