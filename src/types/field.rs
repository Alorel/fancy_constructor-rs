use crate::options::FieldOptions;
use crate::parse::create_span;
use crate::{FmtTuple, ATTR_NAME};
use macroific::prelude::*;
use proc_macro2::Ident;

pub struct Field {
    pub name: Ident,
    pub opts: FieldOptions,
    pub ty: syn::Type,
}

impl Field {
    pub fn resolve_ident(&self) -> &Ident {
        if let Some(ref name) = self.opts.name {
            name
        } else {
            &self.name
        }
    }

    pub fn collect<F, It>(iter: It) -> syn::Result<F>
    where
        F: FromIterator<Field>,
        It: IntoIterator<Item = FmtTuple>,
    {
        iter.into_iter()
            .map(move |(attrs, ident, ty)| {
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
            })
            .collect()
    }
}
