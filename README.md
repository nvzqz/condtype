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

## License

Like the Rust project, this library may be used under either the
[MIT License](https://github.com/nvzqz/condtype/blob/main/LICENSE-MIT) or
[Apache License (Version 2.0)](https://github.com/nvzqz/condtype/blob/main/LICENSE-APACHE).
