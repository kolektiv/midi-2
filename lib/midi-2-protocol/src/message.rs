// =============================================================================
// Message
// =============================================================================

//! UMP Format messages for MIDI 2.x.
//!
//! The message types, and associated field types implemented as part of
//! [`message`](crate::message) provide a typed approach to working with UMP
//! Format messages (as Universal MIDI Packets -- variable-length arrays of N *
//! 32-bits). See the specification ([M2-104-UMP][1]) for the full details of
//! the UMP Format and the MIDI 2.x Protocol.
//!
//! This is only implemented for the MIDI 2.x Protocol -- support for the legacy
//! MIDI 1.0 message types within UMP is not provided, so MIDI 1.0 Channel Voice
//! Messages **([M2-104-UMP 7.3])** (and associated types and values) are not
//! implemented.
//!
//! Note that references are made to the specification throughout, including
//! relevant section numbers where appropriate.
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

use bitvec::{
    field::BitField,
    order::Msb0,
    slice::BitSlice,
    view::BitView,
};
use num_enum::{
    IntoPrimitive,
    TryFromPrimitive,
};

use crate::{
    field::{
        self,
        TryReadFromPacket,
        WriteToPacket,
    },
    message,
    packet::{
        GetBitSlice,
        TryReadField,
    },
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
#[allow(clippy::module_name_repetitions)]
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

field::impl_field_trait_field_traits!(MessageType, u8, 0..=3);

// Group

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
/// // packet is [0x10f80000]...
/// assert_eq!(message.group()?, Group::G1);
///
/// let mut message = message.set_group(Group::G4);
///
/// // packet is now [0x13f80000]...
/// assert_eq!(message.group()?, Group::G4);
/// #
/// # Ok::<(), Error>(())
/// ```
#[derive(Debug, Default, Eq, IntoPrimitive, PartialEq, TryFromPrimitive)]
#[num_enum(error_type(name = Error, constructor = Error::conversion))]
#[repr(u8)]
pub enum Group {
    #[default]
    G1 = 0x0,
    G2 = 0x1,
    G3 = 0x2,
    G4 = 0x3,
    G5 = 0x4,
    G6 = 0x5,
    G7 = 0x6,
    G8 = 0x7,
    G9 = 0x8,
    G10 = 0x9,
    G11 = 0xa,
    G12 = 0xb,
    G13 = 0xc,
    G14 = 0xd,
    G15 = 0xe,
    G16 = 0xf,
}

field::impl_field_trait_field_traits!(Group, u8, 4..=7);

// -----------------------------------------------------------------------------

// Enumeration

/// MIDI 2[.x] Protocol Message (UMP Format)
///
/// The `Message` enumeration allows pattern matching against all possible
/// message types, given data theoretically comprising a Universal MIDI Packet
/// (UMP). This match will return a reference to the specific MIDI 2[.x] message
/// type, allowing read and write access to the message data.
///
/// # Examples
///
/// The following example constructs a new
/// [`NoteOff`](crate::message::voice::NoteOff) message, and then attempts to
/// read the raw message data back via pattern matching.
///
/// ```rust
/// # use midi_2_protocol::*;
/// # use midi_2_protocol::message::*;
/// # use midi_2_protocol::message::voice::*;
/// #
/// // packet begins as an empty message: [0x0, 0x0] (a [u32; 2]);
/// let mut packet = NoteOff::packet();
///
/// // packet is set to a NoteOff message: [0x43854000, 0x7fe90000]
/// NoteOff::try_init(&mut packet, Note::new(64), Velocity::new(32745))?
///     .set_group(Group::G4)
///     .set_channel(Channel::C6)
///     .set_attribute(Attribute::Manufacturer(Manufacturer::new(4353)));
///
/// if let Message::Voice(Voice::NoteOff(note_off)) = Message::try_from(&mut packet[..])? {
///     // the message has the expected message type and opcode
///     assert_eq!(note_off.message_type()?, MessageType::Voice);
///     assert_eq!(note_off.opcode()?, Opcode::NoteOff);
///
///     // the message also has the fields set as part of init
///     assert_eq!(note_off.note()?, Note::new(64));
///     assert_eq!(note_off.velocity()?, Velocity::new(32745));
///
///     // ...and the fields set via builder-style calls
///     assert_eq!(note_off.group()?, Group::G4);
///     assert_eq!(note_off.channel()?, Channel::C6);
///     assert_eq!(
///         note_off.attribute()?,
///         Attribute::Manufacturer(Manufacturer::new(4353))
///     );
/// } else {
///     panic!("Oh No!")
/// }
/// #
/// # Ok::<(), Error>(())
/// ```
#[derive(Debug)]
pub enum Message<'a> {
    System(system::System<'a>),
    Voice(voice::Voice<'a>),
}

message::impl_enumeration_trait_try_from!(Message);

impl<'a> Message<'a> {
    pub(crate) fn try_new(bits: &'a mut BitSlice<u32, Msb0>) -> Result<Self, Error> {
        match bits.try_read_field::<MessageType>()? {
            MessageType::System => Ok(Self::System(system::System::try_new(bits)?)),
            MessageType::Voice => Ok(Self::Voice(voice::Voice::try_new(bits)?)),
            _ => unreachable!(),
        }
    }
}

// -----------------------------------------------------------------------------

// Macros

// Enumeration

macro_rules! impl_enumeration {
    (
        $(#[$meta:meta])*
        $vis:vis $enum:ident, [
            $($message:ident,)*
            ]
    ) => {
        message::impl_enumeration_struct!($($meta)*, $vis, $enum, $($message,)*);
        message::impl_enumeration_trait_try_from!($enum);
    };
}

macro_rules! impl_enumeration_struct {
    ($($meta:meta)*, $vis:vis, $enum:ident, $($message:ident,)*) => {
        $(#[$meta])*
        #[derive(Debug)]
        $vis enum $enum<'a> {
            $($message($message<'a>)),*
        }
    };
}

macro_rules! impl_enumeration_trait_try_from {
    ($enum:ident) => {
        impl<'a> TryFrom<&'a mut [u32]> for $enum<'a> {
            type Error = Error;

            fn try_from(value: &'a mut [u32]) -> Result<Self, Self::Error> {
                Self::try_new(value.view_bits_mut::<Msb0>())
            }
        }
    };
}

// Message

macro_rules! impl_message {
    (
        $(#[$meta:meta])*
        $vis:vis $message:ident { $size:literal, [
            $({ $name:ident, $type:ty },)*
        ] }
    ) => {
        message::impl_message_struct!($($meta)*, $vis, $message);
        message::impl_message_constructor!($message, $size);
        message::impl_message_fields!($message, $({ $name, $type },)*);
        message::impl_message_packet!($message, $size);
        message::impl_message_reset!($message);
        message::impl_message_trait_debug!($message, $({ $name },)*);
        message::impl_message_trait_get_bit_slice!($message);
        message::impl_message_trait_try_from!($message);

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
            pub(crate) fn try_new(bits: &'a mut BitSlice<u32, Msb0>) -> Result<Self, Error> {
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
                pub const fn packet() -> [u32; $size] {
                    [0u32; $size]
                }
            }
        }
    };
}

macro_rules! impl_message_reset {
    ($message:ident) => {
        impl<'a> $message<'a> {
            pub(crate) fn reset(self) -> Self {
                self.bits.fill(false);
                self
            }
        }
    };
}

macro_rules! impl_message_fields {
    ($message:ident, $({ $name:ident, $type:ty },)*) => {
        impl<'a> $message<'a> {
            $(
                ::paste::paste! {
                    #[doc = "Gets the [`" $type "`](" $type ") field from the message if the available,"]
                    #[doc = "otherwise returning an [`Error`](crate::Error)."]
                    #[doc = "# Errors"]
                    #[doc = "Returns an [`Error`](crate::Error) when the data present in the message cannot be"]
                    #[doc = "converted to the field type (not all field types are total across the range of"]
                    #[doc = "possible values)."]
                    pub fn $name(&self) -> Result<$type, Error> {
                        self.try_read_field::<$type>()
                    }

                    #[must_use]
                    pub fn [<set_ $name>](self, $name: $type) -> Self {
                        self.write_field::<$type>($name)
                    }
                }
            )*
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

macro_rules! impl_message_trait_get_bit_slice {
    ($message:ident) => {
        impl<'a> GetBitSlice for $message<'a> {
            fn get_bit_slice(&self) -> &BitSlice<u32, Msb0> {
                &self.bits
            }

            fn get_bit_slice_mut(&mut self) -> &mut BitSlice<u32, Msb0> {
                &mut self.bits
            }
        }
    };
}

macro_rules! impl_message_trait_try_from {
    ($message:ident) => {
        impl<'a> TryFrom<&'a mut [u32]> for $message<'a> {
            type Error = Error;

            fn try_from(value: &'a mut [u32]) -> Result<Self, Self::Error> {
                Self::try_new(value.view_bits_mut::<Msb0>())
            }
        }
    };
}

// -----------------------------------------------------------------------------

// Macro Exports

pub(crate) use impl_enumeration;
pub(crate) use impl_enumeration_struct;
pub(crate) use impl_enumeration_trait_try_from;
pub(crate) use impl_message;
pub(crate) use impl_message_constructor;
pub(crate) use impl_message_fields;
pub(crate) use impl_message_packet;
pub(crate) use impl_message_reset;
pub(crate) use impl_message_struct;
pub(crate) use impl_message_trait_debug;
pub(crate) use impl_message_trait_get_bit_slice;
pub(crate) use impl_message_trait_try_from;
