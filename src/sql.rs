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

use crate::witchvm::Instruction;

// Lexer: Converts raw SQL into tokens
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Keywords
    Select,
    From,
    Where,
    And,
    Or,

    // Symbols
    Asterisk,
    Comma,
    GreaterThan,
    GreaterThanEqual,
    LessThan,
    LessThanEqual,
    Equal,
    NotEqual,

    // Literals
    Identifier(String),
    Number(f64),
    String(String),

    // End of input
    Eof,
}

pub struct Lexer {
    input: Vec<char>,
    position: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Lexer {
            input: input.chars().collect(),
            position: 0,
        }
    }

    fn peek(&self) -> Option<char> {
        self.input.get(self.position).copied()
    }

    fn advance(&mut self) {
        self.position += 1;
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek() {
            if !c.is_whitespace() {
                break;
            }
            self.advance();
        }
    }

    fn read_identifier(&mut self) -> String {
        let start = self.position;
        while let Some(c) = self.peek() {
            if !(c.is_alphanumeric() || c == '_') {
                break;
            }
            self.advance();
        }

        self.input[start..self.position].iter().collect()
    }

    fn read_number(&mut self) -> f64 {
        let start = self.position;
        let mut has_decimal = false;

        while let Some(c) = self.peek() {
            if c.is_digit(10) {
                self.advance();
            } else if c == '.' && !has_decimal {
                has_decimal = true;
                self.advance();
            } else {
                break;
            }
        }

        let num_str: String = self.input[start..self.position].iter().collect();
        num_str.parse().unwrap_or(0.0)
    }

    fn read_string(&mut self) -> String {
        // Skip the opening quote
        self.advance();

        let start = self.position;
        while let Some(c) = self.peek() {
            if c == '\'' {
                break;
            }
            self.advance();
        }

        let string_value = self.input[start..self.position].iter().collect();

        // Skip the closing quote
        if self.peek() == Some('\'') {
            self.advance();
        }

        string_value
    }

    fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        if let Some(c) = self.peek() {
            match c {
                '*' => {
                    self.advance();
                    Token::Asterisk
                }
                ',' => {
                    self.advance();
                    Token::Comma
                }
                '>' => {
                    self.advance();
                    if self.peek() == Some('=') {
                        self.advance();
                        Token::GreaterThanEqual
                    } else {
                        Token::GreaterThan
                    }
                }
                '<' => {
                    self.advance();
                    if self.peek() == Some('=') {
                        self.advance();
                        Token::LessThanEqual
                    } else if self.peek() == Some('>') {
                        self.advance();
                        Token::NotEqual
                    } else {
                        Token::LessThan
                    }
                }
                '=' => {
                    self.advance();
                    Token::Equal
                }
                '0'..='9' => {
                    let value = self.read_number();
                    Token::Number(value)
                }
                '\'' => {
                    let string_value = self.read_string();
                    Token::String(string_value)
                }
                'a'..='z' | 'A'..='Z' | '_' => {
                    let identifier = self.read_identifier();
                    match identifier.to_uppercase().as_str() {
                        "SELECT" => Token::Select,
                        "FROM" => Token::From,
                        "WHERE" => Token::Where,
                        "AND" => Token::And,
                        "OR" => Token::Or,
                        _ => Token::Identifier(identifier),
                    }
                }
                _ => {
                    self.advance();
                    Token::Eof
                }
            }
        } else {
            Token::Eof
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        loop {
            let token = self.next_token();
            if token == Token::Eof {
                tokens.push(token);
                break;
            }
            tokens.push(token);
        }
        tokens
    }
}

// Parser: Constructs AST from tokens
#[derive(Debug)]
pub enum AstNode {
    Select {
        columns: Vec<ColumnExpression>,
        from: String,
        where_clause: Option<Box<AstNode>>,
    },
    BinaryOp {
        left: Box<AstNode>,
        operator: String,
        right: Box<AstNode>,
    },
    Column(String),
    Literal(LiteralValue),
}

#[derive(Debug)]
enum ColumnExpression {
    AllColumns,
    Column(String),
}

#[derive(Debug)]
enum LiteralValue {
    Number(f64),
    String(String),
}

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            position: 0,
        }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.position)
    }

    fn advance(&mut self) {
        self.position += 1;
    }

    fn expect(&mut self, expected: Token) -> Result<(), String> {
        if self.peek() == Some(&expected) {
            self.advance();
            Ok(())
        } else {
            Err(format!("Expected {:?}, got {:?}", expected, self.peek()))
        }
    }

    fn parse_select(&mut self) -> Result<AstNode, String> {
        self.expect(Token::Select)?;

        // Parse columns
        let mut columns = Vec::new();

        if self.peek() == Some(&Token::Asterisk) {
            columns.push(ColumnExpression::AllColumns);
            self.advance();
        } else {
            // Handle column list parsing - simplified for now
            columns.push(ColumnExpression::Column("column".to_string()));
        }

        // Parse FROM clause
        self.expect(Token::From)?;
        let table_name = match self.peek() {
            Some(Token::Identifier(name)) => {
                let name = name.clone();
                self.advance();
                name
            }
            _ => return Err("Expected table name after FROM".to_string()),
        };

        // Parse WHERE clause (if present)
        let where_clause = if self.peek() == Some(&Token::Where) {
            self.advance();
            let condition = self.parse_expression()?;
            Some(Box::new(condition))
        } else {
            None
        };

        Ok(AstNode::Select {
            columns,
            from: table_name,
            where_clause,
        })
    }

    fn parse_expression(&mut self) -> Result<AstNode, String> {
        let mut left = self.parse_comparison()?;

        while let Some(token) = self.peek() {
            match token {
                Token::And | Token::Or => {
                    let operator = match token {
                        Token::And => "AND".to_string(),
                        Token::Or => "OR".to_string(),
                        _ => unreachable!(),
                    };
                    self.advance();
                    let right = self.parse_comparison()?;
                    left = AstNode::BinaryOp {
                        left: Box::new(left),
                        operator,
                        right: Box::new(right),
                    };
                }
                _ => break,
            }
        }

        Ok(left)
    }

    // New method to handle basic comparisons
    fn parse_comparison(&mut self) -> Result<AstNode, String> {
        let left = match self.peek() {
            Some(Token::Identifier(name)) => {
                let name = name.clone();
                self.advance();
                AstNode::Column(name)
            }
            _ => return Err("Expected identifier in WHERE clause".to_string()),
        };

        let operator = match self.peek() {
            Some(Token::GreaterThan) => {
                self.advance();
                ">".to_string()
            }
            Some(Token::GreaterThanEqual) => {
                self.advance();
                ">=".to_string()
            }
            Some(Token::LessThan) => {
                self.advance();
                "<".to_string()
            }
            Some(Token::LessThanEqual) => {
                self.advance();
                "<=".to_string()
            }
            Some(Token::Equal) => {
                self.advance();
                "=".to_string()
            }
            Some(Token::NotEqual) => {
                self.advance();
                "!=".to_string()
            }
            val => return Err(format!("Expected comparison operator, got {:?}", val)),
        };

        let right = match self.peek() {
            Some(Token::Number(n)) => {
                let value = *n;
                self.advance();
                AstNode::Literal(LiteralValue::Number(value))
            }
            Some(Token::String(s)) => {
                let value = s.clone();
                self.advance();
                AstNode::Literal(LiteralValue::String(value))
            }
            _ => return Err("Expected literal value after operator".to_string()),
        };

        Ok(AstNode::BinaryOp {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        })
    }

    pub fn parse(&mut self) -> Result<AstNode, String> {
        self.parse_select()
    }
}

// // Code Generator: Transforms AST into WitchVM instructions
pub struct CodeGenerator {
    pub instructions: Vec<Instruction>,
}

impl CodeGenerator {
    pub fn new() -> Self {
        CodeGenerator {
            instructions: Vec::new(),
        }
    }

    fn emit(&mut self, instruction: Instruction) {
        self.instructions.push(instruction);
    }

    pub fn generate(&mut self, ast: &AstNode) {
        match ast {
            AstNode::Select {
                columns,
                from,
                where_clause,
            } => {
                // Load the table
                self.emit(Instruction::UseStorage { name: from.clone() });

                // Handle WHERE clause if present
                let predicate = match where_clause {
                    Some(condition) => self.generate_condition(condition),
                    None => Box::new(|_: String, _: String| true),
                };

                self.emit(Instruction::FullScan {
                    maybe_filter: Some(crate::witchvm::Filter::Condition(predicate)),
                });
            }
            _ => {} // Other node types would be handled here
        }
    }

    fn generate_condition(&mut self, condition: &AstNode) -> Box<dyn Fn(String, String) -> bool + 'static> {
        match condition {
            AstNode::BinaryOp { left, operator, right } => {
                match operator.as_str() {
                    "AND" => {
                        let left_pred = self.generate_condition(left);
                        let right_pred = self.generate_condition(right);
                        Box::new(move |key: String, value: String| {
                            left_pred(key.clone(), value.clone()) && right_pred(key, value)
                        })
                    }
                    "OR" => {
                        let left_pred = self.generate_condition(left);
                        let right_pred = self.generate_condition(right);
                        Box::new(move |key: String, value: String| {
                            left_pred(key.clone(), value.clone()) || right_pred(key, value)
                        })
                    }
                    _ => {
                        // Handle comparison operators as before
                        match (&**left, &**right) {
                            (AstNode::Column(col), AstNode::Literal(lit)) => {
                                let col = col.clone();
                                match lit {
                                    LiteralValue::Number(n) => {
                                        let operator = operator.clone();
                                        let n = n.clone();
                                        Box::new(move |_, value: String| {
                                            if let Ok(json) =
                                                serde_json::from_str::<serde_json::Value>(&value)
                                            {
                                                if let Some(field) =
                                                    json.get(col.as_str()).and_then(|v| v.as_i64())
                                                {
                                                    return num_cond(field, operator.clone(), n as i64);
                                                }
                                            }
                                            false
                                        })
                                    }
                                    LiteralValue::String(s) => {
                                        let operator = operator.clone();
                                        let s = s.clone();
                                        Box::new(move |_, value: String| {
                                            if let Ok(json) =
                                                serde_json::from_str::<serde_json::Value>(&value)
                                            {
                                                if let Some(name) =
                                                    json.get("name").and_then(|v| v.as_str())
                                                {
                                                    return str_cond(
                                                        name.to_string(),
                                                        operator.clone(),
                                                        s.clone(),
                                                    );
                                                }
                                            }
                                            false
                                        })
                                    }
                                }
                            }
                            _ => Box::new(|_, _| false),
                        }
                    }
                }
            }
            _ => Box::new(|_, _| false),
        }
    }
}

fn num_cond(field: i64, operator: String, value: i64) -> bool {
    match operator.as_str() {
        ">" => field > value,
        ">=" => field >= value,
        "<" => field < value,
        "<=" => field <= value,
        "=" => field == value,
        "!=" => field != value,
        _ => todo!("unhandled case"),
    }
}

fn str_cond(field: String, operator: String, value: String) -> bool {
    match operator.as_str() {
        "=" => field == value,
        "!=" => field != value,
        _ => todo!("unhandled case"),
    }
}
