<!-- cargo-rdme start -->

Derive a highly configurable constructor for your struct

[![MASTER CI status](https://github.com/Alorel/fancy_constructor-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/Alorel/fancy_constructor-rs/actions/workflows/ci.yml?query=branch%3Amaster)
[![crates.io badge](https://img.shields.io/crates/v/fancy_constructor)](https://crates.io/crates/fancy_constructor)
[![docs.rs badge](https://img.shields.io/docsrs/fancy_constructor?label=docs.rs)](https://docs.rs/fancy_constructor)
[![dependencies badge](https://img.shields.io/librariesio/release/cargo/fancy_constructor)](https://libraries.io/cargo/fancy_constructor)

# Examples
## Basic

```rust
use fancy_constructor::new;
#[derive(new, PartialEq, Eq, Debug)]
struct MyStruct {
  foo: String,
  bar: u8,
}

let a = MyStruct::new("#[derive(new)]".into(), 55);
let b = MyStruct { foo: "#[derive(new)]".into(), bar: 55 };
assert_eq!(a, b);
```

Outputs:
```rust
impl MyStruct {
  pub fn new(foo: String, bar: u8) -> Self {
    Self { foo, bar }
  }
}
````

## Options showcase

```rust
#[derive(new, PartialEq, Eq, Debug)]
#[new(vis(pub(crate)), name(construct), comment("Foo"), bounds(T: Clone))]
struct MyStruct<T> {
  #[new(into)]
  a: T,

  #[new(val("Bar".into()))]
  b: String,

  #[new(clone)]
  c: Arc<Whatever>,

  #[new(default)]
  d: Vec<u8>,
}

let we = Arc::new(Whatever::default());
let a = MyStruct::<String>::construct("A", &we);
let b = MyStruct {a: "A".into(), b: "Bar".into(), c: we, d: vec![]};
assert_eq!(a, b);
```

Outputs:

```rust
impl<T> MyStruct<T> {
  /// Foo
  pub(crate) fn construct(a: impl Into<T>, c: &Arc<Whatever>) -> Self where T: Clone {
    Self {
      a: a.into(),
      b: "Bar".into(),
      c: c.clone(),
      d: Default::default(),
    }
  }
}
````

## Private const fn

```rust
#[derive(new, PartialEq, Eq, Debug)]
#[new(const_fn, vis())]
struct Foo(u8);

const FOO: Foo = Foo::new(128);
assert_eq!(FOO, Foo(128));
```

Outputs:

```rust
impl Foo {
  const fn new(f1: u8) -> Self {
    Self(f1)
  }
}
````

<!-- cargo-rdme end -->
