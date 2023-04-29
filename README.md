<h1 align="center">heckcheck</h1>
<div align="center">
  <strong>
    A heckin small test case generator
  </strong>
</div>

<br />

<div align="center">
  <!-- Crates version -->
  <a href="https://crates.io/crates/heckcheck">
    <img src="https://img.shields.io/crates/v/heckcheck.svg?style=flat-square"
    alt="Crates.io version" />
  </a>
  <!-- Downloads -->
  <a href="https://crates.io/crates/heckcheck">
    <img src="https://img.shields.io/crates/d/heckcheck.svg?style=flat-square"
      alt="Download" />
  </a>
  <!-- docs.rs docs -->
  <a href="https://docs.rs/heckcheck">
    <img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square"
      alt="docs.rs docs" />
  </a>
</div>

<div align="center">
  <h3>
    <a href="https://docs.rs/heckcheck">
      API Docs
    </a>
    <span> | </span>
    <a href="https://github.com/yoshuawuyts/heckcheck/releases">
      Releases
    </a>
    <span> | </span>
    <a href="https://github.com/yoshuawuyts/heckcheck/blob/master.github/CONTRIBUTING.md">
      Contributing
    </a>
  </h3>
</div>

## Installation
```sh
$ cargo add heckcheck
```

## Examples

This is a basic roundtrip test for an RGB serializer and parser, which ensures
that the output matches the original input.

```rust
use heckcheck::prelude::*;

/// A color value encoded as Red-Green-Blue
#[derive(Clone, Debug, Arbitrary, PartialEq)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Rgb {
    /// Convert from RGB to Hexadecimal.
    pub fn to_hex(&self) -> String {
        format!("#{:02X}{:02X}{:02X}", self.r, self.g, self.b)
    }

    /// Convert from Hexadecimal to RGB.
    pub fn from_hex(s: String) -> Self {
        let s = s.strip_prefix('#').unwrap();
        Rgb {
            r: u8::from_str_radix(&s[0..2], 16).unwrap(),
            g: u8::from_str_radix(&s[2..4], 16).unwrap(),
            b: u8::from_str_radix(&s[4..6], 16).unwrap(),
        }
    }
}

// Validate values can be converted from RGB to Hex and back.
heckcheck::check(|rgb: Rgb| {
    let hex = rgb.to_hex();
    let res = Rgb::from_hex(hex);
    assert_eq!(rgb, res);
    Ok(())
});
```

## Safety
This crate uses ``#![deny(unsafe_code)]`` to ensure everything is implemented in
100% Safe Rust.

## Contributing
Want to join us? Check out our ["Contributing" guide][contributing] and take a
look at some of these issues:

- [Issues labeled "good first issue"][good-first-issue]
- [Issues labeled "help wanted"][help-wanted]

[contributing]: https://github.com/yoshuawuyts/heckcheck/blob/master.github/CONTRIBUTING.md
[good-first-issue]: https://github.com/yoshuawuyts/heckcheck/labels/good%20first%20issue
[help-wanted]: https://github.com/yoshuawuyts/heckcheck/labels/help%20wanted

## License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br/>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>
