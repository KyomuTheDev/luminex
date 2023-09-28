use std::collections::HashMap;

macro_rules! keywords_map {
    ($($key:expr),*) => {
        {
            let mut map = HashMap::new();
            $( map.insert($key.to_owned(), Kind::Keyword($key.to_owned())); )*
            map
        }
    };
}

macro_rules! symbols_map {
    ($($key:expr),*) => {
        {
            let mut map = HashMap::new();
            $( map.insert($key, Kind::Symbol($key.to_string())); )*
            map
        }
    };
}

#[allow(clippy::all)]
#[allow(dead_code)]
#[derive(Eq, Debug, Clone, Hash)]
pub enum Kind {
    Keyword(String),
    Operator(String),
    Bool(String),
    Identifier(String),
    Comment(String),
    Number(String),
    String(String),
    Symbol(String),

    EOF,
    Invalid,
}

impl PartialEq for Kind {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Kind::Keyword(_), Kind::Keyword(_)) => true,
            (Kind::Operator(_), Kind::Operator(_)) => true,
            (Kind::Bool(_), Kind::Bool(_)) => true,
            (Kind::Identifier(_), Kind::Identifier(_)) => true,
            (Kind::Comment(_), Kind::Comment(_)) => true,
            (Kind::Number(_), Kind::Number(_)) => true,
            (Kind::String(_), Kind::String(_)) => true,
            (Kind::Symbol(_), Kind::Symbol(_)) => true,
            (Kind::EOF, Kind::EOF) => true,
            (Kind::Invalid, Kind::Invalid) => true,
            _ => false,
        }
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
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

pub struct Tokenizer {
    keywords: HashMap<String, Kind>,
    symbols: HashMap<char, Kind>,
    chars: Vec<char>,
    line: usize,
    position: usize,
    line_pos: usize,
    tokens: Vec<Token>,
}

impl Tokenizer {
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
        keywords.insert("true".to_owned(), Kind::Bool("true".to_string()));
        keywords.insert("false".to_owned(), Kind::Bool("false".to_string()));

        let symbols: HashMap<char, Kind> =
            symbols_map!('~', '`', '?', '#', ',', ';', '.', ')', '(', ']', '[', '}', '{');

        let mut lexer = Tokenizer {
            keywords,
            symbols,
            chars: input.chars().collect(),
            line: 0,
            position: 0,
            line_pos: 0,
            tokens: Vec::new(),
        };

        loop {
            let token = {
                let ref mut this = lexer;
                this.whitespace();

                this.identifier()
                    .or_else(|| -> Option<Token> { this.number() })
                    .or_else(|| -> Option<Token> { this.symbol() })
                    .or_else(|| -> Option<Token> { this.string() })
                    .or_else(|| -> Option<Token> { this.operator() })
                    .unwrap_or(this.create_token(Kind::EOF))
            };
            
            lexer.tokens.push(token.clone());

            if token.kind == Kind::EOF { break; }
        }

        lexer
    }

    fn create_token(&mut self, kind: Kind) -> Token {
        let mut pos = self.position;
        let line = self.line;
        let mut line_pos: isize = self.line_pos as isize; // does not apply to strings

        let k: Kind = match kind.clone() {
            Kind::Bool(w)
            | Kind::Keyword(w)
            | Kind::Number(w)
            | Kind::String(w)
            | Kind::Symbol(w)
            | Kind::Identifier(w)
            | Kind::Comment(w)
            | Kind::Operator(w) => {
                let count = w.chars().count() - 1;
                pos -= count;
                line_pos -= count as isize;

                kind
            }

            t => t,
        };

        Token {
            kind: k,
            position: pos,
            line: line,
            line_position: line_pos as usize,
        }
    }

    fn eat(&mut self) {
        self.position += 1;
        self.line_pos += 1;
    }

    pub fn peek(&self) -> Option<char> {
        match self.chars.get(self.position) {
            Some(c) => Some(*c),
            None => None,
        }
    }

    pub fn predict(&self, pos: usize) -> Option<char> {
        match self.chars.get(pos) {
            Some(c) => Some(*c),
            None => None,
        }
    }

    /*
    ! Functions take next character as parameter
    */

    fn letter(&mut self) -> Option<char> {
        let character = self.peek()?;

        match character {
            'a'..='z' | 'A'..='Z' => {
                self.eat();
                Some(character)
            }
            _ => None,
        }
    }

    fn digit(&mut self) -> Option<char> {
        let character = self.peek()?;

        match character {
            '0'..='9' => {
                self.eat();
                Some(character)
            }
            _ => None,
        }
    }

    fn literal(&mut self, chara: char) -> Option<char> {
        let character = self.peek()?;

        if character == chara {
            self.eat();
            Some(character)
        } else {
            None
        }
    }

    fn identifier_start(&mut self) -> Option<char> {
        self.letter()
            .or_else(|| -> Option<char> { self.literal('_') })
    }

    #[rustfmt::skip]
    fn identifier_part(&mut self) -> Option<char> {
        self.letter()
            .or_else(|| -> Option<char> { self.literal('_') })
            .or_else(|| -> Option<char> { self.digit() })
    }

    fn identifier(&mut self) -> Option<Token> {
        let mut identifier = self.identifier_start()?.to_string();

        while let Some(part) = self.identifier_part() {
            identifier.push(part);
        }

        if let Some(kind) = self.keywords.get(&identifier) {
            Some(self.create_token(kind.clone()))
        } else {
            Some(self.create_token(Kind::Identifier(identifier)))
        }
    }

    fn string(&mut self) -> Option<Token> {
        let start = self.peek()?;

        if start != '\'' && start != '\"' {
            return None;
        }

        let mut string = String::new();

        self.eat();

        while let Some(c) = self.peek() {
            if c == start && self.predict(self.position - 1) != Some('\\') {
                self.eat();
                break;
            }

            string.push(c);
            self.eat();
        }

        Some(self.create_token(Kind::String(string)))
    }

    fn number(&mut self) -> Option<Token> {
        let start = self.peek()?;
        let mut found_dot = start == '.';

        let mut number = start.to_string();

        if !start.is_numeric() && !found_dot {
            return None;
        }

        if found_dot {
            let next = self.predict(self.position + 1)?;

            if !next.is_numeric() {
                return None;
            }
        }

        self.eat();

        while let Some(digit) = self.digit().or_else(|| -> Option<char> {
            if found_dot {
                return None;
            }

            self.literal('.')
        }) {
            if digit == '.' && !found_dot {
                found_dot = true;
            }

            number.push(digit);
        }

        Some(self.create_token(Kind::Number(number)))
    }

    fn operator(&mut self) -> Option<Token> {
        let operator = self.peek()?;

        let mut operation = operator.to_string();

        self.eat();

        match operator {
            '*' | '=' | '>' | '<' => {
                let next_op = self.peek()?;

                if operator == next_op || next_op == '=' {
                    self.eat();

                    operation.push(next_op);
                }
            }

            '%' | '+' | '-' | '!' => {
                let next_op = self.peek()?;

                if next_op == '=' {
                    self.eat();

                    operation.push(next_op);
                }
            }

            '&' | '|' | '^' | ':' => {
                let next_op = self.peek()?;

                if next_op == operator {
                    self.eat();

                    operation.push(next_op);
                }
            }

            '/' => {
                let next_op = self.peek()?;

                match next_op {
                    '/' => {
                        operation.clear();

                        self.eat();

                        while let Some(next) = self.peek() {
                            self.eat();

                            operation.push(next);

                            if self.predict(self.position + 1)? == '\n' {
                                break;
                            }
                        }

                        return Some(self.create_token(Kind::Comment(operation)));
                    }

                    '*' => {
                        operation.clear();

                        self.eat();

                        while let Some(next) = self.peek() {
                            let mut predict = self.predict(self.position + 1)?;

                            if predict == '*' {
                                predict = self.predict(self.position + 2)?;

                                if predict == '/' {
                                    self.eat();
                                    self.eat();
                                    self.eat();

                                    break;
                                }
                            }

                            self.eat();

                            operation.push(next);
                        }

                        return Some(self.create_token(Kind::Comment(operation)));
                    }

                    '=' => {
                        operation.push(next_op);
                    }

                    _ => {}
                }
            }

            _ => {
                return None;
            }
        };

        Some(self.create_token(Kind::Operator(operation)))
    }

    fn symbol(&mut self) -> Option<Token> {
        let start = self.peek()?;

        let symbol = self.symbols.get(&start)?.clone();

        self.eat();

        return Some(self.create_token(symbol.clone()));
    }

    fn whitespace(&mut self) -> Option<()> {
        let mut space = self.peek()?;

        if space.is_whitespace() {
            self.eat();

            loop {
                space = self.peek()?;

                if !space.is_whitespace() {
                    break;
                }

                match space {
                    '\n' => {
                        self.line += 1;
                        self.line_pos = 0;
                    }

                    _ => {}
                }

                self.eat();
            }
        }

        Some(())
    }

    pub fn next(&mut self) -> Token {
        self.tokens.remove(0)
    }

    pub fn peek_token(&mut self) -> Token {
        self.tokens.get(0).unwrap().clone()
    }
}
