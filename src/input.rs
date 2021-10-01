mod ops_data;
pub use ops_data;

use std::io::{ stdin, Read, Lines, Result as IoResult };

// Prints a message to the console and then asks for a line of input
pub fn get_input(msg: &str, reader: &mut impl Iterator<Item = IoResult<String>>) -> IoResult<String> {
    println!("{}", msg);
    reader.next()
}



// Returns a string representing the input options given as arguments
pub fn get_ops(ops: impl Iterator<Item = (&str, &str)>) -> String {

}