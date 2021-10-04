use super::*;
use syn::parse::{ Parse, ParseStream, Result as ParseResult, Error as ParseError };
use syn::{ Ident, TypePath, Expr };

enum FormMapValue {
    Data(TokenMap<Ident, TypePath>),
    Func(Expr)
}

impl Parse for ImplFormBody {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        let mut initial_data = TokenMap::parse(input, Ident::parse, |input, key| match &*key.to_string().to_lowercase() {
            "data" => TokenMap::parse(input, Ident::parse, |input, _| TypePath::parse(input), |i| i.clone()).map(FormMapValue::Data),
            "func" => Expr::parse(input).map(FormMapValue::Func),
            _ => Err(ParseError::new_spanned(key, "Keys \"data\" and \"func\" are the only ones allowed"))
        }, |i| i.to_string().to_lowercase())?;

        let data = match initial_data.map.remove("data").unwrap() {
            FormMapValue::Data(data) => data,
            _ => unreachable!()
        };

        let func = match initial_data.map.remove("func").unwrap() {
            FormMapValue::Func(f) => f,
            _ => unreachable!()
        };

        Ok(
            ImplFormBody {
                data, func
            }
        )
    }
}