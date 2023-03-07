use crate::op::OpCode;
use crate::value::Value;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Code {
    byte: u8,
    line: usize,
}

impl Code {
    pub fn new(byte: u8, line: usize) -> Code {
        Code { byte, line }
    }
}


pub struct Chunk {
    code: Vec<Code>,
    consts: Vec<Value>,
}

impl Chunk {
    pub fn new() -> Chunk {
        Chunk { code: vec![], consts: vec![] }
    }
    pub fn write(&mut self, byte: u8, line: usize) {
        self.code.push(Code::new(byte, line));
    }
    pub fn add_const(&mut self, value: Value) -> u8 {
        let l = self.consts.len();
        self.consts.push(value);
        
        // return the index where the const was appended. should be byte-sized
        u8::try_from(l)
            .expect("const pool size should not exceed 256")
    }
    pub fn disasm_all(&self, name: &str) {
        println!("== {} ==", name);

        for (offset, op, slice) in ChunkIterator::new(self) { // @TODO: slice should be &[(u8, usize)] or something..
            
            let line = if offset > 0 && self.code[offset].line == self.code[offset - 1].line {
                String::from("   |")
            } else {
                format!("{:4}", self.code[offset].line)
            };

            match op {
                OpCode::RETURN => {
                    println!("{:04} {} {:?}", offset, line, op);
                },
                OpCode::CONSTANT => {
                    let const_offset = usize::from(slice[1].byte); // u8 -> usize : castable
                    println!("{:04} {} {:?} {} '{}'", offset, line, op, const_offset, self.consts[const_offset]);
                },
                OpCode::UNKNOWN(x) => {
                    println!("{:04} {} Unknown opcode {}", offset, line, x);
                },
            }
        }
    }
}

pub struct ChunkIterator<'a> {
    chunk: &'a Chunk,
    offset: usize,
}

impl<'a> ChunkIterator<'a> {
    fn new(chunk: &'a Chunk) -> ChunkIterator<'a> {
        ChunkIterator { chunk, offset: 0 }
    }
}

impl<'a> Iterator for ChunkIterator<'a> {
    type Item = (usize, OpCode, &'a [Code]);
    fn next(&mut self) -> Option<Self::Item> {

        if let Some(&code) = self.chunk.code.get(self.offset) {
            let op = OpCode::from(code.byte);

            let prev_offset = self.offset;

            let slice = match op {
                OpCode::RETURN => {
                    self.offset += 1;
                    self.chunk.code.get(prev_offset..self.offset)

                    // output "<offset> : OP_RETURN"
                },
                OpCode::CONSTANT => {
                    self.offset += 2;
                    self.chunk.code.get(prev_offset..self.offset)
                }
                OpCode::UNKNOWN(_x) => {
                    self.offset += 1;
                    self.chunk.code.get(prev_offset..self.offset)

                    // output "<offset> : unknown opcode <x> at <b>"
                },
            };
            slice.map(|s| (prev_offset, op, s))
        } else { None }
    }
}