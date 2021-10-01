mod from_input;
pub use from_input::*;

use std::io::{ Result as IoResult };

// Prints a message to the console and then asks for a line of input
pub fn get_multiple_option_input<T: FromInput>(msg: &str, reader: &mut impl Iterator<Item = IoResult<String>>) -> Option<IoResult<String>> {
    println!("{}", msg);
    reader.next()
}