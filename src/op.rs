use std::fmt;
use num_enum::{ FromPrimitive, IntoPrimitive };

#[derive(Debug, Clone, Copy, PartialEq, Eq, FromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum OpCode {
    RETURN = 0,
    CONSTANT,
    #[num_enum(catch_all)]
    UNKNOWN(u8),
}

impl fmt::Display for OpCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

