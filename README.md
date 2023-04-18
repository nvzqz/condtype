# `condtype`

Choose Rust types via boolean conditions, brought to you by
[Nikolai Vazquez](https://hachyderm.io/@nikolai).

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

## Install

This crate is [available on crates.io](https://crates.io/crates/condtype) and can be
used by running the following `cargo` command in your project directory:

```sh
cargo add condtype
```

or by manually adding the following to your project's [`Cargo.toml`](https://doc.rust-lang.org/cargo/reference/manifest.html):

```toml
[dependencies]
condtype = "0.0.0"
```

## License

Like the Rust project, this library may be used under either the
[MIT License](https://github.com/nvzqz/condtype/blob/main/LICENSE-MIT) or
[Apache License (Version 2.0)](https://github.com/nvzqz/condtype/blob/main/LICENSE-APACHE).
