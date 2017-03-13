# option-constructor-derive

[![Travis Build Status](https://travis-ci.org/iovxw/option-constructor-derive.svg)](https://travis-ci.org/iovxw/option-constructor-derive)
[![Crates](https://img.shields.io/crates/v/option-constructor-derive.svg)](https://crates.io/crates/option-constructor-derive)
[![Documentation](https://docs.rs/option-constructor-derive/badge.svg)](https://docs.rs/option-constructor-derive)

## Example

```rust
#[macro_use]
extern crate option_constructor_derive;

#[derive(OptionConstructor, Debug, PartialEq)]
struct Example {
    field1: bool,
    field2: Option<bool>,
    field3: Option<bool>,
}

fn main() {
    let x = Example::new(true).field2(false);
    assert_eq!(x, Example {
        field1: true,
        field2: Some(false),
        field3: None,
    });
}
```

## License

This is free and unencumbered software released into the public domain.

Anyone is free to copy, modify, publish, use, compile, sell, or distribute this software, either in source code form or as a compiled binary, for any purpose, commercial or non-commercial, and by any means.
