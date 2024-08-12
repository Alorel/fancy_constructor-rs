use crate::types::MiniField;
use macroific::prelude::*;
use proc_macro2::Ident;
use syn::punctuated::Punctuated;
use syn::{Expr, LitStr, Token, TypeParam, Visibility};

#[derive(AttributeOptions, Default)]
pub(crate) struct ContainerOptions {
    pub const_fn: bool,
    pub vis: Option<Visibility>,
    pub name: Option<Ident>,
    pub comment: Option<LitStr>,
    pub bounds: Punctuated<TypeParam, Token![,]>,
    pub args: Punctuated<MiniField, Token![,]>,
}

#[derive(AttributeOptions)]
pub(crate) struct FieldOptions {
    pub default: bool,
    pub clone: bool,
    pub into: bool,
    pub name: Option<Ident>,

    #[attr_opts(rename = "val")]
    pub value: Option<Expr>,
}
