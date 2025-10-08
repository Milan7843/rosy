use rosy::parser::{self, BaseExpr, BaseExprData, RecExpr, RecExprData};
use rosy::pipeline::print_error;
use rosy::tokenizer::Error;
use rosy::typechecker::Type::*;
use rosy::typechecker::*;
use rosy::typechecker::FunctionType;
use rosy::parser::BaseExprData::*;
use rosy::parser::RecExprData::*;
use rosy::uniquify;

use std::collections::{HashMap, HashSet};

#[test]
fn test_uniquify_simple() {
    let mut program = (vec![BaseExpr {
        data: BaseExprData::VariableAssignment {
            var_name: "x".to_string(),
            expr: RecExpr {
                data: RecExprData::Number { number: 5 },
                row: 0,
                col_start: 5,
                col_end: 6,
                generic_data: Integer,
            },
        },
        row: 0,
        col_start: 0,
        col_end: 6,
        generic_data: Integer,
    }],
    // The function list is empty for this test
    vec![]
);

    let expected_output = (vec![BaseExpr {
        data: BaseExprData::VariableAssignment {
            var_name: "x0".to_string(),
            expr: RecExpr {
                data: RecExprData::Number { number: 5 },
                row: 0,
                col_start: 5,
                col_end: 6,
                generic_data: Integer,
            },
        },
        row: 0,
        col_start: 0,
        col_end: 6,
        generic_data: Integer,
    }],
    // The function list is empty for this test
    vec![]
);

    uniquify::uniquify(&mut program);
    assert_eq!(program, expected_output);
}

#[test]
fn test_function() {
    let mut program = (vec![BaseExpr {
            data: VariableAssignment {
                var_name: "a".to_string(),
                expr: RecExpr {
                    data: Number {
                        number: 5,
                    },
                    row: 0,
                    col_start: 4,
                    col_end: 5,
                    generic_data: Integer,
                },
            },
            row: 0,
            col_start: 0,
            col_end: 5,
            generic_data: Undefined,
        },
        BaseExpr {
            data: Simple {
                expr: RecExpr {
                    data: FunctionCall {
                        function_name: "f".to_string(),
                        args: vec![
                            RecExpr {
                                data: Variable {
                                    name: "a".to_string(),
                                },
                                row: 3,
                                col_start: 2,
                                col_end: 3,
                                generic_data: Integer,
                            },
                        ],
                    },
                    row: 3,
                    col_start: 0,
                    col_end: 4,
                    generic_data: Integer,
                },
            },
            row: 3,
            col_start: 0,
            col_end: 4,
            generic_data: Integer,
        },],
    vec![FunctionType {
            name: "f".to_string(),
            param_names: vec![
                "a".to_string(),
            ],
            param_types: vec![
                Integer,
            ],
            return_type: Integer,
            content: vec![
                BaseExpr {
                    data: Return {
                        return_value: Some(
                            RecExpr {
                                data: Add {
                                    left: Box::new(RecExpr {
                                        data: Variable {
                                            name: "a".to_string(),
                                        },
                                        row: 2,
                                        col_start: 11,
                                        col_end: 12,
                                        generic_data: Integer,
                                    }),
                                    right: Box::new(RecExpr {
                                        data: Number {
                                            number: 2,
                                        },
                                        row: 2,
                                        col_start: 15,
                                        col_end: 16,
                                        generic_data: Integer,
                                    }),
                                },
                                row: 2,
                                col_start: 11,
                                col_end: 16,
                                generic_data: Integer,
                            },
                        ),
                    },
                    row: 2,
                    col_start: 4,
                    col_end: 16,
                    generic_data: Integer,
                },
            ],
        },]);

    let expected_output = (vec![BaseExpr {
            data: VariableAssignment {
                var_name: "a1".to_string(),
                expr: RecExpr {
                    data: Number {
                        number: 5,
                    },
                    row: 0,
                    col_start: 4,
                    col_end: 5,
                    generic_data: Integer,
                },
            },
            row: 0,
            col_start: 0,
            col_end: 5,
            generic_data: Undefined,
        },
        BaseExpr {
            data: Simple {
                expr: RecExpr {
                    data: FunctionCall {
                        function_name: "f".to_string(),
                        args: vec![
                            RecExpr {
                                data: Variable {
                                    name: "a1".to_string(),
                                },
                                row: 3,
                                col_start: 2,
                                col_end: 3,
                                generic_data: Integer,
                            },
                        ],
                    },
                    row: 3,
                    col_start: 0,
                    col_end: 4,
                    generic_data: Integer,
                },
            },
            row: 3,
            col_start: 0,
            col_end: 4,
            generic_data: Integer,
        },],
    vec![FunctionType {
            name: "f".to_string(),
            param_names: vec![
                "a0".to_string(),
            ],
            param_types: vec![
                Integer,
            ],
            return_type: Integer,
            content: vec![
                BaseExpr {
                    data: Return {
                        return_value: Some(
                            RecExpr {
                                data: Add {
                                    left: Box::new(RecExpr {
                                        data: Variable {
                                            name: "a0".to_string(),
                                        },
                                        row: 2,
                                        col_start: 11,
                                        col_end: 12,
                                        generic_data: Integer,
                                    }),
                                    right: Box::new(RecExpr {
                                        data: Number {
                                            number: 2,
                                        },
                                        row: 2,
                                        col_start: 15,
                                        col_end: 16,
                                        generic_data: Integer,
                                    }),
                                },
                                row: 2,
                                col_start: 11,
                                col_end: 16,
                                generic_data: Integer,
                            },
                        ),
                    },
                    row: 2,
                    col_start: 4,
                    col_end: 16,
                    generic_data: Integer,
                },
            ],
        },]);

    uniquify::uniquify(&mut program);
    assert_eq!(program, expected_output);
}