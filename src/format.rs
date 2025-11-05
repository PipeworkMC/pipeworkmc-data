use core::mem::MaybeUninit;
use std::borrow::Cow;
use netzer::prelude::*;
use netzer::varint::{ VarInt, Leb128 };
use uuid::Uuid;


/// The Minecraft game's network en/decoding format.
#[non_exhaustive]
pub struct Minecraft;
impl netzer::NetFormat for Minecraft { }

macro_rules! impl_netendecode_minecraft_for { ( $ty:ty , $format:ty $(,)? ) => {
    impl_netencode_minecraft_for!( $ty , $format , );
    impl_netdecode_minecraft_for!( $ty , $format , );
}; }
macro_rules! impl_netencode_minecraft_for { ( $ty:ty , $format:ty $(,)? ) => {
    impl NetEncode<Minecraft> for $ty {
        #[inline(always)]
        fn encode<W : netzer::AsyncWrite>(&self, w : W) -> impl Future<Output = netzer::Result> {
            <$ty as NetEncode::<$format>>::encode(self, w)
        }
    }
}; }
macro_rules! impl_netdecode_minecraft_for { ( $ty:ty , $format:ty $(,)? ) => {
    impl NetDecode<Minecraft> for $ty {
        #[inline(always)]
        fn decode<R : netzer::AsyncRead>(r : R) -> impl Future<Output = netzer::Result<Self>> {
            <$ty as NetDecode::<$format>>::decode(r)
        }
    }
}; }
impl_netendecode_minecraft_for!(bool, BigEndian);
impl_netendecode_minecraft_for!(u8, BigEndian);
impl_netendecode_minecraft_for!(i8, BigEndian);
impl_netendecode_minecraft_for!(u16, BigEndian);
impl_netendecode_minecraft_for!(i16, BigEndian);
impl_netendecode_minecraft_for!(u32, BigEndian);
impl_netendecode_minecraft_for!(i32, BigEndian);
impl_netendecode_minecraft_for!(u64, BigEndian);
impl_netendecode_minecraft_for!(i64, BigEndian);
impl_netendecode_minecraft_for!(u128, BigEndian);
impl_netendecode_minecraft_for!(i128, BigEndian);
impl_netendecode_minecraft_for!(f32, BigEndian);
impl_netendecode_minecraft_for!(f64, BigEndian);

impl NetEncode<Minecraft> for Uuid {
    async fn encode<W : netzer::AsyncWrite>(&self, w : W) -> netzer::Result {
        <u128 as NetEncode::<Minecraft>>::encode(&self.as_u128(), w).await
    }
}
impl NetDecode<Minecraft> for Uuid {
    async fn decode<R : netzer::AsyncRead>(r : R) -> netzer::Result<Self> {
        Ok(Uuid::from_u128(<u128 as NetDecode::<Minecraft>>::decode(r).await?))
    }
}

impl_netencode_minecraft_for!(str, Utf8<VarInt<u32>, Leb128>);
impl_netendecode_minecraft_for!(Cow<'_, str>, Utf8<VarInt<u32>, Leb128>);
impl_netendecode_minecraft_for!(String, Utf8<VarInt<u32>, Leb128>);

impl_netendecode_minecraft_for!(VarInt<u32>, Leb128);
impl_netendecode_minecraft_for!(VarInt<u64>, Leb128);

impl<T : NetEncode<Minecraft>> NetEncode<Minecraft> for Option<T> {
    async fn encode<W : netzer::AsyncWrite>(&self, mut w : W) -> netzer::Result { match (self) {
        Some(v) => {
            <bool as NetEncode<Minecraft>>::encode(&true, &mut w).await?;
            <T as NetEncode<Minecraft>>::encode(v, w).await
        },
        None => {
            <bool as NetEncode<Minecraft>>::encode(&false, w).await
        }
    }
}}
impl<T : NetDecode<Minecraft>> NetDecode<Minecraft> for Option<T> {
    async fn decode<R : netzer::AsyncRead>(mut r : R) -> netzer::Result<Self> {
        Ok(match (<bool as NetDecode<Minecraft>>::decode(&mut r).await?) {
            true  => Some(<T as NetDecode<Minecraft>>::decode(r).await?),
            false => None
        })
    }
}

impl<T : NetEncode<Minecraft>> NetEncode<Minecraft> for [T] {
    async fn encode<W : netzer::AsyncWrite>(&self, mut w : W) -> netzer::Result {
        <VarInt<u32> as NetEncode<Minecraft>>::encode(&VarInt::<u32>(self.len() as u32), &mut w).await?;
        for x in self {
            <T as NetEncode<Minecraft>>::encode(x, &mut w).await?;
        }
        Ok(())
    }
}
impl<T : NetEncode<Minecraft>> NetEncode<Minecraft> for Cow<'_, [T]>
where [T] : ToOwned
{ fn encode<W : netzer::AsyncWrite>(&self, w : W) -> impl Future<Output = netzer::Result> {
    <[T] as NetEncode<Minecraft>>::encode(self, w)
} }
impl<T : NetEncode<Minecraft>> NetEncode<Minecraft> for Vec<T>
{ fn encode<W : netzer::AsyncWrite>(&self, w : W) -> impl Future<Output = netzer::Result> {
    <[T] as NetEncode<Minecraft>>::encode(self, w)
} }
impl<T : NetEncode<Minecraft>, const N : usize> NetEncode<Minecraft> for [T; N]
{ fn encode<W : netzer::AsyncWrite>(&self, w : W) -> impl Future<Output = netzer::Result> {
    <[T] as NetEncode<Minecraft>>::encode(self, w)
} }

impl<T : NetDecode<Minecraft>> NetDecode<Minecraft> for Vec<T> {
    async fn decode<R : netzer::AsyncRead>(mut r : R) -> netzer::Result<Self> {
        let len = usize::try_from(<VarInt<u32> as NetDecode<Minecraft>>::decode(&mut r).await?.0)?;
        let mut v = Vec::with_capacity(len);
        for _ in 0..len {
            v.push(<T as NetDecode<Minecraft>>::decode(&mut r).await?);
        }
        Ok(v)
    }
}
impl<T : NetDecode<Minecraft>> NetDecode<Minecraft> for Cow<'_, [T]>
where [T] : ToOwned<Owned = Vec<T>>
{ async fn decode<R : netzer::AsyncRead>(r : R) -> netzer::Result<Self> {
    Ok(Cow::Owned(<Vec<T> as NetDecode<Minecraft>>::decode(r).await?))
} }
impl<T : NetDecode<Minecraft>, const N : usize> NetDecode<Minecraft> for [T; N] {
    async fn decode<R : netzer::AsyncRead>(mut r : R) -> netzer::Result<Self> {
        let len = usize::try_from(<VarInt<u32> as NetDecode<Minecraft>>::decode(&mut r).await?.0)?;
        if (len != N) { return Err("invalid array len".into()); }
        let mut v = [const { MaybeUninit::uninit() }; N];
        for i in 0..len {
            v[i].write(<T as NetDecode<Minecraft>>::decode(&mut r).await?);
        }
        // SAFETY: All items in `v` were written above (len == N).
        Ok(unsafe { MaybeUninit::array_assume_init(v) })
    }
}
