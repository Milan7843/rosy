use rosy::{
    interpreter::{self, Terminal},
    parser::BaseExpr,
    parser::BaseExprData,
    parser::RecExpr,
    parser::RecExprData,
    tokenizer::Error,
};

fn compare(actual: Result<Terminal, Error>, expected: Terminal) {
    match actual {
        Ok(tokens) => assert_eq!(tokens, expected),
        Err(_) => panic!(" error "),
    }
}

#[test]
fn number_test() {
    let program = Vec::from([
        BaseExpr {
            data: BaseExprData::Simple {
                expr: RecExpr {
                    data: RecExprData::FunctionCall {
                        function_name: String::from("println"),
                        args: Vec::from([RecExpr {
                            data: RecExprData::Number { number: 0 },
                            row: 0,
                            col_start: 8,
                            col_end: 9,
                            generic_data: (),
                        }]),
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
        },
        BaseExpr {
            data: BaseExprData::Simple {
                expr: RecExpr {
                    data: RecExprData::FunctionCall {
                        function_name: String::from("println"),
                        args: Vec::from([RecExpr {
                            data: RecExprData::Number { number: 1 },
                            row: 1,
                            col_start: 8,
                            col_end: 9,
                            generic_data: (),
                        }]),
                    },
                    row: 1,
                    col_start: 0,
                    col_end: 10,
                    generic_data: (),
                },
            },
            row: 1,
            col_start: 0,
            col_end: 10,
            generic_data: (),
        },
        BaseExpr {
            data: BaseExprData::Simple {
                expr: RecExpr {
                    data: RecExprData::FunctionCall {
                        function_name: String::from("println"),
                        args: Vec::from([RecExpr {
                            data: RecExprData::Number { number: 12 },
                            row: 2,
                            col_start: 8,
                            col_end: 10,
                            generic_data: (),
                        }]),
                    },
                    row: 2,
                    col_start: 0,
                    col_end: 11,
                    generic_data: (),
                },
            },
            row: 2,
            col_start: 0,
            col_end: 11,
            generic_data: (),
        },
        BaseExpr {
            data: BaseExprData::Simple {
                expr: RecExpr {
                    data: RecExprData::FunctionCall {
                        function_name: String::from("println"),
                        args: Vec::from([RecExpr {
                            data: RecExprData::Number { number: 234589374 },
                            row: 3,
                            col_start: 8,
                            col_end: 16,
                            generic_data: (),
                        }]),
                    },
                    row: 3,
                    col_start: 0,
                    col_end: 17,
                    generic_data: (),
                },
            },
            row: 3,
            col_start: 0,
            col_end: 17,
            generic_data: (),
        },
    ]);

    let actual = interpreter::interpret(program);

    #[rustfmt::skip]
    let expected = Vec::from([
        String::from("0"),
        String::from("1"),
        String::from("12"),
        String::from("234589374"),
        String::from(""),
    ]);

    compare(actual, expected);
}

/*
#[test]
fn string_test() {
    let program = Vec::from([
        BaseExpr::Simple {
            expr: RecExpr::FunctionCall {
                function_name: String::from("println"),
                args: Vec::from([RecExpr::String {
                    value: String::from(""),
                }]),
            },
        },
        BaseExpr::Simple {
            expr: RecExpr::FunctionCall {
                function_name: String::from("println"),
                args: Vec::from([RecExpr::String {
                    value: String::from("s"),
                }]),
            },
        },
        BaseExpr::Simple {
            expr: RecExpr::FunctionCall {
                function_name: String::from("println"),
                args: Vec::from([RecExpr::String {
                    value: String::from(")(*&)(/.._][]+-abdABD123"),
                }]),
            },
        },
    ]);

    let actual = interpreter::interpret(program);

    #[rustfmt::skip]
    let expected = Vec::from([
        String::from(""),
        String::from("s"),
        String::from(")(*&)(/.._][]+-abdABD123"),
        String::from(""),
    ]);

    compare(actual, expected);
}

#[test]
fn addition_test() {
    #[rustfmt::skip]
    let program = Vec::from([
        "print(0)",
        "1",
        "12",
        "234589374"
    ]);
}

*/
