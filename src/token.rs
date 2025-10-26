#[derive(PartialEq, Clone, Debug)]
pub enum Token {
    Literal(std::rc::Rc<str>),
    Identifier(std::rc::Rc<str>),

    KeywordIf,
    KeywordElse,
    KeywordLoop,
    KeywordTable,
    KeywordFunc,
    KeywordReturn,
    KeywordBreak,
    KeywordContinue,

    SymbolEqual,
    SymbolColonEqual,
    SymbolPlus,
    SymbolMinus,
    SymbolAsterisk,
    SymbolSlash,
    SymbolBang,
    SymbolPipe,
    SymbolAmpersand,
    SymbolCaret,
    SymbolLessLess,
    SymbolGreaterGreater,
    SymbolEqualEqual,
    SymbolLess,
    SymbolGreater,
    SymbolBangEqual,
    SymbolLessEqual,
    SymbolGreaterEqual,
    SymbolPipePipe,
    SymbolAmpersandAmpersand,
    SymbolLeftParenthesis,
    SymbolRightParenthesis,
    SymbolLeftBrace,
    SymbolRightBrace,
    SymbolLeftSquareBracket,
    SymbolRightSquareBracket,
    SymbolComma,
    SymbolDot,
    SymbolColon,
    SymbolSemicolon,

    NewLine,
    Eof,
}

impl Default for Token {
    fn default() -> Self {
        Token::Eof
    }
}

impl Token {
    #[must_use]
    pub fn type_as_string(&self) -> &'static str {
        match self {
            Token::Literal(..) => "Literal",
            Token::Identifier(..) => "Identifier",
            Token::KeywordIf => "KeywordIf",
            Token::KeywordElse => "KeywordElse",
            Token::KeywordLoop => "KeywordLoop",
            Token::KeywordTable => "KeywordTable",
            Token::KeywordFunc => "KeywordFunc",
            Token::KeywordReturn => "KeywordReturn",
            Token::KeywordBreak => "KeywordBreak",
            Token::KeywordContinue => "KeywordContinue",
            Token::SymbolEqual => "SymbolEqual",
            Token::SymbolColonEqual => "SymbolColonEqual",
            Token::SymbolPlus => "SymbolPlus",
            Token::SymbolMinus => "SymbolMinus",
            Token::SymbolAsterisk => "SymbolAsterisk",
            Token::SymbolSlash => "SymbolSlash",
            Token::SymbolBang => "SymbolBang",
            Token::SymbolPipe => "SymbolPipe",
            Token::SymbolAmpersand => "SymbolAmpersand",
            Token::SymbolCaret => "SymbolCaret",
            Token::SymbolLessLess => "SymbolLessLess",
            Token::SymbolGreaterGreater => "SymbolGreaterGreater",
            Token::SymbolEqualEqual => "SymbolEqualEqual",
            Token::SymbolLess => "SymbolLess",
            Token::SymbolGreater => "SymbolGreater",
            Token::SymbolBangEqual => "SymbolBangEqual",
            Token::SymbolLessEqual => "SymbolLessEqual",
            Token::SymbolGreaterEqual => "SymbolGreaterEqual",
            Token::SymbolPipePipe => "SymbolPipePipe",
            Token::SymbolAmpersandAmpersand => "SymbolAmpersandAmpersand",
            Token::SymbolLeftParenthesis => "SymbolLeftParenthesis",
            Token::SymbolRightParenthesis => "SymbolRightParenthesis",
            Token::SymbolLeftBrace => "SymbolLeftBrace",
            Token::SymbolRightBrace => "SymbolRightBrace",
            Token::SymbolLeftSquareBracket => "SymbolLeftSquareBracket",
            Token::SymbolRightSquareBracket => "SymbolRightSquareBracket",
            Token::SymbolComma => "SymbolComma",
            Token::SymbolDot => "SymbolDot",
            Token::SymbolColon => "SymbolColon",
            Token::SymbolSemicolon => "SymbolSemicolon",
            Token::NewLine => "NewLine",
            Token::Eof => "Eof",
        }
    }
}
