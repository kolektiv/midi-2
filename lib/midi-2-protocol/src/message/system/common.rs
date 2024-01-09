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
        TryReadFromPacket,
        WriteToPacket,
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
    packet::{
        GetBitSlice,
        TryReadField,
        WriteField,
    },
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

impl TryReadFromPacket for QuarterFrame {
    fn try_read_from_packet<P>(packet: &P) -> Result<Self, Error>
    where
        Self: Sized,
        P: GetBitSlice + ?Sized,
    {
        Ok(Self(packet.try_read_field()?, packet.try_read_field()?))
    }
}

impl WriteToPacket for QuarterFrame {
    fn write_to_packet<P>(self, packet: P) -> P
    where
        P: GetBitSlice,
    {
        packet.write_field(self.0).write_field(self.1)
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
            Type::Frames(significance) => Into::<Self>::into(significance),
            Type::Seconds(significance) => 2 + Into::<Self>::into(significance),
            Type::Minutes(significance) => 4 + Into::<Self>::into(significance),
            Type::Hours(significance) => 6 + Into::<Self>::into(significance),
        }
    }
}

impl TryFrom<u8> for Type {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Frames(Significance::Least)),
            1 => Ok(Self::Frames(Significance::Most)),
            2 => Ok(Self::Seconds(Significance::Least)),
            3 => Ok(Self::Seconds(Significance::Most)),
            4 => Ok(Self::Minutes(Significance::Least)),
            5 => Ok(Self::Minutes(Significance::Most)),
            6 => Ok(Self::Hours(Significance::Least)),
            7 => Ok(Self::Hours(Significance::Most)),
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
    /// TODO
    /// # Examples
    /// TODO
    /// # Errors
    /// TODO
    pub fn try_init(packet: &'a mut [u32], quarter_frame: QuarterFrame) -> Result<Self, Error> {
        Ok(Self::try_init_internal(packet)?.set_quarter_frame(quarter_frame))
    }
}

// -----------------------------------------------------------------------------

// Enumeration

system::impl_enumeration!(
    /// TODO
    /// # Example
    /// TODO
    pub Common, [
        MIDITimeCode,
    ]
);
