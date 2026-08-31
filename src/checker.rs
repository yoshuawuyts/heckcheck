use arbitrary::{Arbitrary, Unstructured};
use base64::{engine::general_purpose::STANDARD, Engine as _};
use rand::prelude::*;
use rand::rngs::StdRng;
use std::any::Any;
use std::panic::{self, catch_unwind, AssertUnwindSafe};

use crate::utils::DropGuard;
use crate::{Shrink, ShrinkReport, Shrinker};

/// The base number of iterations performed to find an error using `heckcheck`.
const MAX_PASSES: u64 = 100;

/// A backstop on the shrink loop, so a third-party `Shrink` implementation
/// which never terminates can't hang the test run.
const MAX_SHRINK_PASSES: u64 = 100_000;

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
    pub fn check<A, F>(&mut self, f: F)
    where
        A: for<'b> Arbitrary<'b>,
        F: FnMut(A) -> arbitrary::Result<()>,
    {
        self.check_with_shrinker::<_, _, Shrinker>(f)
    }

    /// Check the target with the specified shrinker.
    pub fn check_with_shrinker<A, F, S>(&mut self, mut f: F)
    where
        A: for<'b> Arbitrary<'b>,
        F: FnMut(A) -> arbitrary::Result<()>,
        S: Shrink,
    {
        // Make sure we have enough bytes in our buffer before we start testing.
        if self.bytes.len() < A::size_hint(0).0 {
            self.grow_vec(Some(A::size_hint(0).0));
        }

        // Silence panics just for the duration of the run. We can call
        // `DropGuard.dismiss` to restore the original panic handler.
        let guard = DropGuard::new(panic::take_hook(), panic::set_hook);
        panic::set_hook(Box::new(|_| {}));

        for _ in 0..self.max_count {
            self.rng.fill_bytes(&mut self.bytes);
            let mut u = Unstructured::new(&self.bytes);
            let instance = A::arbitrary(&mut u).unwrap();

            // Track whether we should allocate more data for a future loop.
            let mut more_data = false;

            // Call the closure. Handle the return type from `Arbitrary`, and
            // handle possible panics from the closure.
            let res = catch_unwind(AssertUnwindSafe(|| {
                if let Err(arbitrary::Error::NotEnoughData) = f(instance) {
                    more_data = true;
                }
            }));

            // The bytes `A::arbitrary` left unread; used below to find where
            // the data which produced `instance` ends.
            let u_len = u.len();

            // The closure asked for more data than we had; grow for next pass.
            if more_data {
                self.grow_vec(None);
            }

            // Keep going if we didn't panic
            if res.is_ok() {
                continue;
            }

            // Start reducing the test case.
            let upper = self.bytes.len() - u_len;
            let mut shrinker = S::shrink(self.bytes[0..upper].to_owned());
            let mut case = None;
            for _ in 0..MAX_SHRINK_PASSES {
                // Create a new input and call the closure again.
                let report = match A::arbitrary(&mut Unstructured::new(shrinker.next())) {
                    Ok(instance) => invoke_closure(&mut f, instance).into(),
                    // Too little data left to construct an instance at all.
                    Err(_) => ShrinkReport::Pass,
                };

                // Report the outcome to the shrinker, and stop once it's
                // done shrinking.
                if let Some(shrunk) = shrinker.report(report) {
                    case = Some(shrunk.to_owned());
                    break;
                }
            }

            // A shrunk case is only useful if it still fails; if it doesn't we
            // fall back to the full buffer, which is known to reproduce.
            let reproduces = case.as_deref().is_some_and(|case| {
                let mut u = Unstructured::new(case);
                match A::arbitrary(&mut u) {
                    Ok(instance) => invoke_closure(f, instance).is_err(),
                    Err(_) => false,
                }
            });
            let case = match reproduces {
                true => case.unwrap(),
                false => self.bytes.clone(),
            };
            let sequence = STANDARD.encode(case);

            // Restore the previously overwritten panic hook
            // so we can unwind with an error from here.
            panic::set_hook(DropGuard::dismiss(guard));
            match sequence.len() {
                0 => panic!("The failing base64 sequence is: ``. Pass an empty string to `heckcheck::replay` to create a permanent reproduction."),
                _ => panic!("The failing base64 sequence is: `{}`. Pass this to `heckcheck::replay` to create a permanent reproduction.", sequence),
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

    /// Access the value of `seed`.
    pub fn seed(&self) -> u64 {
        self.seed
    }
}

/// Call the closure, but catching any panics and converting them to errors
fn invoke_closure<A, F>(mut f: F, instance: A) -> Result<(), Box<dyn Any + Send + 'static>>
where
    A: for<'b> Arbitrary<'b>,
    F: FnMut(A) -> arbitrary::Result<()>,
{
    catch_unwind(AssertUnwindSafe(|| f(instance).unwrap()))
}
