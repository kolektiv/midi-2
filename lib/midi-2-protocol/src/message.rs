// =============================================================================
// Message
// =============================================================================

//! UMP Format messages for MIDI 2.x.
//!
//! The message types, and associated field types implemented as part of
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
use bitvec::field::BitField;
use num_enum::{
    IntoPrimitive,
    TryFromPrimitive,
};

use crate::{
    field::{
        self,
        Field,
    },
    packet::Packet,
    Error,
};

// -----------------------------------------------------------------------------

// Fields

// Message Type

/// Message Type field type.
///
/// The `MessageType` field type access the 4-bit Message Type field present in
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
/// # use midi_2_protocol::*;
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

field::impl_field_trait_field!(MessageType, u8, 0..=3);

// Group

field::impl_field!(
    /// Group field type.
    ///
    /// The `Group` field type accesses the 4-bit Group field present in most UMP
    /// messages (exluding Utility and Stream messages) **([M2-104-UM 2.1.2])**.
    /// Messages which contain a Group field provide `group(...)` and
    /// `set_group(...)` functions to read and write the Group value.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use midi_2_protocol::*;
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
    pub Group { u8, 4..=7, 4 }
);

// -----------------------------------------------------------------------------

// Macros

// Message

macro_rules! impl_message {
    (
        $(#[$meta:meta])*
        $vis:vis $message:ident { $size:literal, [
            $({ $name:ident, $type:ty $(, $range:expr)? },)*
        ] }
    ) => {
        message::impl_message_struct!($($meta)*, $vis, $message);
        message::impl_message_constructor!($message, $size);
        message::impl_message_packet!($message, $size);
        message::impl_message_trait_bits!($message);
        message::impl_message_trait_debug!($message, $({ $name },)*);
        message::impl_message_fields!($message, $({ $name, $type $(, $range)? },)*);
    };
}

macro_rules! impl_message_struct {
    ($($meta:meta)*, $vis:vis, $message:ident) => {
        $(#[$meta])*
        $vis struct $message<'a> {
            bits: &'a mut BitSlice<u32, Msb0>,
        }
    };
}

macro_rules! impl_message_constructor {
    ($message:ident, $size:literal) => {
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
    ($message:ident, $size:literal) => {
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

macro_rules! impl_message_fields {
    ($message:ident, $({ $name:ident, $type:ty $(, $range:expr)? },)*) => {
        impl<'a> $message<'a> {
            $(
                ::paste::paste! {
                    pub fn $name(&self) -> Result<$type, Error> {
                        self.try_get::<$type>(message::impl_message_fields_range_arg!($($range)?))
                    }

                    #[must_use]
                    pub fn [<set_ $name>](self, $name: $type) -> Self {
                        self.set::<$type>($name, message::impl_message_fields_range_arg!($($range)?))
                    }
                }
            )*
        }
    };
}

macro_rules! impl_message_fields_range_arg {
    ($range:expr) => {
        Some($range)
    };
    () => {
        None
    };
}

macro_rules! impl_message_trait_bits {
    ($message:ident) => {
        impl<'a> Packet for $message<'a> {
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
    ($message:ident, $({ $name:ident },)*) => {
        impl<'a> ::core::fmt::Debug for $message<'a> {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                f.debug_struct(stringify!($message))
                  $(.field(stringify!($name), &self.$name().unwrap()))*
                    .finish()
            }
        }
    };
}

// -----------------------------------------------------------------------------

// Macro Exports

pub(crate) use impl_message;
pub(crate) use impl_message_constructor;
pub(crate) use impl_message_fields;
pub(crate) use impl_message_fields_range_arg;
pub(crate) use impl_message_packet;
pub(crate) use impl_message_struct;
pub(crate) use impl_message_trait_bits;
pub(crate) use impl_message_trait_debug;
