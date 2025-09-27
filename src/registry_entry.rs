//! Reigstr entries.


use crate::{
    ident::Ident,
    nbt::to_network as to_network_nbt
};
use core::fmt::Debug;
use std::io::Write;
use serde::Serialize as Ser;


/// An entry in a core or datapack registry.
pub struct RegistryEntry<T>
where
    T : RegistryEntryType
{
    /// The ID of the entry.
    pub id   : Ident,
    /// The entry data.
    pub data : T
}


/// A type that is used in core or datapack registries.
pub trait RegistryEntryType
where
    Self : Debug + Ser
{
    /// The ID of the registry.
    const REGISTRY_ID : Ident;

    /// Write this registry entry as network NBT.
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
