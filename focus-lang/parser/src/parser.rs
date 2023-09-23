use scanner::{Scanner, ScannerPosition, Token};

use crate::{
    op::{ArithmeticOperator, BooleanOperator, ComparisonOperator},
    stmt::{
        AliasItem, Call, Closure, Expression, For, IfElse, Index, Item, ItemStmt, LetStmt, Literal,
        Match, MatchBranch, Operation, Operator, Range, Stmt, StmtDetails, Struct, StructField,
        StructItem, StructItemField, Visibility,
    },
    FunctionType, ParserError, ParserErrorInfo, Type,
};

#[derive(Clone)]
pub struct Parser<'a> {
    filename: Option<String>,
    scanner: Scanner<'a>,
    depth: usize,
    allow_top_level: bool,
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a str, filename: Option<String>) -> Self {
        Self {
            filename,
            scanner: Scanner::new(source),
            depth: 0,
            allow_top_level: false,
        }
    }

    pub fn new_with_top_level(source: &'a str, filename: Option<String>) -> Self {
        Self {
            filename,
            scanner: Scanner::new(source),
            depth: 0,
            allow_top_level: true,
        }
    }

    pub fn parse_error_details(&mut self) -> Result<StmtDetails, ParserError> {
        let start_position = self.scanner.position();
        self.parse()
            .map_err(|info| ParserError {
                message: self.error_message(start_position.clone()),
                location: self.error_location(),
                info,
            })
            .map(|stmt| StmtDetails {
                file: self.filename.clone(),
                defined_at: start_position.pos..self.scanner.position().pos,
                stmt,
            })
    }

    pub fn parse(&mut self) -> Result<Stmt, ParserErrorInfo> {
        self.skip_empty_lines();

        let visibility = if self.depth == 0 {
            Some(self.parse_vibility()?)
        } else {
            None
        };

        let res = match self.scanner.peek_skip_empty() {
            Token::Let => {
                self.depth += 1;
                let stmt = self.parse_let(visibility).map(|res| Stmt::Let(res))?;
                self.depth -= 1;
                stmt
            }
            Token::Type | Token::Use | Token::Module => self
                .parse_item_stmt(visibility)
                .map(|res| Stmt::Item(res))?,
            _ => {
                self.depth += 1;
                let stmt = self.parse_expr().map(|res| Stmt::Expr(res))?;
                self.depth -= 1;
                stmt
            }
        };

        if !self.scanner.check_skip_empty(Token::NewLine)
            && self.depth == 0
            && !self.scanner.check_skip_empty(Token::Eof)
        {
            Err(ParserErrorInfo::UnexpectedToken {
                found: self.scanner.advance_skip_empty(),
            })
        } else {
            Ok(res)
        }
    }

    fn parse_type(&mut self) -> Result<Type, ParserErrorInfo> {
        if self.scanner.check_and_consume_indented(Token::Ident) {
            Ok(Type::Name(self.scanner.slice().to_string()))
        } else if self.scanner.check_and_consume_indented(Token::Unit) {
            Ok(Type::Unit)
        } else if self.scanner.check_and_consume_indented(Token::LeftParen) {
            let mut args = Vec::new();
            let ret;

            loop {
                let ty = self.parse_type()?;

                if self.scanner.check_and_consume_indented(Token::ThinArrow) {
                    args.push(ty);
                } else if self.scanner.check_and_consume_indented(Token::RightParen) {
                    ret = ty;
                    break;
                } else {
                    return Err(ParserErrorInfo::UnexpectedToken {
                        found: self.scanner.advance_indented().unwrap_or(Token::Unknown),
                    });
                }
            }

            Ok(Type::Function(FunctionType {
                args,
                ret: Box::new(ret),
            }))
        } else {
            Err(ParserErrorInfo::UnexpectedToken {
                found: self.scanner.advance_indented().unwrap_or(Token::Unknown),
            })
        }
    }

    fn skip_empty_lines(&mut self) {
        while self.scanner.peek_skip_empty() == Token::NewLine {
            self.scanner.advance();
        }
    }

    fn parse_vibility(&mut self) -> Result<Visibility, ParserErrorInfo> {
        if self.scanner.check_and_consume_skip_empty(Token::Pub) {
            if self.scanner.check_and_consume_skip_empty(Token::Module) {
                Ok(Visibility::Module)
            } else {
                Ok(Visibility::Public)
            }
        } else {
            Ok(Visibility::Private)
        }
    }

    fn parse_path(&mut self) -> Result<String, ParserErrorInfo> {
        let mut path = String::new();
        self.expect_skip_empty(Token::Ident)?;
        path.push_str(self.scanner.slice());
        loop {
            if !self.scanner.check_and_consume(Token::Dot) {
                break;
            }

            self.expect(Token::Ident)?;
            path.push('.');
            path.push_str(self.scanner.slice());
        }
        Ok(path)
    }

    pub fn parse_let(
        &mut self,
        visibility: Option<Visibility>,
    ) -> Result<LetStmt, ParserErrorInfo> {
        self.expect_skip_empty(Token::Let)?;
        self.expect_skip_empty(Token::Ident)?;
        let ident = self.scanner.slice().to_string();

        let mut args = Vec::new();

        while self.scanner.check_and_consume_indented(Token::Ident) {
            args.push(self.scanner.slice().to_string());
        }

        if self.scanner.check_and_consume_indented(Token::Unit) {
            args.push("".to_string());
        }

        let ty = if self.scanner.check_and_consume_indented(Token::Colon) {
            Some(self.parse_type()?)
        } else {
            None
        };

        let expr = if self.scanner.check_and_consume_indented(Token::Assign) {
            Some(self.parse_block()?.into())
        } else {
            None
        };

        Ok(LetStmt {
            ident,
            visibility,
            args,
            expr,
            ty,
        })
    }

    pub fn parse_expr(&mut self) -> Result<Expression, ParserErrorInfo> {
        if self.depth == 0 && !self.allow_top_level {
            return Err(ParserErrorInfo::ExpressionNotValidInThisPosition);
        }

        match self.scanner.peek_skip_empty() {
            Token::If => self.parse_if(),
            Token::For => self.parse_for(),
            Token::Match => self.parse_match(),
            Token::Range => self.parse_range(),
            Token::Fn => self.parse_closure(),
            Token::Number if self.scanner.peek_skip_empty_nth(2) == Some(Token::Range) => {
                self.parse_range()
            }
            Token::Ident => {
                let mut cloned = self.clone();
                cloned.parse_path()?;
                match cloned.scanner.peek_indented() {
                    Some(Token::Range) => self.parse_range(),
                    Some(Token::LeftCurly) => self.parse_struct(),
                    Some(Token::LeftSquare) => self.parse_index(),
                    Some(t) if t.is_operator() => self.parse_operation(),
                    Some(t) if t != Token::Eof => self.parse_call(),
                    _ => Ok(Expression::Path(self.parse_path()?)),
                }
            }
            Token::Eof => return Err(ParserErrorInfo::Eof),
            t if t.is_primary() => self.parse_operation(),
            _ => Err(ParserErrorInfo::UnexpectedToken {
                found: self.scanner.advance_skip_empty(),
            }),
        }
    }

    fn parse_index(&mut self) -> Result<Expression, ParserErrorInfo> {
        let path = self.parse_path()?;
        self.expect_skip_empty(Token::LeftSquare)?;
        let index = self.parse_operation()?.into();
        self.expect_skip_empty(Token::RightSquare)?;
        Ok(Expression::Index(Index {
            value: Expression::Path(path).into(),
            index,
        }))
    }

    fn parse_struct(&mut self) -> Result<Expression, ParserErrorInfo> {
        self.expect_indented(Token::Ident)?;
        let path = self.scanner.slice().to_string();
        self.expect_ignore(Token::LeftCurly)?;

        let mut fields = Vec::new();

        loop {
            if self.scanner.check_ignore(Token::RightCurly) {
                break;
            }

            self.expect_ignore(Token::Ident)?;
            let ident = self.scanner.slice().to_string();
            self.expect_skip_empty(Token::Colon)?;
            let expr = self.parse_operation()?.into();
            self.scanner.check_and_consume(Token::Comma);
            fields.push(StructField { ident, expr })
        }

        self.expect_ignore(Token::RightCurly)?;
        Ok(Expression::Struct(Struct { path, fields }))
    }

    fn parse_match(&mut self) -> Result<Expression, ParserErrorInfo> {
        self.expect_skip_empty(Token::Match)?;
        let match_expr = self.parse_expr()?.into();
        let mut branches = Vec::new();
        self.expect_skip_empty(Token::NewLine)?;
        loop {
            if self.scanner.check_and_consume_ignore(Token::Pipe) {
                let match_expr = self.parse_expr()?.into();
                let if_expr = if self.scanner.check_and_consume_skip_empty(Token::If) {
                    Some(self.parse_operation()?.into())
                } else {
                    None
                };
                self.expect_skip_empty(Token::ThinArrow)?;
                let expr = self.parse_block()?.into();
                branches.push(MatchBranch {
                    match_expr,
                    if_expr,
                    expr,
                });
            } else {
                break;
            }
        }
        Ok(Expression::Match(Match {
            match_expr,
            branches,
        }))
    }

    fn parse_if(&mut self) -> Result<Expression, ParserErrorInfo> {
        self.expect_skip_empty(Token::If)?;
        let indentation = self.scanner.cursor().line_indentation();
        let condition = self.parse_operation()?.into();
        self.expect_skip_empty(Token::Then)?;
        let if_expr = self.parse_block()?.into();
        let else_expr = if self.scanner.check_and_consume_ignore(Token::Else)
            && self.scanner.cursor().line_indentation() == indentation
        {
            if self.scanner.check_skip_empty(Token::If) {
                Some(self.parse_if()?.into())
            } else {
                Some(self.parse_block()?.into())
            }
        } else {
            None
        };
        Ok(Expression::IfElse(IfElse {
            condition,
            if_expr,
            else_expr,
        }))
    }

    fn parse_for(&mut self) -> Result<Expression, ParserErrorInfo> {
        self.expect_skip_empty(Token::For)?;
        self.expect_skip_empty(Token::Ident)?;
        let ident = self.scanner.slice().to_string();
        self.expect_skip_empty(Token::In)?;
        let in_expr = self.parse_expr()?.into();
        self.expect_skip_empty(Token::Do)?;
        let block = self.parse_block()?.into();
        Ok(Expression::For(For {
            ident,
            in_expr,
            block,
        }))
    }

    fn parse_block(&mut self) -> Result<Expression, ParserErrorInfo> {
        let indentation = self.scanner.cursor().line_indentation();
        let is_multiline = self.scanner.check_and_consume_skip_empty(Token::NewLine);

        if is_multiline {
            if self.scanner.cursor().line_indentation() <= indentation {
                return Err(ParserErrorInfo::InvalidIndentation);
            }

            let mut stmts = Vec::new();
            loop {
                match self.parse() {
                    Ok(expr) => stmts.push(expr),
                    Err(ParserErrorInfo::Eof) => {
                        if stmts.is_empty() {
                            return Err(ParserErrorInfo::EarlyEof);
                        } else {
                            break;
                        }
                    }
                    Err(err) => return Err(err),
                }
                let mut s = self.scanner.clone();
                s.advance_ignore();
                if s.cursor().line_indentation() <= indentation {
                    break;
                }
            }
            if !stmts.is_empty() {
                Ok(Expression::Block(stmts))
            } else {
                Err(ParserErrorInfo::EarlyEof)
            }
        } else {
            Ok(Expression::Block(vec![self.parse()?]))
        }
    }

    fn parse_call(&mut self) -> Result<Expression, ParserErrorInfo> {
        let path = self.parse_path()?;
        let mut params = Vec::new();
        while self.scanner.peek_indented().is_some_and(|t| t.is_primary()) {
            self.scanner.check_and_consume_skip_empty(Token::NewLine);
            params.push(self.parse_primary()?);
        }
        let call = Expression::Call(Call { path, params });

        if self.scanner.check_and_consume_indented(Token::At) {
            self.scanner.check_and_consume_skip_empty(Token::NewLine);
            match self.parse_expr()? {
                Expression::Call(mut c) => {
                    c.params.push(call);
                    Ok(Expression::Call(c))
                }
                Expression::Path(path) => Ok(Expression::Call(Call {
                    path,
                    params: vec![call],
                })),
                expr => Err(ParserErrorInfo::InvalidExpression {
                    found: expr.name().to_string(),
                    expected: "Call".to_string(),
                }),
            }
        } else {
            Ok(call)
        }
    }

    fn parse_operator(&mut self) -> Option<Operator> {
        match self.scanner.peek_indented() {
            Some(Token::Plus) => Some(Operator::Arithmetic(ArithmeticOperator::Add)),
            Some(Token::Dash) => Some(Operator::Arithmetic(ArithmeticOperator::Sub)),
            Some(Token::Slash) => Some(Operator::Arithmetic(ArithmeticOperator::Div)),
            Some(Token::Star) => Some(Operator::Arithmetic(ArithmeticOperator::Mul)),
            Some(Token::Percent) => Some(Operator::Arithmetic(ArithmeticOperator::Mod)),
            Some(Token::And) => Some(Operator::Boolean(BooleanOperator::And)),
            Some(Token::Or) => Some(Operator::Boolean(BooleanOperator::Or)),
            Some(Token::Greater) => Some(Operator::Comparison(ComparisonOperator::Greater)),
            Some(Token::Less) => Some(Operator::Comparison(ComparisonOperator::Less)),
            Some(Token::GreaterEqual) => {
                Some(Operator::Comparison(ComparisonOperator::GreaterEqual))
            }
            Some(Token::LessEqual) => Some(Operator::Comparison(ComparisonOperator::LessEqual)),
            Some(Token::Equal) => Some(Operator::Comparison(ComparisonOperator::Equal)),
            Some(Token::NotEqual) => Some(Operator::Comparison(ComparisonOperator::NotEqual)),
            Some(Token::Assign) => Some(Operator::Assignment),
            _ => None,
        }
    }

    fn parse_operation(&mut self) -> Result<Expression, ParserErrorInfo> {
        let mut left = self.parse_primary()?;
        let mut prev_prec = 0;

        loop {
            let op = match self.parse_operator() {
                Some(op) => op,
                None => return Ok(left),
            };

            let cur_prec = op.precedence();
            if cur_prec < prev_prec {
                return Ok(left);
            }

            self.scanner.advance_indented(); // consume op token
            self.scanner.check_and_consume_skip_empty(Token::NewLine);

            let mut cloned = self.clone();
            cloned.parse_primary()?;
            let next_op = cloned.parse_operator();

            let right = if next_op.is_some_and(|op| op.precedence() > cur_prec) {
                prev_prec += 1;
                self.parse_operation()?
            } else {
                self.parse_primary()?
            };

            left = Expression::Operation(Operation {
                lhs: left.into(),
                rhs: right.into(),
                op,
            })
        }
    }

    fn parse_primary(&mut self) -> Result<Expression, ParserErrorInfo> {
        match self.scanner.peek_skip_empty() {
            t if t.is_literal() => self.parse_literal(),
            Token::Ident => Ok(Expression::Path(self.parse_path()?)),
            Token::LeftParen => {
                self.scanner.advance_skip_empty();
                let expr = self.parse_expr()?;
                self.expect_ignore(Token::RightParen)?;
                Ok(expr)
            }
            Token::Fn => self.parse_closure(),
            Token::LeftSquare => {
                self.scanner.advance_skip_empty();
                let mut exprs = Vec::new();
                loop {
                    let expr = match self.scanner.peek_ignore() {
                        Token::Eof => return Err(ParserErrorInfo::EarlyEof),
                        Token::RightSquare => break,
                        _ => self.parse_expr()?,
                    };
                    exprs.push(expr);
                    if self.scanner.check_and_consume_ignore(Token::Comma) {
                        if self.scanner.check_ignore(Token::RightSquare) {
                            return Err(ParserErrorInfo::UnexpectedToken {
                                found: Token::RightSquare,
                            });
                        }
                    }
                }
                self.scanner.advance_ignore(); // skip right square bracket
                Ok(Expression::Array(exprs))
            }
            _ => Err(ParserErrorInfo::UnexpectedToken {
                found: self.scanner.advance_skip_empty(),
            }),
        }
    }

    fn parse_range(&mut self) -> Result<Expression, ParserErrorInfo> {
        let from = match self.scanner.peek_skip_empty() {
            Token::Number => Some(self.parse_literal()?.into()),
            Token::Ident => Some(Expression::Path(self.parse_path()?).into()),
            Token::Range => None,
            _ => return Err(ParserErrorInfo::InvalidOperandForRangeExpression),
        };
        self.expect_skip_empty(Token::Range)?;
        let to = match self.scanner.peek_skip_empty() {
            Token::Number => Some(self.parse_literal()?.into()),
            Token::Ident => Some(Expression::Path(self.parse_path()?).into()),
            _ => None,
        };
        Ok(Expression::Range(Range { from, to }))
    }

    fn parse_literal(&mut self) -> Result<Expression, ParserErrorInfo> {
        match self.scanner.peek_skip_empty() {
            Token::Number => {
                self.scanner.advance_skip_empty();
                let mut str = self.scanner.slice().to_string();
                if self.scanner.check_and_consume(Token::Dot) {
                    str.push('.');
                    self.expect(Token::Number)?;
                    str.push_str(self.scanner.slice());
                    let num = str
                        .parse::<f64>()
                        .map_err(|e| ParserErrorInfo::UnableToParse {
                            expr: str,
                            err: e.to_string(),
                        })?;
                    Ok(Expression::Literal(Literal::Float(num)))
                } else {
                    let num = str
                        .parse::<i64>()
                        .map_err(|e| ParserErrorInfo::UnableToParse {
                            expr: str,
                            err: e.to_string(),
                        })?;
                    Ok(Expression::Literal(Literal::Integer(num)))
                }
            }
            Token::True => {
                self.scanner.advance_skip_empty();
                Ok(Expression::Literal(Literal::Boolean(true)))
            }
            Token::False => {
                self.scanner.advance_skip_empty();
                Ok(Expression::Literal(Literal::Boolean(false)))
            }
            Token::SingleQuote => {
                self.scanner.advance_skip_empty();
                self.scanner.advance();
                let ch = self.scanner.slice();
                if ch.chars().count() != 1 {
                    return Err(ParserErrorInfo::InvalidCharacterLiteral);
                }
                let ch = ch.chars().next().unwrap();
                self.expect(Token::SingleQuote)?;
                Ok(Expression::Literal(Literal::Char(ch)))
            }
            Token::Unit => {
                self.scanner.advance_skip_empty();
                Ok(Expression::Literal(Literal::Unit))
            }
            Token::DoubleQuote => {
                self.scanner.advance_skip_empty();
                let mut str = String::new();
                loop {
                    let token = self.scanner.advance();
                    if token == Token::Eof {
                        return Err(ParserErrorInfo::EarlyEof);
                    } else if token != Token::DoubleQuote {
                        str.push_str(self.scanner.slice());
                    } else {
                        break;
                    }
                }
                Ok(Expression::Literal(Literal::String(str.into())))
            }
            _ => {
                let token = self.scanner.advance_skip_empty();
                Err(ParserErrorInfo::UnexpectedToken { found: token })
            }
        }
    }

    fn parse_closure(&mut self) -> Result<Expression, ParserErrorInfo> {
        self.expect_indented(Token::Fn)?;
        let mut args = Vec::new();
        while self.scanner.check_and_consume_indented(Token::Ident) {
            args.push(self.scanner.slice().to_string());
        }
        self.expect_indented(Token::ThinArrow)?;
        let block = self.parse_block()?.into();
        Ok(Expression::Closure(Closure { args, block }))
    }

    fn parse_item_stmt(
        &mut self,
        visibility: Option<Visibility>,
    ) -> Result<ItemStmt, ParserErrorInfo> {
        if self.depth > 0 {
            return Err(ParserErrorInfo::StatementNotValidInThisPosition);
        }

        let item = match self.scanner.peek_skip_empty() {
            Token::Type => self.parse_type_stmt()?,
            Token::Module => self.parse_module_stmt()?,
            Token::Use => self.parse_use_stmt()?,
            _ => todo!(),
        };

        Ok(ItemStmt {
            item,
            visibility: visibility.unwrap_or(Visibility::Private),
        })
    }

    fn parse_type_stmt(&mut self) -> Result<Item, ParserErrorInfo> {
        self.expect_skip_empty(Token::Type)?;
        self.expect_skip_empty(Token::Ident)?;
        let ident = self.scanner.slice().to_string();

        if self.scanner.check_and_consume_skip_empty(Token::Assign) {
            if self.scanner.check_and_consume_skip_empty(Token::LeftCurly) {
                let mut fields = Vec::new();

                loop {
                    if self.scanner.check_and_consume_ignore(Token::RightCurly) {
                        break;
                    }

                    self.skip_empty_lines();
                    let visibility = if self.depth == 0 {
                        self.parse_vibility()?
                    } else {
                        Visibility::Private
                    };

                    self.expect_ignore(Token::Ident)?;
                    let ident = self.scanner.slice().to_string();
                    self.expect_ignore(Token::Colon)?;
                    let ty = self.parse_type()?;
                    fields.push(StructItemField {
                        ident,
                        visibility,
                        ty,
                    });
                    self.scanner.check_and_consume_ignore(Token::Comma);
                }

                Ok(Item::Struct(StructItem { ident, fields }))
            } else {
                let path = Some(self.parse_path()?);
                Ok(Item::Alias(AliasItem { ident, path }))
            }
        } else {
            Ok(Item::Alias(AliasItem { ident, path: None }))
        }
    }

    fn parse_module_stmt(&mut self) -> Result<Item, ParserErrorInfo> {
        self.expect_skip_empty(Token::Module)?;
        self.expect_skip_empty(Token::Ident)?;
        let ident = self.scanner.slice().to_string();
        Ok(Item::ModuleDeclaration(ident))
    }

    fn parse_use_stmt(&mut self) -> Result<Item, ParserErrorInfo> {
        self.expect_skip_empty(Token::Use)?;
        let path = self.parse_path()?;
        Ok(Item::UseDeclaration(path))
    }

    fn expect(&mut self, token: Token) -> Result<(), ParserErrorInfo> {
        let next_token = self.scanner.advance();
        if next_token == token {
            Ok(())
        } else if next_token == Token::Eof {
            Err(ParserErrorInfo::EarlyEof)
        } else {
            Err(ParserErrorInfo::UnexpectedTokenExpected {
                found: next_token,
                expected: token,
            })
        }
    }

    fn expect_skip_empty(&mut self, token: Token) -> Result<(), ParserErrorInfo> {
        let next_token = self.scanner.advance_skip_empty();
        if next_token == token {
            Ok(())
        } else if next_token == Token::Eof {
            Err(ParserErrorInfo::EarlyEof)
        } else {
            Err(ParserErrorInfo::UnexpectedTokenExpected {
                found: next_token,
                expected: token,
            })
        }
    }

    fn expect_ignore(&mut self, token: Token) -> Result<(), ParserErrorInfo> {
        let next_token = self.scanner.advance_ignore();
        if next_token == token {
            Ok(())
        } else if next_token == Token::Eof {
            Err(ParserErrorInfo::EarlyEof)
        } else {
            Err(ParserErrorInfo::UnexpectedTokenExpected {
                found: next_token,
                expected: token,
            })
        }
    }

    fn expect_indented(&mut self, token: Token) -> Result<(), ParserErrorInfo> {
        let next_token = self.scanner.advance_indented();
        if next_token == Some(token) {
            Ok(())
        } else if next_token == Some(Token::Eof) {
            Err(ParserErrorInfo::EarlyEof)
        } else {
            Err(ParserErrorInfo::UnexpectedTokenExpected {
                found: next_token.unwrap_or(Token::Eof),
                expected: token,
            })
        }
    }

    fn error_message(&self, start_position: ScannerPosition) -> String {
        let end_position = self.scanner.position();
        let len = end_position.pos - start_position.pos;
        let sentence = self.scanner.source()[start_position.pos..]
            .chars()
            .enumerate()
            .take_while(|(i, c)| *i < len - 1 || *c != '\n')
            .map(|(_, c)| c)
            .collect::<String>();

        let token_len = self.scanner.slice().len();

        format!(
            "{}\n{}{}",
            sentence,
            " ".repeat(end_position.col.checked_sub(token_len).unwrap_or(0)),
            "^".repeat(token_len)
        )
    }

    fn error_location(&self) -> String {
        let position = self.scanner.position();
        let token_len = self.scanner.slice().len();
        format!(
            "{}:{}:{}",
            self.filename.as_ref().unwrap_or(&"<source>".to_string()),
            position.row + 1,
            position.col + 1 - token_len
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::Parser;

    #[test]
    fn parse_operation() {
        let mut parser = Parser::new_with_top_level("1 - 2 * 3 + 4 / 5", None);
        println!("{:?}", parser.parse_expr());
        let mut parser = Parser::new_with_top_level("index = index + 1", None);
        println!("{:?}", parser.parse_expr());
    }

    #[test]
    fn parse_array() {
        let mut parser = Parser::new_with_top_level("[1 + 3, 2, 3, 4, 5]", None);
        println!("{:?}", parser.parse_expr());
    }

    #[test]
    fn parse_if() {
        let mut parser =
            Parser::new_with_top_level("if a == b then 2 else if b == c then 3 else 4", None);
        println!("{:?}", parser.parse_expr());
    }

    #[test]
    fn parse_chained_calls() {
        let mut parser = Parser::new_with_top_level("do_something 1 @ do_something_else", None);
        println!("{:?}", parser.parse_expr());
    }

    #[test]
    fn parse_for() {
        let mut parser = Parser::new_with_top_level("for i in 0..100 do something ()", None);
        println!("{:?}", parser.parse_expr());
    }

    #[test]
    fn parse_for_multiline() {
        let mut parser = Parser::new_with_top_level(
            "for i in 0..100 do \n something () \n\t something_else ()",
            None,
        );
        println!("{:?}", parser.parse_expr());
    }

    #[test]
    fn parse_match() {
        let mut parser = Parser::new_with_top_level(
            r#"match test.a
            | 1 if b > 2 -> 
                do_something ()
                2
            | 2 if b == 2 -> 3"#,
            None,
        );
        println!("{:?}", parser.parse_expr());
    }

    #[test]
    fn parse_closure() {
        let mut parser = Parser::new(
            r#"let fun = 
            fn a -> 
                let b = a + 1
                b + 1"#,
            None,
        );
        println!("{:?}", parser.parse());
    }

    #[test]
    fn parse_if_else() {
        let mut parser = Parser::new(r#"let a = if true then 1"#, None);
        println!("{:?}", parser.parse());
    }

    #[test]
    fn parse_struct() {
        let mut parser = Parser::new(r#"let line = Line{a:a,b:b}"#, None);
        println!("{:?}", parser.parse());
    }
}
