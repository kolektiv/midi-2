use std::ops::RangeInclusive;

use crate::message::{
    self,
    Error,
    Integrals,
    Value,
};

message::impl_value!(pub Manufacturer { u16, 48..=63 });
