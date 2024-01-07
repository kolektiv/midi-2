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

use std::ops::RangeInclusive;

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

// Values

// Status

/// Status field value type.
///
/// The `Status` value type accesses the 8-bit Status field of a `System`
/// message **([M2-104-UM 7.6])**. Messages which contain the `Status` type will
/// have functions for getting and setting the Status value, although this is
/// not usually required as it will generally be set on initialization of a
/// packet.
///
/// # Examples
///
/// ```rust
/// # use midi_2_protocol::message::Error;
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

field::impl_field_trait_field!(Status, u8, 8..=15);

// -----------------------------------------------------------------------------

// Macros

// Message

macro_rules! impl_message {
    (
        $(#[$meta:meta])*
        $vis:vis $message:ident { $status:expr, [
            $({ $value_name:ident, $value_type:ty, $value_range:expr },)*
        ] }
    ) => {
            message::impl_message!(
                $(#[$meta])*
                $vis $message { 1, [
                    { message_type, MessageType, None },
                    { group, Group, None },
                    { status, Status, None },
                  $({ $value_name, $value_type, $value_range },)*
                ] }
            );

            impl<'a> $message<'a> {
                pub(super) const STATUS: Status = $status;

                fn try_init_internal(packet: &'a mut [u32]) -> Result<Self, Error> {
                    Ok(Self::try_new(packet)?
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
            pub fn try_init(packet: &'a mut [u32]) -> Result<Self, Error> {
                Self::try_init_internal(packet)
            }
        }
    };
}

pub(crate) use impl_message;
pub(crate) use impl_message_try_init;
