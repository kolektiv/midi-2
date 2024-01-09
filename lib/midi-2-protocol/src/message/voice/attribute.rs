// =============================================================================
// Attribute
// =============================================================================

//! TODO

use bitvec::field::BitField;

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

// Manufacturer

field::impl_field!(
    /// TODO
    /// # Examples
    /// TODO
    pub Manufacturer { u16, 48..=63 }
);
