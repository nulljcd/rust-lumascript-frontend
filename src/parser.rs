pub struct Parser<'a> {
    lexer: &'a mut super::lexer::Lexer<'a>,
    current_token: super::token::Token,
    next_token: super::token::Token,
    semicolon_skippable_next: bool,
    semicolon_skippable_now: bool,
}

impl<'a> Parser<'a> {
    #[must_use]
    pub fn new(lexer: &'a mut super::lexer::Lexer<'a>) -> Self {
        Parser {
            lexer,
            current_token: super::token::Token::Eof,
            next_token: super::token::Token::Eof,
            semicolon_skippable_next: false,
            semicolon_skippable_now: false,
        }
    }

    fn read_token(&mut self, error_handler: &mut super::error_handler::ErrorHandler) -> Option<()> {
        self.current_token = std::mem::take(&mut self.next_token);

        self.semicolon_skippable_now = self.semicolon_skippable_next;
        self.semicolon_skippable_next = false;

        loop {
            let token: super::token::Token = self.lexer.next(error_handler)?;

            match token {
                super::token::Token::NewLine => self.semicolon_skippable_next = true,
                super::token::Token::SymbolRightBrace => {
                    self.semicolon_skippable_next = true;
                    self.next_token = token;
                    break;
                }
                super::token::Token::Eof => {
                    self.semicolon_skippable_next = true;
                    self.next_token = token;
                    break;
                }
                _ => {
                    self.next_token = token;
                    break;
                }
            }
        }

        Some(())
    }

    fn peek_current_token(&self) -> &super::token::Token {
        &self.current_token
    }

    fn precedence_of_token(token: &super::token::Token) -> super::node::Precedence {
        match token {
            super::token::Token::SymbolEqual | super::token::Token::SymbolColonEqual => {
                super::node::Precedence::Assignment
            }
            super::token::Token::SymbolPipePipe | super::token::Token::SymbolAmpersandAmpersand => {
                super::node::Precedence::Logical
            }
            super::token::Token::SymbolPipe
            | super::token::Token::SymbolAmpersand
            | super::token::Token::SymbolCaret
            | super::token::Token::SymbolLessLess
            | super::token::Token::SymbolGreaterGreater => super::node::Precedence::Bitwise,
            super::token::Token::SymbolEqualEqual
            | super::token::Token::SymbolLessEqual
            | super::token::Token::SymbolGreaterEqual
            | super::token::Token::SymbolBangEqual
            | super::token::Token::SymbolLess
            | super::token::Token::SymbolGreater => super::node::Precedence::Comparative,
            super::token::Token::SymbolPlus | super::token::Token::SymbolMinus => {
                super::node::Precedence::Additive
            }
            super::token::Token::SymbolAsterisk | super::token::Token::SymbolSlash => {
                super::node::Precedence::Multiplicative
            }
            super::token::Token::SymbolLeftParenthesis => super::node::Precedence::Call,
            super::token::Token::SymbolLeftSquareBracket | super::token::Token::SymbolDot => {
                super::node::Precedence::Member
            }

            _ => super::node::Precedence::Lowest,
        }
    }

    pub fn parse(
        &mut self,
        error_handler: &mut super::error_handler::ErrorHandler,
    ) -> Option<super::node::Node> {
        self.read_token(error_handler)?;
        self.read_token(error_handler)?;

        let mut body: Vec<Box<super::node::Node>> = Vec::new();

        while self.peek_current_token() != &super::token::Token::Eof {
            let statement = self.parse_statement(error_handler)?;
            body.push(Box::new(statement));
        }

        Some(super::node::Node::ProgramStatement { body })
    }

    fn parse_statement(
        &mut self,
        error_handler: &mut super::error_handler::ErrorHandler,
    ) -> Option<super::node::Node> {
        match self.peek_current_token() {
            super::token::Token::SymbolLeftBrace => self.parse_block_statement(error_handler),
            super::token::Token::KeywordIf => self.parse_if_statement(error_handler),
            super::token::Token::KeywordLoop => self.parse_loop_statement(error_handler),
            super::token::Token::KeywordReturn => self.parse_return_statement(error_handler),
            super::token::Token::KeywordBreak => self.parse_break_statement(error_handler),
            super::token::Token::KeywordContinue => self.parse_continue_statement(error_handler),
            _ => self.parse_assignment_statement_and_expression_statement(error_handler),
        }
    }

    fn parse_block_statement(
        &mut self,
        error_handler: &mut super::error_handler::ErrorHandler,
    ) -> Option<super::node::Node> {
        self.read_token(error_handler)?;

        let mut body: Vec<Box<super::node::Node>> = Vec::new();

        while self.peek_current_token() != &super::token::Token::SymbolRightBrace {
            if self.peek_current_token() == &super::token::Token::Eof {
                error_handler.set_error(super::error_handler::Error::SyntaxError(format!(
                    "expected token: SymbolRightBrace, but got token: {}",
                    self.peek_current_token().type_as_string()
                )));

                return None;
            }

            let statement: super::node::Node = self.parse_statement(error_handler)?;

            body.push(Box::new(statement));
        }

        self.read_token(error_handler)?;

        Some(super::node::Node::BlockStatement { body })
    }

    fn parse_if_statement(
        &mut self,
        error_handler: &mut super::error_handler::ErrorHandler,
    ) -> Option<super::node::Node> {
        self.read_token(error_handler)?;

        let argument: super::node::Node =
            self.parse_expression(&super::node::Precedence::Lowest, error_handler)?;

        if self.peek_current_token() != &super::token::Token::SymbolLeftBrace {
            error_handler.set_error(super::error_handler::Error::SyntaxError(format!(
                "expected token: SymbolLeftBrace, but got token: {}",
                self.peek_current_token().type_as_string()
            )));

            return None;
        }

        let consequent_body: super::node::Node = self.parse_block_statement(error_handler)?;

        let alternate_body: super::node::Node =
            if self.peek_current_token() == &super::token::Token::KeywordElse {
                self.read_token(error_handler)?;

                if self.peek_current_token() == &super::token::Token::SymbolLeftBrace
                    || self.peek_current_token() == &super::token::Token::KeywordIf
                {
                    self.parse_statement(error_handler)?
                } else {
                    error_handler.set_error(super::error_handler::Error::SyntaxError(format!(
                        "expected token: SymbolLeftBrace or token: KeywordIf, but got token: {}",
                        self.peek_current_token().type_as_string()
                    )));

                    return None;
                }
            } else {
                super::node::Node::EmptyStatement
            };

        Some(super::node::Node::IfStatement {
            argument: Box::new(argument),
            consequent_body: Box::new(consequent_body),
            alternate_body: Box::new(alternate_body),
        })
    }

    fn parse_loop_statement(
        &mut self,
        error_handler: &mut super::error_handler::ErrorHandler,
    ) -> Option<super::node::Node> {
        self.read_token(error_handler)?;

        if self.peek_current_token() != &super::token::Token::SymbolLeftBrace {
            error_handler.set_error(super::error_handler::Error::SyntaxError(format!(
                "expected token: SymbolLeftBrace, but got token: {}",
                self.peek_current_token().type_as_string()
            )));

            return None;
        }

        let body: super::node::Node = self.parse_block_statement(error_handler)?;

        Some(super::node::Node::LoopStatement {
            body: Box::new(body),
        })
    }

    fn parse_return_statement(
        &mut self,
        error_handler: &mut super::error_handler::ErrorHandler,
    ) -> Option<super::node::Node> {
        self.read_token(error_handler)?;

        let argument: super::node::Node =
            self.parse_expression(&super::node::Precedence::Lowest, error_handler)?;

        if self.peek_current_token() == &super::token::Token::SymbolSemicolon {
            self.read_token(error_handler)?;
        } else if !self.semicolon_skippable_now {
            error_handler.set_error(super::error_handler::Error::SyntaxError(format!(
                "expected token: SymbolSemicolon, but got token: {}",
                self.peek_current_token().type_as_string()
            )));

            return None;
        }

        Some(super::node::Node::ReturnStatement {
            argument: Box::new(argument),
        })
    }

    fn parse_break_statement(
        &mut self,
        error_handler: &mut super::error_handler::ErrorHandler,
    ) -> Option<super::node::Node> {
        self.read_token(error_handler)?;

        if self.peek_current_token() == &super::token::Token::SymbolSemicolon {
            self.read_token(error_handler)?;
        } else if !self.semicolon_skippable_now {
            error_handler.set_error(super::error_handler::Error::SyntaxError(format!(
                "expected token: SymbolSemicolon, but got token: {}",
                self.peek_current_token().type_as_string()
            )));

            return None;
        }

        Some(super::node::Node::BreakStatement {})
    }

    fn parse_continue_statement(
        &mut self,
        error_handler: &mut super::error_handler::ErrorHandler,
    ) -> Option<super::node::Node> {
        self.read_token(error_handler)?;

        if self.peek_current_token() == &super::token::Token::SymbolSemicolon {
            self.read_token(error_handler)?;
        } else if !self.semicolon_skippable_now {
            error_handler.set_error(super::error_handler::Error::SyntaxError(format!(
                "expected token: SymbolSemicolon, but got token: {}",
                self.peek_current_token().type_as_string()
            )));

            return None;
        }

        Some(super::node::Node::ContinueStatement {})
    }

    fn parse_assignment_statement_and_expression_statement(
        &mut self,
        error_handler: &mut super::error_handler::ErrorHandler,
    ) -> Option<super::node::Node> {
        let argument: super::node::Node =
            self.parse_expression(&super::node::Precedence::Lowest, error_handler)?;

        match self.peek_current_token() {
            super::token::Token::SymbolEqual => {
                self.read_token(error_handler)?;

                let assignment_argument: super::node::Node =
                    self.parse_expression(&super::node::Precedence::Lowest, error_handler)?;

                if self.peek_current_token() == &super::token::Token::SymbolSemicolon {
                    self.read_token(error_handler)?;
                } else if !self.semicolon_skippable_now {
                    error_handler.set_error(super::error_handler::Error::SyntaxError(format!(
                        "expected token: SymbolSemicolon, but got token: {}",
                        self.peek_current_token().type_as_string()
                    )));

                    return None;
                }

                Some(super::node::Node::AssignmentStatement {
                    target: Box::new(argument),
                    argument: Box::new(assignment_argument),
                    is_decleration: false,
                })
            }
            super::token::Token::SymbolColonEqual => {
                self.read_token(error_handler)?;

                let assignment_argument: super::node::Node =
                    self.parse_expression(&super::node::Precedence::Lowest, error_handler)?;

                if self.peek_current_token() == &super::token::Token::SymbolSemicolon {
                    self.read_token(error_handler)?;
                } else if !self.semicolon_skippable_now {
                    error_handler.set_error(super::error_handler::Error::SyntaxError(format!(
                        "expected token: SymbolSemicolon, but got token: {}",
                        self.peek_current_token().type_as_string()
                    )));

                    return None;
                }

                Some(super::node::Node::AssignmentStatement {
                    target: Box::new(argument),
                    argument: Box::new(assignment_argument),
                    is_decleration: true,
                })
            }
            _ => {
                if self.peek_current_token() == &super::token::Token::SymbolSemicolon {
                    self.read_token(error_handler)?;
                } else if !self.semicolon_skippable_now {
                    error_handler.set_error(super::error_handler::Error::SyntaxError(format!(
                        "expected token: SymbolSemicolon, but got token: {}",
                        self.peek_current_token().type_as_string()
                    )));

                    return None;
                }

                Some(super::node::Node::ExpressionStatement {
                    argument: Box::new(argument),
                })
            }
        }
    }

    fn parse_expression(
        &mut self,
        precedence: &super::node::Precedence,
        error_handler: &mut super::error_handler::ErrorHandler,
    ) -> Option<super::node::Node> {
        let mut left_argument: super::node::Node = match self.peek_current_token() {
            super::token::Token::Literal(..) => self.parse_literal_expression(error_handler),
            super::token::Token::Identifier(..) => self.parse_identifier_expression(error_handler),
            super::token::Token::KeywordTable => self.parse_table_expression(error_handler),
            super::token::Token::KeywordFunc => self.parse_func_expression(error_handler),
            super::token::Token::SymbolLeftParenthesis => {
                self.parse_grouped_expression(error_handler)
            }
            super::token::Token::SymbolMinus
            | super::token::Token::SymbolBang => self.parse_prefix_expression(error_handler),
            _ => {
                error_handler.set_error(super::error_handler::Error::SyntaxError(format!(
                    "unexpected token: {}",
                    self.peek_current_token().type_as_string(),
                )));

                return None;
            }
        }?;

        while precedence < &Self::precedence_of_token(self.peek_current_token()) {
            match self.peek_current_token() {
                super::token::Token::SymbolPlus
                | super::token::Token::SymbolMinus
                | super::token::Token::SymbolAsterisk
                | super::token::Token::SymbolSlash
                | super::token::Token::SymbolPipe
                | super::token::Token::SymbolAmpersand
                | super::token::Token::SymbolCaret
                | super::token::Token::SymbolLessLess
                | super::token::Token::SymbolGreaterGreater
                | super::token::Token::SymbolLess
                | super::token::Token::SymbolGreater
                | super::token::Token::SymbolEqualEqual
                | super::token::Token::SymbolLessEqual
                | super::token::Token::SymbolGreaterEqual
                | super::token::Token::SymbolBangEqual
                | super::token::Token::SymbolPipePipe
                | super::token::Token::SymbolAmpersandAmpersand => {
                    left_argument = self.parse_infix_expression(left_argument, error_handler)?;
                }
                super::token::Token::SymbolLeftParenthesis => {
                    left_argument = self.parse_call_expression(left_argument, error_handler)?;
                }
                super::token::Token::SymbolLeftSquareBracket | super::token::Token::SymbolDot => {
                    left_argument = self.parse_member_expression(left_argument, error_handler)?;
                }
                _ => break,
            }
        }

        Some(left_argument)
    }

    fn parse_literal_expression(
        &mut self,
        error_handler: &mut super::error_handler::ErrorHandler,
    ) -> Option<super::node::Node> {
        let literal_expression: super::node::Node = match self.peek_current_token() {
            super::token::Token::Literal(value) => super::node::Node::LiteralExpression {
                value: value.clone(),
            },
            _ => unreachable!(),
        };

        self.read_token(error_handler)?;

        Some(literal_expression)
    }

    fn parse_identifier_expression(
        &mut self,
        error_handler: &mut super::error_handler::ErrorHandler,
    ) -> Option<super::node::Node> {
        let identifier_expression: super::node::Node = match self.peek_current_token() {
            super::token::Token::Identifier(name) => super::node::Node::IdentifierExpression {
                name: name.clone(),
            },
            _ => unreachable!(),
        };

        self.read_token(error_handler)?;

        Some(identifier_expression)
    }

    fn parse_table_expression(
        &mut self,
        error_handler: &mut super::error_handler::ErrorHandler,
    ) -> Option<super::node::Node> {
        self.read_token(error_handler)?;

        let mut properties: Vec<(Box<super::node::Node>, Box<super::node::Node>)> = Vec::new();

        if self.peek_current_token() == &super::token::Token::SymbolLeftSquareBracket {
            self.read_token(error_handler)?;

            loop {
                let key: super::node::Node = match self.peek_current_token() {
                    super::token::Token::Literal(..) => {
                        self.parse_literal_expression(error_handler)?
                    }
                    super::token::Token::Identifier(..) => {
                        self.parse_identifier_expression(error_handler)?
                    }
                    _ => {
                        error_handler.set_error(super::error_handler::Error::SyntaxError(format!(
                            "expected token: Literal or token: Identifier, but got token: {}",
                            self.peek_current_token().type_as_string()
                        )));

                        return None;
                    }
                };

                if self.peek_current_token() != &super::token::Token::SymbolColon {
                    error_handler.set_error(super::error_handler::Error::SyntaxError(format!(
                        "expected token: SymbolColon, but got token: {}",
                        self.peek_current_token().type_as_string()
                    )));

                    return None;
                }

                self.read_token(error_handler)?;

                let value: super::node::Node =
                    self.parse_expression(&super::node::Precedence::Lowest, error_handler)?;

                properties.push((Box::new(key), Box::new(value)));

                if self.peek_current_token() == &super::token::Token::SymbolComma {
                    self.read_token(error_handler)?;
                } else if self.peek_current_token()
                    == &super::token::Token::SymbolRightSquareBracket
                {
                    self.read_token(error_handler)?;
                    break;
                } else {
                    error_handler.set_error(super::error_handler::Error::SyntaxError(format!(
                        "expected token: SymbolComma, but got token: {}",
                        self.peek_current_token().type_as_string()
                    )));

                    return None;
                }
            }
        }

        Some(super::node::Node::TableExpression { properties })
    }

    fn parse_func_expression(
        &mut self,
        error_handler: &mut super::error_handler::ErrorHandler,
    ) -> Option<super::node::Node> {
        self.read_token(error_handler)?;

        let mut parameters: Vec<Box<super::node::Node>> = Vec::new();

        if matches!(
            self.peek_current_token(),
            super::token::Token::Identifier(..)
        ) {
            loop {
                if !matches!(
                    self.peek_current_token(),
                    super::token::Token::Identifier(..)
                ) {
                    error_handler.set_error(super::error_handler::Error::SyntaxError(format!(
                        "expected token: Identifier, but got token: {}",
                        self.peek_current_token().type_as_string()
                    )));

                    return None;
                }

                let parameter: super::node::Node =
                    self.parse_identifier_expression(error_handler)?;

                parameters.push(Box::new(parameter));

                if self.peek_current_token() == &super::token::Token::SymbolComma {
                    self.read_token(error_handler)?;
                } else if self.peek_current_token() == &super::token::Token::SymbolLeftBrace {
                    break;
                } else {
                    error_handler.set_error(super::error_handler::Error::SyntaxError(format!(
                        "expected token: SymbolComma, but got token: {}",
                        self.peek_current_token().type_as_string()
                    )));

                    return None;
                }
            }
        } else {
            if self.peek_current_token() != &super::token::Token::SymbolLeftBrace {
                error_handler.set_error(super::error_handler::Error::SyntaxError(format!(
                    "expected token: SymbolLeftBrace, but got token: {}",
                    self.peek_current_token().type_as_string()
                )));

                return None;
            }
        }

        let body: super::node::Node = self.parse_block_statement(error_handler)?;

        Some(super::node::Node::FuncExpression {
            parameters,
            body: Box::new(body),
        })
    }

    fn parse_grouped_expression(
        &mut self,
        error_handler: &mut super::error_handler::ErrorHandler,
    ) -> Option<super::node::Node> {
        self.read_token(error_handler)?;

        let expression: super::node::Node =
            self.parse_expression(&super::node::Precedence::Lowest, error_handler)?;

        if self.peek_current_token() != &super::token::Token::SymbolRightParenthesis {
            error_handler.set_error(super::error_handler::Error::SyntaxError(format!(
                "expected token: SymbolRightParenthesis, but got token: {}",
                self.peek_current_token().type_as_string()
            )));

            return None;
        }

        self.read_token(error_handler)?;

        Some(expression)
    }

    fn parse_prefix_expression(
        &mut self,
        error_handler: &mut super::error_handler::ErrorHandler,
    ) -> Option<super::node::Node> {
        let operator: super::token::Token = self.peek_current_token().clone();

        self.read_token(error_handler)?;

        let argument: super::node::Node =
            self.parse_expression(&super::node::Precedence::Prefix, error_handler)?;

        Some(super::node::Node::PrefixExpression {
            argument: Box::new(argument),
            operator,
        })
    }

    fn parse_infix_expression(
        &mut self,
        left_argument: super::node::Node,
        error_handler: &mut super::error_handler::ErrorHandler,
    ) -> Option<super::node::Node> {
        let operator: super::token::Token = self.peek_current_token().clone();

        self.read_token(error_handler)?;

        let precedence: super::node::Precedence = Self::precedence_of_token(&operator);

        let right_argument: super::node::Node =
            self.parse_expression(&precedence, error_handler)?;

        Some(super::node::Node::InfixExpression {
            left_argument: Box::new(left_argument),
            right_argument: Box::new(right_argument),
            operator,
        })
    }

    fn parse_call_expression(
        &mut self,
        target: super::node::Node,
        error_handler: &mut super::error_handler::ErrorHandler,
    ) -> Option<super::node::Node> {
        self.read_token(error_handler)?;

        let mut arguments: Vec<Box<super::node::Node>> = Vec::new();

        if self.peek_current_token() != &super::token::Token::SymbolRightParenthesis {
            loop {
                let argument: super::node::Node =
                    self.parse_expression(&super::node::Precedence::Lowest, error_handler)?;

                arguments.push(Box::new(argument));

                if self.peek_current_token() == &super::token::Token::SymbolComma {
                    self.read_token(error_handler)?;
                } else if self.peek_current_token() == &super::token::Token::SymbolRightParenthesis
                {
                    break;
                } else {
                    error_handler.set_error(super::error_handler::Error::SyntaxError(format!(
                        "expected token: SymbolComma, but got token: {}",
                        self.peek_current_token().type_as_string()
                    )));

                    return None;
                }
            }
        }

        self.read_token(error_handler)?;

        Some(super::node::Node::CallExpression {
            target: Box::new(target),
            arguments,
        })
    }

    fn parse_member_expression(
        &mut self,
        target: super::node::Node,
        error_handler: &mut super::error_handler::ErrorHandler,
    ) -> Option<super::node::Node> {
        match self.peek_current_token() {
            &super::token::Token::SymbolLeftSquareBracket => {
                self.read_token(error_handler)?;

                let argument: super::node::Node =
                    self.parse_expression(&super::node::Precedence::Lowest, error_handler)?;

                if self.peek_current_token() != &super::token::Token::SymbolRightSquareBracket {
                    error_handler.set_error(super::error_handler::Error::SyntaxError(format!(
                        "expected token: SymbolRightSquareBracket, but got token: {}",
                        self.peek_current_token().type_as_string()
                    )));

                    return None;
                }

                self.read_token(error_handler)?;

                Some(super::node::Node::MemberExpression {
                    target: Box::new(target),
                    argument: Box::new(argument),
                    notation_type: super::node::MemberNotationType::Bracket,
                })
            }
            &super::token::Token::SymbolDot => {
                self.read_token(error_handler)?;

                if !matches!(
                    self.peek_current_token(),
                    &super::token::Token::Identifier(..)
                ) {
                    error_handler.set_error(super::error_handler::Error::SyntaxError(format!(
                        "expected token: Identifier, but got token: {}",
                        self.peek_current_token().type_as_string()
                    )));

                    return None;
                }

                let argument: super::node::Node =
                    self.parse_identifier_expression(error_handler)?;

                Some(super::node::Node::MemberExpression {
                    target: Box::new(target),
                    argument: Box::new(argument),
                    notation_type: super::node::MemberNotationType::Dot,
                })
            }
            _ => unreachable!(),
        }
    }
}
