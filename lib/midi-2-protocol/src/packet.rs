// =============================================================================
// Packet
// =============================================================================

use bitvec::{
    order::Msb0,
    slice::BitSlice,
};

// -----------------------------------------------------------------------------

// Traits

// Packet

pub trait Packet {
    fn get(&self) -> &BitSlice<u32, Msb0>;

    fn get_mut(&mut self) -> &mut BitSlice<u32, Msb0>;

    fn reset(self) -> Self;
}