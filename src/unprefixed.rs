//! Types for en/decoding collections without length-prefixing.


use crate::Minecraft;
use core::{
    borrow::Borrow,
    mem::transmute,
    ops::{ Deref, DerefMut }
};
use netzer::prelude::*;


/// A `[T]` who's length when encoding is determined by context.
#[repr(transparent)]
pub struct AllSlice<T>(pub [T]);

impl<T> Deref for AllSlice<T> {
    type Target = [T];
    fn deref(&self) -> &Self::Target { &self.0 }
}
impl<T> DerefMut for AllSlice<T> {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}

impl<'l, T> From<&'l [T]> for &'l AllSlice<T> {
    fn from(value : &'l [T]) -> Self {
        // SAFETY: `AllSlice<T>` is `#[repr(transparent)]` over `[T]`.
        //         Thus, the memory layout of `&[T]` is identical to `&AllSlice<T>`.
        unsafe { transmute::<&'l [T], &'l AllSlice<T>>(value) }
    }
}
impl<'l, T> From<&'l mut [T]> for &'l mut AllSlice<T> {
    fn from(value : &'l mut [T]) -> Self {
        // SAFETY: `AllSlice<T>` is `#[repr(transparent)]` over `[T]`.
        //         Thus, the memory layout of `&mut [T]` is identical to `&mut AllSlice<T>`.
        unsafe { transmute::<&'l mut [T], &'l mut AllSlice<T>>(value) }
    }
}

impl<T : Clone> ToOwned for AllSlice<T> {
    type Owned = AllVec<T>;
    fn to_owned(&self) -> Self::Owned {
        AllVec(self.0.to_vec())
    }
}

impl<T : NetEncode<Minecraft>> NetEncode<Minecraft> for AllSlice<T> {
    async fn encode<W : netzer::AsyncWrite>(&self, mut w : W) -> netzer::Result {
        for x in &self.0 {
            <T as NetEncode<Minecraft>>::encode(x, &mut w).await?;
        }
        Ok(())
    }
}


/// A `Vec<T>` who's length when decoding is determined by context.
#[repr(transparent)]
pub struct AllVec<T>(pub Vec<T>);

impl<T> Deref for AllVec<T> {
    type Target = AllSlice<T>;
    fn deref(&self) -> &Self::Target { <&AllSlice<T>>::from(self.0.as_slice()) }
}
impl<T> DerefMut for AllVec<T> {
    fn deref_mut(&mut self) -> &mut Self::Target { <&mut AllSlice<T>>::from(self.0.as_mut_slice()) }
}

impl<T> Borrow<AllSlice<T>> for AllVec<T> {
    fn borrow(&self) -> &AllSlice<T> { &**self }
}

impl<T : NetDecode<Minecraft>> NetDecode<Minecraft> for AllVec<T> {
    async fn decode<R : netzer::AsyncRead>(_r : R) -> netzer::Result<Self> {
        Err("TODO: AllVec reader".into())
    }
}
