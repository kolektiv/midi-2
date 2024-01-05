// =============================================================================
// System Real Time
// =============================================================================

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
    /// # Example
    ///
    /// ```rust
    /// # use midi_2_protocol::message::*;
    /// # use midi_2_protocol::message::system::*;
    /// # use midi_2_protocol::message::system::real_time::*;
    /// #
    /// let mut packet = TimingClock::packet();
    /// let mut message = TimingClock::try_init(&mut packet)?;
    ///
    /// assert_eq!(message.message_type()?, MessageType::System);
    /// assert_eq!(message.group()?, Group::new(0x0));
    /// assert_eq!(message.status()?, Status::TimingClock);
    ///
    /// assert_eq!(packet, [0x10f80000]);
    /// #
    /// # Ok::<(), Error>(())
    /// ```
    TimingClock { Status::TimingClock, [] }
);

system::impl_message_try_init!(TimingClock);

// -----------------------------------------------------------------------------

// Start

system::impl_message!(
    /// # Start
    ///
    /// The Start message **([M2-104-UM 7.6] and [MA01])** is a System
    /// Real Time message sent using a 32-bit UMP **([M2-104-UM])**.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use midi_2_protocol::message::*;
    /// # use midi_2_protocol::message::system::*;
    /// # use midi_2_protocol::message::system::real_time::*;
    /// #
    /// let mut packet = Start::packet();
    /// let mut message = Start::try_init(&mut packet)?;
    ///
    /// assert_eq!(message.message_type()?, MessageType::System);
    /// assert_eq!(message.group()?, Group::new(0x0));
    /// assert_eq!(message.status()?, Status::Start);
    ///
    /// assert_eq!(packet, [0x10fa0000]);
    /// #
    /// # Ok::<(), Error>(())
    /// ```
    Start { Status::Start, [] }
);

system::impl_message_try_init!(Start);

// -----------------------------------------------------------------------------

// Continue

system::impl_message!(
    /// # Continue
    ///
    /// The Continue message **([M2-104-UM 7.6] and [MA01])** is a System
    /// Real Time message sent using a 32-bit UMP **([M2-104-UM])**.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use midi_2_protocol::message::*;
    /// # use midi_2_protocol::message::system::*;
    /// # use midi_2_protocol::message::system::real_time::*;
    /// #
    /// let mut packet = Continue::packet();
    /// let mut message = Continue::try_init(&mut packet)?;
    ///
    /// assert_eq!(message.message_type()?, MessageType::System);
    /// assert_eq!(message.group()?, Group::new(0x0));
    /// assert_eq!(message.status()?, Status::Continue);
    ///
    /// assert_eq!(packet, [0x10fb0000]);
    /// #
    /// # Ok::<(), Error>(())
    /// ```
    Continue { Status::Continue, [] }
);

system::impl_message_try_init!(Continue);

// -----------------------------------------------------------------------------

// Stop

system::impl_message!(
    /// # Stop
    ///
    /// The Stop message **([M2-104-UM 7.6] and [MA01])** is a System
    /// Real Time message sent using a 32-bit UMP **([M2-104-UM])**.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use midi_2_protocol::message::*;
    /// # use midi_2_protocol::message::system::*;
    /// # use midi_2_protocol::message::system::real_time::*;
    /// #
    /// let mut packet = Stop::packet();
    /// let mut message = Stop::try_init(&mut packet)?;
    ///
    /// assert_eq!(message.message_type()?, MessageType::System);
    /// assert_eq!(message.group()?, Group::new(0x0));
    /// assert_eq!(message.status()?, Status::Stop);
    ///
    /// assert_eq!(packet, [0x10fc0000]);
    /// #
    /// # Ok::<(), Error>(())
    /// ```
    Stop { Status::Stop, [] }
);

system::impl_message_try_init!(Stop);

// -----------------------------------------------------------------------------

// Active Sensing

system::impl_message!(
    /// # Active Sensing
    ///
    /// The Active Sensing message **([M2-104-UM 7.6] and [MA01])** is a System
    /// Real Time message sent using a 32-bit UMP **([M2-104-UM])**.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use midi_2_protocol::message::*;
    /// # use midi_2_protocol::message::system::*;
    /// # use midi_2_protocol::message::system::real_time::*;
    /// #
    /// let mut packet = ActiveSensing::packet();
    /// let mut message = ActiveSensing::try_init(&mut packet)?;
    ///
    /// assert_eq!(message.message_type()?, MessageType::System);
    /// assert_eq!(message.group()?, Group::new(0x0));
    /// assert_eq!(message.status()?, Status::ActiveSensing);
    ///
    /// assert_eq!(packet, [0x10fe0000]);
    /// #
    /// # Ok::<(), Error>(())
    /// ```
    ActiveSensing { Status::ActiveSensing, [] }
);

system::impl_message_try_init!(ActiveSensing);

// -----------------------------------------------------------------------------

// Reset

system::impl_message!(
    /// # Reset
    ///
    /// The Reset message **([M2-104-UM 7.6] and [MA01])** is a System
    /// Real Time message sent using a 32-bit UMP **([M2-104-UM])**.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use midi_2_protocol::message::*;
    /// # use midi_2_protocol::message::system::*;
    /// # use midi_2_protocol::message::system::real_time::*;
    /// #
    /// let mut packet = Reset::packet();
    /// let mut message = Reset::try_init(&mut packet)?;
    ///
    /// assert_eq!(message.message_type()?, MessageType::System);
    /// assert_eq!(message.group()?, Group::new(0x0));
    /// assert_eq!(message.status()?, Status::Reset);
    ///
    /// assert_eq!(packet, [0x10ff0000]);
    /// #
    /// # Ok::<(), Error>(())
    /// ```
    Reset { Status::Reset, [] }
);

system::impl_message_try_init!(Reset);
