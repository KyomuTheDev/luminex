extern crate enum_index;
use crate::lexer::{Kind, Token, Tokenizer};
use std::fmt::Display;

type Result<T> = std::result::Result<T, ParserError>;

/* Error Enum */
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ParserError {
    Unknown,
    UnexpectedToken(Token),
    UnexpectedEof,
    InvalidExpression,
    ExpectedOperator,
}

impl Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str: Vec<char> = format!("{:?}", self).chars().collect();
        let mut index = 0;

        while index < str.len() {
            write!(f, "{}", str[index])?;

            let next_char = if index + 1 < str.len() {
                str[index + 1]
            } else {
                break;
            };

            index += 1;

            if next_char.is_uppercase() {
                write!(f, " ")?;
                write!(f, "{}", next_char.to_lowercase())?;
                index += 1;
            }
        }

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub enum Node {
    String(String),
    Number(String),
    Identifier(String),

    Program(Vec<Node>),

    BinaryOp {
        op: Operator,
        left: Box<Node>,
        right: Box<Node>,
    },

    UrnaryOp {
        op: Operator,
        right: Box<Node>,
    },
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Node::String(_), Node::String(_)) => true,
            (Node::Number(_), Node::Number(_)) => true,
            (Node::Program(_), Node::Program(_)) => true,
            (Node::BinaryOp { .. }, Node::BinaryOp { .. }) => true,
            _ => false,
        }
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

#[derive(Debug, Clone)]
pub enum Operator {
    Add,
    Sub,
    Div,
    Mul,
    Pow,
    Mod,
    Equal,
    NotEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    And,
    Or,
    Not,
}

pub struct Parser {
    tokenizer: Tokenizer,
}

impl Parser {
    pub fn new(input: &str) -> Parser {
        Parser {
            tokenizer: Tokenizer::new(input),
        }
    }

    fn binding_power(&mut self, op: &str) -> (u8, u8) {
        match op {
            ">" | "<" | "==" | "!=" | ">=" | "<=" => (0, 0),
            "+" | "-" => (1, 1),
            "*" | "/"  | "%" => (2, 2),
            "**" => (3, 3),
            _ => panic!("Bad operator: {:?}", op),
        }
    }

    fn operator(&mut self, op: &str) -> Operator {
        match op {
            "+" => Operator::Add,
            "-" => Operator::Sub,
            "*" => Operator::Mul,
            "/" => Operator::Div,
            "**" => Operator::Pow,
            "%" => Operator::Mod,
            "==" => Operator::Equal,
            "!=" => Operator::NotEqual,
            ">=" => Operator::GreaterEqual,
            "<=" => Operator::LessEqual,
            ">" => Operator::Greater,
            "<" => Operator::Less,
            "&&" => Operator::And,
            "||" => Operator::Or,
            "!" => Operator::Not,
            _ => panic!("Bad operator: {:?}", op),
        }
    }

    fn expect(&self, token: &Token, cmp: Kind) -> Result<()> {
        if token.kind != cmp {
            return Err(ParserError::UnexpectedToken(token.clone()));
        }

        Ok(())
    }

    fn expr_bp(&mut self, min_bp: u8) -> Result<Node> {
        let mut lhs = match self.tokenizer.next().kind {
            Kind::Identifier(ident) => Node::Identifier(ident),
            Kind::Number(num) => Node::Number(num),
            t => panic!("Expected identifier or number but got {:?}", t),
        };

        loop {
            let op = match self.tokenizer.peek_token().kind {
                Kind::Operator(op) => op,
                Kind::EOF => {
                    break;
                }
                t => panic!("Expected operator but got {:?}", t),
            };

            let (lhs_bp, rhs_bp) = self.binding_power(&op);

            if lhs_bp < min_bp {
                break;
            }

            self.tokenizer.next();
            let rhs = self.expr_bp(rhs_bp)?;

            lhs = Node::BinaryOp {
                op: self.operator(&op),
                left: Box::new(lhs),
                right: Box::new(rhs),
            };
        }

        Ok(lhs)
    }

    pub fn expr(&mut self) -> Result<Node> {
        self.expr_bp(0)
    }
}
