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


#[test]
fn unary_vs_binary_minus_test() {
    #[rustfmt::skip]
    let program = Vec::from([
        "println(-5)", // Unary minus
        "println(5 - 3)", // Binary minus
        "println(-(2 + 3))", // Unary minus with parentheses
        "println(5 - -3)", // Binary minus followed by unary minus
        "println(-5 + 3)", // Unary minus followed by addition
        "println(5 * -2)", // Unary minus in multiplication
        "println(5 / -2)", // Unary minus in division
        "println(5 ^ --2)", // Unary minus in power
        "println(-5 ^ 2)", // Unary minus in power
        "println(-(-5))", // Double unary minus
        "println((5 - 3) - 2)", // Nested binary minus
    ]);

    let actual = pipeline::run_pipeline(program);

    #[rustfmt::skip]
    let expected = Vec::from([
        "-5",
        "2",
        "-5",
        "8",
        "-2",
        "-10",
        "-2",
        "25",
        "25",
        "5",
        "0",
        "",
    ]);

    compare(actual, str_to_string(expected));
}

#[test]
fn and_or_not_statements_test() {
    #[rustfmt::skip]
    let program = Vec::from([
        "println(not false)",
        "println(not true)",
        "println(not not false)",
        "println(not not (not true or false))",
        
        "println(false and false)",
        "println(false and true)",
        "println(true and false)",
        "println(true and true)",
        "println(false or false)",
        "println(false or true)",
        "println(true or false)",
        "println(true or true)",

        "println(false and true and true)",
        // test the precedence of and over or
        "println(true or false and false)", // should be true (and has higher precedence)
        "println((true or false) and false)", // should be false (parentheses change order)
        "println(not true or true)", // should be true (not has highest precedence)
        "println(not (true or true))", // should be false (negates the whole expression)
        "println(not true and false)", // should be false (not binds tightly to true)
        "println(not (true and false))", // should be true (negates the whole expression)
        "println(not false or false and true)", // should be true (not first, then and, then or)
        "println(false and not false or true)", // should be true (not first, then and, then or)
        "println((false and not false) or true)", // should be true (parentheses around and)
        "println(false and (not false or true))", // should be false (parentheses affect grouping)
    ]);

    let actual = pipeline::run_pipeline(program);

    #[rustfmt::skip]
    let expected = Vec::from([
        "true",
        "false",
        "false",
        "false",

        "false",
        "false",
        "false",
        "true",
        "false",
        "true",
        "true",
        "true",

        "false",
        "true",
        "false",
        "true",
        "false",
        "false",
        "true",
        "true",
        "true",
        "true",
        "false",
        "",
    ]);

    compare(actual, str_to_string(expected));
}

#[test]
fn if_statements_test() {
    // Test if, else if and else
    #[rustfmt::skip]
    let program = Vec::from([
        "if true",
        "    println(1)",
        "else if false",
        "    println(2)",
        "else",
        "    println(3)",

        "if false",
        "    println(4)",
        "else if true",
        "    println(5)",
        "else",
        "    println(6)",

        "if false",
        "    println(7)",
        "else if false",
        "    println(8)",
        "else",
        "    println(9)",

        "if false",
        "    println(10)",
        "else",
        "    println(11)",
        
        "if false",
        "    println(12)",
        "else if false",
        "    println(13)",
        "else if false",
        "    println(14)",
        "else",
        "    println(15)",
        
        "if false",
        "    println(16)",
        "else if false",
        "    println(17)",
        "else if true",
        "    println(18)",
    ]);

    let actual = pipeline::run_pipeline(program);

    #[rustfmt::skip]
    let expected = Vec::from([
        "1",
        "5",
        "9",
        "11",
        "15",
        "18",
        "",
    ]);

    compare(actual, str_to_string(expected));
}

#[test]
fn for_loop_test() {
    #[rustfmt::skip]
    let program = Vec::from([
        "for i in 5",
        "    println(i)",
    ]);

    let actual = pipeline::run_pipeline(program);

    #[rustfmt::skip]
    let expected = Vec::from([
        "0",
        "1",
        "2",
        "3",
        "4",
        "",
    ]);

    compare(actual, str_to_string(expected));
}