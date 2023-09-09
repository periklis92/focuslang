use crate::{Cursor, Token};

#[derive(Debug, Default, Clone)]
pub struct ScannerPosition {
    pub col: usize,
    pub row: usize,
    pub pos: usize,
}

/// Converts a [`str`] source to [`Token`]
#[derive(Clone)]
pub struct Scanner<'a> {
    cursor: Cursor<'a>,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        let cursor = Cursor::new(source);
        Self { cursor }
    }

    pub fn source(&self) -> &str {
        self.cursor.source()
    }

    pub fn cursor(&'a self) -> &'a Cursor<'a> {
        &self.cursor
    }

    pub fn advance(&mut self) -> Token {
        match self.cursor.clone().advance(2).into() {
            Token::Unknown => self.cursor.advance(1).into(),
            token => {
                self.cursor.advance(2);
                token
            }
        }
    }

    pub fn advance_skip_empty(&mut self) -> Token {
        let mut token = self.advance();
        while token == Token::Empty {
            token = self.advance()
        }
        token
    }

    pub fn advance_ignore(&mut self) -> Token {
        let mut token = self.advance();
        while token == Token::Empty || token == Token::NewLine {
            token = self.advance()
        }
        token
    }

    pub fn advance_indented(&mut self) -> Option<Token> {
        let mut start_indentation = None;

        if self.peek_skip_empty() == Token::NewLine {
            start_indentation = Some(self.cursor.line_indentation());
            while self.check_and_consume_skip_empty(Token::NewLine) {}
        }

        if (start_indentation.is_none()
            || start_indentation.is_some_and(|i| i < self.cursor.line_indentation()))
            && self.peek_skip_empty() != Token::NewLine
        {
            Some(self.advance_skip_empty())
        } else {
            None
        }
    }

    pub fn peek(&self) -> Token {
        self.clone().advance()
    }

    pub fn peek_skip_empty(&self) -> Token {
        self.clone().advance_skip_empty()
    }

    pub fn peek_indented(&self) -> Option<Token> {
        self.clone().advance_indented()
    }

    pub fn peek_ignore(&self) -> Token {
        self.clone().advance_ignore()
    }

    pub fn peek_skip_empty_nth(&self, i: usize) -> Option<Token> {
        let mut c = self.clone();
        (0..i).map(|_| c.advance_skip_empty()).last()
    }

    pub fn peek_ignore_nth(&self, i: usize) -> Option<Token> {
        let mut c = self.clone();
        (0..i).map(|_| c.advance_ignore()).last()
    }

    pub fn check(&self, token: Token) -> bool {
        self.peek() == token
    }

    pub fn check_skip_empty(&self, token: Token) -> bool {
        self.peek_skip_empty() == token
    }

    pub fn check_indented(&self, token: Token) -> bool {
        self.peek_indented().is_some_and(|t| t == token)
    }

    pub fn check_ignore(&self, token: Token) -> bool {
        self.peek_ignore() == token
    }

    pub fn check_and_consume(&mut self, token: Token) -> bool {
        if self.check(token) {
            self.advance();
            true
        } else {
            false
        }
    }

    pub fn check_and_consume_skip_empty(&mut self, token: Token) -> bool {
        if self.check_skip_empty(token) {
            self.advance_skip_empty();
            true
        } else {
            false
        }
    }

    pub fn check_and_consume_indented(&mut self, token: Token) -> bool {
        if self.check_indented(token) {
            self.advance_indented();
            true
        } else {
            false
        }
    }

    pub fn check_and_consume_ignore(&mut self, token: Token) -> bool {
        if self.check_ignore(token) {
            self.advance_ignore();
            true
        } else {
            false
        }
    }

    pub fn slice(&self) -> &str {
        &self.cursor.source()[self.cursor.start_position()..self.cursor.position()]
    }

    pub fn position(&self) -> ScannerPosition {
        ScannerPosition {
            col: self.cursor.col(),
            row: self.cursor.row(),
            pos: self.cursor.position(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_tokens() {
        let mut scanner = Scanner::new("let a = 2");
        assert_eq!(scanner.advance(), Token::Let);
        assert_eq!(scanner.advance(), Token::Empty);
        assert_eq!(scanner.advance(), Token::Ident);
        assert_eq!(scanner.advance(), Token::Empty);
        assert_eq!(scanner.advance(), Token::Assign);
        assert_eq!(scanner.advance(), Token::Empty);
        assert_eq!(scanner.advance(), Token::Number);
    }

    #[test]
    fn parse_ident() {
        let mut scanner = Scanner::new("let long_ident");
        assert_eq!(scanner.advance(), Token::Let);
        assert_eq!(scanner.advance(), Token::Empty);
        assert_eq!(scanner.advance(), Token::Ident);
        assert_eq!(scanner.slice(), "long_ident");
    }

    #[test]
    fn skip_empty() {
        let mut scanner = Scanner::new("let a = 2");
        assert_eq!(scanner.advance_skip_empty(), Token::Let);
        assert_eq!(scanner.advance_skip_empty(), Token::Ident);
        assert_eq!(scanner.advance_skip_empty(), Token::Assign);
        assert_eq!(scanner.advance_skip_empty(), Token::Number);
    }

    #[test]
    fn only_indented() {
        let mut scanner = Scanner::new("let a =\n\t2\n3");
        assert_eq!(scanner.advance_indented(), Some(Token::Let));
        assert_eq!(scanner.advance_indented(), Some(Token::Ident));
        assert_eq!(scanner.advance_indented(), Some(Token::Assign));
        assert_eq!(scanner.advance_indented(), Some(Token::Number));
        assert_eq!(scanner.advance_indented(), None);
    }

    #[test]
    fn double_char() {
        let mut scanner = Scanner::new("let a =!true == false");
        assert_eq!(scanner.advance_skip_empty(), Token::Let);
        assert_eq!(scanner.advance_skip_empty(), Token::Ident);
        assert_eq!(scanner.advance_skip_empty(), Token::Assign);
        assert_eq!(scanner.advance_skip_empty(), Token::Exclamation);
        assert_eq!(scanner.advance_skip_empty(), Token::True);
        assert_eq!(scanner.advance_skip_empty(), Token::Equal);
        assert_eq!(scanner.advance_skip_empty(), Token::False);
    }
}
