use syn::parse::Parse;
use syn::parse::{ ParseStream, Result as ParseResult };
use syn::punctuated::Punctuated;

pub fn parse_terminated_with<T, P: Parse>(input: ParseStream, mut parser: impl FnMut(ParseStream) -> ParseResult<T>) -> ParseResult<Punctuated<T, P>> {
    let mut punctuated = Punctuated::new();

    loop {
        if input.is_empty() {
            break;
        }
        let value = parser(input)?;
        punctuated.push_value(value);
        if input.is_empty() {
            break;
        }
        let punct = input.parse()?;
        punctuated.push_punct(punct);
    }

    Ok(punctuated)
}