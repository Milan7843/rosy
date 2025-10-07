use rosy::parser::{self, BaseExpr, BaseExprData, RecExpr, RecExprData};
use rosy::pipeline::print_error;
use rosy::tokenizer::Error;


#[test]
fn single_variable() {
    let program = vec![BaseExpr {
        data: BaseExprData::Simple {
            expr: RecExpr {
                data: RecExprData::Variable { name: "x" },
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
                data: RecExprData::Variable { name: "x0" },
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