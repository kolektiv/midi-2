use bitvec::{
    order::Msb0,
    slice::BitSlice,
    view::BitView,
};

use crate::message::{
    self,
    system::{
        self,
        Status,
    },
    Bits,
    Error,
    Group,
    MessageType,
    Values,
};

// =============================================================================
// System Real Time
// =============================================================================

// -----------------------------------------------------------------------------
// Messages
// -----------------------------------------------------------------------------

// Timing Clock

system::impl_message!(
    /// # Timing Clock
    ///
    /// The Timing Clock message **([M2-104-UM 7.6] and [MA01])** is a System
    /// Real Time message sent using a 32-bit UMP **([M2-104-UM])**.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use midi_2_protocol::message::*;
    /// # use midi_2_protocol::message::system::*;
    /// # use midi_2_protocol::message::system::real_time::*;
    /// #
    /// let mut packet = TimingClock::packet();
    /// let mut timing_clock = TimingClock::try_init(&mut packet).unwrap();
    ///
    /// assert_eq!(timing_clock.message_type().unwrap(), MessageType::System);
    /// assert_eq!(timing_clock.group().unwrap(), Group::new(0x0));
    /// assert_eq!(timing_clock.status().unwrap(), Status::new(0xf8));
    ///
    /// assert_eq!(packet, [0x10f80000]);
    /// ```
    TimingClock { 0xf8, [] }
);

system::impl_message_try_init!(TimingClock);
