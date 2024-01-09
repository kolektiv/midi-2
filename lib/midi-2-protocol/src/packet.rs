// =============================================================================
// Packet
// =============================================================================

use bitvec::{
    order::Msb0,
    slice::BitSlice,
};

use crate::{
    field::{
        TryReadFromPacket,
        WriteToPacket,
    },
    Error,
};

// -----------------------------------------------------------------------------

// Traits

pub trait GetBitSlice {
    fn get_bit_slice(&self) -> &BitSlice<u32, Msb0>;

    fn get_bit_slice_mut(&mut self) -> &mut BitSlice<u32, Msb0>;
}

pub trait TryReadField {
    fn try_read_field<F>(&self) -> Result<F, Error>
    where
        F: TryReadFromPacket;
}

pub trait WriteField {
    fn write_field<F>(self, field: F) -> Self
    where
        F: WriteToPacket;
}

// -----------------------------------------------------------------------------

// Trait Implementations

impl GetBitSlice for BitSlice<u32, Msb0> {
    fn get_bit_slice(&self) -> &BitSlice<u32, Msb0> {
        self
    }

    fn get_bit_slice_mut(&mut self) -> &mut BitSlice<u32, Msb0> {
        self
    }
}

impl<P> TryReadField for P
where
    P: GetBitSlice + ?Sized,
{
    fn try_read_field<F>(&self) -> Result<F, Error>
    where
        F: TryReadFromPacket,
    {
        F::try_read_from_packet(self)
    }
}

impl<P> WriteField for P
where
    P: GetBitSlice,
{
    fn write_field<F>(self, value: F) -> Self
    where
        F: WriteToPacket,
    {
        value.write_to_packet(self)
    }
}
