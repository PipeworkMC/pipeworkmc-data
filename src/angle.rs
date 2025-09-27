//! `u8`-encoded angles.


use core::f64::consts::TAU;
use pipeworkmc_codec::encode::{
    PacketEncode,
    EncodeBuf
};


/// An angle encoded as a `u8`.
#[derive(Clone, Copy, PartialEq, PartialOrd, Debug)]
pub struct Angle(f64);

impl Angle {

    /// Create a new [`Angle`] from a fraction value (0.0-1.0).
    #[inline(always)]
    pub fn frac(frac : f64) -> Self { Self(frac.rem_euclid(1.0)) }

    /// Create a new [`Angle`] from a radian value.
    #[inline(always)]
    pub fn radians(radians : f64) -> Self { Self::frac(radians / TAU) }

    /// Create a new [`Angle`] from a degree value.
    #[inline(always)]
    pub fn degrees(degrees : f64) -> Self { Self::frac(degrees / 360.0) }

}

impl Angle {

    /// Returns the inner value as a fraction (0.0-1.0).
    #[inline(always)]
    pub fn to_frac(&self) -> f64 { self.0 }

    /// Returns the inner value in radians.
    #[inline(always)]
    pub fn to_radians(&self) -> f64 { self.0 * TAU }

    /// Returns the inner value in degrees.
    #[inline(always)]
    pub fn to_degrees(&self) -> f64 { self.0 / 360.0 }

}

unsafe impl PacketEncode for Angle {

    #[inline(always)]
    fn encode_len(&self) -> usize { 1 }

    #[inline(always)]
    unsafe fn encode(&self, buf : &mut EncodeBuf) { unsafe {
        ((self.0 * 256.0) as u8).encode(buf);
    } }

}
