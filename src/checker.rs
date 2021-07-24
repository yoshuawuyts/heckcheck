use arbitrary::{Arbitrary, Unstructured};
use rand::prelude::*;
use rand::rngs::StdRng;
use std::panic::{self, AssertUnwindSafe};

use crate::{Shrink, Shrinker};

/// The base number of iterations performed to find an error using `heckcheck`.
const MAX_PASSES: u64 = 100;

/// The amount of data we initially allocate.
const INITIAL_VEC_LEN: usize = 1024;

/// The main test checker.
#[derive(Debug)]
pub struct HeckCheck {
    bytes: Vec<u8>,
    max_count: u64,
    seed: u64,
    rng: StdRng,
}

impl Default for HeckCheck {
    fn default() -> Self {
        Self::new()
    }
}

impl HeckCheck {
    /// Create a new instance.
    pub fn new() -> Self {
        let seed = rand::random();
        Self::from_seed(seed)
    }

    /// Create a new instance from a seed.
    pub fn from_seed(seed: u64) -> Self {
        let rng = StdRng::seed_from_u64(seed);
        Self {
            seed,
            rng,
            bytes: vec![0u8; INITIAL_VEC_LEN],
            max_count: MAX_PASSES,
        }
    }

    /// Check the target.
    pub fn check<'a, A, F>(&'a mut self, mut f: F)
    where
        A: for<'b> Arbitrary<'b>,
        F: FnMut(A) -> arbitrary::Result<()>,
    {
        // Make sure we have enough bytes in our buffer before we start testing.
        if self.bytes.len() < A::size_hint(0).0 {
            self.grow_vec(Some(A::size_hint(0).0));
        }

        let hook = panic::take_hook();
        panic::set_hook(Box::new(|_| {}));

        for _ in 0..self.max_count {
            self.rng.fill_bytes(&mut self.bytes);
            let mut u = Unstructured::new(&self.bytes);
            let instance = A::arbitrary(&mut u).unwrap();

            // Track whether we should allocate more data for a future loop.
            let mut more_data = false;

            // Call the closure. Handle the return type from `Arbitrary`, and
            // handle possible panics from the closure.
            let res = std::panic::catch_unwind(AssertUnwindSafe(|| {
                if let Err(arbitrary::Error::NotEnoughData) = f(instance) {
                    more_data = true;
                }
            }));

            let u_len = u.len();
            if more_data {
                self.grow_vec(None);
            }

            // If the test panicked we start reducing the test case.
            if res.is_err() {
                let upper = self.bytes.len() - u_len;
                let mut shrinker = Shrinker::shrink(&mut self.bytes[0..upper]);
                loop {
                    let mut u = Unstructured::new(shrinker.next());
                    let instance = A::arbitrary(&mut u).unwrap();

                    let res = std::panic::catch_unwind(AssertUnwindSafe(|| {
                        f(instance).unwrap();
                    }));
                    if let Some(case) = shrinker.report(res.into()) {
                        panic::set_hook(hook);
                        let sequence = base64::encode(case);
                        match sequence.len() {
                            0 => panic!("The failing base64 sequence is: ``. Pass an empty string to `heckcheck::replay` to create a permanent reproduction."),
                            _ => panic!("The failing base64 sequence is: `{}`. Pass this to `heckcheck::replay` to create a permanent reproduction.", sequence),
                        }
                    }
                }
            }
        }
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
