mod token_map;
pub use token_map::*;

mod wrappers;
pub use wrappers::*;

mod parse_terminated_with;
pub use parse_terminated_with::*;

use syn::Token;
use syn::parse::{ ParseStream, Result as ParseResult };

pub struct MapEl<T, U> {
    key: T,
    value: U
}

impl<T, U> MapEl<T, U> {
    fn parse(
        input: ParseStream,
        f1: impl FnOnce(ParseStream) -> ParseResult<T>,
        f2: impl FnOnce(ParseStream, &T) -> ParseResult<U>
    ) -> ParseResult<Self> {
        let key: T = f1(input)?;
        input.parse::<Token![:]>()?;
        let value: U = f2(input, &key)?;
        Ok(
            MapEl { key, value }
        )
    }
}
