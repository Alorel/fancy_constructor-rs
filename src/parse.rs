use macroific::attr_parse::__private::try_collect;
use macroific::prelude::*;
use proc_macro2::{Ident, Span};
use quote::format_ident;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::{Attribute, Data, DeriveInput, Error, Fields as SFields, Type, Variant};

use crate::options::{ContainerOptions, FieldOptions};
use crate::{FancyConstructor, Field, Fields, FieldsSource, ATTR_NAME};

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
            fields: match data {
                Data::Struct(s) => FieldsSource::Struct(s.fields.try_into()?),
                Data::Enum(e) => {
                    let variant = find_variant(&e.enum_token, e.variants)?;

                    FieldsSource::Enum {
                        fields: variant.fields.try_into()?,
                        variant: variant.ident,
                    }
                }
                Data::Union(u) => {
                    return Err(Error::new_spanned(u.union_token, "Unions not supported"));
                }
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
        .reduce(move |a, b| if let Some(span) = a.join(b) { span } else { a })
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
              }| {
            (
                attrs,
                Ok(ident.expect("Failed to get ident off a named field")),
                ty,
            )
        },
    )
}

#[inline]
fn fmt_unnamed(fields: impl IntoIterator<Item = syn::Field>) -> impl Iterator<Item = FmtTuple> {
    fields
        .into_iter()
        .enumerate()
        .map(move |(i, syn::Field { attrs, ty, .. })| (attrs, Err(format_ident!("f{}", i + 1)), ty))
}

fn find_variant<P>(span: &impl Spanned, variants: Punctuated<Variant, P>) -> syn::Result<Variant> {
    let mut out = None;
    for variant in variants {
        if variant
            .attrs
            .iter()
            .any(move |a| a.path().is_ident(ATTR_NAME))
        {
            if out.is_some() {
                return Err(Error::new_spanned(
                    variant.ident,
                    "Multiple variants marked with `#[new]`",
                ));
            }
            out = Some(variant);
        }
    }

    if let Some(out) = out {
        Ok(out)
    } else {
        Err(Error::new(
            span.span(),
            "Expected a variant marked with `#[new]`",
        ))
    }
}

impl TryFrom<SFields> for Fields {
    type Error = Error;

    fn try_from(fields: SFields) -> Result<Self, Self::Error> {
        Ok(match fields {
            SFields::Unit => Fields::Unit,
            SFields::Named(f) => Fields::Named(collect_fields(fmt_named(f.named))?),
            SFields::Unnamed(f) => Fields::Unnamed(collect_fields(fmt_unnamed(f.unnamed))?),
        })
    }
}
