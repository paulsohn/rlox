use crate::scanner::Scanner;
use crate::chunk::Chunk;
use crate::token::{Token, TokenResult};

pub fn compile(src: &str) -> Option<Chunk>{
    let chunk = Chunk::new();

    let scanner = Scanner::from_source(src);
    let parser = Parser::from_scanner(scanner);

    // parser.advance();
    // parser.expression();
    // parser.consume(Err(TokenResult::EOF), "Expect end of expression.");

    Some(chunk)
}

pub struct Parser {
    cur: TokenResult,
    prev: TokenResult,
    had_error: bool,
    panic_mode: bool,
}

impl Parser {
    pub fn from_scanner(scanner: Scanner) -> Self {
        todo!()
    }
}


// spoiler alert : we use single-pass compilation
// (skipping generating AST and generate code directly from token)
// this is an old-school approach because computerse literally didn't have enough memory to store an entire source file's AST
// Nystrom do this because it keeps our compiler simpler.

// single-pass compilers don't work well for all languages
// fortunately, tiny, dynamically typed Lox is well-suited to that (He did design the language specifically for this book)