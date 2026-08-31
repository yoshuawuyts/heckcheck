#![allow(unsafe_code)]

use core::fmt::{self, Debug};
use core::mem::ManuallyDrop;
use core::ops::{Deref, DerefMut};

/// Wrap a value and run a closure when dropped.
pub(crate) struct DropGuard<T, F>
where
    F: FnOnce(T),
{
    inner: ManuallyDrop<T>,
    f: ManuallyDrop<F>,
}

impl<T, F> DropGuard<T, F>
where
    F: FnOnce(T),
{
    /// Create a new instance of `DropGuard`.
    pub(crate) fn new(inner: T, f: F) -> Self {
        Self {
            inner: ManuallyDrop::new(inner),
            f: ManuallyDrop::new(f),
        }
    }

    /// Consumes the `DropGuard`, returning the wrapped value.
    pub(crate) fn dismiss(guard: Self) -> T {
        // First we ensure that dropping the guard will not trigger
        // its destructor
        let mut guard = ManuallyDrop::new(guard);

        // Next we manually read the stored value from the guard.
        //
        // SAFETY: this is safe because we've taken ownership of the guard.
        let value = unsafe { ManuallyDrop::take(&mut guard.inner) };

        // Finally we drop the stored closure. We do this *after* having read
        // the value, so that even if the closure's `drop` function panics,
        // unwinding still tries to drop the value.
        //
        // SAFETY: this is safe because we've taken ownership of the guard.
        unsafe { ManuallyDrop::drop(&mut guard.f) };
        value
    }
}

impl<T, F> Deref for DropGuard<T, F>
where
    F: FnOnce(T),
{
    type Target = T;

    fn deref(&self) -> &T {
        &*self.inner
    }
}

impl<T, F> DerefMut for DropGuard<T, F>
where
    F: FnOnce(T),
{
    fn deref_mut(&mut self) -> &mut T {
        &mut *self.inner
    }
}

impl<T, F> Drop for DropGuard<T, F>
where
    F: FnOnce(T),
{
    fn drop(&mut self) {
        // SAFETY: `DropGuard` is in the process of being dropped.
        let inner = unsafe { ManuallyDrop::take(&mut self.inner) };

        // SAFETY: `DropGuard` is in the process of being dropped.
        let f = unsafe { ManuallyDrop::take(&mut self.f) };

        f(inner);
    }
}

impl<T, F> Debug for DropGuard<T, F>
where
    T: Debug,
    F: FnOnce(T),
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&**self, f)
    }
}
