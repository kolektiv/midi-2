// =============================================================================
// Attribute
// =============================================================================

//! TODO

use bitvec::field::BitField;

use crate::{
    field::{
        self,
        TryReadFromPacket,
        WriteToPacket,
    },
    packet::GetBitSlice,
    Error,
};

// -----------------------------------------------------------------------------

// Fields

// Manufacturer

field::impl_field!(
    /// TODO
    /// # Examples
    /// TODO
    pub Manufacturer { u16, 48..=63 }
);
