use crate::value::Value;
use std::fmt;
use num_enum::{ FromPrimitive, IntoPrimitive };

/// OpCode for internal use.
/// 
#[derive(Debug, Clone, Copy, PartialEq, Eq, FromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum OpPrefix {
    RETURN = 0,
    CONSTANT,
    #[num_enum(catch_all)]
    UNKNOWN(u8),
}

impl fmt::Display for OpPrefix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instr {
    Return,
    Constant{ idx: u8 },
}

pub enum InstrResult {
    Good(Instr),
    BadOp { bytes: Vec<u8>, },
    // BadContext
}
use InstrResult::{ Good, BadOp };

impl InstrResult {
    // inspired from https://doc.rust-lang.org/beta/src/core/str/validations.rs.html#36-70
    // TODO : can we automatically infer length from iterator advancement?
    pub fn next_instr_point<'a, I: Iterator<Item = &'a u8>>(iter: &mut I) -> Option<(Self, usize)>
    {
        let prefix = OpPrefix::from(*iter.next()?); // if None == iter.next(), the instruction is None as expected

        // TODO: we might want try blocks here
        // to get BadOp and count bytes
        let return_val = match prefix {
            OpPrefix::RETURN => {
                // [RETURN]
                (Good(Instr::Return), 1)
            },
            OpPrefix::CONSTANT => {
                // [CONSTANT] [CONST_IDX]
                if let Some(&idx) = iter.next() {
                    (Good(Instr::Constant { idx }), 2)
                    
                    // TODO: check context to see if the constant exists
                } else {
                    // TODO: collect all bytes
                    (BadOp { bytes: vec![prefix.into()] }, 1)
                }
            },
            OpPrefix::UNKNOWN(byte) => {
                (BadOp { bytes: vec![byte] }, 1)
            },
        };

        Some(return_val)
    }

    pub fn with_context<'a>(&'a self, consts: &'a Vec<Value>) -> ContextedInstrResult<'a>{
        ContextedInstrResult { instr: self, consts }
    }
}

pub struct ContextedInstrResult<'a> {
    instr: &'a InstrResult,
    consts: &'a Vec<Value>,
}

impl<'a> fmt::Display for ContextedInstrResult<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.instr {
            Good(Instr::Return) => write!(f, "RETURN"),
            Good(Instr::Constant { idx }) => {
                write!(f, "CONSTANT {}('{}')", idx, self.consts.get(usize::from(*idx)).unwrap())
            },

            BadOp { bytes } => {
                write!(f, "<BadOp {:02X?}>", bytes)
            },
        }
    }
}