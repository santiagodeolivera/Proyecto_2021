use std::collections::HashMap;
use std::hash::Hash;
use std::fmt::Display;
use syn::Token;
use syn::parse::{ ParseStream, Result as ParseResult, Error as ParseError };
use syn::braced;
use syn::punctuated::{ Punctuated, Pair };
use syn::spanned::Spanned;
use super::{ parse_terminated_with, MapEl };

pub struct TokenMap<T, U> where T: Hash + Display {
    pub map: HashMap<T, U>
}

impl<T, U> TokenMap<T, U> where T: Eq + Hash + Display + Spanned {
    pub fn parse<V, F1, F2, F3>(
        input: ParseStream,
        key_f: F1,
        val_f: F2,
        to_key_f: F3
    ) -> ParseResult<Self> where
        F1: Fn(ParseStream) -> ParseResult<V>,
        F2: Fn(ParseStream, &V) -> ParseResult<U>,
        F3: Fn(&V) -> T
    {
        let content;
        braced!(content in input);
        let pairs: Punctuated<MapEl<V, U>, Token![,]> =
            parse_terminated_with(&content, move |st| MapEl::<V, U>::parse(st, &key_f, &val_f))?;
        let map: HashMap<T, U> = pairs.into_pairs().map(|pair| match pair {
            Pair::Punctuated(v, _) => v,
            Pair::End(v) => v
        }).map(|map_el| (map_el.key, map_el.value)).fold(Ok(HashMap::new()), |res, (k, v)| res.and_then(|mut map| {
            let dict_k = to_key_f(&k);
            if map.contains_key(&dict_k) {
                return Err(ParseError::new(dict_k.span(), format!("Keyword {} repeated", dict_k)));
            }
            map.insert(dict_k, v);
            Ok(map)
        }))?;
        Ok(
            TokenMap { map }
        )
    }
}


