// =============================================================================
// Voice
// =============================================================================

pub mod attribute;

use std::ops::RangeInclusive;

use arbitrary_int::UInt;
use bitvec::{
    field::BitField,
    order::Msb0,
    slice::BitSlice,
    view::BitView,
};
use num_enum::{
    IntoPrimitive,
    TryFromPrimitive,
};

use crate::{
    field::{
        self,
        Field,
        Fields,
    },
    message::{
        self,
        voice,
        Group,
        MessageType,
    },
    packet::Packet,
    Error,
};

// -----------------------------------------------------------------------------

// Fields

// Opcode

#[derive(Debug, Eq, IntoPrimitive, PartialEq, TryFromPrimitive)]
#[num_enum(error_type(name = Error, constructor = Error::conversion))]
#[repr(u8)]
pub enum Opcode {
    NoteOff = 0b1000,
    NoteOn = 0b1001,
}

field::impl_field_trait_field!(Opcode, u8, 8..=11);

// Channel

field::impl_field!(pub Channel { u8, 12..=15, 4 });

// Other

field::impl_field!(pub Data {u32, 32..=63 });
field::impl_field!(pub Index { u8, 24..=31, 7 });
field::impl_field!(pub Note { u8, 16..=23, 7 });
field::impl_field!(pub Velocity { u16, 32..=47 });

// -----------------------------------------------------------------------------

// Messages

// Note Off

voice::impl_message!(pub NoteOff { Opcode::NoteOff, [
    { note, Note, None },
    { velocity, Velocity, None },
] });

impl<'a> NoteOff<'a> {
    pub fn try_init(packet: &'a mut [u32], note: Note, velocity: Velocity) -> Result<Self, Error> {
        Ok(Self::try_init_internal(packet)?
            .set_note(note)
            .set_velocity(velocity))
    }
}

// Note On

voice::impl_message!(pub NoteOn { Opcode::NoteOn, [
    { note, Note, None },
    { velocity, Velocity, None },
] });

impl<'a> NoteOn<'a> {
    pub fn try_init(packet: &'a mut [u32], note: Note, velocity: Velocity) -> Result<Self, Error> {
        Ok(Self::try_init_internal(packet)?
            .set_note(note)
            .set_velocity(velocity))
    }
}

// -----------------------------------------------------------------------------

// Macros

// Message

macro_rules! impl_message {
    (
        $(#[$meta:meta])*
        $vis:vis $message:ident { $opcode:expr, [
            $({ $value_name:ident, $value_type:ty, $value_range:expr },)*
        ] }
    ) => {
            message::impl_message!(
                $(#[$meta])*
                $vis $message { 2, [
                    { message_type, MessageType, None },
                    { group, Group, None },
                    { opcode, Opcode, None },
                    { channel, Channel, None },
                  $({ $value_name, $value_type, $value_range },)*
                ] }
            );

            impl<'a> $message<'a> {
                const OPCODE: Opcode = $opcode;

                fn try_init_internal(packet: &'a mut [u32]) -> Result<Self, Error> {
                    Ok(Self::try_new(packet)?
                        .reset()
                        .set_message_type(MessageType::Voice)
                        .set_group(Group::default())
                        .set_opcode(Self::OPCODE)
                        .set_channel(Channel::default()))
                }
            }
    };
}

use impl_message;
