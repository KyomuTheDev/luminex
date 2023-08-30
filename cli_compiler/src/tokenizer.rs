use crate::utils::get_error_message;
use std::collections::HashMap;

macro_rules! match_operator {
    () => {
        '+' | '-' | '*' | '%' | '!' | '<' | '>' | '&' | '|' | '='
    };
}

#[allow(clippy::all)]
#[allow(dead_code)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Kind {
    Keyword(String),
    Operator(String),
    Bool(bool),
    Identifier,
    Comment,
    Number,
    String,

    Tilde,
    Question,
    Hash,
    Comma,
    SemiColon,
    PathSeparator,
    Dot,
    RParen,
    LParen,
    RBracket,
    LBracket,
    RBrace,
    LBrace,

    EOF,

    Invalid,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Token {
    /* The kind of lexical token */
    pub kind: Kind,

    /* The value of the token */
    pub value: String,

    /* The position of the first character in the value in the input string */
    pub position: usize,

    /* The line this token starts on */
    pub line: usize,

    /* The position of the first character in the value relative to the previous newline character */
    pub line_position: usize,
}

/*
! A note to those reading this code

* This lexer implementation was inspired by chatgpt, in normal terms I stole chatgpt's code and changed the fuck out of it to suit my needs.
* When I say I changed it, I mean it. Practically none of this code is the same as when chatgpt wrote it, just the general implementation is.
* So thank you chatgpt for saving me from my previous lexer which was horrendous.
*/

pub struct Tokenizer<'a> {
    keywords: HashMap<&'static str, Kind>,
    input: &'a str,
    line: usize,
    position: usize,
    line_pos: usize,
    found_dot: bool,
}

/*
! There is definitely a better way to do this, but this works for now.
*/
impl<'a> Tokenizer<'a> {
    pub fn new(input: &str) -> Tokenizer {
        let mut keywords: HashMap<&'static str, Kind> = HashMap::new();
        keywords.insert("if", Kind::Keyword("if".to_string()));
        keywords.insert("async", Kind::Keyword("async".to_string()));
        keywords.insert("await", Kind::Keyword("await".to_string()));
        keywords.insert("const", Kind::Keyword("const".to_string()));
        keywords.insert("delete", Kind::Keyword("delete".to_string()));
        keywords.insert("null", Kind::Keyword("null".to_string()));
        keywords.insert("class", Kind::Keyword("class".to_string()));
        keywords.insert("public", Kind::Keyword("public".to_string()));
        keywords.insert("private", Kind::Keyword("private".to_string()));
        keywords.insert("get", Kind::Keyword("get".to_string()));
        keywords.insert("set", Kind::Keyword("set".to_string()));
        keywords.insert("static", Kind::Keyword("static".to_string()));
        keywords.insert("override", Kind::Keyword("override".to_string()));
        keywords.insert("new", Kind::Keyword("new".to_string()));
        keywords.insert("self", Kind::Keyword("self".to_string()));
        keywords.insert("interface", Kind::Keyword("interface".to_string()));
        keywords.insert("enum", Kind::Keyword("enum".to_string()));
        keywords.insert("try", Kind::Keyword("try".to_string()));
        keywords.insert("catch", Kind::Keyword("catch".to_string()));
        keywords.insert("throw", Kind::Keyword("throw".to_string()));
        keywords.insert("return", Kind::Keyword("return".to_string()));
        keywords.insert("break", Kind::Keyword("break".to_string()));
        keywords.insert("continue", Kind::Keyword("continue".to_string()));
        keywords.insert("until", Kind::Keyword("until".to_string()));
        keywords.insert("while", Kind::Keyword("while".to_string()));
        keywords.insert("loop", Kind::Keyword("loop".to_string()));
        keywords.insert("struct", Kind::Keyword("struct".to_string()));
        keywords.insert("typeof", Kind::Keyword("typeof".to_string()));
        keywords.insert("namespace", Kind::Keyword("namespace".to_string()));
        keywords.insert("readonly", Kind::Keyword("readonly".to_string()));
        keywords.insert("for", Kind::Keyword("for".to_string()));
        keywords.insert("true", Kind::Bool(true));
        keywords.insert("false", Kind::Bool(false));

        Tokenizer {
            keywords,
            input,
            line: 0,
            position: 0,
            line_pos: 0,
            found_dot: false,
        }
    }

    fn is_eof(&mut self, input: &str) -> bool {
        self.position >= input.len()
    }

    // spaces are supported for formatted long numbers(Python moment) and are ignored.

    fn is_number(&mut self, char: char) -> bool {
        // char.is_numeric() || char == '.' || char == '_'
        if char == '.' {
            if self.found_dot {
                return false;
            } else {
                self.found_dot = true;
                return true;
            }
        } else if char == '_' || char.is_numeric() {
            return true;
        }

        return false;
    }

    fn is_identifier(&mut self, char: char) -> bool {
        char.is_alphanumeric() || char == '_'
    }

    #[inline]
    fn process_ident(&mut self, chars: &Vec<char>, input: &str) -> (Kind, String) {
        let start = self.position;

        while !self.is_eof(input)
            && self.is_identifier(chars.get(self.position).unwrap().to_owned())
        {
            self.position += 1;
        }

        let word = &input[start..self.position];

        self.position -= 1;

        if let Some(keyword) = self.keywords.get(word) {
            (keyword.clone(), word.to_string())
        } else {
            (Kind::Identifier, word.to_string())
        }
    }

    #[inline]
    fn process_number(&mut self, chars: &Vec<char>, input: &str) -> (Kind, String) {
        return match chars.get(self.position + 1) {
            Some(chara) if chara.is_numeric() => {
                let start = self.position;

                while !self.is_eof(input)
                    && self.is_number(chars.get(self.position).unwrap().to_owned())
                {
                    self.position += 1;
                }

                self.found_dot = false;

                let num = &input[start..self.position];

                self.position -= 1;

                (Kind::Number, num.to_string())
            }
            Some(_) => (Kind::Dot, String::from(".")),
            None => (Kind::Invalid, String::from("")),
        };
    }

    #[inline]
    fn process_op(&mut self, chars: &Vec<char>, input: &str, c: char) -> (Kind, String) {
        if self.is_eof(input) {
            return (Kind::EOF, String::from(""));
        }

        let next_char = chars.get(self.position + 1).unwrap().clone();

        return match c {
            '+' | '-' | '*' | '%' | '!' => {
                if next_char == '=' {
                    let mut op = String::from(c);
                    op.push('=');

                    self.position += 1;
                    (Kind::Operator(op.clone()), String::from(op.clone()))
                } else {
                    (
                        Kind::Operator(String::from(c.clone())),
                        String::from(c.clone()),
                    )
                }
            }

            '<' | '>' => {
                if next_char == '=' || next_char == c {
                    let mut op = String::from(c);
                    op.push(next_char);

                    self.position += 1;
                    (Kind::Operator(op.clone()), String::from(op.clone()))
                } else {
                    (
                        Kind::Operator(String::from(c.clone())),
                        String::from(c.clone()),
                    )
                }
            }

            '&' | '|' | '=' => {
                if next_char == c {
                    let mut op = String::from(c);
                    op.push(next_char);

                    self.position += 1;
                    (Kind::Operator(op.clone()), String::from(op.clone()))
                } else {
                    (
                        Kind::Operator(String::from(c.clone())),
                        String::from(c.clone()),
                    )
                }
            }

            _ => (Kind::Invalid, String::from("")),
        };
    }

    #[inline]
    fn process_string(&mut self, chars: &Vec<char>, input: &str, c: char) -> (Kind, String) {
        let mut str = String::new();

        str.push(c);

        while !self.is_eof(input) {
            self.position += 1;

            let char = chars.get(self.position).unwrap().clone();

            str.push(char);

            if char == c && chars.get(self.position - 1).unwrap().clone() != '\\' {
                break;
            }
        }

        (Kind::String, str)
    }

    #[inline]
    fn process_comment(&mut self, chars: &Vec<char>, input: &str, c: char) -> (Kind, String) {
        if self.is_eof(input) {
            return (Kind::EOF, String::new());
        }

        let next_char = chars.get(self.position + 1).unwrap().clone();

        if next_char == '/' {
            let mut comment = String::new();
            self.position += 1;

            while !self.is_eof(input) {
                self.position += 1;

                if self.is_eof(input) {
                    break;
                }

                let input_char = chars.get(self.position).unwrap().clone();

                if input_char == '\n' {
                    self.position -= 1;
                    break;
                }

                comment.push(input_char);
            }

            (Kind::Comment, comment)
        } else if next_char == '=' {
            (Kind::Operator(String::from("/=")), String::from("/="))
        } else {
            (Kind::Operator(c.to_string()), c.to_string())
        }
    }

    #[inline]
    fn process_path_separator(&mut self, chars: &Vec<char>) -> (Kind, String) {
        let next_char = chars.get(self.position + 1).unwrap().clone();

        if next_char == ':' {
            self.position += 1;

            return (Kind::PathSeparator, String::from("::"));
        }

        (Kind::Invalid, String::from(""))
    }

    pub fn get_next_token(&mut self) -> Token {
        let chars: Vec<char> = self.input.chars().collect();

        if self.is_eof(self.input) {
            return Token {
                kind: Kind::EOF,
                value: String::new(),
                position: self.position + 1,
                line: self.line,
                line_position: self.line_pos,
            };
        }

        let mut c = chars.get(self.position).unwrap().clone();

        if c.is_whitespace() {
            while !self.is_eof(self.input) && chars.get(self.position).unwrap().is_whitespace() {
                self.position += 1;
            }

            c = chars.get(self.position).unwrap().clone();
        }

        let (kind, value) = match c {
			// a macro because I hated that rust-analyzer put the arm across multiple lines
			// ! does not affect performance
            match_operator!() => self.process_op(&chars, self.input, c),

            'a'..='z' | 'A'..='Z' | '_' => self.process_ident(&chars, self.input),

            '0'..='9' | '.' => self.process_number(&chars, self.input),

            '"' | '\'' => self.process_string(&chars, self.input, c),

            '/' => self.process_comment(&chars, self.input, c),

            ':' => self.process_path_separator(&chars),

            '^' => (Kind::Operator(c.to_string()), c.to_string()),
            '~' => (Kind::Tilde, c.to_string()),
            '?' => (Kind::Question, c.to_string()),
            '#' => (Kind::Hash, c.to_string()),
            ',' => (Kind::Comma, c.to_string()),
            ';' => (Kind::SemiColon, c.to_string()),
            '(' => (Kind::LParen, c.to_string()),
            ')' => (Kind::RParen, c.to_string()),
            '{' => (Kind::LBracket, c.to_string()),
            '}' => (Kind::RBracket, c.to_string()),
            '[' => (Kind::LBrace, c.to_string()),
            ']' => (Kind::RBrace, c.to_string()),

            _ => (Kind::Invalid, String::from("")),
        };

        self.position += 1;

        let true_pos = self.position - value.len() + 1;

        Token {
            kind,
            value,
            line: self.line,
            position: true_pos,
            line_position: self.line_pos,
        }
    }

    pub fn tokenize(&mut self, input: &str) -> Vec<Token> {
        let mut tokens = Vec::new();

        loop {
            let token = self.get_next_token();

            tokens.push(token.clone());

            if token.kind == Kind::EOF {
                break;
            }
        }

        tokens
    }
}
