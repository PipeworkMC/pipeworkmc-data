//! [`Debug`] and [`Display`] redaction.


use core::{
    fmt::{ self, Debug, Display, Formatter },
    mem::{ self, ManuallyDrop }
};
use zeroize::zeroize_flat_type as erase;


/// A type which redacts [`Debug`] and [`Display`] implementations,
///  and ensures that this type's memory is zeroed on drop.
#[repr(transparent)]
pub struct Redacted<T> {
    inner : ManuallyDrop<T>
}

impl<T> From<T> for Redacted<T> {
    #[inline(always)]
    fn from(inner : T) -> Self {
        Self { inner : ManuallyDrop::new(inner) }
    }
}

impl<T> Redacted<T> {

    /// Returns a reference to the inner value.
    ///
    /// ### Safety
    /// Though this function is not inherently unsafe, it can leak the `Debug` and `Display` implementations.
    #[inline(always)]
    pub unsafe fn as_ref(&self) -> &T { &self.inner }

    /// Returns a mutable reference to the inner value.
    ///
    /// ### Safety
    /// Though this function is not inherently unsafe, it can leak the `Debug` and `Display` implementations.
    #[inline(always)]
    pub unsafe fn as_mut(&mut self) -> &mut T { &mut self.inner }

    /// Returns the inner value.
    ///
    /// ### Safety
    /// Though this function is not inherently unsafe, it can leak the `Debug` and `Display` implementations.
    #[inline(always)]
    pub unsafe fn into_inner(mut self) -> T {
        let inner = unsafe { ManuallyDrop::take(&mut self.inner) };
        mem::forget(self);
        inner
    }

}


impl<T> Debug for Redacted<T>
where
    T : Debug
{
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "[REDACTED]")
    }
}

impl<T> Display for Redacted<T>
where
    T : Display
{
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "[REDACTED]")
    }
}


impl<T> Drop for Redacted<T> {
    fn drop(&mut self) { unsafe {
        // SAFETY: `self.inner` was not previously dropped.
        ManuallyDrop::drop(&mut self.inner);
        // SAFETY: `self.inner` was dropped in the line above.
        erase((&mut self.inner) as *mut _);
    } }
}
