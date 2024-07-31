#[derive(PartialEq, Debug, Clone)]
pub enum TokenType {
    Semicolon,
    LParenthesis,
    RParenthesis,
    LBrace,
    RBrace,
    LBracket,
    RBracket,

    Assign,
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulus,
    Power,

    Equal,
    NotEqual,
    GreaterThan,
    LessThan,
    GreaterThanOrEqual,
    LessThanOrEqual,

    And,
    Or,
    Not,

    If,
    Else,
    While,
    For,
    Return,
    
    Declare,
    Constant,
    Function,

    Identifier,
    Quotes,
    Number,

    Unknown,
    EOF,
}

#[derive(PartialEq, Debug, Clone)]
pub enum Literal {
    Isize(isize),
    String(String),
    Float(f64),
    Bool(bool),
    None,
}

#[derive(PartialEq, Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: Literal
}

impl Token {
    pub fn new(token_type: TokenType, literal: Literal) -> Self {
        Self {
            token_type,
            literal
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct Lexer {
    input: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Self {
            input: input,
            tokens: vec![],
            start: 0,
            current: 0,
        }
    }

    pub fn tokenize_all(&mut self) -> Vec<Token> {
        while !self.eof() {
            self.tokenize()
        }
        self.add_token_without_literal(TokenType::EOF);
        self.tokens.clone()
    }

    fn tokenize(&mut self) {
        self.start = self.current;
        let ch = self.advance();
        match ch {
            ';' => self.add_token_without_literal(TokenType::Semicolon),
            '(' => self.add_token_without_literal(TokenType::LParenthesis),
            ')' => self.add_token_without_literal(TokenType::RParenthesis),
            '{' => self.add_token_without_literal(TokenType::LBrace),
            '}' => self.add_token_without_literal(TokenType::RBrace),
            '[' => self.add_token_without_literal(TokenType::LBracket),
            ']' => self.add_token_without_literal(TokenType::RBracket),
            '+' => self.add_token_without_literal(TokenType::Add),
            '-' => self.add_token_without_literal(TokenType::Subtract),
            '*' => self.add_token_without_literal(TokenType::Multiply),
            '/' => self.add_token_without_literal(TokenType::Divide),
            '%' => self.add_token_without_literal(TokenType::Modulus),
            '^' => self.add_token_without_literal(TokenType::Power),
            '&' => self.add_token_without_literal(TokenType::And),
            '|' => self.add_token_without_literal(TokenType::Or),
            '=' => {
                let token = if self.expect('=') {
                    TokenType::Assign
                } else {
                    TokenType::Equal
                };
                self.add_token_without_literal(token)
            }
            '!' => {
                let token = if self.expect('=') {
                    TokenType::NotEqual
                } else {
                    TokenType::Not
                };
                self.add_token_without_literal(token)
            }
            '>' => {
                let token = if self.expect('=') {
                    TokenType::GreaterThanOrEqual
                } else {
                    TokenType::GreaterThan
                };
                self.add_token_without_literal(token)
            }
            '<' => {
                let token = if self.expect('=') {
                    TokenType::LessThanOrEqual
                } else {
                    TokenType::LessThan
                };
                self.add_token_without_literal(token)
            }

            ' ' | '\n' | '\r' | '\t' => {}
            '"' | '\'' => self.string(ch), 
            '0'..='9' => self.number(),
            'a'..='z' | 'A'..='Z' | '_' => self.identifier(),
            _ => self.add_token_without_literal(TokenType::Unknown),
        }
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        self.input.chars().nth(self.current-1).unwrap()
    }

    fn peek(&mut self) -> char {
        if self.eof() {
            return '\0';
        }

        self.input.chars().nth(self.current).unwrap()
    }

    fn add_token_without_literal(&mut self, token_type: TokenType) {
        self.add_token_with_literal(token_type, Literal::None)
    }
    
    fn add_token_with_literal(&mut self, token_type: TokenType, literal: Literal) {
        self.tokens.push(Token::new(
                token_type,
                literal,
        ))
    }

    fn expect(&mut self, expected: char) -> bool {
        if self.peek() != expected {
            return false;
        }
        self.current += 1;
        true
    }

    fn eof(&mut self) -> bool {
        self.current >= self.input.len()
    }

    fn number(&mut self) {
        while self.peek().is_digit(10) {
            self.advance();
        }

        if self.peek() == '.' {
            self.advance();

            if !self.peek().is_digit(10) {
                self.add_token_without_literal(TokenType::Unknown);
                return;
            }

            while self.peek().is_digit(10) {
                self.advance();
            }
        }

        let literal = self.input.get(self.start..self.current).unwrap();
        let literal_isize = literal.parse::<isize>();
        let literal_float = literal.parse::<f64>();

        let literal = if literal_isize.is_ok() {
            Literal::Isize(literal_isize.ok().unwrap())
        } else if literal_float.is_ok() {
            Literal::Float(literal_float.ok().unwrap())
        } else {
            panic!("")
        };
        self.add_token_with_literal(TokenType::Number, literal)
    }

    fn string(&mut self, quotation: char) {
        while self.peek() != quotation && !self.eof() {
            self.advance();
        }

        if self.eof() {
            self.add_token_without_literal(TokenType::Unknown);
            return;
        }

        self.advance();

        let str_value = self.input.get(self.start + 1..self.current - 1).unwrap();
        self.add_token_with_literal(TokenType::Quotes, Literal::String(str_value.to_string()));
    }

    fn identifier(&mut self) {
        while self.peek().is_alphanumeric() {
            self.advance();
        }

        let literal = self.input.get(self.start..self.current).unwrap();
        let token_type = match literal {
            "if" => TokenType::If,
            "else" => TokenType::Else,
            "while" => TokenType::While,
            "for" => TokenType::For,
            "return" => TokenType::Return,

            "let" => TokenType::Declare,
            "const" => TokenType::Constant,
            "fn" => TokenType::Function,

            _ => TokenType::Identifier,
        };

        if token_type == TokenType::Identifier {
            self.add_token_with_literal(token_type, Literal::String(literal.to_string()));
        } else {
            self.add_token_without_literal(token_type);
        }
    }
}
