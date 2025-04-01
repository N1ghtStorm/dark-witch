// MIT License
//
// Copyright (c) 2025
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

// MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM
// MMMMMMMMMMMMds+:--------:+sdNMMMMMMMMMMM
// MMMMMMMMms:-+sdNMMMMMMMMNdy+--omMMMMMMMM
// MMMMMMh:` /mMMMMMMMMMMMMMMMMm+ `-yMMMMMM
// MMMMd--hN``--sNMMMMMMMMMMNy:..`md:.hMMMM
// MMM+`yMMMy hd+./hMMMMMMh/.+dd sMMMh`/MMM
// MM:.mMMMMM:.NMMh/.+dd+./hMMM--MMMMMm--NM
// M+`mMMMMMMN`+MMMMm-  .dMMMMo mMMMMMMN.:M
// d yMMMMMMMMy dNy:.omNs--sNm oMMMMMMMMh h
// /`MMMMMMMMMM.`.+dMMMMMMm+.``NMMMMMMMMM-:
// .:MMMMMMMd+./`oMMMMMMMMMMs /.+dMMMMMMM/`
// .:MMMMmo.:yNMs dMMMMMMMMm`oMNy:.omMMMM/`
// /`MNy:.omMMMMM--MMMMMMMM:.MMMMMNs--sNM.:
// d -` :++++++++: /++++++/ :++++++++:  : h
// M+ yddddddddddd+ yddddy /dddddddddddy`/M
// MM/.mMMMMMMMMMMM.-MMMM/.NMMMMMMMMMMm.:NM
// MMMo`sMMMMMMMMMMd sMMy hMMMMMMMMMMy`+MMM
// MMMMd--hMMMMMMMMM+`mN`/MMMMMMMMMh--hMMMM
// MMMMMMh:.omMMMMMMN.:/`NMMMMMMms.:hMMMMMM
// MMMMMMMMNs:./shmMMh  yMMNds/.:smMMMMMMMM
// MMMMMMMMMMMMdy+/---``---:+sdMMMMMMMMMMMM
// MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM

// Lexer: Converts raw Cypher into tokens

use std::str::Chars;
use std::iter::Peekable;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    // Keywords
    Match,
    Return,
    As,
    Where,
    
    // Symbols
    LeftParen,    // (
    RightParen,   // )
    LeftBrace,    // {
    RightBrace,   // }
    Colon,        // :
    Comma,        // ,
    Dot,          // .
    
    // Values
    Identifier(String),
    String(String),
    Number(f64),
    
    // End of input
    EOF,
}

pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
    current: char,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut chars = input.chars().peekable();
        let current = chars.next().unwrap_or('\0');
        Lexer { input: chars, current }
    }

    fn advance(&mut self) {
        self.current = self.input.next().unwrap_or('\0');
    }

    fn peek(&mut self) -> char {
        *self.input.peek().unwrap_or(&'\0')
    }

    fn skip_whitespace(&mut self) {
        while self.current.is_whitespace() {
            self.advance();
        }
    }

    fn read_identifier(&mut self) -> String {
        let mut identifier = String::new();
        while self.current.is_alphanumeric() || self.current == '_' {
            identifier.push(self.current);
            self.advance();
        }
        identifier
    }

    fn read_string(&mut self) -> String {
        let mut string = String::new();
        self.advance(); // Skip opening quote
        while self.current != '\'' && self.current != '\0' {
            string.push(self.current);
            self.advance();
        }
        self.advance(); // Skip closing quote
        string
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let token = match self.current {
            '\0' => Token::EOF,
            '(' => {
                self.advance();
                Token::LeftParen
            }
            ')' => {
                self.advance();
                Token::RightParen
            }
            '{' => {
                self.advance();
                Token::LeftBrace
            }
            '}' => {
                self.advance();
                Token::RightBrace
            }
            ':' => {
                self.advance();
                Token::Colon
            }
            ',' => {
                self.advance();
                Token::Comma
            }
            '.' => {
                self.advance();
                Token::Dot
            }
            '\'' => Token::String(self.read_string()),
            c if c.is_alphabetic() => {
                let ident = self.read_identifier();
                match ident.to_uppercase().as_str() {
                    "MATCH" => Token::Match,
                    "RETURN" => Token::Return,
                    "AS" => Token::As,
                    "WHERE" => Token::Where,
                    _ => Token::Identifier(ident),
                }
            }
            _ => {
                self.advance();
                Token::EOF
            }
        };

        token
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer() {
        let input = "MATCH (keanu:Person {name:'Keanu Reeves'}) RETURN keanu.name AS name";
        let mut lexer = Lexer::new(input);
        
        let expected_tokens = vec![
            Token::Match,
            Token::LeftParen,
            Token::Identifier("keanu".to_string()),
            Token::Colon,
            Token::Identifier("Person".to_string()),
            Token::LeftBrace,
            Token::Identifier("name".to_string()),
            Token::Colon,
            Token::String("Keanu Reeves".to_string()),
            Token::RightBrace,
            Token::RightParen,
            Token::Return,
            Token::Identifier("keanu".to_string()),
            Token::Dot,
            Token::Identifier("name".to_string()),
            Token::As,
            Token::Identifier("name".to_string()),
            Token::EOF,
        ];

        for expected in expected_tokens {
            let token = lexer.next_token();
            assert_eq!(token, expected);
        }
    }
}
