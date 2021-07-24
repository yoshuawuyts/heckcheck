/// Test case shrinking.
pub trait Shrink<'a> {
    /// Start shrinking the provided data.
    fn shrink(source: &'a mut [u8]) -> Self;

    /// Get the next test case.
    fn next(&mut self) -> &[u8];

    /// Report to the shrinker whether the last shrunk case passed or failed.
    ///
    /// Returns the final shrinkage case if no more cases are left to report.
    #[must_use = "The shrunk test case must be handled"]
    fn report(&mut self, report: ShrinkReport) -> Option<&[u8]>;
}

/// The result of a shrinking pass.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ShrinkReport {
    /// The last test passed.
    Pass,
    /// The last test failed.
    Fail,
}

impl ShrinkReport {
    /// Returns `true` if the shrink_result is [`Pass`].
    ///
    /// [`Pass`]: ShrinkReport::Pass
    pub fn is_pass(&self) -> bool {
        matches!(self, Self::Pass)
    }

    /// Returns `true` if the shrink_result is [`Fail`].
    ///
    /// [`Fail`]: ShrinkReport::Fail
    pub fn is_fail(&self) -> bool {
        matches!(self, Self::Fail)
    }
}

impl<T> From<std::thread::Result<T>> for ShrinkReport {
    fn from(res: std::thread::Result<T>) -> Self {
        match res {
            Ok(_) => ShrinkReport::Pass,
            Err(_) => ShrinkReport::Fail,
        }
    }
}

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
