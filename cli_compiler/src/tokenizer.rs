use crate::utils::get_error_message;
use std::collections::HashMap;

macro_rules! match_operator {
    () => {
        '+' | '-' | '*' | '/' | '%' | '!' | '<' | '>' | '&' | '|' | '^' | '='
    };
}

macro_rules! keywords_map {
    ($($key:expr),*) => {
        {
            let mut map = HashMap::new();
            $( map.insert($key.to_owned(), Kind::Keyword($key.to_owned())); )*
            map
        }
    };
}

#[allow(clippy::all)]
#[allow(dead_code)]
#[derive(PartialEq, Eq, Debug, Clone, Hash)]
pub enum Kind {
    Keyword(String),
    Operator(String),
    Bool(bool),
    Identifier(String),
    Comment(String),
    Number(String),
    String(String),

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

    /* The position of the last character in the value in the input string */
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
    keywords: HashMap<String, Kind>,
    input: &'a str,
    chars: Vec<char>,
    line: usize,
    position: usize,
    line_pos: usize,
    found_dot: bool,
}

impl<'a> Tokenizer<'a> {
    pub fn new(input: &str) -> Tokenizer {
        let mut keywords: HashMap<String, Kind> = keywords_map!(
            "if",
            "async",
            "await",
            "const",
            "delete",
            "null",
            "class",
            "public",
            "private",
            "get",
            "set",
            "static",
            "override",
            "new",
            "self",
            "enum",
            "try",
            "catch",
            "throw",
            "return",
            "break",
            "continue",
            "until",
            "while",
            "loop",
            "struct",
            "typeof",
            "namespace",
            "readonly",
            "for"
        );
        keywords.insert("true".to_owned(), Kind::Bool(true));
        keywords.insert("false".to_owned(), Kind::Bool(false));

        Tokenizer {
            keywords,
            input,
            chars: input.chars().collect(),
            line: 0,
            position: 0,
            line_pos: 0,
            found_dot: false,
        }
    }

    fn create_token(&mut self, kind: Kind) -> Token {
        let pos = self.position;
        let line = self.line;
        let line_pos = self.line_pos;

        let mut deincrement = 1;

        match &kind {
            Kind::Keyword(w)
            | Kind::Identifier(w)
            | Kind::Operator(w)
            | Kind::Comment(w)
            | Kind::Number(w)
            | Kind::String(w) => {
                deincrement = w.len();
            }

            Kind::Bool(b) => {
                if *b {
                    deincrement = 4;
                } else {
                    deincrement = 5;
                }
            }

            Kind::PathSeparator => {
                deincrement = 2;
            }

            _ => (),
        };

        Token {
            kind,
            position: pos - deincrement,
            line,
            line_position: line_pos - deincrement,
        }
    }

    fn whitespace(&mut self, char: char) -> bool {
        match char {
            '\n' => {
                self.line += 1;
                self.line_pos = 0;
                self.position += 1;
            }
            '\t' => {
                self.line_pos += 1;
                self.position += 1;
            }
            _ => (),
        };

        char.is_whitespace()
    }

    fn op(&mut self, op: char) -> Token {
        let next = match self.peek() {
            Some(c) => c,
            None => {
                return self.create_token(Kind::Invalid)
            }
        };

        let mut inc = 1;

        let k = match op {
            '+' | '-' | '*' | '/' | '<' | '>' | '=' => {
                if op == next || next == '=' {
                    let mut fin = op.to_string();
                    fin.push(next);

                    inc = 2;

                    Kind::Operator(fin)
                } else {
                    Kind::Operator(op.to_string())
                }
            }

            '&' | '|' => {
                if next == op {
                    let mut fin = op.to_string();
                    fin.push(next);

                    inc = 2;

                    Kind::Operator(fin)
                } else {
                    Kind::Operator(op.to_string())
                }
            }

            '!' | '%' | '^' => Kind::Operator(op.to_string()),

            _ => Kind::Invalid,
        };

        self.position += inc;

        self.create_token(k)
    }

    fn number(&mut self, start: char) -> Token {
        let mut found_dot = false;
        let mut number = start.to_string();

        if start == '.' {
            found_dot = true;
        }

        loop {
            let c = match self.peek() {
                Some(c) => c,
                None => return self.create_token(Kind::Invalid),
            };

            self.position += 1;

            if c.is_numeric() {
                number.push(c);
            } else if c == '.' {
                if found_dot {
                    break;
                } else {
                    found_dot = true;
                    number.push(c);
                }
            } else {
                break;
            }
        }

        self.create_token(Kind::Number(number))
    }

    fn string(&mut self, start: char) -> Token {
        let mut string = start.to_string();

        loop {
            let c = match self.peek() {
                Some(c) => c,
                None => return self.create_token(Kind::Invalid),
            };

            self.position += 1;

            if c == start && !self.check(self.position - 1, '\\') {
                string.push(c);
                break;
            }

            string.push(c);
        }

        self.create_token(Kind::String(string))
    }

    fn identifier(&mut self, start: char) -> Token {
        let mut identifier = start.to_string();

        loop {
            let c = match self.peek() {
                Some(c) => c,
                None => return self.create_token(Kind::Invalid),
            };

            match c {
                'a'..='z' | 'A'..='Z' | '_' => {
                    self.position += 1;
                    identifier.push(c);
                }
                _ => break,
            }
        }

        if let Some(key) = self.keywords.get(identifier.as_str()) {
            self.create_token(key.clone())
        } else {
            self.create_token(Kind::Identifier(identifier))
        }
    }

    fn comment(&mut self, start: char) -> Token {
        let mut comment = start.to_string();

        loop {
            let c = match self.peek() {
                Some(c) => c,
                None => return self.create_token(Kind::Invalid),
            };

            if c == '*' {
                self.position += 1;

                let next = match self.peek() {
                    Some(c) => c,
                    None => return self.create_token(Kind::Invalid),
                };

                if next == '/' {
                    self.position += 1;
                    break ();
                } else {
                    return self.op(c);
                }
            }

            comment.push(c);
        }

        self.create_token(Kind::Comment(comment))
    }

    fn peek(&self) -> Option<char> {
        self.chars.get(self.position + 1).map(|c| *c)
    }

    fn check(&self, pos: usize, char: char) -> bool {
        self.chars.get(pos).map(|c| *c) == Some(char)
    }

    pub fn next(&mut self) {}
}
