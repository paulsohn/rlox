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
                println!("    {:?}", self.stack);

                // if self.ip should be a pointer, change this also
                self.chunk.disasm(&ires, self.ip)
            }
            self.ip += len; // instr ptr proceeds

            match ires {
                InstrResult::Good(instr) => match instr {
                    Instr::Return => {
                        let val = self.stack.pop().unwrap(); // unwrap_or(Value::Nil)
                        println!("{}", val);
                        return Ok(());
                    },
                    Instr::Constant { idx } => {
                        let val = self.chunk.get_const(idx);
                        self.stack.push(val);
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