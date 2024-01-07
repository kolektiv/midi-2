mod field;
mod packet;

pub mod message;

use thiserror::Error;

// =============================================================================
// MIDI 2 Protocol
// =============================================================================

// Errors

#[derive(Debug, Error)]
pub enum Error {
    #[error("Conversion: Attempted to convert from {0}, not a valid variant.")]
    Conversion(u8),
    #[error("Overflow: Attempted to store value {0} in a {1} bit type.")]
    Overflow(u64, u8),
    #[error("Size: Expected a packet of {0} bits, but found {1} bits.")]
    Size(u8, u8),
}

impl Error {
    pub(crate) const fn conversion(value: u8) -> Self {
        Self::Conversion(value)
    }

    pub(crate) fn overflow(value: impl Into<u64>, size: u8) -> Self {
        Self::Overflow(value.into(), size)
    }

    pub(crate) const fn size(expected: u8, actual: u8) -> Self {
        Self::Size(expected, actual)
    }
}
