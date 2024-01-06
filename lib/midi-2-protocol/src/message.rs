// =============================================================================
// Message
// =============================================================================

//! UMP Format messages for MIDI 2.x.
//!
//! The message types, and associated value types implemented as part of
//! [`message`](crate::message) implement a typed approach to working with UMP
//! Format messages (as Universal MIDI Packets -- variable-length arrays of N *
//! 32-bits). See the specification ([M2-104-UMP][1]) for the full details of
//! the UMP Format and the MIDI 2.x Protocol.
//!
//! (Note that references are made to the specification throughout, including
//! relevant section numbers where appropriate).
//!
//! This is only implemented for the MIDI 2.x Protocol -- support for the legacy
//! MIDI 1.0 message types within UMP is not provided, so MIDI 1.0 Channel Voice
//! Messages **([M2-104-UMP 7.3])** (and associated types and values) are not
//! implemented.
//!
//! # Examples
//!
//! Working with typed messages uses a layered approach. It is assumed that the
//! underlying data will always be some form of N * 32-bit storage, which may
//! either be received (and thus need reading in-place) or which may be created,
//! and then modified in-place.
//!
//! For this reason, the message types implement several options for working
//! with new or existing data.
//!
//! ## New Messages
//!
//! Each message type implements a `packet()` function, which will create a
//! `u32` array of the correct length to hold the data for that message type
//! (e.g. calling `packet()` on a Voice message, which is a 64-bit message type
//! in UMP will return a `[u32; 2]` array).
//!
//! This can then be initialized using the `try_init(...)` function for the
//! message type (this may fail if given a packet of incorrect size). This will
//! initialize the packet to contain the supplied message data, and return a
//! type which can further modify the packet as needed.
//!
//! [1]: https://midi.org/specifications/universal-midi-packet-ump-and-midi-2-0-protocol-specification/download

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

// -----------------------------------------------------------------------------
// Values
// -----------------------------------------------------------------------------

// Message Type

/// Message Type field value type.
///
/// The `MessageType` value type access the 4-bit Message Type field present in
/// all UMP messages **([M2-104-UM 2.1.2])**.
///
/// All messages provide `message_type(...)` and `set_message_type(...)`
/// functions to read and write the Message Type value, however this is not
/// likely to be rwquired in normal usage -- Message Types are set on
/// initialization where applicable, and changing the type of an existing
/// message is not likely to be a logically useful operation.
///
/// Reading the Message Type directly is also likely to be rare, as using
/// provided pattern matching functions is likely to be more ergonomic.
///
/// # Examples
///
/// ```rust
/// # use midi_2_protocol::message::*;
/// # use midi_2_protocol::message::system::real_time::*;
/// #
/// let mut packet = TimingClock::packet();
/// let mut message = TimingClock::try_init(&mut packet)?;
///
/// assert_eq!(message.message_type()?, MessageType::System);
/// #
/// # Ok::<(), Error>(())
/// ```
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

impl_arbitrary_value_trait_value!(MessageType { u8, 0..=3 });

// -----------------------------------------------------------------------------

// Group

impl_arbitrary_value!(
    /// Group field value type.
    ///
    /// The `Group` value type accesses the 4-bit Group field present in most UMP
    /// messages (exluding Utility and Stream messages) **([M2-104-UM 2.1.2])**.
    /// Messages which contain a Group field provide `group(...)` and
    /// `set_group(...)` functions to read and write the Group value.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use midi_2_protocol::message::*;
    /// # use midi_2_protocol::message::system::real_time::*;
    /// #
    /// let mut packet = TimingClock::packet();
    /// let mut message = TimingClock::try_init(&mut packet)?;
    ///
    /// assert_eq!(message.group()?, Group::new(0x0));
    /// // packet is [0x10f80000]...
    ///
    /// let mut message = message.set_group(Group::new(0x3));
    ///
    /// assert_eq!(message.group()?, Group::new(0x3));
    /// // packet is now [0x13f80000]...
    /// #
    /// # Ok::<(), Error>(())
    /// ```
    pub Group { u8, 4, 4..=7 }
);

// -----------------------------------------------------------------------------
// Traits
// -----------------------------------------------------------------------------

// Bits

pub(crate) trait Bits {
    fn get(&self) -> &BitSlice<u32, Msb0>;
    fn get_mut(&mut self) -> &mut BitSlice<u32, Msb0>;
    fn reset(self) -> Self;
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
        self.get()[range].load_be::<I>()
    }

    fn set_integral<I>(mut self, range: RangeInclusive<usize>, integral: I) -> Self
    where
        I: Integral,
    {
        self.get_mut()[range].store_be::<I>(integral);
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
    #[error("Overflow: Attempted to store value {0} in a {1} bit type.")]
    Overflow(u64, u8),
    #[error("Size: Expected a packet of {0} bits, but found {1} bits.")]
    Size(u8, u8),
}

impl Error {
    pub(crate) fn conversion(value: u8) -> Self {
        Self::Conversion(value)
    }

    pub(crate) fn overflow(value: impl Into<u64>, size: u8) -> Self {
        Self::Overflow(value.into(), size)
    }

    pub(crate) fn size(expected: u8, actual: u8) -> Self {
        Self::Size(expected, actual)
    }
}

// -----------------------------------------------------------------------------
// Macros
// -----------------------------------------------------------------------------

// Message

macro_rules! impl_message {
    (
        $(#[$meta:meta])*
        $vis:vis $message:ident { $size:literal, [
            $({ $value_name:ident, $value_type:ty },)*
        ] }
    ) => {
        $crate::message::impl_message_type!(
            $(#[$meta])*
            $vis $message
        );

        $crate::message::impl_message_constructors!($message { $size });
        $crate::message::impl_message_packet!($message { $size });
        $crate::message::impl_message_trait_bits!($message);
        $crate::message::impl_message_trait_debug!($message {[ $({ $value_name, $value_type },)* ]});
        $crate::message::impl_message_values!($message {[ $({ $value_name, $value_type },)* ]});
    };
}

macro_rules! impl_message_type {
    (
        $(#[$meta:meta])*
        $vis:vis $message:ident
    ) => {
        $(#[$meta])*
        $vis struct $message<'a> {
            bits: &'a mut BitSlice<u32, Msb0>,
        }
    };
}

macro_rules! impl_message_constructors {
    ($message:ident { $size:literal }) => {
        impl<'a> $message<'a> {
            pub(crate) fn try_new(packet: &'a mut [u32]) -> Result<Self, Error> {
                let bits = packet.view_bits_mut();

                match bits.len() {
                    len if len == $size * 32 => Ok(Self { bits }),
                    len => Err(Error::size($size * 32, len.try_into().unwrap())),
                }
            }
        }
    };
}

macro_rules! impl_message_packet {
    ($message:ident { $size:literal }) => {
        ::paste::paste! {
            impl<'a> $message<'a> {
                #[doc = "Returns an appropriately sized `u32` array for a `" $message "` message."]
                #[doc = "# Examples"]
                #[doc = "```rust"]
                #[doc = concat!("# use ", std::module_path!(), "::")]
                #[doc = "# " $message ";"]
                #[doc = "let mut packet = " $message "::packet(); // Returns a [u32; " $size "]"]
                #[doc = ""]
                #[doc = "// ...initializing (and potentially modifying) the packet using the " ]
                #[doc = "// " $message " type would normally follow..."]
                #[doc = ""]
                #[doc = "// let message = " $message "::try_init(&mut packet, ...) ..."]
                #[doc = "```"]
                #[must_use]
                pub fn packet() -> [u32; $size] {
                    [0u32; $size]
                }
            }
        }
    };
}

macro_rules! impl_message_values {
    ($message:ident { [$({ $value_name:ident, $value_type:ty },)*] }) => {
        impl<'a> $message<'a> {
            $(
                pub fn $value_name(&self) -> Result<$value_type, Error> {
                    self.get_value::<$value_type>()
                }

                ::paste::paste! {
                    #[must_use]
                    pub fn [<set_ $value_name>](self, $value_name: $value_type) -> Self {
                        self.set_value::<$value_type>($value_name)
                    }
                }
            )*
        }
    };
}

macro_rules! impl_message_trait_bits {
    ($message:ident) => {
        impl<'a> Bits for $message<'a> {
            fn get(&self) -> &BitSlice<u32, Msb0> {
                &self.bits
            }

            fn get_mut(&mut self) -> &mut BitSlice<u32, Msb0> {
                &mut self.bits
            }

            fn reset(self) -> Self {
                self.bits.fill(false);
                self
            }
        }
    };
}

macro_rules! impl_message_trait_debug {
    ($message:ident { [$({ $value_name:ident, $value_type:ty },)*] }) => {
        impl<'a> ::core::fmt::Debug for $message<'a> {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                f.debug_struct(stringify!($message))
                  $(.field(stringify!($value_name), &self.$value_name().unwrap()))*
                    .finish()
            }
        }
    };
}

pub(crate) use impl_message;
pub(crate) use impl_message_constructors;
pub(crate) use impl_message_packet;
pub(crate) use impl_message_trait_bits;
pub(crate) use impl_message_trait_debug;
pub(crate) use impl_message_type;
pub(crate) use impl_message_values;

// -----------------------------------------------------------------------------

// Value (Arbitrary)

macro_rules! impl_arbitrary_value {
    (
        $(#[$meta:meta])*
        $vis:vis $value:ident { $integral:ty, $size:literal, $range:expr }
    ) => {
        $crate::message::impl_arbitrary_value_type!(
            $(#[$meta])*
            $vis $value { $integral, $size }
        );

        $crate::message::impl_arbitrary_value_constructors!($value { $integral, $size });
        $crate::message::impl_arbitrary_value_trait_from!($value { $integral });
        $crate::message::impl_arbitrary_value_trait_try_from!($value { $integral, $size });
        $crate::message::impl_arbitrary_value_trait_value!($value { $integral, $range });
    };
}

macro_rules! impl_arbitrary_value_type {
    (
        $(#[$meta:meta])*
        $vis:vis $value:ident { $integral:ty, $size:literal }
    ) => {
        $(#[$meta])*
        #[derive(Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
        $vis struct $value(UInt<$integral, $size>);
    };
}

macro_rules! impl_arbitrary_value_constructors {
    ($value:ident { $integral:ty, $size:literal }) => {
        impl $value {
            ::paste::paste! {
                #[must_use]
                pub const fn new(value: $integral) -> Self {
                    Self(UInt::<$integral, $size>::new(value))
                }

                pub fn try_new(value: $integral) -> Result<Self, Error> {
                    Self::try_from(value)
                }
            }
        }
    };
}

macro_rules! impl_arbitrary_value_trait_from {
    ($value:ident { $integral:ty }) => {
        impl From<$value> for $integral {
            fn from(value: $value) -> Self {
                value.0.value()
            }
        }
    };
}

macro_rules! impl_arbitrary_value_trait_try_from {
    ($value:ident { $integral:ty, $size:literal }) => {
        impl TryFrom<$integral> for $value {
            type Error = Error;

            fn try_from(value: $integral) -> Result<Self, Self::Error> {
                UInt::<$integral, $size>::try_new(value)
                    .map_err(|_| Error::overflow(value, $size))
                    .map($value)
            }
        }
    };
}

macro_rules! impl_arbitrary_value_trait_value {
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

pub(crate) use impl_arbitrary_value;
pub(crate) use impl_arbitrary_value_constructors;
pub(crate) use impl_arbitrary_value_trait_from;
pub(crate) use impl_arbitrary_value_trait_try_from;
pub(crate) use impl_arbitrary_value_trait_value;
pub(crate) use impl_arbitrary_value_type;

// -----------------------------------------------------------------------------

// Value (Primitive)

macro_rules! impl_primitive_value {
    (
        $(#[$meta:meta])*
        $vis:vis $value:ident { $integral:ty, $range:expr }
    ) => {
        $crate::message::impl_primitive_value_type!(
            $(#[$meta])*
            $vis $value { $integral }
        );

        $crate::message::impl_primitive_value_constructors!($value { $integral });
        $crate::message::impl_primitive_value_trait_from!($value { $integral });
        $crate::message::impl_primitive_value_trait_try_from!($value { $integral });
        $crate::message::impl_primitive_value_trait_value!($value { $integral, $range });
    };
}

macro_rules! impl_primitive_value_type {
    (
        $(#[$meta:meta])*
        $vis:vis $value:ident { $integral:ty }
    ) => {
        $(#[$meta])*
        #[derive(Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
        $vis struct $value($integral);
    };
}

macro_rules! impl_primitive_value_constructors {
    ($value:ident { $integral:ty }) => {
        impl $value {
            ::paste::paste! {
                #[must_use]
                pub const fn new(value: $integral) -> Self {
                    Self(value)
                }
            }
        }
    };
}

macro_rules! impl_primitive_value_trait_from {
    ($value:ident { $integral:ty }) => {
        impl From<$value> for $integral {
            fn from(value: $value) -> Self {
                value.0
            }
        }
    };
}

macro_rules! impl_primitive_value_trait_try_from {
    ($value:ident { $integral:ty }) => {
        impl TryFrom<$integral> for $value {
            type Error = Error;

            fn try_from(value: $integral) -> Result<Self, Self::Error> {
                Ok($value(value))
            }
        }
    };
}

macro_rules! impl_primitive_value_trait_value {
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

pub(crate) use impl_primitive_value;
pub(crate) use impl_primitive_value_constructors;
pub(crate) use impl_primitive_value_trait_from;
pub(crate) use impl_primitive_value_trait_try_from;
pub(crate) use impl_primitive_value_trait_value;
pub(crate) use impl_primitive_value_type;
