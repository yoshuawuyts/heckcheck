//! A heckin small test generator.
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
//! // tbi
//! ```

#![forbid(unsafe_code, future_incompatible, rust_2018_idioms)]
#![deny(missing_debug_implementations, nonstandard_style)]
#![warn(missing_docs, missing_doc_code_examples, unreachable_pub)]

pub use arbitrary::Arbitrary;

use arbitrary::Unstructured;
use rand::RngCore;

/// The main test checker.
#[derive(Debug, Default)]
pub struct HeckCheck {
    bytes: Vec<u8>,
}

impl HeckCheck {
    /// Create a new instance.
    pub fn new() -> Self {
        Self {
            bytes: vec![0u8; 1024],
        }
    }

    /// Check the target.
    pub fn check<'a, A, F>(&'a mut self, mut f: F)
    where
        A: Arbitrary<'a>,
        F: for<'r> FnMut(&'r mut A) -> arbitrary::Result<()>,
    {
        let mut rng = rand::thread_rng();
        rng.fill_bytes(&mut self.bytes);
        if self.bytes.len() < dbg!(A::size_hint(0).0) {
            todo!("not enough bytes"); // make this request more data
        }
        let mut u = Unstructured::new(&self.bytes);
        let mut instance = A::arbitrary(&mut u).unwrap(); // todo: handle "not enough data".
        f(&mut instance).unwrap();
    }
}
