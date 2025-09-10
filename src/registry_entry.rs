use crate::{
    ident::Ident,
    nbt::to_network as to_network_nbt
};
use core::fmt::Debug;
use std::io::Write;
use serde::Serialize as Ser;


pub struct RegistryEntry<T>
where
    T : RegistryEntryType
{
    pub id   : Ident,
    pub data : T
}


pub trait RegistryEntryType
where
    Self : Debug + Ser
{
    const REGISTRY_ID : Ident;

    fn to_network_nbt<W>(&self, writer : W) -> bool
    where
        W : Write;
}


impl<T> RegistryEntryType for &T
where
    T : RegistryEntryType
{
    const REGISTRY_ID : Ident = <T as RegistryEntryType>::REGISTRY_ID;

    #[inline]
    fn to_network_nbt<W>(&self, writer : W) -> bool
    where
        W : Write
    {
        to_network_nbt(writer, self).unwrap();
        true
    }
}


impl<T> RegistryEntryType for &mut T
where
    T : RegistryEntryType
{
    const REGISTRY_ID : Ident = <T as RegistryEntryType>::REGISTRY_ID;

    #[inline]
    fn to_network_nbt<W>(&self, writer : W) -> bool
    where
        W : Write
    {
        to_network_nbt(writer, self).unwrap();
        true
    }
}


impl<T> RegistryEntryType for Option<T>
where
    T : RegistryEntryType
{
    const REGISTRY_ID : Ident = <T as RegistryEntryType>::REGISTRY_ID;

    fn to_network_nbt<W>(&self, writer : W) -> bool
    where
        W : Write
    { self.as_ref().is_some_and(|inner| T::to_network_nbt(inner, writer)) }
}
