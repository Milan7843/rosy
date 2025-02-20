use rosy::{
    interpreter::{self, Terminal},
    parser::{BaseExpr, RecExpr},
    pipeline,
};

fn str_to_string(strs: Vec<&str>) -> Vec<String> {
    strs.iter().map(|s| String::from(*s)).collect()
}

fn compare(actual: Result<Terminal, String>, expected: Terminal) {
    match actual {
        Ok(tokens) => assert_eq!(tokens, expected),
        Err(e) => panic!("{}", e),
    }
}

#[test]
fn addition_test() {
    #[rustfmt::skip]
    let program = Vec::from([
        "a = 5",
        "b = 3",
        "c = a + b",
        "println(c)",
    ]);

    let actual = pipeline::run_pipeline(program);

    #[rustfmt::skip]
    let expected = Vec::from([
        "8",
        "",
    ]);

    compare(actual, str_to_string(expected));
}

#[test]
fn simple_arithmetic_test() {
    #[rustfmt::skip]
    let program = Vec::from([
        "a = 7",
        "b = 3",
        "c = a + b",
        "d = a - b",
        "e = a * b",
        "f = a / b",
        "println(c)",
        "println(d)",
        "println(e)",
        "println(f)",
    ]);

    let actual = pipeline::run_pipeline(program);

    #[rustfmt::skip]
    let expected = Vec::from([
        "10",
        "4",
        "21",
        "2",
        "",
    ]);

    compare(actual, str_to_string(expected));
}

#[test]
fn advanced_arithmetic_with_parentheses_test() {
    #[rustfmt::skip]
    let program = Vec::from([
        "a = 7",
        "b = 3",
        "c = a + b * 2",
        "d = (a - b) * 2",
        "e = a * (b + 2)",
        "f = a / (b + 2)",
        "g = (a / (b + 1)) ^ 2",
        "println(c)",
        "println(d)",
        "println(e)",
        "println(f)",
        "println(g)",
    ]);

    let actual = pipeline::run_pipeline(program);

    #[rustfmt::skip]
    let expected = Vec::from([
        "13",
        "8",
        "21",
        "2",
        "4",
        "",
    ]);

    compare(actual, str_to_string(expected));
}

#[test]
fn test_order_of_operations() {
    #[rustfmt::skip]
    let program = Vec::from([
        // addition and subtraction: no precedence, just left to right
        "println(1 + 2 - 3)",
        // multiplication and division: precendence over addition and subtraction
        "println(1 + 2 * 3)",
        "println(1 - 2 * 3)",
        "println(1 * 2 + 3)",
        "println(1 * 2 - 3)",
        "println(2 * 6 / 3)",
        "println(1 / 2 * 3)",
        // exponentials: precedence over everything
        "println(1 + 2 ^ 3)",
        "println(1 ^ 2 + 3)",
        "println(1 ^ 2 * 3)",
        "println(1 * 2 ^ 3)",
        "println(1 ^ 2 / 3)",
        "println(1 / 2 ^ 3)",
        "println(1 ^ 2 + 3 ^ 4)",
        "println(1 ^ 2 * 3 ^ 4)",
        "println(1 * 2 ^ 3 + 4)",
        "println(1 * 2 + 3 ^ 4)",
        "println(1 ^ 2 + 3 * 4)",
        "println(1 + 2 ^ 3 * 4)",
        "println(1 + 2 * 3 ^ 4)",
        "println(1 + 2 ^ 3 / 4)",
        "println(1 + 2 / 3 ^ 4)",
        // parentheses: precedence over everything
        "println((1 + 2) * 3)",
        "println(1 + (2 * 3))",
    ]);

    let actual = pipeline::run_pipeline(program);

    #[rustfmt::skip]
    let expected = Vec::from([
        "0",
        "7",
        "-5",
        "5",
        "-1",
        "4",
        "1",
        "9",
        "4",
        "3",
        "8",
        "0",
        "0",
        "82",
        "81",
        "48",
        "82",
        "10",
        "17",
        "65",
        "9",
        "3",
        "7",
        "9",
    ]);

    compare(actual, str_to_string(expected));
}