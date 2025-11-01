//! `u8`-encoded angles.


use crate::Minecraft;
use core::f64::consts::TAU;
use netzer::prelude::*;


/// An angle encoded as a `u8`.
#[derive(Clone, Copy, PartialEq, PartialOrd, Debug, NetEncode, NetDecode)]
pub struct Angle(
    #[netzer(encode_with = "encode_angle", decode_with = "decode_angle")]
    f64
);
async fn encode_angle<W : netzer::AsyncWrite>(v : &f64, w : W) -> netzer::Result {
    <u8 as NetEncode<Minecraft>>::encode(&((*v * 256.0) as u8), w).await
}
async fn decode_angle<R : netzer::AsyncRead>(r : R) -> netzer::Result<f64> {
    Ok((<u8 as NetDecode<Minecraft>>::decode(r).await? as f64) / 256.0)
}

impl Angle {

    /// Create a new [`Angle`] from a fraction value (0.0-1.0).
    #[inline]
    pub fn frac(frac : f64) -> Self { Self(frac.rem_euclid(1.0)) }

    /// Create a new [`Angle`] from a radian value.
    #[inline]
    pub fn radians(radians : f64) -> Self { Self::frac(radians / TAU) }

    /// Create a new [`Angle`] from a degree value.
    #[inline]
    pub fn degrees(degrees : f64) -> Self { Self::frac(degrees / 360.0) }

}

impl Angle {

    /// Returns the inner value as a fraction (0.0-1.0).
    #[inline]
    pub fn to_frac(&self) -> f64 { self.0 }

    /// Returns the inner value in radians.
    #[inline]
    pub fn to_radians(&self) -> f64 { self.0 * TAU }

    /// Returns the inner value in degrees.
    #[inline]
    pub fn to_degrees(&self) -> f64 { self.0 / 360.0 }

}
