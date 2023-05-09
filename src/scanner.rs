use std::str::Chars;
// use std::iter::Peekable;
use peekmore::{PeekMore, PeekMoreIterator};

use crate::token::{ TokenType, Token, Handler, TokenResult };

macro_rules! patt {
    // modified https://doc.rust-lang.org/src/core/macros/mod.rs.html#342
    ( $(|)? $( $pattern:pat_param )|+ $( if $guard: expr )? $(,)?) => {
        |ch: &char| match *ch {
            $( $pattern )|+ $( if $guard )? => true,
            _ => false
        }
    };
}

pub struct Scanner<'a> {
    chars: PeekMoreIterator<Chars<'a>>,
    line: usize, // 0 if EOF token has been emitted.
    lex: Vec<char>,
}

impl<'a> Scanner<'a> {
    pub fn from_source(src: &'a str) -> Self{
        Scanner {
            chars: src.chars().peekmore(),
            line: 1,
            lex: vec![]
        }
    }

    /// advance `self.chars` and returns the character.
    /// this is a wrapper for `self.chars.next()` with line incrementing and stack pushing.
    fn advance(&mut self) -> Option<char> {
        let result = self.chars.next();
        if let Some(c) = result {
            if c == '\n' {
                self.line += 1;
            }
            self.lex.push(c);
        }
        result
    }

    /// Advance `self.chars` if the following character satisfies the condition `func`.
    /// Returns the matched character.
    fn advance_if(&mut self, func: impl FnOnce(&char) -> bool) -> Option<char> {
        // let result = self.chars.next_if(func);
        // if let Some(c) = result {
        //     if c == '\n' {
        //         self.line += 1;
        //     }
        //     self.lex.push(c);
        // }
        // result

        let result = self.chars.peek();
        if let Some(&c) = result {
            if func(&c) {
                self.chars.next(); // advance
                if c == '\n' {
                    self.line += 1;
                }
                self.lex.push(c);
                return Some(c);
            }
        }
        None
    }

    /// Advance `self.chars` if the following character matches to any candidates in `cand`.
    /// Returns the matched character.
    /// This is a shortcut of `self.advance_if`.
    #[inline]
    fn advance_on(&mut self, cand: &[char]) -> Option<char> {
        self.advance_if(|ch| cand.contains(ch))
    }

    /// Keep advancing `self.chars` as long as the following character satisfies the condition `func`.
    #[inline]
    fn advance_while(&mut self, func: impl Fn(&char) -> bool) {
        while self.advance_if(&func).is_some() { }
    }

    /// Advance `self.chars` if the prefix exactly matches with `expected`.
    /// returns whether the match succeeded.
    /// **Note:** this method assumes that `expected` doesn't contain any newline characters.
    fn advance_on_exact(&mut self, expected: &[char]) -> bool {
        let expected_view: Vec<Option<char>> = expected.iter().map(|ch| Some(*ch)).collect();

        if self.chars.peek_range(0, expected.len()) != &expected_view[..] {
            return false;
        }

        let newline_cnt = expected.iter().filter(|&ch| *ch == '\n').count();

        self.line += newline_cnt;
        self.lex.extend_from_slice(expected);
        self.chars.truncate_iterator_to_cursor();

        true
    }

    /// advance `self.chars` until it reached to `target`.
    /// if found, consume the target also.
    /// returns whether the target has been found.
    fn advance_until(&mut self, target: char) -> bool {
        self.advance_while(|ch| *ch != target);

        // at this point, either target was found or reached the end.
        self.advance().map(|c| c == target).unwrap_or(false)
    }

    fn skip_whitespace(&mut self) {
        // skip whitespaces until comments
        // while let Some(c) = self.advance_on(&[' ', '\r', '\t', '\n']) {
        while let Some(_) = self.advance_if(patt!(' ' | '\r' | '\t' | '\n')) {}

        // skip // comments
        if self.advance_on_exact(&['/', '/']) {
            self.advance_until('\n');
        }

        // skip /* comments */
        if self.advance_on_exact(&['/', '*']) {
            loop {
                // no * found until the end of file
                if !self.advance_until('*') {
                    break;
                }

                // */ found
                if let Some(_) = self.advance_on(&['/']) {
                    break;
                }
                // if it loops, then single * is found.
                // keep looping
            }
        }

        match self.chars.peek() {
            Some(' ' | '\r' | '\t' | '\n' | '/') => {
                // loop more to skip further whitespaces (and comments)
                self.skip_whitespace();
            },
            _ => ()
        };

        self.lex.clear();
    }

    fn make_token(&mut self, typ: TokenType) -> Option<TokenResult> {
        let token = Token::new(
            typ,
            self.lex.iter().collect(),
            self.line,
        );

        self.lex.clear();

        Some(Ok(token))
    }

    fn make_error(&mut self, message: &'static str) -> Option<TokenResult> {
        Some(Err(
            Handler::error(
                message,
                self.line
            )
        ))
    }
}

impl<'a> Iterator for Scanner<'a> {
    type Item = TokenResult;
    fn next(&mut self) -> Option<Self::Item> {
        self.skip_whitespace();

        if let Some(c) = self.advance() {
            // do something
            match c {
                '(' => self.make_token(TokenType::LParen),
                ')' => self.make_token(TokenType::RParen),
                '{' => self.make_token(TokenType::LBrace),
                '}' => self.make_token(TokenType::RBrace),
                ';' => self.make_token(TokenType::Semicolon),
                ',' => self.make_token(TokenType::Comma),
                '.' => self.make_token(TokenType::Dot),
                '-' => self.make_token(TokenType::Minus),
                '+' => self.make_token(TokenType::Plus),
                '*' => self.make_token(TokenType::Star),
                '/' => self.make_token(TokenType::Slash),

                '!' => match self.advance_on(&['=']) {
                    Some(_) => self.make_token(TokenType::BangEq),
                    None => self.make_token(TokenType::Bang),
                },
                '=' => match self.advance_on(&['=']) {
                    Some(_) => self.make_token(TokenType::EqEq),
                    None => self.make_token(TokenType::Eq),
                },
                '<' => match self.advance_on(&['=']) {
                    Some(_) => self.make_token(TokenType::LtEq),
                    None => self.make_token(TokenType::Lt),
                },
                '>' => match self.advance_on(&['=']) {
                    Some(_) => self.make_token(TokenType::GtEq),
                    None => self.make_token(TokenType::Gt),
                },

                // string literal
                '"' => match self.advance_until('"') {
                    true => self.make_token(TokenType::String), // this will contain wrapping ""
                    false => self.make_error("Unterminated string")
                },
                // number literal
                '0'..='9' => {
                    self.advance_while(patt!('0'..='9'));
                    if self.advance_on(&['.']).is_some() {
                        // unlike original lox spec, `123.` is a valid literal. 
                        // todo
                        self.advance_while(patt!('0'..='9'));
                    }
                    self.make_token(TokenType::Number)
                },
                // identifiers and keywords
                'a'..='z' | 'A'..='Z' | '_' => {
                    self.advance_while(patt!('0'..='9' | 'a'..='z' | 'A'..='Z' | '_'));

                    let typ = TokenType::identify(&self.lex);
                    self.make_token(typ)
                },

                _ => self.make_error("Unexpected character"),
            }
        } else if self.line > 0 {
            self.line = 0; // next token will be None.
            Some(Err(Handler::EOF))
        } else { None }
    }
}