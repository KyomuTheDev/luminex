use std::collections::HashMap;

use crate::{
    tokenizer::{Kind, Tokenizer, Token},
    utils::get_error_message,
};

#[derive(Clone, Debug, PartialEq, Eq)]
enum TokenKind {
    If,

    Add,
    Sub,
    Mul,
    Div,
    Mod,

    InitializedVariable,
    UninitializedVariable,
    Destroy,

    Function,
    FunctionCall,
    Lambda,

    New,
    TryCatch,
    Break,
    Return,
    Throw,
    Continue,

    Class,
    Struct,
    Enum,
    Interface,
    Namespace,

    Typeof,

    Loop,
    LoopUntil,
    While,
    For,

    Const,
    Public,
    Private,
    Protected,
    Static,
    Override,
    Get,
    Set,
    Async,
    Readonly,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ASTToken {
    kind: TokenKind,
    name: String,
    scope: Vec<ASTToken>,
}

fn binding_power(op: String) -> (u8, u8) {
    match op.as_str() {
        "+" | "-" => (1, 2),
        "*" | "/" | "%" => (2, 3),
        _ => panic!("Bad operator."),
    }
}

pub struct Parser {
    index: usize,
    // precedence_map: HashMap<Kind, u8>,
}

impl Parser {
    pub fn new(input: &str) -> Parser {
        /*
        * Precedence map for if I decide to compile to asm.
        */
        // let mut precedence_map = HashMap::new();

        // // logical operators
        // precedence_map.insert(Kind::Operator(">".to_owned()), 0); // greater than
        // precedence_map.insert(Kind::Operator("<".to_owned()), 0); // less than
        // precedence_map.insert(Kind::Operator("==".to_owned()), 0); // equality
        // precedence_map.insert(Kind::Operator("!=".to_owned()), 0); // inequality
        // precedence_map.insert(Kind::Operator(">=".to_owned()), 0); // greater than or equal
        // precedence_map.insert(Kind::Operator("<=".to_owned()), 0); // less than or equal

        // // arithmetic operators
        // precedence_map.insert(Kind::Operator("+".to_owned()), 1); // addition
        // precedence_map.insert(Kind::Operator("-".to_owned()), 1); // subtraction
        // precedence_map.insert(Kind::Operator("*".to_owned()), 2); // multiplication
        // precedence_map.insert(Kind::Operator("**".to_owned()), 2); // exponentiation
        // precedence_map.insert(Kind::Operator("/".to_owned()), 2); // division
        // precedence_map.insert(Kind::Operator("%".to_owned()), 2); // modulus

        Parser {
            index: 0,
            // precedence_map
        }
    }

    fn program(&mut self) {}

    fn eat(&mut self, kind: Kind) -> Token {
        
    }
}
