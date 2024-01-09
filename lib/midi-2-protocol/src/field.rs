// =============================================================================
// Field
// =============================================================================

use crate::{
    packet::GetBitSlice,
    Error,
};

// -----------------------------------------------------------------------------

// Traits

// Field

pub trait TryReadFromPacket {
    fn try_read_from_packet<P>(packet: &P) -> Result<Self, Error>
    where
        Self: Sized,
        P: GetBitSlice + ?Sized;
}

pub trait WriteToPacket {
    fn write_to_packet<P>(self, packet: P) -> P
    where
        P: GetBitSlice;
}

// -----------------------------------------------------------------------------

// Macros

// Field

macro_rules! impl_field {
    (
        $(#[$meta:meta])*
        $vis:vis $field:ident { $integral:ty, $range:expr $(, $size:literal)? }
    ) => {
        field::impl_field_struct!($($meta)*, $vis, $field, $integral $(, $size)?);
        field::impl_field_constructor!($field, $integral $(, $size)?);
        field::impl_field_trait_from!($field, $integral $(, $size)?);
        field::impl_field_trait_try_from!($field, $integral $(, $size)?);
        field::impl_field_trait_field!($field, $integral, $range);
    };
}

// Field Struct

macro_rules! impl_field_struct {
    ($($meta:meta)*, $vis:vis, $field:ident, $integral:ty, $size:literal) => {
        $(#[$meta])*
        #[derive(Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
        $vis struct $field(UInt<$integral, $size>);
    };
    ($($meta:meta)*, $vis:vis, $field:ident, $integral:ty) => {
        $(#[$meta])*
        #[derive(Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
        $vis struct $field($integral);
    };
}

// Field Constructor

macro_rules! impl_field_constructor {
    ($field:ident, $integral:ty $(, $size:literal)?) => {
        impl $field {
            field::impl_field_constructor_fns!($field, $integral $(, $size)?);
        }
    };
}

macro_rules! impl_field_constructor_fns {
    ($field:ident, $integral:ty, $size:literal) => {
        ::paste::paste! {
            #[must_use]
            pub const fn new(value: $integral) -> Self {
                Self(UInt::<$integral, $size>::new(value))
            }

            #[doc = "Attempts to create a new [`" $field "`](" $field ") from the given value, if the given value"]
            #[doc = "is valid (note that not all field types are total with regard to value)."]
            #[doc = "# Errors"]
            #[doc = "Returns an [`Error`](crate::Error) if the given value is not valid for the"]
            #[doc = "field type."]
            pub fn try_new(value: $integral) -> Result<Self, Error> {
                Self::try_from(value)
            }
        }
    };
    ($field:ident, $integral:ty) => {
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
    ($field:ident, $integral:ty $(, $size:literal)?) => {
        impl From<$field> for $integral {
            field::impl_field_trait_from_fns!($field, $integral $(, $size)?);
        }
    };
}

macro_rules! impl_field_trait_from_fns {
    ($field:ident, $integral:ty, $size:literal) => {
        fn from(value: $field) -> Self {
            value.0.value()
        }
    };
    ($field:ident, $integral:ty) => {
        fn from(value: $field) -> Self {
            value.0
        }
    };
}

// Field Trait - Try From

macro_rules! impl_field_trait_try_from {
    ($field:ident, $integral:ty $(, $size:literal)?) => {
        impl TryFrom<$integral> for $field {
            type Error = Error;

            field::impl_field_trait_try_from_fns!($field, $integral $(, $size)?);
        }
    };
}

macro_rules! impl_field_trait_try_from_fns {
    ($field:ident, $integral:ty, $size:literal) => {
        fn try_from(value: $integral) -> Result<Self, Self::Error> {
            UInt::<$integral, $size>::try_new(value)
                .map_err(|_| Error::overflow(value, $size))
                .map($field)
        }
    };
    ($field:ident, $integral:ty) => {
        fn try_from(value: $integral) -> Result<Self, Self::Error> {
            Ok($field(value))
        }
    };
}

// Field Trait - Field

macro_rules! impl_field_trait_field {
    ($field:ident, $integral:ty, $range:expr) => {
        impl TryReadFromPacket for $field {
            fn try_read_from_packet<P>(packet: &P) -> Result<Self, Error>
            where
                P: GetBitSlice + ?Sized,
            {
                let bit_slice = packet.get_bit_slice();
                let integral = bit_slice[$range].load_be::<$integral>();

                Self::try_from(integral)
            }
        }

        impl WriteToPacket for $field {
            fn write_to_packet<P>(self, mut packet: P) -> P
            where
                P: GetBitSlice,
            {
                let bit_slice = packet.get_bit_slice_mut();
                let integral = <$integral>::from(self);

                bit_slice[$range].store_be::<$integral>(integral);
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
