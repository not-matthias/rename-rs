# rename_derive

[![crates.io](https://img.shields.io/crates/v/rename_derive.svg)](https://crates.io/crates/rename_derive)
[![docs.rs](https://docs.rs/rename_derive/badge.svg)](https://docs.rs/rename_derive)
![Lines of Code](https://tokei.rs/b1/github/not-matthias/rename_derive)
[![GPLv3 license](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://github.com/not-matthias/rename_derive/blob/master/LICENSE)

Easy modification of structure names.

## Example

```rust
#[rename(prepend = "One", name = "Two", append = "Three")]
struct Placeholder {
    pub one: u64,
    pub two: u64,
    pub three: u64
}
```
The name of the struct is now `OneTwoThree`. 

# Why?

When working with [declarative macros](https://doc.rust-lang.org/book/ch19-06-macros.html#declarative-macros-with-macro_rules-for-general-metaprogramming) you can't create new [idents](https://doc.rust-lang.org/reference/identifiers.html). You could also remove this problem by using a [proc-macro](https://doc.rust-lang.org/reference/procedural-macros.html), but this would increase the complexity. However, there's the macro [concat_idents](https://doc.rust-lang.org/std/macro.concat_idents.html) which allows you to, as you can probably guess, concat identifiers. The big problem with this macro is, that it's not yet stable and doesn't allow the creation of new identifiers. Thus I decided to create my own little helper crate that solves this problem. 

The implementation for the aforementioned problem can be found [here](./examples/macro.rs).
