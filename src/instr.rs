use crate::value::Value;
use std::fmt;
use num_enum::{ FromPrimitive, IntoPrimitive };

/// OpCode for internal use.
/// 
#[derive(Debug, Clone, Copy, PartialEq, Eq, FromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum OpPrefix {
    CONSTANT = 0,
    NIL,
    TRUE,
    FALSE,
    ADD,
    SUBTRACT,
    MULTIPLY,
    DIVIDE,
    NEGATE,
    RETURN,
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
    Constant{ idx: u8 },
    Nil,
    True,
    False,
    Equal,
    Greater,
    Less,
    Add,
    Subtract,
    Multiply,
    Divide,
    Negate,
    Return,
}

pub enum InstrResult {
    Good(Instr),
    BadOp { bytes: Vec<u8>, },
    // BadContext
}
use InstrResult::*;

impl InstrResult {
    // inspired from https://doc.rust-lang.org/beta/src/core/str/validations.rs.html#36-70
    // TODO : can we automatically infer length from iterator advancement?
    pub fn next_instr_point<'a, I: Iterator<Item = &'a u8>>(iter: &mut I) -> Option<(Self, usize)>
    {
        let prefix = OpPrefix::from(*iter.next()?); // if None == iter.next(), the instruction is None as expected

        // TODO: we might want try blocks here
        // to get BadOp and count bytes
        let return_val = match prefix {
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
            OpPrefix::NIL => { (Good(Instr::Nil), 1) }, // [NIL]
            OpPrefix::TRUE => { (Good(Instr::True), 1) }, // [TRUE]
            OpPrefix::FALSE => { (Good(Instr::False), 1) }, // [FALSE]
            OpPrefix::ADD => { (Good(Instr::Add), 1) }, // [ADD]
            OpPrefix::SUBTRACT => { (Good(Instr::Subtract), 1) }, // [SUBTRACT]
            OpPrefix::MULTIPLY => { (Good(Instr::Multiply), 1) }, // [MULTIPLY]
            OpPrefix::DIVIDE => { (Good(Instr::Divide), 1) }, // [DIVIDE]
            OpPrefix::NEGATE => { (Good(Instr::Negate), 1) }, // [NEGATE]
            OpPrefix::RETURN => { (Good(Instr::Return), 1) }, // [RETURN]
            
            OpPrefix::UNKNOWN(byte) => {
                (BadOp { bytes: vec![byte] }, 1)
            },
        };

        Some(return_val)
    }

    pub fn with_context<'a>(&'a self, consts: &'a Vec<Value>) -> ContextedInstrResult<'a>{
        ContextedInstrResult { ires: self, consts }
    }
}


pub struct ContextedInstrResult<'a> {
    ires: &'a InstrResult,
    consts: &'a Vec<Value>,
}

impl<'a> fmt::Display for ContextedInstrResult<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.ires {
            Good(instr) => match instr {
                Instr::Constant { idx } => {
                    write!(f, "Constant [{}] = {}", idx, self.consts.get(usize::from(*idx)).unwrap())
                },
                // Instr::Nil => { write!(f, "Nil") },
                // Instr::True => { write!(f, "True") },
                // Instr::False => { write!(f, "False") },
                // Instr::Equal => { write!(f, "Equal") },
                // Instr::Greater => { write!(f, "Greater") },
                // Instr::Less => { write!(f, "Less") },
                // Instr::Add => { write!(f, "Add") },
                // Instr::Subtract => { write!(f, "Subtract") },
                // Instr::Multiply => { write!(f, "Multiply") },
                // Instr::Divide => { write!(f, "Divide") },
                // Instr::Negate => { write!(f, "Negate") },
                // Instr::Return => { write!(f, "Return") },
                _ => { write!(f, "{:?}", instr) },
            },
            BadOp { bytes } => {
                write!(f, "<BadOp {:02X?}>", bytes)
            },
        }
    }
}