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

message::impl_value!(Status { u8, 8, 8..=15 });
