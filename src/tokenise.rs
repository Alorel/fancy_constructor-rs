use macroific::elements::GenericImpl;
use macroific::prelude::*;
use proc_macro2::{Delimiter, Group, Ident, Punct, TokenStream};
use quote::{quote, ToTokens, TokenStreamExt};
use syn::punctuated::Punctuated;
use syn::Token;

use crate::options::{ContainerOptions, FieldOptions};
use crate::types::MiniField;
use crate::{FancyConstructor, Field, Fields, FieldsSource};

impl FancyConstructor {
    #[inline]
    pub fn into_token_stream(self) -> TokenStream {
        let Self {
            struct_name,
            generics,
            fields,
            opts,
        } = self;

        let header = GenericImpl::new(generics).with_target(struct_name);
        let body = make_container_body(opts, fields);
        quote! {
            #[automatically_derived]
            #[allow(clippy::all)]
            #header {
                #body
            }
        }
    }
}

#[inline]
fn make_container_body(opts: ContainerOptions, fields: FieldsSource) -> TokenStream {
    let ContainerOptions {
        const_fn,
        vis,
        name,
        comment,
        bounds,
        args,
    } = opts;

    let mut tokens = if let Some(comment) = comment {
        quote!(#[doc = #comment])
    } else {
        quote!(#[doc = "Constructs a new instance of the struct."])
    };

    if let Some(vis) = vis {
        vis.to_tokens(&mut tokens);
    } else {
        tokens.append(Ident::create("pub"));
    }

    if const_fn {
        tokens.append(Ident::create("const"));
    }

    tokens.append(Ident::create("fn"));
    tokens.append(if let Some(name) = name {
        name
    } else {
        Ident::create("new")
    });

    tokens.append(Group::new(Delimiter::Parenthesis, make_args(&fields, args)));

    <Token![->]>::default().to_tokens(&mut tokens);
    tokens.append(Ident::create("Self"));

    if !bounds.is_empty() {
        tokens.append(Ident::create("where"));
        tokens.append_separated(bounds, <Token![,]>::default());
    }

    tokens.append(Group::new(Delimiter::Brace, make_fn_body(fields)));

    tokens
}

#[inline]
fn make_fn_body(fields: FieldsSource) -> TokenStream {
    let mut tokens = quote!(Self);

    let fields = match fields {
        FieldsSource::Struct(fields) => fields.into_vec(),
        FieldsSource::Enum { variant, fields } => {
            tokens.append(Punct::new_joint(':'));
            tokens.append(Punct::new_joint(':'));
            tokens.append(variant);
            fields.into_vec()
        }
    };

    let (named, fields) = match fields {
        None => return tokens,
        Some(v) => v,
    };

    let delim = if named {
        Delimiter::Brace
    } else {
        Delimiter::Parenthesis
    };

    tokens.append(Group::new(delim, {
        let iter = fields.into_iter().map(move |field| {
            let mut tokens = TokenStream::new();
            if named {
                field.name.to_tokens(&mut tokens);
                tokens.append(Punct::new_alone(':'));
            }

            if field.opts.default {
                tokens.extend(quote!(::core::default::Default::default()));
            } else if let Some(ref value) = field.opts.value {
                value.to_tokens(&mut tokens);
            } else {
                field.resolve_ident().to_tokens(&mut tokens);

                if field.opts.clone {
                    tokens.extend(quote!(.clone()));
                }

                if field.opts.into {
                    tokens.extend(quote!(.into()));
                }
            }

            tokens
        });

        let mut tokens = TokenStream::new();
        tokens.append_separated(iter, <Token![,]>::default());

        tokens
    }));

    tokens
}

fn make_args(fields: &FieldsSource, args: Punctuated<MiniField, impl ToTokens>) -> TokenStream {
    let mut tokens = TokenStream::new();
    let Some((_, fields)) = fields.fields().to_vec() else {
        return tokens;
    };

    let iter_args = args.into_iter().map(MiniField::into_token_stream);

    let iter_fields = fields.iter().filter_map(move |field| {
        if field.opts.should_skip_args() {
            return None;
        }

        let mut tokens = field.resolve_ident().to_token_stream();
        tokens.append(Punct::new_alone(':'));

        if field.opts.uses_reference() {
            tokens.append(Punct::new_joint('&'));
        }

        if field.opts.into {
            let ty = &field.ty;
            tokens.extend(quote!(impl ::core::convert::Into<#ty>));
        } else {
            field.ty.to_tokens(&mut tokens);
        }

        Some(tokens)
    });

    tokens.append_separated(iter_args.chain(iter_fields), <Token![,]>::default());

    tokens
}

impl FieldOptions {
    #[inline]
    pub fn uses_reference(&self) -> bool {
        self.clone
    }

    #[inline]
    fn should_skip_args(&self) -> bool {
        self.default || self.value.is_some()
    }
}

impl FieldsSource {
    fn fields(&self) -> &Fields {
        match *self {
            FieldsSource::Struct(ref fields) | FieldsSource::Enum { ref fields, .. } => fields,
        }
    }
}

impl Fields {
    fn to_vec(&self) -> Option<(bool, &[Field])> {
        match *self {
            Fields::Unit => None,
            Fields::Named(ref fields) => Some((true, fields)),
            Fields::Unnamed(ref fields) => Some((false, fields)),
        }
    }

    fn into_vec(self) -> Option<(bool, Vec<Field>)> {
        match self {
            Fields::Unit => None,
            Fields::Named(fields) => Some((true, fields)),
            Fields::Unnamed(fields) => Some((false, fields)),
        }
    }
}

impl Field {
    pub fn resolve_ident(&self) -> &Ident {
        if let Some(ref name) = self.opts.name {
            name
        } else {
            &self.name
        }
    }
}
