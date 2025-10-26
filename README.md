# rust-lumascript-frontend
A lexer and parser implementation for my custom scripting language in rust.
### features
- Full recursive descent + Pratt parser.
- Syntax is a combination of my favorite languages syntax; its mainly a combination of javascript, go, rust and lua.
- Has smart and predicable automatic semicolon insertion (asi)
### examples
```go
fib := func n {
    if n < 2 {
        return n;
    } else {
        return fib(n - 1) + fib(n - 2);
    }
};

fib(24);
```
or
```go
fib := func n {
    if n < 2 {
        return n
    } else {
        return fib(n - 1) + fib(n - 2)
    }
}

fib(24)
```
```
ProgramStatement {
    body: [
        AssignmentStatement {
            target: IdentifierExpression {
                name: "fib",
            },
            argument: FuncExpression {
                parameters: [
                    IdentifierExpression {
                        name: "n",
                    },
                ],
                body: BlockStatement {
                    body: [
                        IfStatement {
                            argument: InfixExpression {
                                left_argument: IdentifierExpression {
                                    name: "n",
                                },
                                right_argument: LiteralExpression {
                                    value: "2",
                                },
                                operator: SymbolLess,
                            },
                            consequent_body: BlockStatement {
                                body: [
                                    ReturnStatement {
                                        argument: IdentifierExpression {
                                            name: "n",
                                        },
                                    },
                                ],
                            },
                            alternate_body: BlockStatement {
                                body: [
                                    ReturnStatement {
                                        argument: InfixExpression {
                                            left_argument: CallExpression {
                                                target: IdentifierExpression {
                                                    name: "fib",
                                                },
                                                arguments: [
                                                    InfixExpression {
                                                        left_argument: IdentifierExpression {
                                                            name: "n",
                                                        },
                                                        right_argument: LiteralExpression {
                                                            value: "1",
                                                        },
                                                        operator: SymbolMinus,
                                                    },
                                                ],
                                            },
                                            right_argument: CallExpression {
                                                target: IdentifierExpression {
                                                    name: "fib",
                                                },
                                                arguments: [
                                                    InfixExpression {
                                                        left_argument: IdentifierExpression {
                                                            name: "n",
                                                        },
                                                        right_argument: LiteralExpression {
                                                            value: "2",
                                                        },
                                                        operator: SymbolMinus,
                                                    },
                                                ],
                                            },
                                            operator: SymbolPlus,
                                        },
                                    },
                                ],
                            },
                        },
                    ],
                },
            },
            is_decleration: true,
        },
        ExpressionStatement {
            argument: CallExpression {
                target: IdentifierExpression {
                    name: "fib",
                },
                arguments: [
                    LiteralExpression {
                        value: "24",
                    },
                ],
            },
        },
    ],
}
```
