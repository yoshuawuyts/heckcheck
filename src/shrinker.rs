use crate::{Shrink, ShrinkReport};

/// The default test case shrinker.
#[derive(Debug)]
pub struct Shrinker {
    offset_start: usize,
    source: Vec<u8>,
}

impl Shrink for Shrinker {
    fn shrink(source: Vec<u8>) -> Self {
        Self {
            source,
            offset_start: 0,
        }
    }

    fn next(&mut self) -> &[u8] {
        &self.source[..self.offset_start]
    }

    fn report(&mut self, report: ShrinkReport) -> Option<&[u8]> {
        match report {
            ShrinkReport::Pass => {
                self.offset_start += 1;
                None
            }
            ShrinkReport::Fail => Some(&self.source[..self.offset_start]),
        }
    }
}

use std::ops::Div;

/// Shrinker that keeps "reducing" bytes (get them closer to Zero) until they
/// stop triggering the bug.
#[derive(Debug)]
struct ZeroShrinker {
    source: Vec<u8>,
    index: usize,
    value: u8,
}

impl ZeroShrinker {
    /// Find the next index to alter, skipping indices that already contain zeroes.
    fn find_next_index_to_zeroify(&mut self) {
        while self.index < self.source.len() && self.source[self.index] == 0 {
            self.index += 1;
        }
    }
}

impl Shrink for ZeroShrinker {
    fn shrink(source: Vec<u8>) -> Self {
        let mut shrinker = Self {
            source,
            index: 0,
            value: 0,
        };
        shrinker.find_next_index_to_zeroify();
        shrinker
    }

    fn next(&mut self) -> &[u8] {
        if self.index < self.source.len() {
            assert_ne!(self.source[self.index], 0);
            self.value = self.source[self.index];
            self.source[self.index] = self.source[self.index].div(2);
            assert_ne!(self.source[self.index], self.value);
        }
        &self.source
    }

    fn report(&mut self, report: ShrinkReport) -> Option<&[u8]> {
        if report == ShrinkReport::Pass {
            // The test case doesn't trigger the bug anymore, revert to the
            // previous value and move on.
            self.source[self.index] = self.value;
            self.index += 1;
        }

        self.find_next_index_to_zeroify();

        if self.index >= self.source.len() {
            Some(&self.source)
        } else {
            None
        }
    }
}
