// #![allow(unused_variables)]
// #![allow(unused_imports)]

pub mod library;

use std::convert::From;
use std::str::FromStr;
use std::path::PathBuf;

use library::input::InputManager;
use library::program::input::line_reader;
use library::program::memory::FileMemory;
use library::program::index::run;


fn main() {
    let mut input = line_reader();
    let input = InputManager::from( &mut input );
    let memory = FileMemory::from(PathBuf::from_str("./data.json").expect("Error when parsing memory file"));
    run(input, memory);
}
