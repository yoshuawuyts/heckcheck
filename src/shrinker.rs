use crate::{Shrink, ShrinkReport};

/// The default test case shrinker.
#[derive(Debug)]
pub struct Shrinker<'a> {
    offset_start: usize,
    source: &'a mut [u8],
}

impl<'a> Shrink<'a> for Shrinker<'a> {
    fn shrink(source: &'a mut [u8]) -> Self {
        Self {
            source,
            offset_start: 0,
        }
    }

    fn next(&mut self) -> &[u8] {
        if self.offset_start == (self.source.len() - 1) {
            return self.next();
        }
        &self.source[..self.offset_start]
    }

    fn report(&mut self, report: ShrinkReport) -> Option<&[u8]> {
        let res = match report {
            ShrinkReport::Pass => None,
            ShrinkReport::Fail => Some(&self.source[..self.offset_start]),
        };
        self.offset_start += 1;
        res
    }
}
