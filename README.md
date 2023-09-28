<!-- cargo-rdme start -->

Derive a highly configurable constructor for your struct

[![MASTER CI status](https://github.com/Alorel/fancy_constructor-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/Alorel/fancy_constructor-rs/actions/workflows/ci.yml?query=branch%3Amaster)
[![crates.io badge](https://img.shields.io/crates/v/fancy_constructor)](https://crates.io/crates/fancy_constructor)
[![docs.rs badge](https://img.shields.io/docsrs/fancy_constructor?label=docs.rs)](https://docs.rs/fancy_constructor)
[![dependencies badge](https://img.shields.io/librariesio/release/cargo/fancy_constructor)](https://libraries.io/cargo/fancy_constructor)

# Examples

<details><summary>Basic</summary>

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

</details>
<details><summary>Options showcase</summary>

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

</details>
<details><summary>Private const fn</summary>

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

</details>
<details><summary>Computed values</summary>

```rust
#[derive(new)]
struct Foo {
  is_bar: bool,
  #[new(val(if is_bar { 100 } else { 5 }))]
  barness_level: u8,
}

assert_eq!(Foo::new(true).barness_level, 100);
assert_eq!(Foo::new(false).barness_level, 5);
```

</details>
<details><summary>Enums</summary>

```rust
#[derive(new, Eq, PartialEq, Debug)]
enum MyEnum {
  #[new]
  Foo { #[new(into)] bar: u8 },
  Qux,
}

assert_eq!(MyEnum::new(5), MyEnum::Foo { bar: 5 });
```

Outputs:

```rust
impl MyEnum {
  pub fn new(bar: Into<u8>) -> Self {
    Self::Foo { bar: bar.into() }
  }
}
````

</details>

<!-- cargo-rdme end -->
