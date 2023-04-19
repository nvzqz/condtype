# `condtype`

[![docs.rs](https://img.shields.io/crates/v/condtype.svg?style=flat-square&label=docs&color=blue&logo=rust)](https://docs.rs/condtype) [![crates.io](https://img.shields.io/crates/d/condtype.svg?style=flat-square)](https://crates.io/crates/condtype) [![github](https://img.shields.io/github/stars/nvzqz/condtype.svg?style=flat-square&color=black)][github]

Choose Rust types via boolean conditions, brought to you by
[Nikolai Vazquez](https://hachyderm.io/@nikolai).

If you find this crate useful, consider
[starring it][github] as well as
[sponsoring](https://github.com/sponsors/nvzqz) or
[donating once](https://paypal.me/nvzqz). 💖

[github]: https://github.com/nvzqz/condtype

## Examples

This crate enables choosing a type based on a compile-time constant, just like
[`std::conditional_t` in C++](https://en.cppreference.com/w/cpp/types/conditional).

```rust
use condtype::CondType;

let str: CondType<true,  &str, i32> = "hello";
let int: CondType<false, &str, i32> = 42;

// Unsized types are also supported:
let str: &CondType<true, str, [u8]> = "world";
```

## Limitations

It is currently not possible to use `CondType` with a generic constant, since
Rust does not yet consider trait implementations based on booleans to be
exhaustive (see [issue](https://github.com/rust-lang/project-const-generics/issues/26)).

```rust,ignore
fn generic<const B: bool>() {
    let x: CondType<B, i32, u8> = ...
}
```

## Install

This crate is [available on crates.io](https://crates.io/crates/condtype) and can be
used by running the following `cargo` command in your project directory:

```sh
cargo add condtype
```

or by manually adding the following to your project's [`Cargo.toml`](https://doc.rust-lang.org/cargo/reference/manifest.html):

```toml
[dependencies]
condtype = "1.0.0"
```

## License

Like the Rust project, this library may be used under either the
[MIT License](https://github.com/nvzqz/condtype/blob/main/LICENSE-MIT) or
[Apache License (Version 2.0)](https://github.com/nvzqz/condtype/blob/main/LICENSE-APACHE).
