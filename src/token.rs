use std::fmt;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum TokenType {
    LParen, RParen,
    LBrace, RBrace,
    Comma, Dot, Minus, Plus,
    Semicolon, Slash, Star,

    Bang, BangEq,
    Eq, EqEq,
    Gt, GtEq,
    Lt, LtEq,

    String, Number, Ident,

    And, Else, False,
    For, Fun, If, Nil, Or,
    Print, Return, True, Var, While,
    // Class, Super, This,
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl TokenType {
    pub fn identify(lex: &[char]) -> Self {
        let lex_str: &str = &lex.iter().collect::<String>();

        // todo: implement automaton match
        // here keywords are few enough to brute-force
        match lex_str {
            "and" => Self::And,
            // "class" => Self::Class,
            "else" => Self::Else,
            "false" => Self::False,
            "for" => Self::For,
            "fun" => Self::Fun,
            "if" => Self::If,
            "nil" => Self::Nil,
            "or" => Self::Or,
            "print" => Self::Print,
            "return" => Self::Return,
            // "super" => Self::Super,
            // "this" => Self::This,
            "true" => Self::True,
            "var" => Self::Var,
            "while" => Self::While,
            _ => Self::Ident,
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Token {
    typ: TokenType,
    lexeme: String, // to reflect clox better, &'a str should be used... but this is more rust-ish and we're using UTF-8 anyway
    line: usize,
}
impl Token {
    pub fn new(typ: TokenType, lexeme: String, line: usize) -> Self {
        Token { typ, lexeme, line }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Handler {
    Error {
        message: &'static str,
        line: usize,
    },
    EOF
}
impl Handler {
    pub fn error(message: &'static str, line: usize) -> Self {
        Self::Error { message, line }
    }
    pub fn eof() -> Self {
        Self::EOF
    }
}

pub type TokenResult = Result<Token, Handler>;