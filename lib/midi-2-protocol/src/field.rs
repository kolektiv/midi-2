// =============================================================================
// Field
// =============================================================================

use crate::{
    packet::Packet,
    Error,
};

// -----------------------------------------------------------------------------

// Traits

// Field

pub trait Field {
    fn try_read<P>(packet: &P) -> Result<Self, Error>
    where
        Self: Sized,
        P: Packet;

    fn write<P>(self, packet: P) -> P
    where
        P: Packet;
}

// Fields

pub trait Fields {
    fn try_get<F>(&self) -> Result<F, Error>
    where
        F: Field;

    fn set<F>(self, field: F) -> Self
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
    fn try_get<V>(&self) -> Result<V, Error>
    where
        V: Field,
    {
        V::try_read(self)
    }

    fn set<V>(self, value: V) -> Self
    where
        V: Field,
    {
        value.write(self)
    }
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
        impl Field for $field {
            fn try_read<I>(packet: &I) -> Result<Self, Error>
            where
                I: Packet,
            {
                let integral = packet.get()[$range].load_be::<$integral>();

                Self::try_from(integral)
            }

            fn write<I>(self, mut packet: I) -> I
            where
                I: Packet,
            {
                let integral = <$integral>::from(self);

                packet.get_mut()[$range].store_be::<$integral>(integral);
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
