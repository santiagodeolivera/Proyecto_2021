use std::io::Result as IoResult;

pub fn get_form_input<T, I>(input: &mut I) where
    T: FromFormInput,
    I: Iterator<Item = IoResult<String>> {
        
    }

#[cfg(test)]
mod tests {
    use crate::library::program::input::line_reader;

    #[test]
    fn test_log_in() {
        let input = line_reader();
        let data: Option<LogInData> = get_form_input(&mut input);
    }
}