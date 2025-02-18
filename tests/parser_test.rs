use rosy::parser::{self, BaseExpr};

fn compare(actual: Result<Vec<BaseExpr>, String>, expected: Vec<BaseExpr>) {
    match actual {
        Ok(tokens) => assert_eq!(tokens, expected),
        Err(e) => panic!("{}", e),
    }
}

fn compare_linewise(actual: Result<Vec<BaseExpr>, String>, expected: Vec<BaseExpr>) {
    match actual {
        Ok(tokens) => {

            if tokens.len() != expected.len() {
                panic!("Expected and actual have differing lengths ({} and {})", expected.len(), tokens.len());
            }
            
            let it = tokens.iter().zip(expected.iter());

            for (_, (act, exp)) in it.enumerate() {
                assert_eq!(act, exp);
            }
            
        },
        Err(e) => panic!("{}", e),
    }
}

#[test]
fn simple_variable() {
    #[rustfmt::skip]
    let program = Vec::from([
        "a_b",
        "long_variable",
        "var"
    ]);
    let expressions = parser::parse_strings(program);
    let expected = Vec::from([
        BaseExpr::Simple {
            expr: parser::RecExpr::Variable {
                name: String::from("a_b"),
            },
        },
        BaseExpr::Simple {
            expr: parser::RecExpr::Variable {
                name: String::from("long_variable"),
            },
        },
        BaseExpr::Simple {
            expr: parser::RecExpr::Variable {
                name: String::from("var"),
            },
        },
    ]);

    compare_linewise(expressions, expected);
}

#[test]
fn simple_integer() {
    #[rustfmt::skip]
    let program = Vec::from([
        "0",
        "1",
        "12",
        "234589374"
    ]);
    let expressions = parser::parse_strings(program);
    let expected = Vec::from([
        BaseExpr::Simple {
            expr: parser::RecExpr::Number { number: 0 },
        },
        BaseExpr::Simple {
            expr: parser::RecExpr::Number { number: 1 },
        },
        BaseExpr::Simple {
            expr: parser::RecExpr::Number { number: 12 },
        },
        BaseExpr::Simple {
            expr: parser::RecExpr::Number { number: 234589374 },
        },
    ]);

    compare_linewise(expressions, expected);
}

#[test]
fn simple_boolean() {
    #[rustfmt::skip]
    let program = Vec::from([
        "true",
        "false",
    ]);
    let expressions = parser::parse_strings(program);
    let expected = Vec::from([
        BaseExpr::Simple {
            expr: parser::RecExpr::Boolean { value: true },
        },
        BaseExpr::Simple {
            expr: parser::RecExpr::Boolean { value: false },
        },
    ]);

    compare_linewise(expressions, expected);
}

#[test]
fn simple_string() {
    #[rustfmt::skip]
    let program = Vec::from([
        "\"blah\"",
        "\"fun in for loop  { } () (*)^)*& _+-=    spaces\"",
    ]);
    let expressions = parser::parse_strings(program);
    let expected = Vec::from([
        BaseExpr::Simple {
            expr: parser::RecExpr::String {
                value: String::from("blah"),
            },
        },
        BaseExpr::Simple {
            expr: parser::RecExpr::String {
                value: String::from("fun in for loop  { } () (*)^)*& _+-=    spaces"),
            },
        },
    ]);

    compare_linewise(expressions, expected);
}

#[test]
fn simple_arithmetic() {
    #[rustfmt::skip]
    let program = Vec::from([
        "1 + 2",
        "12 * 3",
        "12 / 3",
        "12 ^ 3",
        "12 - 3",
        "12 + 3",
        "12 * 3 + 4",
        "12 * (3 + 4)",
        "12 * (3 + (4 / 2)) - 5 / (6 ^ 7) + 8",
    ]);
    let expressions = parser::parse_strings(program);
    let expected = Vec::from([
        BaseExpr::Simple {
            expr: parser::RecExpr::Add {
                left: Box::new(parser::RecExpr::Number { number: 1 }),
                right: Box::new(parser::RecExpr::Number { number: 2 }),
            },
        },
        BaseExpr::Simple {
            expr: parser::RecExpr::Multiply {
                left: Box::new(parser::RecExpr::Number { number: 12 }),
                right: Box::new(parser::RecExpr::Number { number: 3 }),
            },
        },
        BaseExpr::Simple {
            expr: parser::RecExpr::Divide {
                left: Box::new(parser::RecExpr::Number { number: 12 }),
                right: Box::new(parser::RecExpr::Number { number: 3 }),
            },
        },
        BaseExpr::Simple {
            expr: parser::RecExpr::Power {
                left: Box::new(parser::RecExpr::Number { number: 12 }),
                right: Box::new(parser::RecExpr::Number { number: 3 }),
            },
        },
        BaseExpr::Simple {
            expr: parser::RecExpr::Subtract {
                left: Box::new(parser::RecExpr::Number { number: 12 }),
                right: Box::new(parser::RecExpr::Number { number: 3 }),
            },
        },
        BaseExpr::Simple {
            expr: parser::RecExpr::Add {
                left: Box::new(parser::RecExpr::Number { number: 12 }),
                right: Box::new(parser::RecExpr::Number { number: 3 }),
            },
        },
        BaseExpr::Simple {
            expr: parser::RecExpr::Add {
                left: Box::new(parser::RecExpr::Multiply {
                    left: Box::new(parser::RecExpr::Number { number: 12 }),
                    right: Box::new(parser::RecExpr::Number { number: 3 }),
                }),
                right: Box::new(parser::RecExpr::Number { number: 4 }),
            },
        },
        BaseExpr::Simple {
            expr: parser::RecExpr::Multiply {
                left: Box::new(parser::RecExpr::Number { number: 12 }),
                right: Box::new(parser::RecExpr::Add {
                    left: Box::new(parser::RecExpr::Number { number: 3 }),
                    right: Box::new(parser::RecExpr::Number { number: 4 }),
                }),
            },
        },
        BaseExpr::Simple {
            expr: parser::RecExpr::Subtract {
                left: Box::new(parser::RecExpr::Multiply {
                    left: Box::new(parser::RecExpr::Number { number: 12 }),
                    right: Box::new(parser::RecExpr::Add {
                        left: Box::new(parser::RecExpr::Number { number: 3 }),
                        right: Box::new(parser::RecExpr::Divide {
                            left: Box::new(parser::RecExpr::Number { number: 4 }),
                            right: Box::new(parser::RecExpr::Number { number: 2 }),
                        }),
                    }),
                }),
                right: Box::new(parser::RecExpr::Add {
                    left: Box::new(parser::RecExpr::Divide {
                        left: Box::new(parser::RecExpr::Number { number: 5 }),
                        right: Box::new(parser::RecExpr::Power {
                            left: Box::new(parser::RecExpr::Number { number: 6 }),
                            right: Box::new(parser::RecExpr::Number { number: 7 }),
                        }),
                    }),
                    right: Box::new(parser::RecExpr::Number { number: 8 }),
                }),
            },
        },
    ]);

    compare_linewise(expressions, expected);
}

#[test]
fn variable_assignment_test() {
    #[rustfmt::skip]
    let program = Vec::from([
        "a = 1",
        "a1_b2 = 25",
        "a1_b2 = 25 * 2",
        "a1_b2 = (25 * 2)",
        "a1_b2 = \"string\"",
    ]);
    let expressions = parser::parse_strings(program);
    let expected = Vec::from([
        BaseExpr::VariableAssignment {
            var_name: String::from("a"),
            expr: parser::RecExpr::Number { number: 1 },
        },
        BaseExpr::VariableAssignment {
            var_name: String::from("a1_b2"),
            expr: parser::RecExpr::Number { number: 25 },
        },
        BaseExpr::VariableAssignment {
            var_name: String::from("a1_b2"),
            expr: parser::RecExpr::Multiply {
                left: Box::new(parser::RecExpr::Number { number: 25 }),
                right: Box::new(parser::RecExpr::Number { number: 2 }),
            },
        },
        BaseExpr::VariableAssignment {
            var_name: String::from("a1_b2"),
            expr: parser::RecExpr::Multiply {
                left: Box::new(parser::RecExpr::Number { number: 25 }),
                right: Box::new(parser::RecExpr::Number { number: 2 }),
            },
        },
        BaseExpr::VariableAssignment {
            var_name: String::from("a1_b2"),
            expr: parser::RecExpr::String {
                value: String::from("string"),
            },
        },
    ]);

    compare_linewise(expressions, expected);
}


#[test]
fn if_statements_test_small() {
    // do all if statement combinations and also recursive if statements
    #[rustfmt::skip]
    let program = Vec::from([
        "if true",
        "    1",
        "if false",
        "    2",
        "else if true",
        "    if true",
        "        3",
    ]);

    let expressions = parser::parse_strings(program);

    let expected = Vec::from([
        BaseExpr::IfStatement {
            condition: parser::RecExpr::Boolean { value: true },
            body: Vec::from([BaseExpr::Simple {
                expr: parser::RecExpr::Number { number: 1 },
            }]),
            else_statement: None
        },
        BaseExpr::IfStatement {
            condition: parser::RecExpr::Boolean { value: false },
            body: Vec::from([BaseExpr::Simple {
                expr: parser::RecExpr::Number { number: 2 },
            }]),
            else_statement: Some(Box::new(
                BaseExpr::ElseIfStatement {
                    condition: parser::RecExpr::Boolean { value: true },
                    body: Vec::from([
                        BaseExpr::IfStatement {
                            condition: parser::RecExpr::Boolean { value: true },
                            body: Vec::from([BaseExpr::Simple {
                                expr: parser::RecExpr::Number { number: 3 },
                            }]),
                            else_statement: None
                        }
                    ]),
                    else_statement: None
                }
            ))
        },
    ]);

    compare(expressions, expected);
}

#[test]
fn if_statements_test() {
    // do all if statement combinations and also recursive if statements
    #[rustfmt::skip]
    let program = Vec::from([
        "if true",
        "    1",
        "if false",
        "    2",
        "else if true",
        "    if true",
        "        3",
        "if true",
        "    if false",
        "        4",
        "    5",
        "else if false",
        "    if true",
        "        6",
        "    7",
        "else",
        "    8",
        "    if false",
        "        9",
        "    10",
    ]);

    let expressions = parser::parse_strings(program);

    let expected = Vec::from([
        BaseExpr::IfStatement {
            condition: parser::RecExpr::Boolean { value: true },
            body: Vec::from([BaseExpr::Simple {
                expr: parser::RecExpr::Number { number: 1 },
            }]),
            else_statement: None
        },
        BaseExpr::IfStatement {
            condition: parser::RecExpr::Boolean { value: false },
            body: Vec::from([BaseExpr::Simple {
                expr: parser::RecExpr::Number { number: 2 },
            }]),
            else_statement: Some(Box::new(
                BaseExpr::ElseIfStatement {
                    condition: parser::RecExpr::Boolean { value: true },
                    body: Vec::from([
                        BaseExpr::IfStatement {
                            condition: parser::RecExpr::Boolean { value: true },
                            body: Vec::from([BaseExpr::Simple {
                                expr: parser::RecExpr::Number { number: 3 },
                            }]),
                            else_statement: None
                        }
                    ]),
                    else_statement: None
                }
            ))
        },
        BaseExpr::IfStatement {
            condition: parser::RecExpr::Boolean { value: true },
            body: Vec::from([
                BaseExpr::IfStatement {
                    condition: parser::RecExpr::Boolean { value: false },
                    body: Vec::from([BaseExpr::Simple {
                        expr: parser::RecExpr::Number { number: 4 },
                    }]),
                    else_statement: None
                },
                BaseExpr::Simple {
                    expr: parser::RecExpr::Number { number: 5 },
                }
            ]),
            else_statement: Some(Box::new(
                BaseExpr::ElseIfStatement {
                    condition: parser::RecExpr::Boolean { value: false },
                    body: Vec::from([
                        BaseExpr::IfStatement {
                            condition: parser::RecExpr::Boolean { value: true },
                            body: Vec::from([BaseExpr::Simple {
                                expr: parser::RecExpr::Number { number: 6 },
                            }]),
                            else_statement: None
                        },
                        BaseExpr::Simple {
                            expr: parser::RecExpr::Number { number: 7 },
                        }
                    ]),
                    else_statement: Some(Box::new(
                        BaseExpr::ElseStatement {
                            body: Vec::from([
                                BaseExpr::Simple {
                                    expr: parser::RecExpr::Number { number: 8 },
                                },
                                BaseExpr::IfStatement {
                                    condition: parser::RecExpr::Boolean { value: false },
                                    body: Vec::from([BaseExpr::Simple {
                                        expr: parser::RecExpr::Number { number: 9 },
                                    }]),
                                    else_statement: None
                                },
                                BaseExpr::Simple {
                                    expr: parser::RecExpr::Number { number: 10 },
                                }
                            ])
                        }
                    ))
                }
            ))
        },
    ]);

    compare(expressions, expected);
}