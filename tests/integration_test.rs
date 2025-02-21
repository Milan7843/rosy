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
        "g = (8 / (b + 1)) ^ 2",
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
        "35",
        "1",
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
        "println(4 / 2 * 3)",
        // exponentials: precedence over everything
        "println(1 + 2 ^ 3)",
        "println(1 ^ 2 + 3)",
        "println(1 ^ 2 * 3)",
        "println(1 * 2 ^ 3)",
        "println(4 ^ 2 / 3)",
        "println(24 / 2 ^ 3)",
        "println(1 ^ 2 + 3 ^ 4)", // 82
        "println(1 ^ 2 * 3 ^ 4)", // 81
        "println(4 * 2 ^ 3 + 4)", // 36
        "println(1 * 2 + 3 ^ 4)", // 83
        "println(1 ^ 2 + 3 * 4)", // 13
        "println(1 + 2 ^ 3 * 4)", // 33
        "println(1 + 2 * 3 ^ 4)", // 163
        "println(1 + 2 ^ 3 / 4)", // 3
        "println(1 + 243 / 3 ^ 4)", // 4
        // parentheses: precedence over everything
        "println((1 + 2) * 3)", // 9
        "println(1 + (2 * 3))", // 7
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
        "6",
        "9",
        "4",
        "3",
        "8",
        "5",
        "3",
        "82",
        "81",
        "36",
        "83",
        "13",
        "33",
        "163",
        "3",
        "4",
        "9",
        "7",
        "",
    ]);

    compare(actual, str_to_string(expected));
}

#[test]
fn test_variable_shadowing() {
    #[rustfmt::skip]
    let program = Vec::from([
        "a = 5",
        "b = 3",
        "c = a + b",
        "a = 7",
        "d = a + b",
        "println(c)",
        "println(d)",
        // Same for +=
        "a = 5",
        "b = 3",
        "c = a + b",
        "a += 2",
        "d = a + b",
        "println(c)",
        "println(d)",
        // Variables should be shadowed in a function
        "fun test(a)",
        "    println(a)",
        "a = 3",
        "test(4)",
        "println(a)",
        // Even if they are assigned to within the function
        "fun test(a)",
        "    a = 5",
        "    println(a)",
        "a = 3",
        "test(4)",
        "println(a)",
        // outside variable should be accessible in a function
        "fun test()",
        "    println(a)",
        "a = 7",
        "test()",
    ]);

    let actual = pipeline::run_pipeline(program);

    #[rustfmt::skip]
    let expected = Vec::from([
        "8",
        "10",
        "8",
        "10",
        "4",
        "3",
        "5",
        "3",
        "7",
        "",
    ]);

    compare(actual, str_to_string(expected));
}
