pub struct Lexer<'a> {
    char_indices: std::str::CharIndices<'a>,
    current_character: Option<char>,
    next_character: Option<char>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut lexer: Lexer<'_> = Lexer {
            char_indices: input.char_indices(),
            current_character: None,
            next_character: None,
        };

        lexer.read_character();
        lexer.read_character();

        lexer
    }

    fn read_character(&mut self) {
        self.current_character = std::mem::take(&mut self.next_character);

        if let Some((_, c)) = self.char_indices.next() {
            self.next_character = Some(c);
        }
    }

    fn peek_current_character(&self) -> Option<&char> {
        self.current_character.as_ref()
    }

    fn peek_next_character(&mut self) -> Option<&char> {
        self.next_character.as_ref()
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek_current_character() {
            match c {
                ' ' | '\t' => {
                    self.read_character();
                }
                _ => break,
            }
        }
    }

    fn consume_int(&mut self) -> Option<super::token::Token> {
        let mut value: String = String::new();

        while let Some(c) = self.peek_current_character() {
            match c {
                '0'..='9' => {
                    value.push(*c);
                    self.read_character();
                }
                _ => break,
            }
        }

        Some(super::token::Token::Literal(std::rc::Rc::from(value)))
    }

    fn consume_word(&mut self) -> Option<super::token::Token> {
        let mut value: String = String::new();

        while let Some(c) = self.peek_current_character() {
            match c {
                'a'..='z' | 'A'..='Z' | '_' | '0'..='9' => {
                    value.push(*c);
                    self.read_character();
                }
                _ => break,
            }
        }

        Some(match value.as_str() {
            "none" | "true" | "false" => super::token::Token::Literal(std::rc::Rc::from(value)),
            "if" => super::token::Token::KeywordIf,
            "else" => super::token::Token::KeywordElse,
            "loop" => super::token::Token::KeywordLoop,
            "table" => super::token::Token::KeywordTable,
            "func" => super::token::Token::KeywordFunc,
            "return" => super::token::Token::KeywordReturn,
            "break" => super::token::Token::KeywordBreak,
            "continue" => super::token::Token::KeywordContinue,
            _ => super::token::Token::Identifier(std::rc::Rc::from(value)),
        })
    }

    pub fn next(
        &mut self,
        error_handler: &mut super::error_handler::ErrorHandler,
    ) -> Option<super::token::Token> {
        self.skip_whitespace();

        if let Some(c) = self.peek_current_character() {
            match c {
                '0'..='9' => self.consume_int(),
                'a'..='z' | 'A'..='Z' | '_' => self.consume_word(),
                '=' => {
                    let token: super::token::Token = if self.peek_next_character() == Some(&'=') {
                        self.read_character();
                        super::token::Token::SymbolEqualEqual
                    } else {
                        super::token::Token::SymbolEqual
                    };

                    self.read_character();

                    Some(token)
                }
                '+' => {
                    let token: super::token::Token = super::token::Token::SymbolPlus;

                    self.read_character();

                    Some(token)
                }
                '-' => {
                    let token: super::token::Token = super::token::Token::SymbolMinus;

                    self.read_character();

                    Some(token)
                }
                '*' => {
                    let token: super::token::Token = super::token::Token::SymbolAsterisk;

                    self.read_character();

                    Some(token)
                }
                '/' => {
                    let token: super::token::Token = super::token::Token::SymbolSlash;

                    self.read_character();

                    Some(token)
                }
                '!' => {
                    let token: super::token::Token = if self.peek_next_character() == Some(&'=') {
                        self.read_character();
                        super::token::Token::SymbolBangEqual
                    } else {
                        super::token::Token::SymbolBang
                    };

                    self.read_character();

                    Some(token)
                }
                '<' => {
                    let token: super::token::Token = if self.peek_next_character() == Some(&'<') {
                        self.read_character();
                        super::token::Token::SymbolLessLess
                    } else if self.peek_next_character() == Some(&'=') {
                        self.read_character();
                        super::token::Token::SymbolLessEqual
                    } else {
                        super::token::Token::SymbolLess
                    };

                    self.read_character();

                    Some(token)
                }
                '>' => {
                    let token: super::token::Token = if self.peek_next_character() == Some(&'>') {
                        self.read_character();
                        super::token::Token::SymbolGreaterGreater
                    } else if self.peek_next_character() == Some(&'=') {
                        self.read_character();
                        super::token::Token::SymbolGreaterEqual
                    } else {
                        super::token::Token::SymbolGreater
                    };

                    self.read_character();

                    Some(token)
                }
                '&' => {
                    let token: super::token::Token = if self.peek_next_character() == Some(&'&') {
                        self.read_character();
                        super::token::Token::SymbolAmpersandAmpersand
                    } else {
                        super::token::Token::SymbolAmpersand
                    };

                    self.read_character();

                    Some(token)
                }
                '|' => {
                    let token: super::token::Token = if self.peek_next_character() == Some(&'|') {
                        self.read_character();
                        super::token::Token::SymbolPipePipe
                    } else {
                        super::token::Token::SymbolPipe
                    };

                    self.read_character();

                    Some(token)
                }
                '^' => {
                    let token: super::token::Token = super::token::Token::SymbolCaret;

                    self.read_character();

                    Some(token)
                }
                '(' => {
                    let token: super::token::Token = super::token::Token::SymbolLeftParenthesis;

                    self.read_character();

                    Some(token)
                }
                ')' => {
                    let token: super::token::Token = super::token::Token::SymbolRightParenthesis;

                    self.read_character();

                    Some(token)
                }
                '{' => {
                    let token: super::token::Token = super::token::Token::SymbolLeftBrace;

                    self.read_character();

                    Some(token)
                }
                '}' => {
                    let token: super::token::Token = super::token::Token::SymbolRightBrace;

                    self.read_character();

                    Some(token)
                }
                '[' => {
                    let token: super::token::Token = super::token::Token::SymbolLeftSquareBracket;

                    self.read_character();

                    Some(token)
                }
                ']' => {
                    let token: super::token::Token = super::token::Token::SymbolRightSquareBracket;

                    self.read_character();

                    Some(token)
                }
                ',' => {
                    let token: super::token::Token = super::token::Token::SymbolComma;

                    self.read_character();

                    Some(token)
                }
                '.' => {
                    let token: super::token::Token = super::token::Token::SymbolDot;

                    self.read_character();

                    Some(token)
                }
                ':' => {
                    let token: super::token::Token = if self.peek_next_character() == Some(&'=') {
                        self.read_character();
                        super::token::Token::SymbolColonEqual
                    } else {
                        super::token::Token::SymbolColon
                    };

                    self.read_character();

                    Some(token)
                }
                ';' => {
                    let token: super::token::Token = super::token::Token::SymbolSemicolon;

                    self.read_character();

                    Some(token)
                }
                '\n' => {
                    let token: super::token::Token = super::token::Token::NewLine;

                    self.read_character();

                    Some(token)
                }
                _ => {
                    error_handler.set_error(super::error_handler::Error::SyntaxError(format!(
                        "unexpected character: {c}"
                    )));

                    None
                }
            }
        } else {
            Some(super::token::Token::Eof)
        }
    }
}
