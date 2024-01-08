use bitvec::field::BitField;

use crate::{
    field::{
        self,
        Field,
    },
    packet::Packet,
    Error,
};

field::impl_field!(pub Manufacturer { u16, 48..=63 });
