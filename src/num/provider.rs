use crate::{
    box_cow::BoxCow,
    num::weighted::Weighted
};
use core::{
    fmt::{ self, Formatter },
    marker::PhantomData
};
use std::borrow::Cow;
use serde::{
    Serialize as Ser,
    Deserialize as Deser,
    de::{
        Deserializer as Deserer,
        Visitor,
        MapAccess,
        value::MapAccessDeserializer,
        Error as DeserError
    }
};
use syndebug::SynDebug;


#[derive(Clone, Ser, Debug, SynDebug)]
#[serde(tag = "type")]
pub enum IntProvider<'l, T>
where
    T : Clone + SynDebug + TryFrom<u64> + TryFrom<i64>
{

    #[serde(rename = "minecraft:constant", alias = "constant")]
    Constant {
        value : T
    },

    #[serde(rename = "minecraft:uniform", alias = "uniform")]
    Uniform {
        min_inclusive : T,
        max_inclusive : T
    },

    #[serde(rename = "minecraft:biased_to_bottom", alias = "biased_to_bottom")]
    BiasedToBottom {
        min_inclusive : T,
        max_inclusive : T
    },

    #[serde(rename = "minecraft:clamped", alias = "clamped")]
    Clamped {
        min_inclusive : T,
        max_inclusive : T,
        source        : BoxCow<'l, IntProvider<'l, T>>
    },

    #[serde(rename = "minecraft:clamped_normal", alias = "clamped_normal")]
    ClampedNormal {
        mean          : f32,
        deviation     : f32,
        min_inclusive : T,
        max_inclusive : T
    },

    #[serde(rename = "minecraft:weighted_list", alias = "weighted_list")]
    WeightedList {
        distribution : Cow<'l, [Weighted<IntProvider<'l, T>>]>
    }

}

impl<'de, 'l, T> Deser<'de> for IntProvider<'l, T>
where
    T : Deser<'de> + Clone + SynDebug + TryFrom<u64> + TryFrom<i64>
{
    fn deserialize<D>(deserer : D) -> Result<Self, D::Error>
    where
        D : Deserer<'de>
    { deserer.deserialize_any(IntProviderVisitor { _marker : PhantomData }) }
}


struct IntProviderVisitor<'l, T>
where
    T : Clone + SynDebug + TryFrom<u64> + TryFrom<i64>
{
    _marker : PhantomData<fn() -> IntProvider<'l, T>>
}

impl<'de, 'l, T> Visitor<'de> for IntProviderVisitor<'l, T>
where
    T : Clone + Deser<'de> + SynDebug + TryFrom<u64> + TryFrom<i64>
{
    type Value = IntProvider<'l, T>;

    #[inline]
    fn expecting(&self, f : &mut Formatter) -> fmt::Result {
        write!(f, "enum IntProvider")
    }

    fn visit_u64<E>(self, v : u64) -> Result<Self::Value, E>
    where
        E : DeserError
    { Ok(IntProvider::Constant { value : T::try_from(v).map_err(|_| E::custom("constant out of range"))? }) }

    fn visit_i64<E>(self, v : i64) -> Result<Self::Value, E>
    where
        E : DeserError
    { Ok(IntProvider::Constant { value : T::try_from(v).map_err(|_| E::custom("constant out of range"))? }) }

    fn visit_map<A>(self, map : A) -> Result<Self::Value, A::Error>
    where
        A : MapAccess<'de>
    {
        let map = MapAccessDeserializer::new(map);
        Ok(match (MapIntProvider::deserialize(map)?) {
            MapIntProvider::Constant       { value }                                         => IntProvider::Constant       { value },
            MapIntProvider::Uniform        { min_inclusive, max_inclusive }                  => IntProvider::Uniform        { min_inclusive, max_inclusive },
            MapIntProvider::BiasedToBottom { min_inclusive, max_inclusive }                  => IntProvider::BiasedToBottom { min_inclusive, max_inclusive },
            MapIntProvider::Clamped        { min_inclusive, max_inclusive, source }          => IntProvider::Clamped        { min_inclusive, max_inclusive, source },
            MapIntProvider::ClampedNormal  { mean, deviation, min_inclusive, max_inclusive } => IntProvider::ClampedNormal  { mean, deviation, min_inclusive, max_inclusive },
            MapIntProvider::WeightedList   { distribution }                                  => IntProvider::WeightedList   { distribution },
        })
    }

}


#[derive(Clone, Ser, Deser, Debug, SynDebug)]
#[serde(tag = "type")]
enum MapIntProvider<'l, T>
where
    T : Clone + SynDebug + TryFrom<u64> + TryFrom<i64>
{

    #[serde(rename = "minecraft:constant", alias = "constant")]
    Constant {
        value : T
    },

    #[serde(rename = "minecraft:uniform", alias = "uniform")]
    Uniform {
        min_inclusive : T,
        max_inclusive : T
    },

    #[serde(rename = "minecraft:biased_to_bottom", alias = "biased_to_bottom")]
    BiasedToBottom {
        min_inclusive : T,
        max_inclusive : T
    },

    #[serde(rename = "minecraft:clamped", alias = "clamped")]
    Clamped {
        min_inclusive : T,
        max_inclusive : T,
        source        : BoxCow<'l, IntProvider<'l, T>>
    },

    #[serde(rename = "minecraft:clamped_normal", alias = "clamped_normal")]
    ClampedNormal {
        mean          : f32,
        deviation     : f32,
        min_inclusive : T,
        max_inclusive : T
    },

    #[serde(rename = "minecraft:weighted_list", alias = "weighted_list")]
    WeightedList {
        distribution : Cow<'l, [Weighted<IntProvider<'l, T>>]>
    }

}
