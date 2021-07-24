//! A heckin small test generator.
//!
//! See the documentation of [`HeckCheck`] to get started.
//!
//! # What is test generation?
//!
//! A test generator is a program which writes programs to test other programs.
//! The idea is that we can find more bugs if we test more paths in a program,
//! and the best way to do that is to provide a wide range of inputs to our
//! programs. And the best way to do _that_ is to write a program designed to
//! generate inputs.
//!
//! You may have heard this being referred to as "fuzz testing" or "property
//! testing" as well.
//!
//! # What makes heckcheck special?
//!
//! This is written mainly for two reasons:
//!
//! 1. To learn how to write a test generator (and maybe even teach others how
//! to do it too).
//! 2. To explore the feasilility of using the `arbitrary` crate outside of
//! `cargo-fuzz` contexts.
//!
//! # Examples
//!
//! ```
//!  use heckcheck::Arbitrary;
//!
//! /// A color value encoded as Red-Green-Blue
//! #[derive(Clone, Debug, Arbitrary, PartialEq)]
//! pub struct Rgb {
//!     pub r: u8,
//!     pub g: u8,
//!     pub b: u8,
//! }
//!
//! impl Rgb {
//!     /// Convert from RGB to Hexadecimal.
//!     pub fn to_hex(&self) -> String {
//!         format!("#{:02X}{:02X}{:02X}", self.r, self.g, self.b)
//!     }
//!
//!     /// Convert from Hexadecimal to RGB.
//!     pub fn from_hex(s: String) -> Self {
//!         let s = s.strip_prefix('#').unwrap();
//!         Rgb {
//!             r: u8::from_str_radix(&s[0..2], 16).unwrap(),
//!             g: u8::from_str_radix(&s[2..4], 16).unwrap(),
//!             b: u8::from_str_radix(&s[4..6], 16).unwrap(),
//!         }
//!     }
//! }
//!
//! // Validate values can be converted from RGB to Hex and back.
//! heckcheck::check(|rgb: Rgb| {
//!     let hex = rgb.to_hex();
//!     let res = Rgb::from_hex(hex);
//!     assert_eq!(rgb, res);
//!     Ok(())
//! });
//! ```

#![forbid(unsafe_code, future_incompatible, rust_2018_idioms)]
#![deny(missing_debug_implementations, nonstandard_style)]
#![warn(missing_docs, unreachable_pub)]

mod checker;
mod shrink;

pub use arbitrary::{Arbitrary, Unstructured};
pub use checker::HeckCheck;
pub use shrink::{Shrink, ShrinkReport, Shrinker};

/// Check a target.
pub fn check<A, F>(f: F)
where
    A: for<'b> Arbitrary<'b>,
    F: FnMut(A) -> arbitrary::Result<()>,
{
    let mut checker = HeckCheck::new();
    checker.check(f);
}

/// Replay a failing test from a seed.
///
/// Pass a known seed and failing test case to this to repeat a failure.
pub fn replay<A, F>(bytes: &str, mut f: F)
where
    A: for<'b> Arbitrary<'b>,
    F: FnMut(A) -> arbitrary::Result<()>,
{
    let bytes = base64::decode(bytes).unwrap();
    let mut u = Unstructured::new(&bytes);
    let instance = A::arbitrary(&mut u).unwrap();
    f(instance).unwrap();
}
