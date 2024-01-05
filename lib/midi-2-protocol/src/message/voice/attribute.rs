use crate::message::{
    self,
    Error,
    Integrals,
    Value,
};

message::impl_value!(Manufacturer { u16, 48..=63 });
