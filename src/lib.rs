//! Derive a highly configurable constructor for your struct
//!
//! [![MASTER CI status](https://github.com/Alorel/fancy_constructor-rs/actions/workflows/test.yml/badge.svg)](https://github.com/Alorel/fancy_constructor-rs/actions/workflows/test.yml?query=branch%3Amaster)
//! [![crates.io badge](https://img.shields.io/crates/v/fancy_constructor)](https://crates.io/crates/fancy_constructor)
//! [![dependencies badge](https://img.shields.io/librariesio/release/cargo/fancy_constructor)](https://libraries.io/cargo/fancy_constructor)
//! [![Coverage Status](https://coveralls.io/repos/github/Alorel/fancy_constructor-rs/badge.png)](https://coveralls.io/github/Alorel/fancy_constructor-rs)
//!
//! # Examples
//!
//! <details><summary>Basic</summary>
//!
//! ```
//! use fancy_constructor::new;
//! #[derive(new, PartialEq, Eq, Debug)]
//! struct MyStruct {
//!   foo: String,
//!   bar: u8,
//! }
//!
//! let a = MyStruct::new("#[derive(new)]".into(), 55);
//! let b = MyStruct { foo: "#[derive(new)]".into(), bar: 55 };
//! assert_eq!(a, b);
//! ```
//!
//! Outputs:
#![cfg_attr(doctest, doc = " ````no_test")]
//! ```
//! impl MyStruct {
//!   pub fn new(foo: String, bar: u8) -> Self {
//!     Self { foo, bar }
//!   }
//! }
//! ````
//!
//! </details>
//! <details><summary>Options showcase</summary>
//!
//! ```
//! # use fancy_constructor::new;
//! # use std::sync::Arc;
//! # #[derive(Eq, PartialEq, Copy, Clone, Default, Debug)]
//! # struct Whatever;
//! #[derive(new, PartialEq, Eq, Debug)]
//! #[new(vis(pub(crate)), name(construct), comment("Foo"), bounds(T: Clone))]
//! struct MyStruct<T> {
//!   #[new(into)]
//!   a: T,
//!
//!   #[new(val("Bar".into()))]
//!   b: String,
//!
//!   #[new(clone)]
//!   c: Arc<Whatever>,
//!
//!   #[new(default)]
//!   d: Vec<u8>,
//! }
//!
//! let we = Arc::new(Whatever::default());
//! let a = MyStruct::<String>::construct("A", &we);
//! let b = MyStruct {a: "A".into(), b: "Bar".into(), c: we, d: vec![]};
//! assert_eq!(a, b);
//! ```
//!
//! Outputs:
//!
#![cfg_attr(doctest, doc = " ````no_test")]
//! ```
//! impl<T> MyStruct<T> {
//!   /// Foo
//!   pub(crate) fn construct(a: impl Into<T>, c: &Arc<Whatever>) -> Self where T: Clone {
//!     Self {
//!       a: a.into(),
//!       b: "Bar".into(),
//!       c: c.clone(),
//!       d: Default::default(),
//!     }
//!   }
//! }
//! ````
//!
//! </details>
//! <details><summary>Private const fn</summary>
//!
//! ```
//! # use fancy_constructor::new;
//! #[derive(new, PartialEq, Eq, Debug)]
//! #[new(const_fn, vis())]
//! struct Foo(u8);
//!
//! const FOO: Foo = Foo::new(128);
//! assert_eq!(FOO, Foo(128));
//! ```
//!
//! Outputs:
//!
#![cfg_attr(doctest, doc = " ````no_test")]
//! ```
//! impl Foo {
//!   const fn new(f1: u8) -> Self {
//!     Self(f1)
//!   }
//! }
//! ````
//!
//! </details>
//! <details><summary>Computed values</summary>
//!
//! ```
//! # use fancy_constructor::new;
//! #[derive(new)]
//! struct Foo {
//!   is_bar: bool,
//!   #[new(val(if is_bar { 100 } else { 5 }))]
//!   barness_level: u8,
//! }
//!
//! assert_eq!(Foo::new(true).barness_level, 100);
//! assert_eq!(Foo::new(false).barness_level, 5);
//! ```
//!
//! </details>
//!
//! <details><summary>Custom constructor args</summary>
//!
//! ```
//! # use fancy_constructor::new;
//! #[derive(new)]
//! #[new(args(input_string: &str))]
//! struct Foo {
//!   #[new(val(input_string.to_lowercase()))]
//!   pub lowercase: String,
//!
//!   #[new(val(input_string.to_uppercase()))]
//!   pub uppercase: String,
//! }
//!
//! let foo = Foo::new("Foo");
//! assert_eq!(foo.lowercase.as_str(), "foo");
//! assert_eq!(foo.uppercase.as_str(), "FOO");
//! ```
//!
//! </details>
//! <details><summary>Renaming constructor args</summary>
//!
//! ```
//! # use fancy_constructor::new;
//! #[derive(new)]
//! struct MyNewtype(#[new(name(my_value))] u8);
//! ```
//!
//! Outputs:
//!
#![cfg_attr(doctest, doc = " ````no_test")]
//! ```
//! impl MyNewtype {
//!   pub fn new(my_value: u8) -> Self {
//!     Self(my_value)
//!   }
//! }
//! ````
//!
//! </details>
//! <details><summary>Enums</summary>
//!
//! ```
//! # use fancy_constructor::new;
//! #[derive(new, Eq, PartialEq, Debug)]
//! enum MyEnum {
//!   #[new]
//!   Foo { #[new(into)] bar: u8 },
//!   Qux,
//! }
//!
//! assert_eq!(MyEnum::new(5), MyEnum::Foo { bar: 5 });
//! ```
//!
//! Outputs:
//!
#![cfg_attr(doctest, doc = " ````no_test")]
//! ```
//! impl MyEnum {
//!   pub fn new(bar: Into<u8>) -> Self {
//!     Self::Foo { bar: bar.into() }
//!   }
//! }
//! ````
//!
//! </details>
//!
//! <details><summary>Invalid inputs</summary>
//!
//! ```compile_fail
//! #[derive(fancy_constructor::new)]
//! enum Foo {
//!   Bar, // no variants marked with `#[new]`
//! }
//! ```
//!
//! ```compile_fail
//! #[derive(fancy_constructor::new)]
//! enum Foo {
//!   #[new] Bar, // multiple variants marked with `#[new]`
//!   #[new] Qux,
//! }
//! ```
//!
//! ```compile_fail
//! #[derive(fancy_constructor::new)]
//! union Foo { // Unions not supported
//!   bar: u8,
//!   qux: u8,
//! }
//! ```
//!
//! </details>

#![deny(clippy::correctness, clippy::suspicious)]
#![warn(clippy::complexity, clippy::perf, clippy::style, clippy::pedantic)]
#![allow(clippy::needless_pass_by_value)]
#![warn(missing_docs)]

mod options;
mod parse;
mod tokenise;
mod types;

extern crate proc_macro;

const ATTR_NAME: &str = "new";

/// See [crate-level docs](crate) for a usage and output example.
///
/// # Container options
///
/// | Opt | Default | Description |
/// | --- | --- | --- |
/// | `new(const_fn)` | `false` | Whether to make the constructor a `const fn` |
/// | `new(vis(visibility))` | `pub` | The visibility of the constructor |
/// | `new(name(ident))` | `new` | Constructor fn name |
/// | `new(comment(literal))` | | A doc comment to add to the constructor |
/// | `new(bounds(T: Whatever))` | | Generic type bounds for the implementation |
/// | `new(args(foo: u8, bar: String))` | | Additional arguments to add to the **beginning** of the args list |
///
/// # Field options
///
/// | Opt | Description |
/// | --- | --- |
/// | `new(default)` | Omit the field from the constructor and use [`Default::default`] |
/// | `new(clone)` | Make the argument pass-by-reference and clone it |
/// | `new(into)` | Make the argument [`Into<T>`](Into) |
/// | `new(name(ident))` | Rename the function argument - useful for newtype structs |
/// | `new(val(expr))` | Initialise the value with the following expression instead of a constructor argument |
///
#[proc_macro_derive(new, attributes(new))]
pub fn derive_fancy_constructor(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    syn::parse_macro_input!(input as FancyConstructor)
        .into_token_stream()
        .into()
}

enum Fields {
    Unit,
    Named(Vec<Field>),
    Unnamed(Vec<Field>),
}

enum FieldsSource {
    Struct(Fields),
    Enum {
        variant: proc_macro2::Ident,
        fields: Fields,
    },
}

struct FancyConstructor {
    struct_name: proc_macro2::Ident,
    generics: syn::Generics,
    fields: FieldsSource,
    opts: options::ContainerOptions,
}

struct Field {
    name: proc_macro2::Ident,
    opts: options::FieldOptions,
    ty: syn::Type,
}
