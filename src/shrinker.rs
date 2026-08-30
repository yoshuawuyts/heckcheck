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
