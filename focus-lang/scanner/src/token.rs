#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Token {
    Ampersand,   // &
    At,          // @
    Backslash,   // \
    Colon,       // :
    Comma,       // ,
    Dash,        // -
    Dot,         // .
    DoubleQuote, // "
    Empty,       // \t or space
    Eof,         // End of file
    Assign,      // =
    Exclamation, // !
    Greater,     // >
    Ident,
    LeftCurly,   // {
    LeftParen,   // (
    LeftSquare,  // [
    Less,        // <
    NewLine,     // \n
    Number,      // 123
    Plus,        // +
    Pipe,        // |
    RightCurly,  // }
    RightParen,  // )
    RightSquare, // ]
    SingleQuote, // '
    Slash,       // /
    Star,        // *
    Hat,         // ^
    Percent,     // %
    Unknown,

    ThinArrow,    // ->
    Equal,        // ==
    GreaterEqual, // >=
    LessEqual,    // <=
    NotEqual,     // !=
    Or,           // ||
    And,          // &&
    Unit,         // ()
    Range,        // ..

    // Keywords
    Let,
    Match,
    If,
    Then,
    Else,
    For,
    Use,
    Module,
    Pub,
    Trait,
    Type,
    True,
    False,
    In,
    Do,
    Fn,
}

impl From<&str> for Token {
    fn from(value: &str) -> Self {
        match value {
            "\n" => Token::NewLine,
            "&" => Token::Ampersand,
            "@" => Token::At,
            "\\" => Token::Backslash,
            ":" => Token::Colon,
            "," => Token::Comma,
            "-" => Token::Dash,
            "." => Token::Dot,
            "\"" => Token::DoubleQuote,
            "=" => Token::Assign,
            "!" => Token::Exclamation,
            ">" => Token::Greater,
            "{" => Token::LeftCurly,
            "(" => Token::LeftParen,
            "[" => Token::LeftSquare,
            "<" => Token::Less,
            "+" => Token::Plus,
            "|" => Token::Pipe,
            "}" => Token::RightCurly,
            ")" => Token::RightParen,
            "]" => Token::RightSquare,
            "'" => Token::SingleQuote,
            "/" => Token::Slash,
            "*" => Token::Star,
            "^" => Token::Hat,
            "%" => Token::Percent,

            "->" => Token::ThinArrow,
            "==" => Token::Equal,
            ">=" => Token::GreaterEqual,
            "<=" => Token::LessEqual,
            "||" => Token::Or,
            "&&" => Token::And,
            "!=" => Token::NotEqual,
            "()" => Token::Unit,
            ".." => Token::Range,

            "let" => Token::Let,
            "match" => Token::Match,
            "if" => Token::If,
            "then" => Token::Then,
            "else" => Token::Else,
            "for" => Token::For,
            "use" => Token::Use,
            "module" => Token::Module,
            "pub" => Token::Pub,
            "type" => Token::Type,
            "true" => Token::True,
            "false" => Token::False,
            "in" => Token::In,
            "do" => Token::Do,
            "fn" => Token::Fn,
            c if c.chars().next().is_some_and(|c| c.is_whitespace()) => Token::Empty,
            c if c.chars().next().is_some_and(|c| c.is_numeric()) => Token::Number,
            c if c
                .chars()
                .next()
                .is_some_and(|c| c.is_alphanumeric() || c == '_') =>
            {
                Token::Ident
            }
            "" => Token::Eof,
            _ => Token::Unknown,
        }
    }
}

impl Token {
    pub fn is_empty(&self) -> bool {
        matches!(self, Token::NewLine | Token::Empty | Token::Eof)
    }

    pub fn is_operator(&self) -> bool {
        matches!(
            self,
            Token::Plus
                | Token::Dash
                | Token::Star
                | Token::Slash
                | Token::Percent
                | Token::Assign
                | Token::Equal
                | Token::NotEqual
                | Token::Greater
                | Token::GreaterEqual
                | Token::Less
                | Token::LessEqual
                | Token::And
                | Token::Or
        )
    }

    pub fn is_literal(&self) -> bool {
        matches!(
            self,
            Token::Unit
                | Token::Number
                | Token::DoubleQuote
                | Token::True
                | Token::False
                | Token::SingleQuote
        )
    }

    pub fn is_primary(&self) -> bool {
        self.is_literal()
            || matches!(
                self,
                Token::LeftParen
                    | Token::Ident
                    | Token::LeftSquare
                    | Token::Ampersand
                    | Token::Star
            )
    }

    pub fn is_keyword(&self) -> bool {
        matches!(
            self,
            Token::Let
                | Token::Match
                | Token::If
                | Token::Then
                | Token::Else
                | Token::For
                | Token::Use
                | Token::Module
                | Token::Pub
                | Token::Trait
                | Token::Type
                | Token::True
                | Token::False
                | Token::In
                | Token::Fn
        )
    }
}
