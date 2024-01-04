pub mod system;
pub mod voice;

use std::ops::RangeInclusive;

use arbitrary_int::UInt;
use bitvec::{
    field::BitField,
    order::Msb0,
    slice::BitSlice,
};
use funty::Integral;
use num_enum::{
    IntoPrimitive,
    TryFromPrimitive,
};
use thiserror::Error;

// =============================================================================
// Message
// =============================================================================

// -----------------------------------------------------------------------------
// Values
// -----------------------------------------------------------------------------

// Message Type

#[derive(Debug, Eq, IntoPrimitive, PartialEq, TryFromPrimitive)]
#[num_enum(error_type(name = Error, constructor = Error::conversion))]
#[repr(u8)]
pub enum MessageType {
    Utility = 0x0,
    System = 0x1,
    SystemExclusiveData = 0x3,
    Voice = 0x4,
    Data = 0x5,
    FlexData = 0xd,
    Stream = 0xf,
}

impl_value_trait_value!(MessageType { u8, 0..=3 });

// -----------------------------------------------------------------------------

// Group

impl_value!(Group { u8, 4, 4..=7 });

// -----------------------------------------------------------------------------
// Traits
// -----------------------------------------------------------------------------

// Bits

pub(crate) trait Bits {
    fn read(&self) -> &BitSlice<u32, Msb0>;

    fn write(&mut self) -> &mut BitSlice<u32, Msb0>;
}

// -----------------------------------------------------------------------------

// Integrals

pub(crate) trait Integrals {
    fn get_integral<I>(&self, range: RangeInclusive<usize>) -> I
    where
        I: Integral;

    fn set_integral<I>(self, range: RangeInclusive<usize>, integral: I) -> Self
    where
        I: Integral;
}

// -----------------------------------------------------------------------------

// Values

pub(crate) trait Values {
    fn get_value<V>(&self) -> Result<V, Error>
    where
        V: Value;

    fn set_value<V>(self, value: V) -> Self
    where
        V: Value;
}

// -----------------------------------------------------------------------------

// Value

pub(crate) trait Value {
    fn try_read<I>(integrals: &I) -> Result<Self, Error>
    where
        Self: Sized,
        I: Integrals;

    fn write<I>(self, integrals: I) -> I
    where
        I: Integrals;
}

// -----------------------------------------------------------------------------
// Trait Implementations
// -----------------------------------------------------------------------------

// Integrals

impl<B> Integrals for B
where
    B: Bits,
{
    fn get_integral<I>(&self, range: RangeInclusive<usize>) -> I
    where
        I: Integral,
    {
        self.read()[range].load_be::<I>()
    }

    fn set_integral<I>(mut self, range: RangeInclusive<usize>, integral: I) -> Self
    where
        I: Integral,
    {
        self.write()[range].store_be::<I>(integral);
        self
    }
}

// -----------------------------------------------------------------------------

// Values

impl<I> Values for I
where
    I: Integrals,
{
    fn get_value<V>(&self) -> Result<V, Error>
    where
        V: Value,
    {
        V::try_read(self)
    }

    fn set_value<V>(self, value: V) -> Self
    where
        V: Value,
    {
        value.write(self)
    }
}

// -----------------------------------------------------------------------------
// Errors
// -----------------------------------------------------------------------------

#[derive(Debug, Error)]
pub enum Error {
    #[error("Conversion: Attempted to convert from {0}, not a valid variant.")]
    Conversion(u8),
    #[error("Overflow: Attempted to store value {0} in a {1}bit type.")]
    Overflow(u64, u8),
}

impl Error {
    pub(crate) fn conversion(value: u8) -> Self {
        Self::Conversion(value)
    }

    pub(crate) fn overflow<V>(value: impl Into<u64>, size: u8) -> Self {
        Self::Overflow(value.into(), size)
    }
}

// -----------------------------------------------------------------------------
// Macros
// -----------------------------------------------------------------------------

// Value

macro_rules! impl_value {
    (
        $(#[$meta:meta])*
        $value:ident { $integral:ty, $size:literal, $range:expr }
    ) => {
        $crate::message::impl_value_type!(
            $(#[$meta])*
            $value { $integral, $size }
        );

        $crate::message::impl_value_constructors!($value { $integral, $size });
        $crate::message::impl_value_trait_from!($value { $integral });
        $crate::message::impl_value_trait_try_from!($value { $integral, $size });
        $crate::message::impl_value_trait_value!($value { $integral, $range });
    };
}

macro_rules! impl_value_type {
    (
        $(#[$meta:meta])*
        $value:ident { $integral:ty, $size:literal }
    ) => {
        $(#[$meta])*
        #[derive(Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
        pub struct $value(UInt<$integral, $size>);
    };
}

macro_rules! impl_value_constructors {
    ($value:ident { $integral:ty, $size:literal }) => {
        impl $value {
            pub const fn new(value: $integral) -> Self {
                Self(UInt::<$integral, $size>::new(value))
            }

            pub fn try_new(value: $integral) -> Result<Self, Error> {
                Ok(Self::try_from(value)?)
            }
        }
    };
}

macro_rules! impl_value_trait_from {
    ($value:ident { $integral:ty }) => {
        impl From<$value> for $integral {
            fn from(value: $value) -> Self {
                value.0.value()
            }
        }
    };
}

macro_rules! impl_value_trait_try_from {
    ($value:ident { $integral:ty, $size:literal }) => {
        impl TryFrom<$integral> for $value {
            type Error = Error;

            fn try_from(value: $integral) -> Result<Self, Self::Error> {
                Ok($value(
                    UInt::<$integral, $size>::try_new(value)
                        .map_err(|_| Error::overflow::<$integral>(value, $size))?,
                ))
            }
        }
    };
}

macro_rules! impl_value_trait_value {
    ($value:ident { $integral:ty, $range:expr }) => {
        impl Value for $value {
            fn try_read<I>(integrals: &I) -> Result<Self, Error>
            where
                I: Integrals,
            {
                Self::try_from(integrals.get_integral::<$integral>($range))
            }

            fn write<I>(self, integrals: I) -> I
            where
                I: Integrals,
            {
                integrals.set_integral::<$integral>($range, <$integral>::from(self))
            }
        }
    };
}

pub(crate) use impl_value;
pub(crate) use impl_value_constructors;
pub(crate) use impl_value_trait_from;
pub(crate) use impl_value_trait_try_from;
pub(crate) use impl_value_trait_value;
pub(crate) use impl_value_type;
