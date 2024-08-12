use macroific::elements::ModulePrefix;
use macroific::prelude::*;
use proc_macro2::{Delimiter, Group, Ident, Punct, TokenStream};
use quote::{quote, ToTokens, TokenStreamExt};
use syn::punctuated::Punctuated;
use syn::{Generics, Token};

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

        let mut tokens = quote! {
            #[automatically_derived]
            #[allow(clippy::all)]
            impl
        };
        append_generics(generics, struct_name, &mut tokens);

        tokens.append(Group::new(
            Delimiter::Brace,
            make_container_body(opts, fields),
        ));

        tokens
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
                ModulePrefix::new(&["core", "default", "Default", "default"])
                    .to_tokens(&mut tokens);
                tokens.append(Group::new(Delimiter::Parenthesis, TokenStream::new()));
            } else if let Some(ref value) = field.opts.value {
                value.to_tokens(&mut tokens);
            } else {
                field.resolve_ident().to_tokens(&mut tokens);

                if field.opts.clone {
                    append_method_call("clone", &mut tokens);
                }

                if field.opts.into {
                    append_method_call("into", &mut tokens);
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

fn append_method_call(name: &str, tokens: &mut TokenStream) {
    tokens.append(Punct::new_joint('.'));
    tokens.append(Ident::create(name));
    tokens.append(Group::new(Delimiter::Parenthesis, TokenStream::new()));
}

#[inline]
fn make_args(fields: &FieldsSource, args: Punctuated<MiniField, impl ToTokens>) -> TokenStream {
    let mut tokens = TokenStream::new();
    let (_, fields) = match fields.fields().to_vec() {
        Some(v) => v,
        None => return tokens,
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
            tokens.append(Ident::create("impl"));
            ModulePrefix::new(&["core", "convert", "Into"]).to_tokens(&mut tokens);
            tokens.append(Punct::new_joint('<'));
            field.ty.to_tokens(&mut tokens);
            tokens.append(Punct::new_alone('>'));
        } else {
            field.ty.to_tokens(&mut tokens);
        }

        Some(tokens)
    });

    tokens.append_separated(iter_args.chain(iter_fields), <Token![,]>::default());

    tokens
}

#[inline]
fn append_generics(generics: Generics, struct_name: Ident, tokens: &mut TokenStream) {
    let (g1, g2, g3) = generics.split_for_impl();
    g1.to_tokens(tokens);
    tokens.append(struct_name);
    g2.to_tokens(tokens);
    g3.to_tokens(tokens);
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
