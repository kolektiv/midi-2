// =============================================================================
// System
// =============================================================================

//! System (Common and Real Time) message and value types.
//!
//! The [`system`](crate::message::system) module contains modules for both
//! System Common ([`common`](crate::message::system::common)) and System Real
//! Time ([`real_time`](crate::message::system::real_time)) messages and values,
//! as defined by **([M2-104-UM 7.6])**.
//!
//! Additionally, common value types and a unifying enumeration type combining
//! Common and Real Time messages are provided.

pub mod common;
pub mod real_time;

use bitvec::{
    field::BitField,
    prelude::Msb0,
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

// Status

/// Status field type.
///
/// The `Status` field type accesses the 8-bit Status field of a `System`
/// message **([M2-104-UM 7.6])**. Messages which contain the `Status` type will
/// have functions for getting and setting the Status value, although this is
/// not usually required as it will generally be set on initialization of a
/// packet.
///
/// # Examples
///
/// ```rust
/// # use midi_2_protocol::Error;
/// # use midi_2_protocol::message::system::Status;
/// # use midi_2_protocol::message::system::real_time::TimingClock;
/// #
/// let mut packet = TimingClock::packet();
/// let mut timing_clock = TimingClock::try_init(&mut packet)?;
///
/// assert_eq!(timing_clock.status()?, Status::TimingClock);
/// #
/// # Ok::<(), Error>(())
/// ```
#[derive(Debug, Eq, IntoPrimitive, PartialEq, TryFromPrimitive)]
#[num_enum(error_type(name = Error, constructor = Error::conversion))]
#[repr(u8)]
pub enum Status {
    MIDITimeCode = 0xf1,
    SongPositionPointer = 0xf2,
    SongSelect = 0xf3,
    TuneRequest = 0xf6,
    TimingClock = 0xf8,
    Start = 0xfa,
    Continue = 0xfb,
    Stop = 0xfc,
    ActiveSensing = 0xfe,
    Reset = 0xff,
}

field::impl_field_trait_field_traits!(Status, u8, 8..=15);

// -----------------------------------------------------------------------------

// Enumeration

/// TODO
/// # Examples
/// TODO
#[derive(Debug)]
pub enum System<'a> {
    Common(common::Common<'a>),
    RealTime(real_time::RealTime<'a>),
}

message::impl_enumeration_trait_try_from!(System);

impl<'a> System<'a> {
    pub(crate) fn try_new(bits: &'a mut BitSlice<u32, Msb0>) -> Result<Self, Error> {
        match bits.try_read_field::<Status>()? {
            Status::MIDITimeCode
            | Status::SongPositionPointer
            | Status::SongSelect
            | Status::TuneRequest => Ok(Self::Common(common::Common::try_new(bits)?)),
            Status::TimingClock
            | Status::Start
            | Status::Continue
            | Status::Stop
            | Status::ActiveSensing
            | Status::Reset => Ok(Self::RealTime(real_time::RealTime::try_new(bits)?)),
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
        message::impl_enumeration!(
            $(#[$meta])*
            $vis $enum, [
                $($message,)*
            ]
        );

        impl<'a> $enum<'a> {
            pub(crate) fn try_new(bits: &'a mut BitSlice<u32, Msb0>) -> Result<Self, Error> {
                match bits.try_read_field::<Status>()? {
                    $(Status::$message => Ok(Self::$message($message::try_new(bits)?)),)*
                    _ => unreachable!(),
                }
            }
        }
    };
}

// Message

macro_rules! impl_message {
    (
        $(#[$meta:meta])*
        $vis:vis $message:ident { $status:expr, [
            $({ $name:ident, $type:ty },)*
        ] }
    ) => {
            message::impl_message!(
                $(#[$meta])*
                $vis $message { 1, [
                    { message_type, MessageType },
                    { group, Group },
                    { status, Status },
                  $({ $name, $type },)*
                ] }
            );

            impl<'a> $message<'a> {
                pub(crate) const STATUS: Status = $status;

                fn try_init_internal(packet: &'a mut [u32]) -> Result<Self, Error> {
                    Ok(Self::try_from(packet)?
                        .reset()
                        .set_message_type(MessageType::System)
                        .set_group(Group::default())
                        .set_status(Self::STATUS))
                }
            }
    };
}

macro_rules! impl_message_try_init {
    ($message:ident) => {
        impl<'a> $message<'a> {
            ::paste::paste! {
                #[doc = "TODO"]
                #[doc = "# Errors"]
                #[doc = "TODO"]
                pub fn try_init(packet: &'a mut [u32]) -> Result<Self, Error> {
                    Self::try_init_internal(packet)
                }
            }
        }
    };
}

// -----------------------------------------------------------------------------

// Macro Exports

pub(crate) use impl_enumeration;
pub(crate) use impl_message;
pub(crate) use impl_message_try_init;
