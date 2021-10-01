mod library;
pub use library::{
    program::input::line_reader,
    program::input::sign_up_or_log_in
};

fn main() {
    let mut input = line_reader();
    sign_up_or_log_in(&mut input);
}
