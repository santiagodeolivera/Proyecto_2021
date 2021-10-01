use std::io::stdout;
use std::io::Write;
pub fn flush_stdout() {
    stdout().flush().ok().expect("Failure when flushing stdout");
}

// use std::str::FromStr;
// pub type StrResult<T> = Result<T, <T as FromStr>::Err>;