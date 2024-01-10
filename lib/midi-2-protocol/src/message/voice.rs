// =============================================================================
// Voice
// =============================================================================

//! TODO

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
        TryReadFromPacket,
        WriteToPacket,
    },
    message::{
        self,
        voice,
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

// Opcode

/// TODO
/// # Examples
/// TODO
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

field::impl_field_trait_field_traits!(Opcode, u8, 8..=11);

// Attribute

#[derive(Debug, Eq, PartialEq)]
pub enum Attribute {
    None,
    Manufacturer(Manufacturer),
    Profile(Profile),
    Pitch(Pitch, Fractional),
}

impl TryReadFromPacket for Attribute {
    fn try_read_from_packet<P>(packet: &P) -> Result<Self, Error>
    where
        Self: Sized,
        P: GetBitSlice + ?Sized,
    {
        match packet.try_read_field::<AttributeType>()? {
            AttributeType::None => Ok(Attribute::None),
            AttributeType::Manufacturer => Ok(Attribute::Manufacturer(packet.try_read_field()?)),
            AttributeType::Profile => Ok(Attribute::Profile(packet.try_read_field()?)),
            AttributeType::Pitch => Ok(Attribute::Pitch(
                packet.try_read_field()?,
                packet.try_read_field()?,
            )),
        }
    }
}

impl WriteToPacket for Attribute {
    fn write_to_packet<P>(self, packet: P) -> P
    where
        P: GetBitSlice,
    {
        match self {
            Self::None => packet.write_field(AttributeType::None),
            Self::Manufacturer(manufacturer) => packet
                .write_field(AttributeType::Manufacturer)
                .write_field(manufacturer),
            Self::Profile(profile) => packet
                .write_field(AttributeType::Profile)
                .write_field(profile),
            Self::Pitch(pitch, fractional) => packet
                .write_field(AttributeType::Pitch)
                .write_field(pitch)
                .write_field(fractional),
        }
    }
}

#[derive(Debug, Eq, IntoPrimitive, PartialEq, TryFromPrimitive)]
#[num_enum(error_type(name = Error, constructor = Error::conversion))]
#[repr(u8)]
pub(crate) enum AttributeType {
    None = 0x0,
    Manufacturer = 0x1,
    Profile = 0x2,
    Pitch = 0x3,
}

field::impl_field_trait_field_traits!(AttributeType, u8, 24..=31);

field::impl_field!(
    /// TODO
    /// # Examples
    /// TODO
    pub Manufacturer { u16, 48..=63 }
);

field::impl_field!(
    /// TODO
    /// # Example
    /// TODO
    pub Profile { u16, 48..=63 }
);

field::impl_field!(
    /// TODO
    /// # Examples
    /// TODO
    pub Pitch { u8, 48..=54, 7 }
);

field::impl_field!(
    /// TODO
    /// # Examples
    /// TODO
    pub Fractional { u16, 55..=63, 9 }
);

// Channel

/// TODO
/// # Examples
/// TODO
#[derive(Debug, Default, Eq, IntoPrimitive, PartialEq, TryFromPrimitive)]
#[num_enum(error_type(name = Error, constructor = Error::conversion))]
#[repr(u8)]
pub enum Channel {
    #[default]
    C1 = 0x0,
    C2 = 0x1,
    C3 = 0x2,
    C4 = 0x3,
    C5 = 0x4,
    C6 = 0x5,
    C7 = 0x6,
    C8 = 0x7,
    C9 = 0x8,
    C10 = 0x9,
    C11 = 0xa,
    C12 = 0xb,
    C13 = 0xc,
    C14 = 0xd,
    C15 = 0xe,
    C16 = 0xf,
}

field::impl_field_trait_field_traits!(Channel, u8, 12..=15);

// Other

field::impl_field!(
    /// TODO
    /// # Examples
    /// TODO
    pub Bank { u8, 16..=23, 7 }
);

field::impl_field!(
    /// TODO
    /// # Examples
    /// TODO
    pub Controller { u8, 24..=31, 7 }
);

field::impl_field!(
    /// TODO
    /// # Examples
    /// TODO
    pub Data {u32, 32..=63 }
);

field::impl_field!(
    /// TODO
    /// # Examples
    /// TODO
    pub Note { u8, 16..=23, 7 }
);

field::impl_field!(
    /// TODO
    /// # Examples
    /// TODO
    pub PerNoteController { u8, 24..=31 }
);

field::impl_field!(
    /// TODO
    /// # Examples
    /// TODO
    pub Velocity { u16, 32..=47 }
);

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
        { attribute, Attribute },
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
        { attribute, Attribute },
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

// Enumeration

voice::impl_enumeration!(
    /// TODO
    /// # Example
    /// TODO
    pub Voice, [
        RegisteredPerNoteController,
        AssignablePerNoteController,
        RegisteredController,
        AssignableController,
        RelativeRegisteredController,
        RelativeAssignableController,
        PerNotePitchBend,
        NoteOff,
        NoteOn,
        // PolyPressure,
        // ControlChange,
        // ProgramChange,
        // ChannelPressure,
        // PitchBend,
        // PerNoteManagement,
    ]
);

// -----------------------------------------------------------------------------

// Macros

// Enumeration

macro_rules! impl_enumeration {
    (
        $(#[$meta:meta])*
        $vis:vis $enum:ident, [
            $($message:ident,)*
        ]
    ) => {
        message::impl_enumeration!(
            $(#[$meta])*
            $vis $enum, [
                $($message,)*
            ]
        );

        impl<'a> $enum<'a> {
            pub(crate) fn try_new(bits: &'a mut BitSlice<u32, Msb0>) -> Result<Self, Error> {
                match bits.try_read_field::<Opcode>()? {
                    $(Opcode::$message => Ok(Self::$message($message::try_new(bits)?)),)*
                    _ => unreachable!()
                }
            }
        }
    };
}

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
                    Ok(Self::try_from(packet)?
                        .reset()
                        .set_message_type(MessageType::Voice)
                        .set_group(Group::default())
                        .set_opcode(Self::OPCODE)
                        .set_channel(Channel::default()))
                }
            }
    };
}

// -----------------------------------------------------------------------------

// Macro Exports

pub(crate) use impl_enumeration;
pub(crate) use impl_message;
