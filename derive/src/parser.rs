mod parse_map;
pub use parse_map::*;

pub mod structs;

use syn::{ Ident, Expr, TypePath, Token, LitStr };
use syn::punctuated::Punctuated;

mod interpreter;
mod evaluator;

enum FormMapValue {
    Data(TokenMap<Ident, TypePath>),
    Func(Expr)
}

struct ImplFormBody {
    data: TokenMap<Ident, TypePath>,
    func: Expr
}

struct ImplSimpleBody {
    param_name: Ident,
    body: Expr
}

struct MatchOp {
    pattern: Punctuated<LitStr, Token![|]>,
    expr: Expr
}

struct ImplMOBody {
    data: TokenMap<LitStrWrapper, MatchOp>
}

enum ImplBody {
    Form(ImplFormBody),
    Simple(ImplSimpleBody),
    MultipleOption(ImplMOBody)
}

struct ImplStruct {
    attributes: Vec<Ident>,
    t: TypePath,
    body: ImplBody
}

struct StructVec {
    v: Vec<ImplStruct>
}