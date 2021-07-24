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

const MAX_COUNT: u64 = 100;

pub use arbitrary::{Arbitrary, Unstructured};
use rand::prelude::*;

/// The main test checker.
#[derive(Debug, Default)]
pub struct HeckCheck {
    bytes: Vec<u8>,
    max_count: u64,
}

impl HeckCheck {
    /// Create a new instance.
    pub fn new() -> Self {
        Self {
            bytes: vec![0u8; 1024],
            max_count: MAX_COUNT,
        }
    }

    /// Check the target.
    pub fn check<'a, A, F>(&'a mut self, mut f: F)
    where
        A: for<'b> Arbitrary<'b>,
        F: FnMut(A) -> arbitrary::Result<()>,
    {
        let mut rng = rand::thread_rng();
        if self.bytes.len() < A::size_hint(0).0 {
            self.grow_vec(Some(A::size_hint(0).0));
        }

        let mut successes = 0usize;

        for n in 0..self.max_count {
            dbg!(n);
            rng.fill_bytes(&mut self.bytes);
            let mut u = Unstructured::new(&self.bytes);
            let instance = A::arbitrary(&mut u).unwrap();
            match f(instance) {
                Ok(_) => successes += 1,
                Err(err) => match err {
                    arbitrary::Error::NotEnoughData => self.grow_vec(None),
                    err => panic!("{}", err),
                },
            }
        }
        println!("ran {} times successfully", successes);
    }

    fn grow_vec(&mut self, target: Option<usize>) {
        match target {
            Some(target) => {
                if target.checked_sub(self.bytes.len()).is_some() {
                    self.bytes.resize_with(target, || 0);
                }
            }
            None => self.bytes.resize_with(self.bytes.len() * 2, || 0),
        };
    }
}
