use crate::options::{ContainerOptions, FieldOptions};
use crate::{FancyConstructor, Field, Fields, ATTR_NAME};
use macroific::attr_parse::__private::try_collect;
use macroific::extract_fields::Rejection;
use macroific::prelude::*;
use proc_macro2::{Ident, Span};
use quote::format_ident;
use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned;
use syn::{Attribute, Data, DeriveInput, Error, Fields as SFields, Type};

impl Parse for FancyConstructor {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let DeriveInput {
            ident: struct_name,
            attrs,
            generics,
            data,
            vis: _,
        } = input.parse()?;

        Ok(Self {
            fields: match get_fields(data)? {
                SFields::Unit => Fields::Unit,
                SFields::Named(f) => Fields::Named(collect_fields(fmt_named(f.named))?),
                SFields::Unnamed(f) => Fields::Unnamed(collect_fields(fmt_unnamed(f.unnamed))?),
            },
            opts: if attrs.is_empty() {
                ContainerOptions::default()
            } else {
                let span = create_span(&attrs);
                ContainerOptions::from_iter_named(ATTR_NAME, span, attrs)?
            },
            struct_name,
            generics,
        })
    }
}

fn create_span<'a, S>(it: impl IntoIterator<Item = &'a S>) -> Span
where
    S: Spanned + 'a,
{
    it.into_iter()
        .map(Spanned::span)
        .reduce(move |a, b| a.join(b).unwrap())
        .unwrap_or_else(Span::call_site)
}

fn collect_fields<F>(src_iter: impl Iterator<Item = FmtTuple>) -> syn::Result<F>
where
    F: FromIterator<Field>,
{
    let iter = src_iter.map(move |(attrs, ident, ty)| {
        Ok(Field {
            name: match ident {
                Ok(ident) | Err(ident) => ident,
            },
            opts: {
                let span = create_span(&attrs);
                FieldOptions::from_iter_named(ATTR_NAME, span, attrs)
            }?,
            ty,
        })
    });

    try_collect(iter)
}

type FmtTuple = (Vec<Attribute>, Result<Ident, Ident>, Type);

#[inline]
fn fmt_named(fields: impl IntoIterator<Item = syn::Field>) -> impl Iterator<Item = FmtTuple> {
    fields.into_iter().map(
        move |syn::Field {
                  attrs, ident, ty, ..
              }| { (attrs, Ok(ident.unwrap()), ty) },
    )
}

#[inline]
fn fmt_unnamed(fields: impl IntoIterator<Item = syn::Field>) -> impl Iterator<Item = FmtTuple> {
    fields
        .into_iter()
        .enumerate()
        .map(move |(i, syn::Field { attrs, ty, .. })| (attrs, Err(format_ident!("f{}", i + 1)), ty))
}

#[inline]
fn get_fields(data: Data) -> syn::Result<SFields> {
    let span = match data.extract_struct() {
        Ok(data) => return Ok(data.fields),
        Err(Rejection::A(a)) => a.enum_token.span,
        Err(Rejection::B(b)) => b.union_token.span,
    };

    Err(Error::new(span, "Expected a struct"))
}
