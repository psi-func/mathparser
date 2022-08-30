use std::iter::Peekable;
use std::str::{self, Chars};

use super::token::Token;

pub struct Tokenizer<'c> {
    expr: Peekable<Chars<'c>>,
}

impl<'c> Tokenizer<'c> {
    pub fn new(new_expr: &'c str) -> Self {
        Tokenizer {
            expr: new_expr.chars().peekable(),
        }
    }
}

impl<'c> Iterator for Tokenizer<'c> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        let next_char = self.expr.next();

        match next_char {
            Some('0'..='9') => {
                let mut number = next_char?.to_string();

                while let Some(next_char) = self.expr.peek() {
                    if next_char.is_numeric() || next_char == &'.' {
                        number.push(self.expr.next()?);
                    }
                    else if next_char == &'{' {
                        return None;
                    }
                    else {
                        break;
                    }
                }
                Some(Token::Num(number.parse::<f64>().unwrap()))            
            },
            Some('+') => Some(Token::Add),
            Some('-') => Some(Token::Subtract),
            Some('*') => Some(Token::Multiply),
            Some('/') => Some(Token::Divide),
            Some('^') => Some(Token::Caret),
            Some('(') => Some(Token::LeftParen),
            Some(')') => Some(Token::RightParen),
            None => Some(Token::EOF),
            Some(_) => None,
        }
    }

}
