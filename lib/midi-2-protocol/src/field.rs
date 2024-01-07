// =============================================================================
// Field
// =============================================================================

use std::ops::RangeInclusive;

use crate::{
    packet::Packet,
    Error,
};

// -----------------------------------------------------------------------------

// Traits

// Field

pub(crate) trait Field {
    fn try_read<P>(packet: &P, range: Option<RangeInclusive<usize>>) -> Result<Self, Error>
    where
        Self: Sized,
        P: Packet;

    fn write<P>(self, packet: P, range: Option<RangeInclusive<usize>>) -> P
    where
        P: Packet;
}

// Fields

pub(crate) trait Fields {
    fn try_get<F>(&self, range: Option<RangeInclusive<usize>>) -> Result<F, Error>
    where
        F: Field;

    fn set<F>(self, field: F, range: Option<RangeInclusive<usize>>) -> Self
    where
        F: Field;
}

// -----------------------------------------------------------------------------

// Trait Implementations

// Fields

impl<P> Fields for P
where
    P: Packet,
{
    fn try_get<V>(&self, range: Option<RangeInclusive<usize>>) -> Result<V, Error>
    where
        V: Field,
    {
        V::try_read(self, range)
    }

    fn set<V>(self, value: V, range: Option<RangeInclusive<usize>>) -> Self
    where
        V: Field,
    {
        value.write(self, range)
    }
}

// -----------------------------------------------------------------------------

// Macros

// Field

macro_rules! impl_field {
    (
        $(#[$meta:meta])*
        $vis:vis $value:ident { $integral:ty, $range:expr $(, $size:literal)? }
    ) => {
        $crate::field::impl_field_struct!($($meta)*, $vis, $value, $integral $(, $size)?);
        $crate::field::impl_field_constructor!($value, $integral $(, $size)?);
        $crate::field::impl_field_trait_from!($value, $integral $(, $size)?);
        $crate::field::impl_field_trait_try_from!($value, $integral $(, $size)?);
        $crate::field::impl_field_trait_field!($value, $integral, $range);
    };
}

// Field Struct

macro_rules! impl_field_struct {
    ($($meta:meta)*, $vis:vis, $value:ident, $integral:ty, $size:literal) => {
        $(#[$meta])*
        #[derive(Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
        $vis struct $value(UInt<$integral, $size>);
    };
    ($($meta:meta)*, $vis:vis, $value:ident, $integral:ty) => {
        $(#[$meta])*
        #[derive(Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
        $vis struct $value($integral);
    };
}

// Field Constructor

macro_rules! impl_field_constructor {
    ($value:ident, $integral:ty $(, $size:literal)?) => {
        impl $value {
            crate::field::impl_field_constructor_fns!($integral $(, $size)?);
        }
    };
}

macro_rules! impl_field_constructor_fns {
    ($integral:ty, $size:literal) => {
        ::paste::paste! {
            #[must_use]
            pub const fn new(value: $integral) -> Self {
                Self(UInt::<$integral, $size>::new(value))
            }

            pub fn try_new(value: $integral) -> Result<Self, Error> {
                Self::try_from(value)
            }
        }
    };
    ($integral:ty) => {
        ::paste::paste! {
            #[must_use]
            pub const fn new(value: $integral) -> Self {
                Self(value)
            }
        }
    };
}

// Field Trait - From

macro_rules! impl_field_trait_from {
    ($value:ident, $integral:ty $(, $size:literal)?) => {
        impl From<$value> for $integral {
            crate::field::impl_field_trait_from_fns!($value, $integral $(, $size)?);
        }
    };
}

macro_rules! impl_field_trait_from_fns {
    ($value:ident, $integral:ty, $size:literal) => {
        fn from(value: $value) -> Self {
            value.0.value()
        }
    };
    ($value:ident, $integral:ty) => {
        fn from(value: $value) -> Self {
            value.0
        }
    };
}

// Field Trait - Try From

macro_rules! impl_field_trait_try_from {
    ($value:ident, $integral:ty $(, $size:literal)?) => {
        impl TryFrom<$integral> for $value {
            type Error = Error;

            crate::field::impl_field_trait_try_from_fns!($value, $integral $(, $size)?);
        }
    };
}

macro_rules! impl_field_trait_try_from_fns {
    ($value:ident, $integral:ty, $size:literal) => {
        fn try_from(value: $integral) -> Result<Self, Self::Error> {
            UInt::<$integral, $size>::try_new(value)
                .map_err(|_| Error::overflow(value, $size))
                .map($value)
        }
    };
    ($value:ident, $integral:ty) => {
        fn try_from(value: $integral) -> Result<Self, Self::Error> {
            Ok($value(value))
        }
    };
}

// Field Trait - Field

macro_rules! impl_field_trait_field {
    ($field:ident, $integral:ty, $range:expr) => {
        impl Field for $field {
            fn try_read<I>(packet: &I, range: Option<RangeInclusive<usize>>) -> Result<Self, Error>
            where
                I: Packet,
            {
                let range = range.unwrap_or($range);
                let integral = packet.get()[range].load_be::<$integral>();

                Self::try_from(integral)
            }

            fn write<I>(self, mut packet: I, range: Option<RangeInclusive<usize>>) -> I
            where
                I: Packet,
            {
                let range = range.unwrap_or($range);
                let integral = <$integral>::from(self);

                packet.get_mut()[range].store_be::<$integral>(integral);
                packet
            }
        }
    };
}

// -----------------------------------------------------------------------------

// Macro Exports

pub(crate) use impl_field;
pub(crate) use impl_field_constructor;
pub(crate) use impl_field_constructor_fns;
pub(crate) use impl_field_struct;
pub(crate) use impl_field_trait_field;
pub(crate) use impl_field_trait_from;
pub(crate) use impl_field_trait_from_fns;
pub(crate) use impl_field_trait_try_from;
pub(crate) use impl_field_trait_try_from_fns;