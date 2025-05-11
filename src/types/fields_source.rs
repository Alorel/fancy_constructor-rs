use super::{Field, Fields};
use crate::FmtTuple;
use proc_macro2::Ident;
use quote::format_ident;
use syn::Error;

pub enum FieldsSource {
    Struct(Fields),
    Enum { variant: Ident, fields: Fields },
}

impl FieldsSource {
    pub(crate) fn fields(&self) -> &Fields {
        match *self {
            FieldsSource::Struct(ref fields) | FieldsSource::Enum { ref fields, .. } => fields,
        }
    }
}

impl TryFrom<syn::Fields> for Fields {
    type Error = Error;

    fn try_from(fields: syn::Fields) -> Result<Self, Self::Error> {
        Ok(match fields {
            syn::Fields::Unit => Fields::Unit,
            syn::Fields::Named(f) => Fields::Named(Field::collect(fmt_named(f.named))?),
            syn::Fields::Unnamed(f) => Fields::Unnamed(Field::collect(fmt_unnamed(f.unnamed))?),
        })
    }
}

fn fmt_named(fields: impl IntoIterator<Item = syn::Field>) -> impl Iterator<Item = FmtTuple> {
    fields.into_iter().map(move |f| {
        (
            f.attrs,
            Ok(f.ident.expect("Failed to get ident off a named field")),
            f.ty,
        )
    })
}

fn fmt_unnamed(fields: impl IntoIterator<Item = syn::Field>) -> impl Iterator<Item = FmtTuple> {
    fields
        .into_iter()
        .enumerate()
        .map(move |(i, syn::Field { attrs, ty, .. })| (attrs, Err(format_ident!("f{}", i + 1)), ty))
}
