use std::io::Result as IoResult;

pub fn notify_error(msg: &str, input: &mut impl Iterator<Item = IoResult<String>>) {
    println!("\n{}\nPress enter to continue...", msg);
    input.next();
}

