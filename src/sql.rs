// SELECT * FROM main WHERE age >= 30

use std::fmt;

// Lexer: Converts raw SQL into tokens
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Keywords
    Select,
    From,
    Where,
    
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
                },
                ',' => {
                    self.advance();
                    Token::Comma
                },
                '>' => {
                    self.advance();
                    if self.peek() == Some('=') {
                        self.advance();
                        Token::GreaterThanEqual
                    } else {
                        Token::GreaterThan
                    }
                },
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
                },
                '=' => {
                    self.advance();
                    Token::Equal
                },
                '0'..='9' => {
                    let value = self.read_number();
                    Token::Number(value)
                },
                '\'' => {
                    let string_value = self.read_string();
                    Token::String(string_value)
                },
                'a'..='z' | 'A'..='Z' | '_' => {
                    let identifier = self.read_identifier();
                    match identifier.to_uppercase().as_str() {
                        "SELECT" => Token::Select,
                        "FROM" => Token::From,
                        "WHERE" => Token::Where,
                        _ => Token::Identifier(identifier),
                    }
                },
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
            },
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
        // Simplified expression parsing - would need more logic for complex expressions
        let left = match self.peek() {
            Some(Token::Identifier(name)) => {
                let name = name.clone();
                self.advance();
                AstNode::Column(name)
            },
            _ => return Err("Expected identifier in WHERE clause".to_string()),
        };
        
        let operator = match self.peek() {
            Some(Token::GreaterThan) => {
                self.advance();
                ">".to_string()
            },
            Some(Token::GreaterThanEqual) => {
                self.advance();
                ">=".to_string()
            },
            Some(Token::LessThan) => {
                self.advance();
                "<".to_string()
            },
            Some(Token::LessThanEqual) => {
                self.advance();
                "<=".to_string()
            },
            Some(Token::Equal) => {
                self.advance();
                "=".to_string()
            },
            _ => return Err("Expected comparison operator".to_string()),
        };
        
        let right = match self.peek() {
            Some(Token::Number(n)) => {
                let value = *n;
                self.advance();
                AstNode::Literal(LiteralValue::Number(value))
            },
            Some(Token::String(s)) => {
                let value = s.clone();
                self.advance();
                AstNode::Literal(LiteralValue::String(value))
            },
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
// struct CodeGenerator {
//     instructions: Vec<String>,
// }

// impl CodeGenerator {
//     fn new() -> Self {
//         CodeGenerator {
//             instructions: Vec::new(),
//         }
//     }
    
//     fn emit(&mut self, instruction: &str) {
//         self.instructions.push(instruction.to_string());
//     }
    
//     fn generate(&mut self, ast: &AstNode) {
//         match ast {
//             AstNode::Select { columns, from, where_clause } => {
//                 // Load the table
//                 self.emit(&format!("LOAD_TABLE \"{}\"", from));
                
//                 // If there's a WHERE clause, generate condition code
//                 if let Some(condition) = where_clause {
//                     self.generate_condition(condition);
//                     self.emit("FILTER");
//                 }
                
//                 // Project columns
//                 match columns.as_slice() {
//                     [ColumnExpression::AllColumns] => {
//                         self.emit("PROJECT_ALL");
//                     },
//                     _ => {
//                         // For specific columns, we would need to emit column names
//                         self.emit("BEGIN_PROJECT");
//                         for col in columns {
//                             if let ColumnExpression::Column(name) = col {
//                                 self.emit(&format!("ADD_COLUMN \"{}\"", name));
//                             }
//                         }
//                         self.emit("END_PROJECT");
//                     }
//                 }
                
//                 // Finalize the query
//                 self.emit("MATERIALIZE");
//             },
//             _ => {
//                 // Other node types would be handled here
//             }
//         }
//     }
    
//     fn generate_condition(&mut self, condition: &AstNode) {
//         match condition {
//             AstNode::BinaryOp { left, operator, right } => {
//                 match (&**left, &**right) {
//                     (AstNode::Column(col), AstNode::Literal(lit)) => {
//                         // Push column name
//                         self.emit(&format!("PUSH_COLUMN \"{}\"", col));
                        
//                         // Push literal value
//                         match lit {
//                             LiteralValue::Number(n) => self.emit(&format!("PUSH_NUMBER {}", n)),
//                             LiteralValue::String(s) => self.emit(&format!("PUSH_STRING \"{}\"", s)),
//                         }
                        
//                         // Apply comparison operator
//                         match operator.as_str() {
//                             ">" => self.emit("GT"),
//                             ">=" => self.emit("GTE"),
//                             "<" => self.emit("LT"),
//                             "<=" => self.emit("LTE"),
//                             "=" => self.emit("EQ"),
//                             "!=" => self.emit("NEQ"),
//                             _ => {} // Unhandled operator
//                         }
//                     },
//                     _ => {
//                         // Handle more complex expressions here
//                     }
//                 }
//             },
//             _ => {
//                 // Other condition types would be handled here
//             }
//         }
//     }
// }

// // Compiler: Orchestrates the entire compilation process
// pub struct SqlCompiler;

// impl SqlCompiler {
//     pub fn new() -> Self {
//         SqlCompiler
//     }
    
//     pub fn compile(&self, sql: &str) -> Result<Vec<String>, String> {
//         // Tokenize the SQL query
//         let mut lexer = Lexer::new(sql);
//         let tokens = lexer.tokenize();
        
//         // Parse tokens into AST
//         let mut parser = Parser::new(tokens);
//         let ast = parser.parse()?;
        
//         // Generate code from AST
//         let mut generator = CodeGenerator::new();
//         generator.generate(&ast);
        
//         Ok(generator.instructions)
//     }
// }

// // Usage example
// pub fn compile_sql_example(sql: &str) -> Result<(), String> {
//     let compiler = SqlCompiler::new();
//     let result = compiler.compile(sql)?;
    
//     println!("Compiled SQL: {}", sql);
//     println!("WitchVM Instructions:");
//     for instruction in result {
//         println!("  {}", instruction);
//     }
    
//     Ok(())
// }

// // Example function to demonstrate usage
// fn main() -> Result<(), String> {
//     compile_sql_example("SELECT * FROM main WHERE age >= 30")
// }