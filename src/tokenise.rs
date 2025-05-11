use macroific::elements::{GenericImpl, ModulePrefix};
use macroific::prelude::*;
use proc_macro2::{Delimiter, Group, Ident, Punct, TokenStream};
use quote::{quote, ToTokens, TokenStreamExt};
use syn::punctuated::Punctuated;
use syn::{Generics, Token};

use crate::options::ContainerOptions;
use crate::types::{FieldsSource, MiniField};
use crate::FancyConstructor;

const TRAIT_DEFAULT: ModulePrefix<3> = ModulePrefix::new(["core", "default", "Default"]);
const NAME_DEFAULT: &str = "new";

impl FancyConstructor {
    #[inline]
    pub fn into_token_stream(self) -> TokenStream {
        let Self {
            struct_name,
            generics,
            fields,
            opts,
        } = self;

        let header = GenericImpl::new(&generics).with_target(&struct_name);
        let default = make_default(&generics, &struct_name, &opts);
        let body = make_container_body(opts, fields);

        quote! {
            #[automatically_derived]
            #[allow(clippy::all)]
            #header {
                #body
            }

            #default
        }
    }
}

#[inline]
fn make_default(generics: &Generics, struct_name: &Ident, opts: &ContainerOptions) -> TokenStream {
    if !opts.default {
        return TokenStream::new();
    }

    let header = GenericImpl::new(&generics)
        .with_trait(TRAIT_DEFAULT)
        .with_target(&struct_name);

    let new_name = if let Some(name) = &opts.name {
        name.clone()
    } else {
        Ident::create(NAME_DEFAULT)
    };

    quote! {
        #[automatically_derived]
        #[allow(clippy::all)]
        #header {
            #[inline]
            fn default() -> Self {
                #struct_name::#new_name()
            }
        }
    }
}

#[inline]
fn make_container_body(opts: ContainerOptions, fields: FieldsSource) -> TokenStream {
    let ContainerOptions {
        const_fn,
        default: _,
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
        Ident::create(NAME_DEFAULT)
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
                tokens.extend(quote!(#TRAIT_DEFAULT::default()));
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
    let Some((_, fields)) = fields.fields().to_slice() else {
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
