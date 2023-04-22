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

This crate can be used to change types based on platform-specific constants. In
the following example, the `RlimOption` type can either be `Option<rlim_t>` or
`rlim_t` itself, where `rlim_t::MAX` can be considered to be a sentinel value
for `Option::None`. This enables some platforms to use a smaller-sized type.

```rust
use condtype::{condval, CondType};
use libc::{rlim_t, RLIM_INFINITY};

const RLIM_INFINITY_IS_MAX: bool = RLIM_INFINITY == rlim_t::MAX;

type RlimOption = CondType<RLIM_INFINITY_IS_MAX, Option<rlim_t>, rlim_t>;

const RLIM_NONE: RlimOption = condval!(if RLIM_INFINITY_IS_MAX {
    None::<rlim_t>
} else {
    rlim_t::MAX
});

// Convert from either `RlimOption` type to `Option` via the `Into` trait:
let rlim_none: Option<rlim_t> = RLIM_NONE.into();
```

## Limitations

It is currently not possible to use [`CondType`] with a generic constant because
[Rust does not yet consider trait implementations based on booleans to be exhaustive](https://github.com/rust-lang/project-const-generics/issues/26).
Once that issue is resolved, all versions of this crate should _just work_ with
generic constants.

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

[`CondType`]: https://docs.rs/condtype/latest/condtype/type.CondType.html
