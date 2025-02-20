use rosy::{
    interpreter::{self, Terminal},
    parser::BaseExpr,
    parser::RecExpr,
};

fn compare(actual: Result<Terminal, String>, expected: Terminal) {
    match actual {
        Ok(tokens) => assert_eq!(tokens, expected),
        Err(e) => panic!("{}", e),
    }
}

#[test]
fn number_test() {
    let program = Vec::from([
        BaseExpr::Simple {
            expr: RecExpr::FunctionCall {
                function_name: String::from("println"),
                args: Vec::from([RecExpr::Number { number: 0 }]),
            },
        },
        BaseExpr::Simple {
            expr: RecExpr::FunctionCall {
                function_name: String::from("println"),
                args: Vec::from([RecExpr::Number { number: 1 }]),
            },
        },
        BaseExpr::Simple {
            expr: RecExpr::FunctionCall {
                function_name: String::from("println"),
                args: Vec::from([RecExpr::Number { number: 12 }]),
            },
        },
        BaseExpr::Simple {
            expr: RecExpr::FunctionCall {
                function_name: String::from("println"),
                args: Vec::from([RecExpr::Number { number: 234589374 }]),
            },
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

#[test]
fn string_test() {
    let program = Vec::from([
        BaseExpr::Simple {
            expr: RecExpr::FunctionCall {
                function_name: String::from("println"),
                args: Vec::from([RecExpr::String { value: String::from("") }]),
            },
        },
        BaseExpr::Simple {
            expr: RecExpr::FunctionCall {
                function_name: String::from("println"),
                args: Vec::from([RecExpr::String { value: String::from("s") }]),
            },
        },
        BaseExpr::Simple {
            expr: RecExpr::FunctionCall {
                function_name: String::from("println"),
                args: Vec::from([RecExpr::String { value: String::from(")(*&)(/.._][]+-abdABD123") }]),
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
