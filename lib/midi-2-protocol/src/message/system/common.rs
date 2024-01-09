// =============================================================================
// Common
// =============================================================================

//! TODO

use arbitrary_int::UInt;
use bitvec::{
    field::BitField,
    order::Msb0,
    slice::BitSlice,
    view::BitView,
};

use crate::{
    field::{
        self,
        Field,
        Fields,
    },
    message::{
        self,
        system::{
            self,
            Status,
        },
        Group,
        MessageType,
    },
    packet::Packet,
    Error,
};

// -----------------------------------------------------------------------------

// Fields

// Quarter Frame

/// TODO
/// # Examples
/// TODO
#[derive(Debug)]
pub struct QuarterFrame(pub Data, pub Type);

impl Field for QuarterFrame {
    fn try_read<P>(packet: &P) -> Result<Self, Error>
    where
        Self: Sized,
        P: Packet,
    {
        Ok(Self(packet.try_get()?, packet.try_get()?))
    }

    fn write<P>(self, packet: P) -> P
    where
        P: Packet,
    {
        packet.set(self.0).set(self.1)
    }
}

// Data

field::impl_field!(
    /// TODO
    /// # Examples
    /// TODO
    pub Data { u8, 20..=23, 4 }
);

// Type

/// TODO
/// # Examples
/// TODO
#[derive(Debug)]
pub enum Type {
    Frames(Significance),
    Seconds(Significance),
    Minutes(Significance),
    Hours(Significance),
}

field::impl_field_trait_field!(Type, u8, 17..=19);

impl From<Type> for u8 {
    fn from(value: Type) -> Self {
        match value {
            Type::Frames(significance) => 0 + Into::<u8>::into(significance),
            Type::Seconds(significance) => 2 + Into::<u8>::into(significance),
            Type::Minutes(significance) => 4 + Into::<u8>::into(significance),
            Type::Hours(significance) => 6 + Into::<u8>::into(significance),
        }
    }
}

impl TryFrom<u8> for Type {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Type::Frames(Significance::Least)),
            1 => Ok(Type::Frames(Significance::Most)),
            2 => Ok(Type::Seconds(Significance::Least)),
            3 => Ok(Type::Seconds(Significance::Most)),
            4 => Ok(Type::Minutes(Significance::Least)),
            5 => Ok(Type::Minutes(Significance::Most)),
            6 => Ok(Type::Hours(Significance::Least)),
            7 => Ok(Type::Hours(Significance::Most)),
            _ => Err(Error::conversion(value)),
        }
    }
}

/// TODO
#[derive(Debug)]
pub enum Significance {
    Least,
    Most,
}

impl From<Significance> for u8 {
    fn from(value: Significance) -> Self {
        match value {
            Significance::Least => 0,
            Significance::Most => 1,
        }
    }
}

// -----------------------------------------------------------------------------

// Messages

// MIDI Time Code

system::impl_message!(
    /// TODO
    /// # Examples
    /// TODO
    pub MIDITimeCode { Status::MIDITimeCode, [
        { quarter_frame, QuarterFrame },
    ]}
);

impl<'a> MIDITimeCode<'a> {
    pub fn try_init(packet: &'a mut [u32], quarter_frame: QuarterFrame) -> Result<Self, Error> {
        Ok(Self::try_init_internal(packet)?.set_quarter_frame(quarter_frame))
    }
}
