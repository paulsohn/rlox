use crate::chunk::Chunk;
use crate::value::Value;
use crate::instr::{InstrResult, Instr};

pub struct VM<'a> {
    chunk: &'a mut Chunk,
    ip: usize, // original clox uses pointer ip. here we only use code index of the chunk
    stack: Vec<Value>,
}

impl<'a> VM<'a> {
    pub fn new(chunk: &'a mut Chunk) -> Self {
        VM {
            chunk,
            ip: 0,
            stack: Vec::new()
        }
    }

    pub fn run(&mut self) -> InterpretResult{
        loop {
            // Our VM is sequental: it just decode the next instruction at once.
            // real machines implement pipelining, and have different stages for fetching and decoding respectively.
            let res = self.chunk.read(self.ip);
            if res.is_none() {
                return Ok(()); // code evaluated successfully
            }

            let (ires, len) = res.unwrap();
            #[cfg(debug_assertions)]
            {
                println!("    {:.3?}", self.stack);

                // if self.ip should be a pointer, change this also
                self.chunk.disasm(&ires, self.ip)
            }
            self.ip += len; // instr ptr proceeds

            match ires {
                InstrResult::Good(instr) => match instr {
                    Instr::Constant { idx } => {
                        let val = self.chunk.get_const(idx);
                        self.stack.push(val);
                    },
                    Instr::Nil => {
                        self.stack.push(Value::Nil);
                    },
                    Instr::True => {
                        self.stack.push(Value::Bool(true));
                    },
                    Instr::False => {
                        self.stack.push(Value::Bool(false));
                    },
                    Instr::Equal => {
                        let b = self.stack.pop().unwrap_or(Value::Nil);
                        let a = self.stack.pop().unwrap_or(Value::Nil);

                        self.stack.push(Value::Bool(a == b)); // PartialEq for Value
                    },
                    Instr::Greater => {
                        let b = self.stack.pop().unwrap_or(Value::Nil);
                        let a = self.stack.pop().unwrap_or(Value::Nil);

                        self.stack.push(Value::Bool(a > b)); // PartialOrd for Value
                    },
                    Instr::Less => {
                        let b = self.stack.pop().unwrap_or(Value::Nil);
                        let a = self.stack.pop().unwrap_or(Value::Nil);

                        self.stack.push(Value::Bool(a < b)); // PartialOrd for Value
                    },

                    // TODO: macros for here
                    Instr::Add => {
                        let b = self.stack.pop().unwrap_or(Value::Nil);
                        let a = self.stack.pop().unwrap_or(Value::Nil);
                        match a.checked_add(b) {
                            Ok(val) => self.stack.push(val),
                            _ => return Err(InterpretError::RuntimeError) // TODO
                        };
                    },
                    Instr::Subtract => {
                        let b = self.stack.pop().unwrap_or(Value::Nil);
                        let a = self.stack.pop().unwrap_or(Value::Nil);
                        match a.checked_sub(b) {
                            Ok(val) => self.stack.push(val),
                            _ => return Err(InterpretError::RuntimeError) // TODO
                        };
                    },
                    Instr::Multiply => {
                        let b = self.stack.pop().unwrap_or(Value::Nil);
                        let a = self.stack.pop().unwrap_or(Value::Nil);
                        match a.checked_mul(b) {
                            Ok(val) => self.stack.push(val),
                            _ => return Err(InterpretError::RuntimeError) // TODO
                        };
                    },
                    Instr::Divide => {
                        let b = self.stack.pop().unwrap_or(Value::Nil);
                        let a = self.stack.pop().unwrap_or(Value::Nil);
                        match a.checked_div(b) {
                            Ok(val) => self.stack.push(val),
                            _ => return Err(InterpretError::RuntimeError) // TODO
                        };
                    },
                    Instr::Negate => {
                        let a = self.stack.pop().unwrap_or(Value::Nil);
                        match a.checked_neg() {
                            Ok(val) => self.stack.push(val),
                            _ => return Err(InterpretError::RuntimeError) // TODO
                        }
                    }
                    
                    Instr::Return => {
                        let val = self.stack.pop().unwrap_or(Value::Nil);
                        println!("RESULT: {}", val);
                        return Ok(());
                    },
                },
                // InstrResult::BadOp { bytes } => {},
                _ => {
                    return Err(InterpretError::RuntimeError);
                }
            }
        }
    }
}

pub enum InterpretError {
    CompileError,
    RuntimeError,
}
type InterpretResult = Result<(), InterpretError>;

// fn interpret(chunk: &mut Chunk) -> InterpretResult {
//     let mut vm = VM::new(chunk);
//     vm.run()
// }