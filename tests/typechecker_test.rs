use rosy::parser::{self, BaseExpr, BaseExprData, RecExpr, RecExprData};
use rosy::tokenizer::Error;
use rosy::typechecker;
use rosy::typechecker::Type;

#[test]
fn simple_number() {
    let program = BaseExpr {
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
    };

    let expected = BaseExpr {
        data: BaseExprData::Simple {
            expr: RecExpr {
                data: RecExprData::Number { number: 42 },
                row: 0,
                col_start: 0,
                col_end: 2,
                generic_data: Type::Integer,
            },
        },
        row: 0,
        col_start: 0,
        col_end: 2,
        generic_data: Type::Integer,
    };
    let result = typechecker::get_type(program);
    let actual = match result {
        Err(e) => panic!("Typechecker returned an error: {:?}", e),
        Ok(t) => t,
    };

    assert_eq!(actual, expected);
}

#[test]
fn simple_string() {
    let program = BaseExpr {
        data: BaseExprData::Simple {
            expr: RecExpr {
                data: RecExprData::String {
                    value: String::from("Hello, World!"),
                },
                row: 0,
                col_start: 0,
                col_end: 16,
                generic_data: (),
            },
        },
        row: 0,
        col_start: 0,
        col_end: 16,
        generic_data: (),
    };

    let expected = BaseExpr {
        data: BaseExprData::Simple {
            expr: RecExpr {
                data: RecExprData::String {
                    value: String::from("Hello, World!"),
                },
                row: 0,
                col_start: 0,
                col_end: 16,
                generic_data: Type::String,
            },
        },
        row: 0,
        col_start: 0,
        col_end: 16,
        generic_data: Type::String,
    };

    let result = typechecker::get_type(program);
    let actual = match result {
        Err(e) => panic!("Typechecker returned an error: {:?}", e),
        Ok(t) => t,
    };

    assert_eq!(actual, expected);
}

#[test]
fn addition_of_integers() {
    let program = BaseExpr {
        data: BaseExprData::Simple {
            expr: RecExpr {
                data: RecExprData::Add {
                    left: Box::new(RecExpr {
                        data: RecExprData::Number { number: 5 },
                        row: 1,
                        col_start: 1,
                        col_end: 2,
                        generic_data: (),
                    }),
                    right: Box::new(RecExpr {
                        data: RecExprData::Number { number: 10 },
                        row: 1,
                        col_start: 5,
                        col_end: 6,
                        generic_data: (),
                    }),
                },
                row: 1,
                col_start: 1,
                col_end: 6,
                generic_data: (),
            },
        },
        row: 1,
        col_start: 1,
        col_end: 6,
        generic_data: (),
    };

    let expected = BaseExpr {
        data: BaseExprData::Simple {
            expr: RecExpr {
                data: RecExprData::Add {
                    left: Box::new(RecExpr {
                        data: RecExprData::Number { number: 5 },
                        row: 1,
                        col_start: 1,
                        col_end: 2,
                        generic_data: Type::Integer,
                    }),
                    right: Box::new(RecExpr {
                        data: RecExprData::Number { number: 10 },
                        row: 1,
                        col_start: 5,
                        col_end: 6,
                        generic_data: Type::Integer,
                    }),
                },
                row: 1,
                col_start: 1,
                col_end: 6,
                generic_data: Type::Integer,
            },
        },
        row: 1,
        col_start: 1,
        col_end: 6,
        generic_data: Type::Integer,
    };
    let result = typechecker::get_type(program);
    let actual = match result {
        Err(e) => panic!("Typechecker returned an error: {:?}", e),
        Ok(t) => t,
    };

    assert_eq!(actual, expected);
}

#[test]
fn type_error_in_addition() {
    let program = BaseExpr {
        data: BaseExprData::Simple {
            expr: RecExpr {
                data: RecExprData::Add {
                    left: Box::new(RecExpr {
                        data: RecExprData::Number { number: 5 },
                        row: 1,
                        col_start: 1,
                        col_end: 2,
                        generic_data: (),
                    }),
                    right: Box::new(RecExpr {
                        data: RecExprData::String {
                            value: String::from("Hello"),
                        },
                        row: 1,
                        col_start: 5,
                        col_end: 10,
                        generic_data: (),
                    }),
                },
                row: 1,
                col_start: 1,
                col_end: 10,
                generic_data: (),
            },
        },
        row: 1,
        col_start: 1,
        col_end: 10,
        generic_data: (),
    };

    let result = typechecker::get_type(program);
    match result {
        Err(e) => match e {
            Error::TypeError {
                message,
                expected,
                found,
                row,
                col_start,
                col_end,
            } => {
                //assert_eq!(message, "Cannot add types Integer and String");
                assert_eq!(expected, Type::Integer);
                assert_eq!(found, Type::String);
                assert_eq!(row, 1);
                assert_eq!(col_start, 1);
                assert_eq!(col_end, 10);
            }
            _ => panic!("Expected a TypeError, but got a different error: {:?}", e),
        },
        Ok(t) => panic!(
            "Typechecker returned a type when an error was expected: {:?}",
            t
        ),
    };
}

#[test]
fn simple_variable() {}
