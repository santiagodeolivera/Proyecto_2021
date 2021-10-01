use std::io::{ BufRead, BufReader, stdin, Result as IoResult };

pub fn line_reader() -> impl Iterator<Item = IoResult<String>> {
    BufReader::new(stdin()).lines()
}
