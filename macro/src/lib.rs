mod node;

use syn::{Result, Ident, Token, parse_macro_input, parenthesized, bracketed, parse::{Parse, ParseStream}, ext::IdentExt, token::{Paren, Bracket}};
use crate::node::Children;

#[proc_macro]
pub fn view(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let children = parse_macro_input!(input as Children);

    unimplemented!("{:?}", children);
}
