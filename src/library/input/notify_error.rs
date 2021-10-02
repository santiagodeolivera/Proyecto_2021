use std::io::Result as IoResult;

pub fn notify_error(msg: &str, input: &mut (impl Iterator<Item = IoResult<String>> + ?Sized)) {
    println!("\n{}\nPress enter to continue...", msg);
    input.next();
}

