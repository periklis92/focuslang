/// Splits a [`str`] source to multiple tokens.
#[derive(Clone)]
pub struct Cursor<'a> {
    source: &'a str,
    position: usize,
    row: usize,
    col: usize,
    last_token_start_position: usize,
    indentation: usize,
}

impl<'a> Cursor<'a> {
    pub fn new(source: &'a str) -> Self {
        let mut cursor = Self {
            source,
            position: 0,
            row: 0,
            col: 0,
            last_token_start_position: 0,
            indentation: 0,
        };
        cursor.calculate_indentation();
        cursor
    }

    fn calculate_indentation(&mut self) {
        self.indentation = self
            .source
            .chars()
            .skip(self.position)
            .take_while(|c| c.is_whitespace())
            .count();
    }

    pub fn advance(&mut self, min_chars: usize) -> &str {
        self.last_token_start_position = self.position;
        let start_position = self.position;
        let mut end_position = self.position;
        let chars = self.source[start_position..].chars();

        let is_whitespace = chars
            .clone()
            .peekable()
            .peek()
            .is_some_and(|c| c.is_whitespace());

        let is_alpha = chars
            .clone()
            .peekable()
            .peek()
            .is_some_and(|c| c.is_alphanumeric() || *c == '_');

        let mut recalc_indentation = false;
        let mut chars_count = 0;

        for c in chars {
            if c.is_whitespace() && is_whitespace {
                end_position += c.len_utf8();
                self.col += 1;
                if c == '\n' {
                    self.col = 0;
                    self.row += 1;
                    recalc_indentation = true;
                }
                break;
            } else if (c.is_alphanumeric() == is_alpha
                && !c.is_whitespace()
                && (is_alpha || (!is_alpha && chars_count < min_chars)))
                || (is_alpha && c == '_')
            {
                end_position += c.len_utf8();
                self.col += 1;
                chars_count += 1;
            } else {
                break;
            }
        }

        self.position = end_position;
        if recalc_indentation {
            self.calculate_indentation();
        }
        &self.source[start_position..end_position]
    }

    pub fn source(&self) -> &str {
        self.source
    }

    pub fn line_indentation(&self) -> usize {
        self.indentation
    }

    pub fn col(&self) -> usize {
        self.col
    }

    pub fn row(&self) -> usize {
        self.row
    }

    pub fn start_position(&self) -> usize {
        self.last_token_start_position
    }

    pub fn position(&self) -> usize {
        self.position
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn multiline_text() {
        let mut cursor = Cursor::new("let a = b==-2 \n a\nb");
        assert_eq!(cursor.advance(1), "let");
        assert_eq!(cursor.advance(1), " ");
        assert_eq!(cursor.advance(1), "a");
        assert_eq!(cursor.advance(1), " ");
        assert_eq!(cursor.advance(1), "=");
        assert_eq!(cursor.advance(1), " ");
        assert_eq!(cursor.advance(1), "b");
        assert_eq!(cursor.advance(2), "==");
        assert_eq!(cursor.advance(1), "-");
        assert_eq!(cursor.advance(1), "2");
        assert_eq!(cursor.advance(1), " ");
        assert_eq!(cursor.advance(1), "\n");
        assert_eq!(cursor.advance(1), " ");
        assert_eq!(cursor.advance(1), "a");
        assert_eq!(cursor.advance(1), "\n");
        assert_eq!(cursor.advance(1), "b");
    }

    #[test]
    fn utf8() {
        let mut cursor = Cursor::new("let a = \"Καλησπέρα Κόσμε.\"");
        assert_eq!(cursor.advance(1), "let");
        assert_eq!(cursor.advance(1), " ");
        assert_eq!(cursor.advance(1), "a");
        assert_eq!(cursor.advance(1), " ");
        assert_eq!(cursor.advance(1), "=");
        assert_eq!(cursor.advance(1), " ");
        assert_eq!(cursor.advance(1), "\"");
        assert_eq!(cursor.advance(1), "Καλησπέρα");
        assert_eq!(cursor.advance(1), " ");
        assert_eq!(cursor.advance(1), "Κόσμε");
        assert_eq!(cursor.advance(1), ".");
        assert_eq!(cursor.advance(1), "\"");
    }

    #[test]
    fn var_name() {
        let mut cursor = Cursor::new("let long_name");
        assert_eq!(cursor.advance(1), "let");
        assert_eq!(cursor.advance(1), " ");
        assert_eq!(cursor.advance(1), "long_name");
    }

    #[test]
    fn indentation() {
        let mut cursor = Cursor::new("let first\n\t");
        assert_eq!(cursor.advance(1), "let");
        assert_eq!(cursor.advance(1), " ");
        assert_eq!(cursor.line_indentation(), 0);
        assert_eq!(cursor.advance(1), "first");
        assert_eq!(cursor.line_indentation(), 0);
        assert_eq!(cursor.advance(1), "\n");
        assert_eq!(cursor.line_indentation(), 1);
        assert_eq!(cursor.advance(1), "\t");
        assert_eq!(cursor.line_indentation(), 1);
    }
}
