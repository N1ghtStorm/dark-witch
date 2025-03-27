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

use crate::error::Error;
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
    Order,
    By,

    // Symbols
    Asterisk,
    Comma,
    GreaterThan,
    LessThan,
    Equal,
    Not,

    // Composed tokens
    GreaterThanEqual,
    LessThanEqual,
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
                '!' => {
                    self.advance();
                    if self.peek() == Some('=') {
                        self.advance();
                        Token::NotEqual
                    } else {
                        Token::Not
                    }
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
                        "ORDER" => Token::Order,
                        "BY" => Token::By,
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
#[allow(private_interfaces)]
pub enum AstNode {
    Select {
        fields: Vec<FieldExpression>,
        from: String,
        where_clause: Option<Box<AstNode>>,
        order_by: Option<String>,
    },
    BinaryOp {
        left: Box<AstNode>,
        operator: String,
        right: Box<AstNode>,
    },
    Column(String),
    Literal(LiteralValue),
}

#[derive(Debug, Clone)]
enum FieldExpression {
    AllColumns,
    Field(String),
}

#[derive(Debug, Clone)]
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

    fn expect(&mut self, expected: Token) -> Result<(), Error> {
        if self.peek() == Some(&expected) {
            self.advance();
            Ok(())
        } else {
            Err(Error::SyntaxError(format!(
                "Expected {:?}, got {:?}",
                expected,
                self.peek()
            )))
        }
    }

    fn parse_select(&mut self) -> Result<AstNode, Error> {
        self.expect(Token::Select)?;

        // Parse columns
        let mut fields = Vec::new();

        if self.peek() == Some(&Token::Asterisk) {
            fields.push(FieldExpression::AllColumns);
            self.advance();
        } else {
            // Parse first field
            match self.peek() {
                Some(Token::Identifier(name)) => {
                    fields.push(FieldExpression::Field(name.clone()));
                    self.advance();
                }
                _ => {
                    return Err(Error::SyntaxError(
                        "Expected identifier in SELECT".to_string(),
                    ))
                }
            }

            // Parse additional fields after commas
            while self.peek() == Some(&Token::Comma) {
                self.advance(); // consume comma
                match self.peek() {
                    Some(Token::Identifier(name)) => {
                        fields.push(FieldExpression::Field(name.clone()));
                        self.advance();
                    }
                    _ => {
                        return Err(Error::SyntaxError(
                            "Expected identifier after comma".to_string(),
                        ))
                    }
                }
            }
        }

        // Parse FROM clause
        self.expect(Token::From)?;
        let table_name = match self.peek() {
            Some(Token::Identifier(name)) => {
                let name = name.clone();
                self.advance();
                name
            }
            _ => {
                return Err(Error::SyntaxError(
                    "Expected table name after FROM".to_string(),
                ))
            }
        };

        // Parse WHERE clause (if present)
        let where_clause = if self.peek() == Some(&Token::Where) {
            self.advance();
            let condition = self.parse_expression()?;
            Some(Box::new(condition))
        } else {
            None
        };

        // Parse ORDER BY clause (if present)
        let order_by = if self.peek() == Some(&Token::Order) {
            self.advance();
            if self.peek() != Some(&Token::By) {
                return Err(Error::SyntaxError("Expected BY after ORDER".to_string()));
            }
            self.advance();

            match self.peek() {
                Some(Token::Identifier(name)) => {
                    let name = name.clone();
                    self.advance();
                    Some(name)
                }
                _ => {
                    return Err(Error::SyntaxError(
                        "Expected column name after ORDER BY".to_string(),
                    ))
                }
            }
        } else {
            None
        };

        Ok(AstNode::Select {
            fields,
            from: table_name,
            where_clause,
            order_by,
        })
    }

    fn parse_expression(&mut self) -> Result<AstNode, Error> {
        let mut left = self.parse_comparison()?;

        while let Some(token) = self.peek() {
            match token {
                Token::And | Token::Or => {
                    let operator = match token {
                        Token::And => "AND".to_string(),
                        Token::Or => "OR".to_string(),
                        _ => {
                            return Err(Error::SyntaxError(format!(
                                "Expected operator AND and OR, got {:?}",
                                token
                            )))
                        }
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
    fn parse_comparison(&mut self) -> Result<AstNode, Error> {
        let left = match self.peek() {
            Some(Token::Identifier(name)) => {
                let name = name.clone();
                self.advance();
                AstNode::Column(name)
            }
            _ => {
                return Err(Error::SyntaxError(
                    "Expected identifier in WHERE clause".to_string(),
                ))
            }
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
            val => {
                return Err(Error::SyntaxError(format!(
                    "Expected comparison operator, got {:?}",
                    val
                )))
            }
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
            _ => {
                return Err(Error::SyntaxError(
                    "Expected literal value after operator".to_string(),
                ))
            }
        };

        Ok(AstNode::BinaryOp {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        })
    }

    pub fn parse(&mut self) -> Result<AstNode, Error> {
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

    pub fn generate(&mut self, ast: &AstNode) -> Result<(), Error> {
        match ast {
            AstNode::Select {
                fields,
                from,
                where_clause,
                order_by,
            } => {
                // Load the table
                self.emit(Instruction::UseStorage { name: from.clone() });

                // Handle WHERE clause if present
                let predicate = match where_clause {
                    Some(condition) => self.generate_condition(condition)?,
                    None => Box::new(|_: String, _: String| true),
                };

                let (string_fields_values, number_fields_values) = match where_clause {
                    Some(condition) => {
                        let lefts_rights = self.get_where_lefts_rights(condition)?;
                        let mut string_fields_values = Vec::new();
                        let mut number_fields_values = Vec::new();
                        for (field, value) in lefts_rights {
                            match value {
                                LiteralValue::String(s) => string_fields_values.push((field, s)),
                                LiteralValue::Number(n) => number_fields_values.push((field, n)),
                            }
                        }
                        (string_fields_values, number_fields_values)
                    }
                    None => (Vec::new(), Vec::new()),
                };

                self.emit(Instruction::Scan {
                    filter: crate::witchvm::Filter::Condition(predicate),
                    string_fields_values,
                    number_fields_values,
                });

                if fields.len() == 1 {
                    match &fields[0] {
                        FieldExpression::AllColumns => {}
                        FieldExpression::Field(name) => {
                            let name = name.clone();
                            let instruction = Instruction::MapOutput {
                                map_fn: Box::new(move |json_string: String| {
                                    // take Json field with name and return Json with only that field
                                    let json: serde_json::Value =
                                        match serde_json::from_str(&json_string) {
                                            Ok(json) => json,
                                            Err(e) => {
                                                println!("Error parsing JSON: {}", e);
                                                let new_json = serde_json::Map::new();
                                                return serde_json::Value::Object(new_json)
                                                    .to_string();
                                            }
                                        };
                                    let value = match json.get(&name) {
                                        Some(value) => value.clone(),
                                        None => serde_json::Value::Null,
                                    };

                                    // create new json with only the value
                                    let mut new_json = serde_json::Map::new();
                                    new_json.insert(name.clone(), value.clone());
                                    serde_json::Value::Object(new_json).to_string()
                                }),
                            };
                            self.emit(instruction);
                        }
                    }
                } else if fields.len() > 1 {
                    for field in fields {
                        // check if all fields are not "*"
                        match field {
                            FieldExpression::AllColumns => {
                                return Err(Error::SyntaxError(
                                    "Syntax error: Multiple fields not allowed in SELECT with *"
                                        .to_string(),
                                ));
                            }
                            FieldExpression::Field(_) => {}
                        }
                    }

                    let fields = fields.clone();
                    let instruction = Instruction::MapOutput {
                        map_fn: Box::new(move |json_string: String| {
                            let json: serde_json::Value = match serde_json::from_str(&json_string) {
                                Ok(json) => json,
                                Err(e) => {
                                    println!("Error parsing JSON: {}", e);
                                    let new_json = serde_json::Map::new();
                                    return serde_json::Value::Object(new_json).to_string();
                                }
                            };

                            let mut new_json = serde_json::Map::new();

                            for field in fields.clone() {
                                match field {
                                    FieldExpression::AllColumns => {
                                        // will never happen, but
                                        let new_json = serde_json::Map::new();
                                        return serde_json::Value::Object(new_json).to_string();
                                    }
                                    FieldExpression::Field(name) => {
                                        let value = match json.get(&name) {
                                            Some(value) => value.clone(),
                                            None => serde_json::Value::Null,
                                        };

                                        new_json.insert(name.clone(), value.clone());
                                    }
                                }
                            }

                            serde_json::Value::Object(new_json).to_string()
                        }),
                    };
                    self.emit(instruction);
                } else {
                    return Err(Error::SyntaxError(
                        "Syntax error: No fields in SELECT".to_string(),
                    ));
                }

                if let Some(order_by) = order_by {
                    let instruction = Instruction::SortOutput {
                        field: order_by.clone(),
                    };
                    self.emit(instruction);
                }

                Ok(())
            }
            _ => Err(Error::SyntaxError("unhandled case".to_string())), // Other node types would be handled here
        }
    }

    fn generate_condition(
        &mut self,
        condition: &AstNode,
    ) -> Result<Box<dyn Fn(String, String) -> bool + 'static>, Error> {
        match condition {
            AstNode::BinaryOp {
                left,
                operator,
                right,
            } => {
                match operator.as_str() {
                    "AND" => {
                        let left_pred = self.generate_condition(left)?;
                        let right_pred = self.generate_condition(right)?;
                        Ok(Box::new(move |key: String, value: String| {
                            left_pred(key.clone(), value.clone()) && right_pred(key, value)
                        }))
                    }
                    "OR" => {
                        let left_pred = self.generate_condition(left)?;
                        let right_pred = self.generate_condition(right)?;
                        Ok(Box::new(move |key: String, value: String| {
                            left_pred(key.clone(), value.clone()) || right_pred(key, value)
                        }))
                    }
                    _ => {
                        // Handle comparison operators
                        match (&**left, &**right) {
                            (AstNode::Column(col), AstNode::Literal(lit)) => {
                                let col = col.clone();
                                match lit {
                                    LiteralValue::Number(n) => {
                                        let operator = operator.clone();
                                        let n = n.clone();
                                        Ok(Box::new(move |_, value: String| {
                                            if let Ok(json) =
                                                serde_json::from_str::<serde_json::Value>(&value)
                                            {
                                                if let Some(field) =
                                                    json.get(col.as_str()).and_then(|v| v.as_i64())
                                                {
                                                    return num_cond(
                                                        field,
                                                        operator.clone(),
                                                        n as i64,
                                                    );
                                                }
                                            }
                                            false
                                        }))
                                    }
                                    LiteralValue::String(s) => {
                                        let operator = operator.clone();
                                        let s = s.clone();
                                        Ok(Box::new(move |_, value: String| {
                                            if let Ok(json) =
                                                serde_json::from_str::<serde_json::Value>(&value)
                                            {
                                                if let Some(name) = json
                                                    .get(col.clone().as_str())
                                                    .and_then(|v| v.as_str())
                                                {
                                                    return str_cond(
                                                        name.to_string(),
                                                        operator.clone(),
                                                        s.clone(),
                                                    );
                                                }
                                            }
                                            false
                                        }))
                                    }
                                }
                            }
                            _ => Err(Error::SyntaxError("Unhandled Condition".to_string())),
                        }
                    }
                }
            }
            _ => Err(Error::SyntaxError("Unhandled Condition".to_string())),
        }
    }

    fn get_where_lefts_rights(
        &self,
        condition: &AstNode,
    ) -> Result<Vec<(String, LiteralValue)>, Error> {
        let mut result: Vec<(String, LiteralValue)> = Vec::new();
        match condition {
            AstNode::BinaryOp {
                left,
                operator,
                right,
            } => match operator.as_str() {
                "OR" => {
                    result.extend(self.get_where_lefts_rights(left)?);
                    result.extend(self.get_where_lefts_rights(right)?);
                }
                "AND" => {}
                _ => {
                    let field = match &**left {
                        AstNode::Column(name) => name.clone(),
                        _ => return Err(Error::SyntaxError("Unhandled Condition".to_string())),
                    };

                    let field_value = match &**right {
                        AstNode::Literal(val) => val.clone(),
                        _ => return Err(Error::SyntaxError("Unhandled Condition".to_string())),
                    };
                    result.push((field, field_value));
                }
            },
            _ => return Err(Error::SyntaxError("Unhandled Condition".to_string())),
        }

        Ok(result)
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
        _ => false,
    }
}

fn str_cond(field: String, operator: String, value: String) -> bool {
    match operator.as_str() {
        "=" => field == value,
        "!=" => field != value,
        _ => false,
    }
}

mod tests {
    #[allow(unused_imports)]
    use super::*;
}
