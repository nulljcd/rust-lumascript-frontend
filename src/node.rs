#[derive(Debug)]
pub enum Node {
    ProgramStatement {
        body: Vec<Box<Node>>,
    },
    BlockStatement {
        body: Vec<Box<Node>>,
    },
    IfStatement {
        argument: Box<Node>,
        consequent_body: Box<Node>,
        alternate_body: Box<Node>,
    },
    LoopStatement {
        body: Box<Node>,
    },
    ReturnStatement {
        argument: Box<Node>,
    },
    BreakStatement,
    ContinueStatement,
    AssignmentStatement {
        target: Box<Node>,
        argument: Box<Node>,
        is_decleration: bool,
    },
    ExpressionStatement {
        argument: Box<Node>,
    },
    EmptyStatement,

    LiteralExpression {
        value: std::rc::Rc<str>,
    },
    IdentifierExpression {
        name: std::rc::Rc<str>,
    },
    InfixExpression {
        left_argument: Box<Node>,
        right_argument: Box<Node>,
        operator: super::token::Token,
    },
    PrefixExpression {
        argument: Box<Node>,
        operator: super::token::Token,
    },
    TableExpression {
        properties: Vec<(Box<Node>, Box<Node>)>,
    },
    MemberExpression {
        target: Box<Node>,
        argument: Box<Node>,
        notation_type: MemberNotationType,
    },
    FuncExpression {
        parameters: Vec<Box<Node>>,
        body: Box<Node>,
    },
    CallExpression {
        target: Box<Node>,
        arguments: Vec<Box<Node>>,
    },
}

#[derive(PartialEq, PartialOrd)]
pub enum Precedence {
    Lowest,
    Assignment,
    Logical,
    Bitwise,
    Comparative,
    Additive,
    Multiplicative,
    Prefix,
    Call,
    Member,
}

#[derive(PartialEq, Debug)]
pub enum MemberNotationType {
    Bracket,
    Dot,
}
