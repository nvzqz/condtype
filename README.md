# `condtype`

[![docs.rs](https://img.shields.io/crates/v/condtype.svg?style=flat-square&label=docs&color=blue&logo=rust)](https://docs.rs/condtype) [![crates.io](https://img.shields.io/crates/d/condtype.svg?style=flat-square)](https://crates.io/crates/condtype) [![github](https://img.shields.io/github/stars/nvzqz/condtype.svg?style=flat-square&color=black)][github]

Choose Rust types at compile-time via boolean constants, brought to you by
[Nikolai Vazquez](https://hachyderm.io/@nikolai).

If you find this crate useful, consider
[starring it][github] as well as
[sponsoring](https://github.com/sponsors/nvzqz) or
[donating once](https://paypal.me/nvzqz). 💖

[github]: https://github.com/nvzqz/condtype

## Conditional Typing

The [`CondType`] type and [`condval!`] macro choose types at compile-time using
[`bool`] constants, just like [`std::conditional_t` in C++](https://en.cppreference.com/w/cpp/types/conditional).
Unlike [`Either`], the chosen type is directly used, rather than wrapping it
with an [`enum`] type. This may be considered a form of [dependent typing](https://en.wikipedia.org/wiki/Dependent_type),
but it is limited in ability and is restricted to compile-time constants rather
than runtime values.

## Examples

In the following example, [`CondType`] aliases [`&str`] or [`i32`]:

```rust
use condtype::CondType;

let str: CondType<true,  &str, i32> = "hello";
let int: CondType<false, &str, i32> = 42;

// Unsized types are also supported:
let str: &CondType<true, str, [u8]> = "world";
```

[`condval!`] enables choosing differently-typed values without specifying types.
In the following example, `val` is inferred to be either [`&str`] or [`i32`],
depending on `COND`.

```rust
use condtype::condval;

const COND: bool = true;

let val = condval!(if COND {
    "hello"
} else {
    42
});
```

This crate also enables making code for some platforms more efficient by using
smaller-sized types depending on platform-specific constants. In the following
example, the `RlimOption` type can either be `Option<rlim_t>` or `rlim_t`
itself, where `rlim_t::MAX` can be treated as a sentinel value for
`Option::None`.

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

Without this crate, one could otherwise use [`cfg_if!`] to achieve the same
goal. However, using [`#[cfg]`][cfg] requires maintaining a list of platforms
and being more fine-grained if `RLIM_INFINITY` is dependent on CPU architecture.

```rust
use cfg_if::cfg_if;
use libc::rlim_t;

cfg_if! {
    // Platforms where `RLIM_INFINITY != rlim_t::MAX`:
    if #[cfg(any(
        target_os = "macos",
        target_os = "freebsd",
        target_os = "solaris",
        // ad nauseam...
    ))] {
        type RlimOption = rlim_t;
        const RLIM_NONE: RlimOption = rlim_t::MAX;
    } else {
        type RlimOption = Option<rlim_t>;
        const RLIM_NONE: RlimOption = None;
    }
}
```

## Limitations

It is currently not possible to use [`CondType`] or [`condval!`] with a generic
constant because [Rust does not yet consider trait implementations based on
booleans to be exhaustive](https://github.com/rust-lang/project-const-generics/issues/26).
Once that issue is resolved, all versions of this crate should _just work_ with
generic constants.

```rust,ignore
fn generic<const B: bool>() {
    let val: CondType<B, &str, i32> = condval!(if B {
        "hello"
    } else {
        42
    });
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
condtype = "1.1.0"
```

## License

Like the Rust project, this library may be used under either the
[MIT License](https://github.com/nvzqz/condtype/blob/main/LICENSE-MIT) or
[Apache License (Version 2.0)](https://github.com/nvzqz/condtype/blob/main/LICENSE-APACHE).

[`CondType`]: https://docs.rs/condtype/latest/condtype/type.CondType.html
[`condval!`]: https://docs.rs/condtype/latest/condtype/macro.condval.html
[`Either`]:   https://docs.rs/either/latest/either/enum.Either.html
[`cfg_if!`]:  https://docs.rs/cfg-if/latest/cfg_if/macro.cfg_if.html

[`const`]: https://doc.rust-lang.org/std/keyword.const.html
[`enum`]:  https://doc.rust-lang.org/std/keyword.enum.html
[`bool`]:  https://doc.rust-lang.org/std/primitive.bool.html
[`i32`]:   https://doc.rust-lang.org/std/primitive.i32.html
[`&str`]:  https://doc.rust-lang.org/std/primitive.str.html
[cfg]:     https://doc.rust-lang.org/rust-by-example/attribute/cfg.html
