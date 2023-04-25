use crate::instr::{ InstrResult };
use crate::value::Value;

use std::cmp::Ordering;
use std::slice;

pub struct Chunk {
    code: Vec<u8>,
    line_begins: Vec<usize>,
    consts: Vec<Value>,
}

impl Chunk {
    pub fn new() -> Chunk {
        Chunk { code: vec![], line_begins: vec![0], consts: vec![] }
    }

    pub fn iter(&self) -> CodeIterator {
        CodeIterator::new(&self.code)
    }

    pub fn write(&mut self, byte: u8, line: usize) {
        let cur_len = self.line_begins.len();
        let target_len = line + 1;

        match cur_len.cmp(&target_len) {
            Ordering::Equal => {}, // do nothing.
            Ordering::Less => {
                // currently byte num is `self.code.len()`.
                let curr_bytes = self.code.len();
                
                self.line_begins.resize(target_len, curr_bytes);

                // now `self.line_begins.get( (prev_len)..=line )` are equal to curr_bytes
            },
            Ordering::Greater => {
                panic!("Line number should be monotonically increasing. Current: Line {}, Incoming: Line {}.", cur_len.wrapping_sub(1), line);
            },
        };

        self.code.push(byte);
    }
    pub fn add_const(&mut self, value: Value) -> u8 {
        let l = self.consts.len();
        self.consts.push(value);
        
        // return the index where the const was appended. should be byte-sized
        u8::try_from(l)
            .expect("const pool size should not exceed 256")
    }


    pub fn disasm_all(&self, name: &str) {
        println!("=== {} ===", name);

        let mut prev_line_no = usize::MAX;

        for (res, offset) in self.iter() {
            let line_no = self.line_begins.partition_point(|&x| x <= offset).wrapping_sub(1);
            
            let line = if prev_line_no == line_no {
                String::from("   |")
            } else {
                prev_line_no = line_no;
                format!("{:4}", line_no)
            };

            println!("{:04} {} {}", offset, line, res.with_context(&self.consts));
        }
    }
}


pub struct CodeIterator<'a> {
    iter: slice::Iter<'a, u8>,
    offset: usize,
}

impl<'a> CodeIterator<'a> {
    fn new(code: &'a [u8]) -> Self {
        Self { iter: code.iter(), offset: 0 }
    }
}

impl<'a> Iterator for CodeIterator<'a> {
    type Item = (InstrResult, usize);
    fn next(&mut self) -> Option<Self::Item> {
        let prev_offset = self.offset;
        
        let (res, len) = InstrResult::next_instr_point(&mut self.iter)?;
        self.offset += len;

        Some((res, prev_offset))
    }
}