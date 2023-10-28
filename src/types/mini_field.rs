use macroific::attr_parse::ParseOption;
use proc_macro2::{Ident, TokenStream};
use quote::ToTokens;
use syn::parse::{Parse, ParseStream};
use syn::Token;

#[derive(ParseOption)]
#[attr_opts(from_parse)]
pub struct MiniField {
    pub ident: Ident,
    pub colon_token: Token![:],
    pub ty: syn::Type,
}

impl Parse for MiniField {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            ident: input.parse()?,
            colon_token: input.parse()?,
            ty: input.parse()?,
        })
    }
}

impl ToTokens for MiniField {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.ident.to_tokens(tokens);
        self.colon_token.to_tokens(tokens);
        self.ty.to_tokens(tokens);
    }
}
