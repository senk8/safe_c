/* TODO: Would like to enable this to be executed web browser with Wasm */



use std::str::from_utf8;

use crate::types::annotation::*;
use crate::types::token::DelimitorKind::*;
use crate::types::token::KeywordKind::*;
use crate::types::token::OperatorKind::*;
use crate::types::token::TokenKind::*;
use crate::types::token::*;

use wasm_bindgen::prelude::*;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Default, Hash)]
#[wasm_bindgen]
pub struct Lexer<'a> {
    /* it is only used by error_at */
    txt: &'a [u8],

    /* Cursor */
    pos: usize,
}

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(lexer:Lexer<'a>) {
    alert(lexer.next());
}


impl<'a> Iterator for Lexer<'a> {
    type Item = Token;
    fn next(&mut self) -> Option<Token> {
        // Consumes as long as a '\n' or ' ' remains.
        loop {
            match *self.txt.get(self.pos)?{
                b' ' | b'\n' => self.pos += 1,
                _ => break,
            }
        }
        match self.txt[self.pos..] {
            [b'r', b'e', b't', b'u', b'r', b'n', b' ', ..] => self.lex_token(Key(Return), 6),
            [b'i', b'f', b'(', ..] => self.lex_token(Key(If), 2),
            [b'w', b'h', b'i', b'l', b'e',b'(', ..] => self.lex_token(Key(While), 5),
            [b'e', b'l', b's', b'e', b' ', ..] => self.lex_token(Key(Else), 4),
            [b'f', b'o', b'r', b'(',..] => self.lex_token(Key(For), 3),
            [b'<', b'=', ..] => self.lex_token(Opr(Leq), 2),
            [b'>', b'=', ..] => self.lex_token(Opr(Geq), 2),
            [b'=', b'=', ..] => self.lex_token(Opr(Eq), 2),
            [b'!', b'=', ..] => self.lex_token(Opr(Neq), 2),
            [b'+', ..] => self.lex_token(Opr(Add), 1),
            [b'-', ..] => self.lex_token(Opr(Sub), 1),
            [b'*', ..] => self.lex_token(Opr(Mul), 1),
            [b'/', ..] => self.lex_token(Opr(Div), 1),
            [b'<', ..] => self.lex_token(Opr(Lt), 1),
            [b'>', ..] => self.lex_token(Opr(Gt), 1),
            [b'=', ..] => self.lex_token(Opr(Assign), 1),
            [b';', ..] => self.lex_token(Delim(Semicolon), 1),
            [b'(', ..] => self.lex_token(Delim(Lc), 1),
            [b')', ..] => self.lex_token(Delim(Rc), 1),
            [b'{', ..] => self.lex_token(Delim(LCurl), 1),
            [b'}', ..] => self.lex_token(Delim(RCurl), 1),
            [b'0'..=b'9', ..] => self.lex_num(),
            [b'a'..=b'z', ..] => self.lex_ident(),
            _ => panic!(self.error_at("unexpected token")),
        }
    }
}

#[wasm_bindgen]
impl<'a> Lexer<'a> {
    pub fn new(input: &'a [u8]) -> Lexer<'a> {
        let txt = input;
        let pos = 0;
        Lexer { txt, pos }
    }
}

impl<'a> Lexer<'a> {
 
    //文字列の最初を取り除く
    fn consume(&mut self, n: usize) -> Option<()> {
        if self.pos + n <= self.txt.len() {
            self.pos += n;
            Some(())
        } else {
            None
        }
    }

    fn lex_token(&mut self, val: TokenKind, len: usize) -> Option<Token> {
        let pos = Pos(1,self.pos);
        self.consume(len)?;
        Some((val, pos))
    }

    //文字列を数字である限り消費する。
    fn lex_num(&mut self) -> Option<Token> {
        let begin = self.pos;

        while self.pos < self.txt.len() && self.txt[self.pos].is_ascii_digit() {
            self.pos += 1;
        }

        let pos = Pos(begin, self.pos);

        /* TODO : TokenizeError isn't used  */
        let val = Num(from_utf8(&self.txt[begin..self.pos])
            .map(|s| usize::from_str_radix(s, 10))
            .unwrap()
            .unwrap());

        Some((val, pos))
    }

    //文字列をアルファベットである限り消費する。
    fn lex_ident(&mut self) -> Option<Token> {
        let begin = self.pos;

        while self.pos < self.txt.len() && self.txt[self.pos].is_ascii_alphabetic() {
            self.pos += 1;
        }

        let pos = Pos(begin, self.pos);

        /* TODO : TokenizeError isn't used  */
        let val = Id(from_utf8(&self.txt[begin..self.pos])
            .map(|s| String::from(s))
            .unwrap());

        Some((val, pos))
    }
}

impl<'a> Lexer<'a> {
    fn error_at(&self, description: &str) -> String {
        let pos = self.pos;
        let mut message = format!("\n{}\n", from_utf8(self.txt).unwrap());
        message.push_str(&format!("{:>width$}", "^", width = pos + 1));
        message.push_str(&format!("\n{}", description));
        return message;
    }
}
