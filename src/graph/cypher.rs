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

use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    // Keywords
    Match,
    Return,
    As,
    Where,

    // Symbols
    LeftParen,  // (
    RightParen, // )
    LeftBrace,  // {
    RightBrace, // }
    Colon,      // :
    Comma,      // ,
    Dot,        // .

    // Values
    Identifier(String),
    String(String),
    Number(f64),

    // End of input
    EOF,
}

pub struct Lexer {
    input: Vec<char>,
    position: usize,
    current: char,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        let chars: Vec<char> = input.chars().collect();
        let current = chars.first().copied().unwrap_or('\0');
        Lexer {
            input: chars,
            position: 0,
            current,
        }
    }

    fn advance(&mut self) {
        self.position += 1;
        self.current = self.input.get(self.position).copied().unwrap_or('\0');
    }

    fn peek(&self) -> char {
        self.input.get(self.position + 1).copied().unwrap_or('\0')
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

// AST Structures
#[derive(Debug, PartialEq, Clone)]
pub struct Query {
    pub match_clause: MatchClause,
    pub return_clause: ReturnClause,
}

#[derive(Debug, PartialEq, Clone)]
pub struct MatchClause {
    pub patterns: Vec<Pattern>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Pattern {
    pub variable: String,
    pub labels: Vec<String>,
    pub properties: Vec<Property>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Property {
    pub key: String,
    pub value: Value,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    String(String),
    Number(f64),
    Boolean(bool),
}

#[derive(Debug, PartialEq, Clone)]
pub struct ReturnClause {
    pub items: Vec<ReturnItem>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ReturnItem {
    pub expression: Expression,
    pub alias: Option<String>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    Property {
        variable: String,
        property: String,
    },
    Variable(String),
}

// Parser Implementation
pub struct Parser {
    lexer: Lexer,
    current_token: Token,
}

impl Parser {
    pub fn new(input: &str) -> Self {
        let mut lexer = Lexer::new(input);
        let current_token = lexer.next_token();
        Parser {
            lexer,
            current_token,
        }
    }

    fn advance(&mut self) {
        self.current_token = self.lexer.next_token();
    }

    fn expect_token(&mut self, expected: Token) -> Result<(), String> {
        if self.current_token == expected {
            self.advance();
            Ok(())
        } else {
            Err(format!(
                "Expected {:?}, got {:?}",
                expected, self.current_token
            ))
        }
    }

    pub fn parse_query(&mut self) -> Result<Query, String> {
        let match_clause = self.parse_match_clause()?;
        let return_clause = self.parse_return_clause()?;

        Ok(Query {
            match_clause,
            return_clause,
        })
    }

    fn parse_match_clause(&mut self) -> Result<MatchClause, String> {
        self.expect_token(Token::Match)?;
        
        let mut patterns = Vec::new();
        patterns.push(self.parse_pattern()?);
        
        while self.current_token == Token::Comma {
            self.advance();
            patterns.push(self.parse_pattern()?);
        }

        Ok(MatchClause { patterns })
    }

    fn parse_pattern(&mut self) -> Result<Pattern, String> {
        self.expect_token(Token::LeftParen)?;
        
        let variable = match &self.current_token {
            Token::Identifier(name) => {
                let var = name.clone();
                self.advance();
                var
            }
            _ => return Err("Expected identifier".to_string()),
        };

        let mut labels = Vec::new();
        let mut properties = Vec::new();

        // Parse labels
        while self.current_token == Token::Colon {
            self.advance();
            if let Token::Identifier(label) = &self.current_token {
                labels.push(label.clone());
                self.advance();
            }
        }

        // Parse properties
        if self.current_token == Token::LeftBrace {
            self.advance();
            while self.current_token != Token::RightBrace {
                properties.push(self.parse_property()?);
                if self.current_token == Token::Comma {
                    self.advance();
                }
            }
            self.advance(); // consume RightBrace
        }

        self.expect_token(Token::RightParen)?;

        Ok(Pattern {
            variable,
            labels,
            properties,
        })
    }

    fn parse_property(&mut self) -> Result<Property, String> {
        let key = match &self.current_token {
            Token::Identifier(name) => {
                let k = name.clone();
                self.advance();
                k
            }
            _ => return Err("Expected property key".to_string()),
        };

        self.expect_token(Token::Colon)?;

        let value = {
            let token = &self.current_token;
            let val = match token {
                Token::String(s) => Value::String(s.clone()),
                Token::Number(n) => Value::Number(*n),
                _ => return Err("Expected property value".to_string()),
            };
            self.advance();
            val
        };


        Ok(Property { key, value })
    }

    fn parse_return_clause(&mut self) -> Result<ReturnClause, String> {
        self.expect_token(Token::Return)?;

        let mut items = Vec::new();
        loop {
            items.push(self.parse_return_item()?);
            
            if self.current_token != Token::Comma {
                break;
            }
            self.advance();
        }

        Ok(ReturnClause { items })
    }

    fn parse_return_item(&mut self) -> Result<ReturnItem, String> {
        let expression = self.parse_expression()?;
        
        let alias = if self.current_token == Token::As {
            self.advance();
            if let Token::Identifier(name) = &self.current_token {
                let alias = Some(name.clone());
                self.advance();
                alias
            } else {
                return Err("Expected identifier after AS".to_string());
            }
        } else {
            None
        };

        Ok(ReturnItem { expression, alias })
    }

    fn parse_expression(&mut self) -> Result<Expression, String> {
        let variable = match &self.current_token {
            Token::Identifier(name) => {
                let var = name.clone();
                self.advance();
                var
            }
            _ => return Err("Expected identifier".to_string()),
        };

        if self.current_token == Token::Dot {
            self.advance();
            if let Token::Identifier(property) = &self.current_token {
                let prop = property.clone();
                self.advance();
                Ok(Expression::Property {
                    variable,
                    property: prop,
                })
            } else {
                Err("Expected property name".to_string())
            }
        } else {
            Ok(Expression::Variable(variable))
        }
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

    #[test]
    fn test_parser() {
        let input = "MATCH (keanu:Person {name:'Keanu Reeves'}) RETURN keanu.name AS name, keanu.born AS born";
        let mut parser = Parser::new(input);
        let query = parser.parse_query().unwrap();

        // Verify match clause
        assert_eq!(query.match_clause.patterns.len(), 1);
        let pattern = &query.match_clause.patterns[0];
        assert_eq!(pattern.variable, "keanu");
        assert_eq!(pattern.labels, vec!["Person"]);
        assert_eq!(pattern.properties.len(), 1);
        assert_eq!(pattern.properties[0].key, "name");
        assert_eq!(
            pattern.properties[0].value,
            Value::String("Keanu Reeves".to_string())
        );

        // Verify return clause
        assert_eq!(query.return_clause.items.len(), 2);
        
        let first_item = &query.return_clause.items[0];
        match &first_item.expression {
            Expression::Property { variable, property } => {
                assert_eq!(variable, "keanu");
                assert_eq!(property, "name");
            }
            _ => panic!("Expected property expression"),
        }
        assert_eq!(first_item.alias, Some("name".to_string()));

        let second_item = &query.return_clause.items[1];
        match &second_item.expression {
            Expression::Property { variable, property } => {
                assert_eq!(variable, "keanu");
                assert_eq!(property, "born");
            }
            _ => panic!("Expected property expression"),
        }
        assert_eq!(second_item.alias, Some("born".to_string()));
    }
}
