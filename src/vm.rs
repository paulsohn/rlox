use crate::chunk::Chunk;
use crate::value::Value;
use crate::instr::Instr;
use crate::compiler::compile;

pub struct VM {
    chunk: Chunk,
    ip: usize, // original clox uses pointer ip. here we only use code index of the chunk
    stack: Vec<Value>,
}

impl VM {
    pub fn new(chunk: Chunk) -> Self {
        VM {
            chunk,
            ip: 0,
            stack: Vec::new()
        }
    }

    fn stack_push<V>(&mut self, val: V) where V: Into<Value> {
        self.stack.push(val.into());
    }

    fn stack_pop(&mut self) -> Value {
        self.stack.pop().unwrap_or(Value::Nil)
    }

    /// run the instruction.
    fn run(&mut self) -> InterpretResult {
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

            if let Ok(instr) = ires {
                match instr {
                    Instr::Constant { idx } => {
                        let val = self.chunk.get_const(idx);
                        self.stack_push(val);
                    },
                    Instr::Nil => {
                        self.stack_push(());
                    },
                    Instr::True => {
                        self.stack_push(true);
                    },
                    Instr::False => {
                        self.stack_push(false);
                    },
                    Instr::Equal => {
                        let b = self.stack_pop();
                        let a = self.stack_pop();

                        self.stack_push(a == b); // PartialEq for Value
                    },
                    Instr::Greater => {
                        let b = self.stack_pop();
                        let a = self.stack_pop();

                        self.stack_push(a > b); // PartialOrd for Value
                    },
                    Instr::Less => {
                        let b = self.stack_pop();
                        let a = self.stack_pop();

                        self.stack_push(a < b); // PartialOrd for Value
                    },

                    // TODO: macros for here
                    Instr::Add => {
                        let b = self.stack_pop();
                        let a = self.stack_pop();
                        match a.checked_add(b) {
                            Ok(val) => self.stack_push(val),
                            _ => return Err(InterpretError::RuntimeError) // TODO
                        };
                    },
                    Instr::Subtract => {
                        let b = self.stack_pop();
                        let a = self.stack_pop();
                        match a.checked_sub(b) {
                            Ok(val) => self.stack_push(val),
                            _ => return Err(InterpretError::RuntimeError) // TODO
                        };
                    },
                    Instr::Multiply => {
                        let b = self.stack_pop();
                        let a = self.stack_pop();
                        match a.checked_mul(b) {
                            Ok(val) => self.stack_push(val),
                            _ => return Err(InterpretError::RuntimeError) // TODO
                        };
                    },
                    Instr::Divide => {
                        let b = self.stack_pop();
                        let a = self.stack_pop();
                        match a.checked_div(b) {
                            Ok(val) => self.stack_push(val),
                            _ => return Err(InterpretError::RuntimeError) // TODO
                        };
                    },
                    Instr::Negate => {
                        let a = self.stack_pop();
                        match a.checked_neg() {
                            Ok(val) => self.stack_push(val),
                            _ => return Err(InterpretError::RuntimeError) // TODO
                        }
                    }
                    
                    Instr::Return => {
                        let val = self.stack_pop();
                        println!("RESULT: {}", val);
                        return Ok(());
                    },
                }
            } else { // else if let ?
                return Err(InterpretError::RuntimeError);
            }
        }
    }

    /// mere combination of `compiler::compile` and `vm::run`.
    /// compile source code `src` and run it immediately.
    pub fn interpret(&mut self, src: &str) -> InterpretResult {
        if let Some(chunk) = compile(src) {
            self.chunk = chunk;
            self.run()
        } else { Err(InterpretError::CompileError) }
    }
}

pub enum InterpretError {
    CompileError,
    RuntimeError,
}
pub type InterpretResult = Result<(), InterpretError>;

// fn interpret(chunk: &mut Chunk) -> InterpretResult {
//     let mut vm = VM::new(chunk);
//     vm.run()
// }