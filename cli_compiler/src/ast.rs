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

pub struct AST {
    index: usize,
    tokens: Vec<Token>,
}

impl AST {
    pub fn new(input: &str) -> AST {
        let tokens = Tokenizer::new(input).tokenize(input);

        AST {
            index: 0,
            tokens
        }
    }

    fn peek(&mut self, ahead: Option<usize>) -> Option<&Token> {
        let look_ahead = ahead.unwrap_or(1);
        let ahead_token = self.tokens.get(self.index + look_ahead).unwrap();

        if ahead_token.kind == Kind::EOF {
            return None;
        }
        
        Some(ahead_token)
    }

    fn consume(&mut self) {}

    pub fn parse(&mut self, input: Vec<Token>) {
        while self.peek(Some(1)).is_some() {

        }
    }
}
