// =============================================================================
// Voice
// =============================================================================

pub mod attribute;

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
    RegisteredPerNoteController = 0x0,
    AssignablePerNoteController = 0x1,
    RegisteredController = 0x2,
    AssignableController = 0x3,
    RelativeRegisteredController = 0x4,
    RelativeAssignableController = 0x5,
    PerNotePitchBend = 0x6,
    NoteOff = 0x8,
    NoteOn = 0x9,
    PolyPressure = 0xa,
    ControlChange = 0xb,
    ProgramChange = 0xc,
    ChannelPressure = 0xd,
    PitchBend = 0xe,
    PerNoteManagement = 0xf,
}

field::impl_field_trait_field!(Opcode, u8, 8..=11);

// Channel

field::impl_field!(pub Channel { u8, 12..=15, 4 });

// Other

field::impl_field!(pub Bank { u8, 16..=23, 7 });
field::impl_field!(pub Controller { u8, 24..=31, 7 });
field::impl_field!(pub Data {u32, 32..=63 });
field::impl_field!(pub Note { u8, 16..=23, 7 });
field::impl_field!(pub PerNoteController { u8, 24..=31 });
field::impl_field!(pub Velocity { u16, 32..=47 });

// -----------------------------------------------------------------------------

// Messages

// Registered Per-Note Controller

voice::impl_message!(
    /// TODO
    /// # Examples
    /// TODO
    pub RegisteredPerNoteController { Opcode::RegisteredPerNoteController, [
        { note, Note },
        { per_note_controller, PerNoteController },
        { data, Data },
    ] }
);

impl<'a> RegisteredPerNoteController<'a> {
    /// TODO
    /// # Errors
    /// TODO
    pub fn try_init(
        packet: &'a mut [u32],
        note: Note,
        per_note_controller: PerNoteController,
    ) -> Result<Self, Error> {
        Ok(Self::try_init_internal(packet)?
            .set_note(note)
            .set_per_note_controller(per_note_controller))
    }
}

// Assignable Per-Note Controller

voice::impl_message!(
    /// TODO
    /// # Examples
    /// TODO
    pub AssignablePerNoteController { Opcode::AssignablePerNoteController, [
        { note, Note },
        { per_note_controller, PerNoteController },
        { data, Data },
    ] }
);

impl<'a> AssignablePerNoteController<'a> {
    /// TODO
    /// # Errors
    /// TODO
    pub fn try_init(
        packet: &'a mut [u32],
        note: Note,
        per_note_controller: PerNoteController,
    ) -> Result<Self, Error> {
        Ok(Self::try_init_internal(packet)?
            .set_note(note)
            .set_per_note_controller(per_note_controller))
    }
}

// Registered Controller

voice::impl_message!(
    /// TODO
    /// # Examples
    /// TODO
    pub RegisteredController { Opcode::RegisteredController, [
        { bank, Bank },
        { controller, Controller },
        { data, Data },
    ] }
);

impl<'a> RegisteredController<'a> {
    /// TODO
    /// # Errors
    /// TODO
    pub fn try_init(
        packet: &'a mut [u32],
        bank: Bank,
        controller: Controller,
    ) -> Result<Self, Error> {
        Ok(Self::try_init_internal(packet)?
            .set_bank(bank)
            .set_controller(controller))
    }
}

// Assignable Controller

voice::impl_message!(
    /// TODO
    /// # Examples
    /// TODO
    pub AssignableController { Opcode::AssignableController, [
        { bank, Bank },
        { controller, Controller },
        { data, Data },
    ] }
);

impl<'a> AssignableController<'a> {
    /// TODO
    /// # Errors
    /// TODO
    pub fn try_init(
        packet: &'a mut [u32],
        bank: Bank,
        controller: Controller,
    ) -> Result<Self, Error> {
        Ok(Self::try_init_internal(packet)?
            .set_bank(bank)
            .set_controller(controller))
    }
}

// Relative Registered Controller

voice::impl_message!(
    /// TODO
    /// # Examples
    /// TODO
    pub RelativeRegisteredController { Opcode::RelativeRegisteredController, [
        { bank, Bank },
        { controller, Controller },
        { data, Data },
    ] }
);

impl<'a> RelativeRegisteredController<'a> {
    /// TODO
    /// # Errors
    /// TODO
    pub fn try_init(
        packet: &'a mut [u32],
        bank: Bank,
        controller: Controller,
    ) -> Result<Self, Error> {
        Ok(Self::try_init_internal(packet)?
            .set_bank(bank)
            .set_controller(controller))
    }
}

// Relative Assignable Controller

voice::impl_message!(
    /// TODO
    /// # Examples
    /// TODO
    pub RelativeAssignableController { Opcode::RelativeAssignableController, [
        { bank, Bank },
        { controller, Controller },
        { data, Data },
    ] }
);

impl<'a> RelativeAssignableController<'a> {
    /// TODO
    /// # Errors
    /// TODO
    pub fn try_init(
        packet: &'a mut [u32],
        bank: Bank,
        controller: Controller,
    ) -> Result<Self, Error> {
        Ok(Self::try_init_internal(packet)?
            .set_bank(bank)
            .set_controller(controller))
    }
}

// Per-Note Pitch Bend

voice::impl_message!(
    /// TODO
    /// # Examples
    /// TODO
    pub PerNotePitchBend { Opcode::PerNotePitchBend, [
        { note, Note },
        { data, Data },
    ] }
);

impl<'a> PerNotePitchBend<'a> {
    /// TODO
    /// # Errors
    /// TODO
    pub fn try_init(packet: &'a mut [u32], note: Note) -> Result<Self, Error> {
        Ok(Self::try_init_internal(packet)?.set_note(note))
    }
}

// Note Off

voice::impl_message!(
    /// TODO
    /// # Examples
    /// TODO
    pub NoteOff { Opcode::NoteOff, [
        { note, Note },
        { velocity, Velocity },
    ] }
);

impl<'a> NoteOff<'a> {
    /// TODO
    /// # Errors
    /// TODO
    pub fn try_init(packet: &'a mut [u32], note: Note, velocity: Velocity) -> Result<Self, Error> {
        Ok(Self::try_init_internal(packet)?
            .set_note(note)
            .set_velocity(velocity))
    }
}

// Note On

voice::impl_message!(
    /// TODO
    /// # Examples
    /// TODO
    pub NoteOn { Opcode::NoteOn, [
        { note, Note },
        { velocity, Velocity },
    ] }
);

impl<'a> NoteOn<'a> {
    /// TODO
    /// # Errors
    /// TODO
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
            $({ $name:ident, $type:ty },)*
        ] }
    ) => {
            message::impl_message!(
                $(#[$meta])*
                $vis $message { 2, [
                    { message_type, MessageType },
                    { group, Group },
                    { opcode, Opcode },
                    { channel, Channel },
                  $({ $name, $type },)*
                ] }
            );

            impl<'a> $message<'a> {
                pub(crate) const OPCODE: Opcode = $opcode;

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
