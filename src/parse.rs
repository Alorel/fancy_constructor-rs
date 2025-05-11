use macroific::prelude::*;
use proc_macro2::Span;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::{Data, DeriveInput, Error, Variant};

use crate::options::ContainerOptions;
use crate::types::FieldsSource;
use crate::{FancyConstructor, ATTR_NAME};

impl Parse for FancyConstructor {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let DeriveInput {
            ident: struct_name,
            attrs,
            generics,
            data,
            vis: _,
        } = input.parse()?;

        let fields = match data {
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
        };

        Ok(Self {
            opts: if attrs.is_empty() {
                ContainerOptions::default()
            } else {
                let span = create_span(&attrs);
                let opts = ContainerOptions::from_iter_named(ATTR_NAME, span, attrs)?;

                if opts.default {
                    validate_opts(&opts, &fields, span)?;
                }

                opts
            },
            fields,
            struct_name,
            generics,
        })
    }
}

fn validate_opts(opts: &ContainerOptions, fields: &FieldsSource, span: Span) -> Result<(), Error> {
    if opts.args.is_empty() && fields.fields().is_argless() {
        Ok(())
    } else {
        Err(Error::new(
            span,
            "The constructor cannot have any arguments if the `default` option is used",
        ))
    }
}

pub fn create_span<'a, S>(it: impl IntoIterator<Item = &'a S>) -> Span
where
    S: Spanned + 'a,
{
    it.into_iter()
        .map(Spanned::span)
        .reduce(move |a, b| if let Some(span) = a.join(b) { span } else { a })
        .unwrap_or_else(Span::call_site)
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
