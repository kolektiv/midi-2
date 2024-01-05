//! The [`system`](crate::message::system) module contains modules for both
//! System Common ([`common`](crate::message::system::common)) and System Real
//! Time ([`real_time`](crate::message::system::real_time)) messages, as defined
//! by **([M2-104-UM 7.6])**.

pub mod common;
pub mod real_time;

use arbitrary_int::UInt;

use crate::message::{
    self,
    Error,
    Integrals,
    Value,
};

// =============================================================================
// System
// =============================================================================

// -----------------------------------------------------------------------------
// Values
// -----------------------------------------------------------------------------

// Universal

message::impl_value!(
    /// # Status
    ///
    /// The Status value type accesses the 8 status bits of a System message
    /// **([M2-104-UM 7.6])**. Messages which contain the status type will have
    /// functions for getting and setting the status value, although this is
    /// not usually required as it will generally be set on initialization of a
    /// packet.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use midi_2_protocol::message::system::*;
    /// # use midi_2_protocol::message::system::real_time::*;
    /// #
    /// let mut packet = TimingClock::packet();
    /// let mut timing_clock = TimingClock::try_init(&mut packet).unwrap();
    ///
    /// assert_eq!(timing_clock.status().unwrap(), Status::new(0xf8));
    /// ```
    Status { u8, 8, 8..=15 }
);

// -----------------------------------------------------------------------------
// Macros
// -----------------------------------------------------------------------------

// Message

macro_rules! impl_message {
    (
        $(#[$meta:meta])*
        $message:ident { $status:tt, [
            $({ $value_name:ident, $value_type:ty },)*
        ] }
    ) => {
            message::impl_message!(
                $(#[$meta])*
                $message { 1, [
                    { message_type, MessageType },
                    { group, Group },
                    { status, Status },
                  $({ $value_name, $value_type },)*
                ] }
            );

            impl<'a> $message<'a> {
                pub(super) const STATUS: Status = Status::new($status);

                fn try_init_internal(packet: &'a mut [u32]) -> Result<Self, Error> {
                    Ok(Self::try_new(packet)?
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
