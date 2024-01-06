// =============================================================================
// Voice
// =============================================================================

pub mod attribute;

use arbitrary_int::UInt;
use bitvec::{
    order::Msb0,
    slice::BitSlice,
    view::BitView,
};
use num_enum::{
    IntoPrimitive,
    TryFromPrimitive,
};

use crate::message::{
    self,
    voice,
    Bits,
    Error,
    Group,
    Integrals,
    MessageType,
    Value,
    Values,
};

// -----------------------------------------------------------------------------
// Values
// -----------------------------------------------------------------------------

// Opcode

#[derive(Debug, Eq, IntoPrimitive, PartialEq, TryFromPrimitive)]
#[num_enum(error_type(name = Error, constructor = Error::conversion))]
#[repr(u8)]
pub enum Opcode {
    NoteOff = 0b1000,
    NoteOn = 0b1001,
}

message::impl_arbitrary_value_trait_value!(Opcode { u8, 8..=11 });

// -----------------------------------------------------------------------------

// Channel

message::impl_arbitrary_value!(pub Channel { u8, 4, 12..=15});

// -----------------------------------------------------------------------------

// Partial

message::impl_primitive_value!(pub Data {u32, 32..=63 });
message::impl_primitive_value!(pub Index { u8, 24..=31 });
message::impl_arbitrary_value!(pub Note { u8, 7, 16..=23 });
message::impl_primitive_value!(pub Velocity { u16, 32..=47 });

// -----------------------------------------------------------------------------
// Messages
// -----------------------------------------------------------------------------

// Note Off

voice::impl_message!(pub NoteOff { Opcode::NoteOff, [
    { note, Note },
    { velocity, Velocity },
] });

impl<'a> NoteOff<'a> {
    pub fn try_init(packet: &'a mut [u32], note: Note, velocity: Velocity) -> Result<Self, Error> {
        Ok(Self::try_init_internal(packet)?
            .set_note(note)
            .set_velocity(velocity))
    }
}

// -----------------------------------------------------------------------------

// Note On

voice::impl_message!(pub NoteOn { Opcode::NoteOn, [
    { note, Note },
    { velocity, Velocity },
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
// -----------------------------------------------------------------------------

// Message

macro_rules! impl_message {
    (
        $(#[$meta:meta])*
        $vis:vis $message:ident { $opcode:expr, [
            $({ $value_name:ident, $value_type:ty },)*
        ] }
    ) => {
            message::impl_message!(
                $(#[$meta])*
                $vis $message { 2, [
                    { message_type, MessageType },
                    { group, Group },
                    { opcode, Opcode },
                    { channel, Channel },
                  $({ $value_name, $value_type },)*
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
