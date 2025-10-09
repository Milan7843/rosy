use rosy::desugarer;
use rosy::parser::{self, BaseExpr, BaseExprData, RecExpr, RecExprData};

#[test]
fn simple_number() {
    let program = vec![BaseExpr {
        data: BaseExprData::Simple {
            expr: RecExpr {
                data: RecExprData::Number { number: 42 },
                row: 0,
                col_start: 0,
                col_end: 2,
                generic_data: (),
            },
        },
        row: 0,
        col_start: 0,
        col_end: 2,
        generic_data: (),
    }];

    let expected_output = vec![BaseExpr {
        data: BaseExprData::Simple {
            expr: RecExpr {
                data: RecExprData::Number { number: 42 },
                row: 0,
                col_start: 0,
                col_end: 2,
                generic_data: (),
            },
        },
        row: 0,
        col_start: 0,
        col_end: 2,
        generic_data: (),
    }];

    let desugared_program = desugarer::desugar(program);
    assert_eq!(desugared_program, expected_output);
}

#[test]
fn plus_equals_statement() {
    // x += (5 * 3)
    // should desugar to
    // x = x + (5 * 3)
    let program = vec![BaseExpr {
        data: BaseExprData::PlusEqualsStatement {
            var_name: String::from("x"),
            expr: RecExpr {
                data: RecExprData::Multiply {
                    left: Box::new(RecExpr {
                        data: RecExprData::Number { number: 5 },
                        row: 0,
                        col_start: 5,
                        col_end: 6,
                        generic_data: (),
                    }),
                    right: Box::new(RecExpr {
                        data: RecExprData::Number { number: 3 },
                        row: 0,
                        col_start: 9,
                        col_end: 10,
                        generic_data: (),
                    }),
                },
                row: 0,
                col_start: 5,
                col_end: 10,
                generic_data: (),
            },
        },
        row: 0,
        col_start: 0,
        col_end: 10,
        generic_data: (),
    }];

    let expected_output = vec![BaseExpr {
        data: BaseExprData::VariableAssignment {
            var_name: String::from("x"),
            expr: RecExpr {
                data: RecExprData::Add {
                    left: Box::new(RecExpr {
                        data: RecExprData::Variable {
                            name: String::from("x"),
                        },
                        row: 0,
                        col_start: 0,
                        col_end: 1,
                        generic_data: (),
                    }),
                    right: Box::new(RecExpr {
                        data: RecExprData::Multiply {
                            left: Box::new(RecExpr {
                                data: RecExprData::Number { number: 5 },
                                row: 0,
                                col_start: 5,
                                col_end: 6,
                                generic_data: (),
                            }),
                            right: Box::new(RecExpr {
                                data: RecExprData::Number { number: 3 },
                                row: 0,
                                col_start: 9,
                                col_end: 10,
                                generic_data: (),
                            }),
                        },
                        row: 0,
                        col_start: 5,
                        col_end: 10,
                        generic_data: (),
                    }),
                },
                row: 0,
                col_start: 0,
                col_end: 10,
                generic_data: (),
            },
        },
        row: 0,
        col_start: 0,
        col_end: 10,
        generic_data: (),
    }];

    let desugared_program = desugarer::desugar(program);
    assert_eq!(desugared_program, expected_output);
}

#[test]
fn plus_equals_statement_simple() {
    // x += 5
    // should desugar to
    // x = x + 5
    let program = vec![BaseExpr {
        data: BaseExprData::PlusEqualsStatement {
            var_name: String::from("x"),
            expr: RecExpr {
                data: RecExprData::Number { number: 5 },
                row: 0,
                col_start: 5,
                col_end: 6,
                generic_data: (),
            },
        },
        row: 0,
        col_start: 0,
        col_end: 6,
        generic_data: (),
    }];

    let expected_output = vec![BaseExpr {
        data: BaseExprData::VariableAssignment {
            var_name: String::from("x"),
            expr: RecExpr {
                data: RecExprData::Add {
                    left: Box::new(RecExpr {
                        data: RecExprData::Variable {
                            name: String::from("x"),
                        },
                        row: 0,
                        col_start: 0,
                        col_end: 1,
                        generic_data: (),
                    }),
                    right: Box::new(RecExpr {
                        data: RecExprData::Number { number: 5 },
                        row: 0,
                        col_start: 5,
                        col_end: 6,
                        generic_data: (),
                    }),
                },
                row: 0,
                col_start: 0,
                col_end: 6,
                generic_data: (),
            },
        },
        row: 0,
        col_start: 0,
        col_end: 6,
        generic_data: (),
    }];

    let desugared_program = desugarer::desugar(program);
    assert_eq!(desugared_program, expected_output);
}

#[test]
fn nested_plus_equals_statement() {
    // for i in 5
    //     x += 5
    // should desugar to
    // for i in 5
    //     x = x + 5
    let program = vec![BaseExpr {
        data: BaseExprData::ForLoop {
            var_name: String::from("i"),
            until: RecExpr {
                data: RecExprData::Number { number: 5 },
                row: 0,
                col_start: 9,
                col_end: 10,
                generic_data: (),
            },
            body: vec![BaseExpr {
                data: BaseExprData::PlusEqualsStatement {
                    var_name: String::from("x"),
                    expr: RecExpr {
                        data: RecExprData::Number { number: 5 },
                        row: 1,
                        col_start: 5,
                        col_end: 6,
                        generic_data: (),
                    },
                },
                row: 1,
                col_start: 4,
                col_end: 10,
                generic_data: (),
            }],
        },
        row: 0,
        col_start: 0,
        col_end: 10,
        generic_data: (),
    }];

    let expected_output = vec![BaseExpr {
        data: BaseExprData::ForLoop {
            var_name: String::from("i"),
            until: RecExpr {
                data: RecExprData::Number { number: 5 },
                row: 0,
                col_start: 9,
                col_end: 10,
                generic_data: (),
            },
            body: vec![BaseExpr {
                data: BaseExprData::VariableAssignment {
                    var_name: String::from("x"),
                    expr: RecExpr {
                        data: RecExprData::Add {
                            left: Box::new(RecExpr {
                                data: RecExprData::Variable {
                                    name: String::from("x"),
                                },
                                row: 1,
                                col_start: 4,
                                col_end: 5,
                                generic_data: (),
                            }),
                            right: Box::new(RecExpr {
                                data: RecExprData::Number { number: 5 },
                                row: 1,
                                col_start: 5,
                                col_end: 6,
                                generic_data: (),
                            }),
                        },
                        row: 1,
                        col_start: 4,
                        col_end: 10,
                        generic_data: (),
                    },
                },
                row: 1,
                col_start: 4,
                col_end: 10,
                generic_data: (),
            }],
        },
        row: 0,
        col_start: 0,
        col_end: 10,
        generic_data: (),
    }];

    let desugared_program = desugarer::desugar(program);
    assert_eq!(desugared_program, expected_output);
}
