mod from_input;
pub use from_input::*;

mod ops_data;
pub use ops_data::*;

mod error;
pub use error::*;

use std::io::{ Result as IoResult };

// Prints a message to the console and then asks for a line of input
pub fn get_multiple_option_input<T, I>(msg: &str, reader: &mut I) -> MultipleOptionResult<T>
    where T: FromMultipleOptionInput,
          I: Iterator<Item = IoResult<String>> {
    println!("{}", msg);
    match reader.next() {
        None =>             Err(MultipleOptionError::EOI),
        Some(Err(e)) =>     Err(MultipleOptionError::IoErr(e)),
        Some(Ok(v)) =>      T::from_str(&v).map_err( |e| MultipleOptionError::ToStrErr(e) )
    }
}