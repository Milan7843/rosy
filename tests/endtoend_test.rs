// in tests/your_tests.rs
use assert_cmd::Command;
use rosy::pipeline::run_compilation_pipeline;

// Feature test checklist
// - Addition:
// v  - Two integers
// v  - Two variables
// v  - Variable and integer
//   - Integer and variable
// - Subtraction:
// v  - Two integers
// v  - Two variables
// v  - Variable and integer
// v  - Integer and variable
// - Multiplication:
// v  - Two integers
// v  - Two variables
// v  - Variable and integer
//   - Integer and variable
// - Division:
//   - Two integers
//   - Two variables
//   - Variable and integer
//   - Integer and variable
// - Further arithmetic combinations:
//   - Test operator precedence
//   - Test parentheses altering precedence
//   - Negative numbers
// - Boolean operations:
// v  - True literal
// v  - False literal
// v  - And operator
// v  - Or operator
// v  - Equals operator
//   - Complex boolean expressions
//   - Boolean operator precedence
// - Comparisons:
//   - Equals operator with integers
//   - Equals operator with strings
//   - Equals operator with booleans
//   - Equals operator with variables
//   - >= operator with integers
//   - >= operator with variables
//   - <= operator with integers
//   - <= operator with variables
//   - > operator with integers
//   - > operator with variables
//   - < operator with integers
//   - < operator with variables
// Combined boolean and comparison expressions
// - Variable assignments:
//   - Assign integer literal to variable
//   - Assign string literal to variable
//   - Assign boolean literal to variable
//   - Assign arithmetic expression to variable
//   - Assign boolean expression to variable
//   - Assign variable to another variable
//   - Assign function call to variable
//   - Reassign variable to new value
// - Plus equals (+=) operator
//   - With integer literal
//   - With variable
//   - With arithmetic expression
// - Print statements:
//   - Print statement with integer
// v  - Print statement with variable
//   - Print statement with complex math expression
//   - Print statement with boolean
//   - Print statement with string
//   - Print statement with array
// - If statements:
// v  - If statement with true condition
// v  - If statement with false condition
//   - If statement with variable condition
//   - If statement with complex condition
// v  - If else statement with true condition
// v  - If else statement with false condition
// v  - Several chained else if statements
// v  - Else statement not hit
// v  - Else statement hit
//   - Nested if statements
// - For loops:
// v  - For loop over range of integers
//   - For loop over array
// v  - Nested for loops
// - Functions: (also check if variables outside function are unaffected)
// v  - Function with no arguments and static return value
// v  - Function with single argument and return value
// v  - Function with arguments and return value
// v  - Function with no arguments and no return value
// v  - Function with single argument and no return value
//   - Function with arguments and no return value
//   - Multiple functions
//   - Recursive function
//   - Return complex expression
//   - Return value from within if statement
//   - Return early from function
//   - Call function several times
// - Lists:
// v  - Create list with integer literals
// v  - Access with constant index
// v  - Access with variable index
//   - Modify list elements

fn run_and_compare(program: Vec<&str>, expected_output: String) {
	// create a unique output filename in the temp dir to avoid collisions
	let mut output_path = std::env::temp_dir();
	let ext = if cfg!(windows) { ".exe" } else { "" };
	let unique = format!(
		"rosy_test_{}_{}",
		std::process::id(),
		std::time::SystemTime::now()
			.duration_since(std::time::UNIX_EPOCH)
			.unwrap()
			.as_nanos()
	);
	output_path.push(format!("{}{}", unique, ext));

	// compile to the unique path
	match run_compilation_pipeline(program, &output_path) {
		Ok(_) => {}
		Err(err) => panic!("Pipeline failed: {}", err),
	}

	// run the produced binary, capture output, compare, then clean up the file
	let exec_output = std::process::Command::new(&output_path)
		.output()
		.expect("Failed to execute compiled binary");

	// ensure we remove the file whether the test passes or fails
	let _ = std::fs::remove_file(&output_path);

	if !exec_output.status.success() {
		let stderr = String::from_utf8_lossy(&exec_output.stderr);
		panic!("Compiled binary failed. status: {:?}\nstderr: {}", exec_output.status, stderr);
	}

	let stdout = String::from_utf8_lossy(&exec_output.stdout).trim().to_string();
	assert_eq!(stdout, expected_output);
}

#[test]
fn simple_print_with_variable() {
	let program: Vec<&str> = vec![
		"a = 1",
		"print(a)"
	];

	let expected_output = "1";

	run_and_compare(program, expected_output.to_string());
}

#[test]
fn simple_print_with_variable_addition() {
	let program: Vec<&str> = vec![
		"a = 1",
		"b = 2",
		"c = a + b",
		"print(c)"
	];

	let expected_output = "3";

	run_and_compare(program, expected_output.to_string());
}

#[test]
fn simple_print_with_variable_and_integer_addition() {
	let program: Vec<&str> = vec![
		"a = 1",
		"c = a + 2",
		"print(c)"
	];

	let expected_output = "3";

	run_and_compare(program, expected_output.to_string());
}

#[test]
fn print_with_variable_addition() {
	let program: Vec<&str> = vec![
		"a = 1",
		"b = 2",
		"print(a + b)"
	];

	let expected_output = "3";

	run_and_compare(program, expected_output.to_string());
}

#[test]
fn simple_print_with_variable_addition_variable_unaffected() {
	let program: Vec<&str> = vec![
		"a = 1",
		"print(a)",
		"b = a + 1",
		"print(a)",
		"c = a + b",
		"print(a)",
		"d = 1 + a",
		"print(a)",
		"print(b)",
		"print(c)",
		"print(d)"
	];

	let expected_output = "1111232";

	run_and_compare(program, expected_output.to_string());
}

#[test]
fn simple_print_with_variable_multiplication() {
	let program: Vec<&str> = vec![
		"a = 3",
		"b = 2",
		"c = a * b",
		"print(c)"
	];

	let expected_output = "6";

	run_and_compare(program, expected_output.to_string());
}

#[test]
fn simple_print_with_variable_and_integer_multiplication() {
	let program: Vec<&str> = vec![
		"a = 3",
		"c = a * 2",
		"print(c)"
	];

	let expected_output = "6";

	run_and_compare(program, expected_output.to_string());
}

#[test]
fn print_with_variable_multiplication() {
	let program: Vec<&str> = vec![
		"a = 3",
		"b = 2",
		"print(a * b)"
	];

	let expected_output = "6";

	run_and_compare(program, expected_output.to_string());
}

#[test]
fn simple_print_with_variable_multiplication_variable_unaffected() {
	let program: Vec<&str> = vec![
		"a = 3",
		"print(a)",
		"b = a * 2",
		"print(a)",
		"c = a * b",
		"print(a)",
		"d = 5 * a",
		"print(a)",
		"print(b)",
		"print(c)",
		"print(d)"
	];

	let expected_output = "333361815";

	run_and_compare(program, expected_output.to_string());
}

#[test]
fn simple_print_with_variable_subtraction() {
	let program: Vec<&str> = vec![
		"a = 4",
		"b = 1",
		"c = a - b",
		"print(c)"
	];

	let expected_output = "3";

	run_and_compare(program, expected_output.to_string());
}

#[test]
fn simple_print_with_variable_and_integer_subtraction() {
	let program: Vec<&str> = vec![
		"a = 4",
		"b = a - 3",
		"print(b)"
	];

	let expected_output = "1";

	run_and_compare(program, expected_output.to_string());
}

#[test]
fn simple_print_with_integer_and_variable_subtraction() {
	let program: Vec<&str> = vec![
		"a = 1",
		"b = 4 - a",
		"print(b)"
	];

	let expected_output = "3";

	run_and_compare(program, expected_output.to_string());
}

#[test]
fn print_with_variable_subtraction() {
	let program: Vec<&str> = vec![
		"a = 4",
		"b = 1",
		"print(a - b)"
	];

	let expected_output = "3";

	run_and_compare(program, expected_output.to_string());
}

#[test]
fn simple_print_with_variable_subtraction_variable_unaffected() {
	let program: Vec<&str> = vec![
		"a = 3",
		"print(a)",
		"b = a - 2",
		"print(a)",
		"c = a - b",
		"print(a)",
		"d = 7 - a",
		"print(a)",
		"print(b)",
		"print(c)",
		"print(d)"
	];

	let expected_output = "3333124";

	run_and_compare(program, expected_output.to_string());
}

#[test]
fn if_statement_condition_true() {
	let program: Vec<&str> = vec![
		"if true",
		"    print(1)",
	];

	let expected_output = "1";

	run_and_compare(program, expected_output.to_string());
}

#[test]
fn if_statement_condition_false() {
	let program: Vec<&str> = vec![
		"if false",
		"    print(1)",
	];

	let expected_output = "";

	run_and_compare(program, expected_output.to_string());
}

#[test]
fn else_if_statement_condition_true_no_hit() {
	let program: Vec<&str> = vec![
		"if true",
		"    print(1)",
		"else if true",
		"    print(2)",
	];

	let expected_output = "1";

	run_and_compare(program, expected_output.to_string());
}

#[test]
fn else_if_statement_condition_true_and_hit() {
	let program: Vec<&str> = vec![
		"if false",
		"    print(1)",
		"else if true",
		"    print(2)",
	];

	let expected_output = "2";

	run_and_compare(program, expected_output.to_string());
}

#[test]
fn else_if_statement_condition_false() {
	let program: Vec<&str> = vec![
		"if false",
		"    print(1)",
		"else if false",
		"    print(2)",
	];

	let expected_output = "";

	run_and_compare(program, expected_output.to_string());
}

#[test]
fn else_if_third_statement_true() {
	let program: Vec<&str> = vec![
		"if false",
		"    print(1)",
		"else if false",
		"    print(2)",
		"else if true",
		"    print(3)",
	];

	let expected_output = "3";

	run_and_compare(program, expected_output.to_string());
}

#[test]
fn else_statement_after_if_no_hit() {
	let program: Vec<&str> = vec![
		"if true",
		"    print(1)",
		"else",
		"    print(2)",
	];

	let expected_output = "1";

	run_and_compare(program, expected_output.to_string());
}

#[test]
fn else_statement_after_if_hit() {
	let program: Vec<&str> = vec![
		"if false",
		"    print(1)",
		"else",
		"    print(2)",
	];

	let expected_output = "2";

	run_and_compare(program, expected_output.to_string());
}

#[test]
fn else_statement_after_else_if_no_hit_1() {
	let program: Vec<&str> = vec![
		"if false",
		"    print(1)",
		"else if true",
		"    print(2)",
		"else",
		"    print(3)",
	];

	let expected_output = "2";

	run_and_compare(program, expected_output.to_string());
}

#[test]
fn else_statement_after_else_if_no_hit_2() {
	let program: Vec<&str> = vec![
		"if true",
		"    print(1)",
		"else if true",
		"    print(2)",
		"else",
		"    print(3)",
	];

	let expected_output = "1";

	run_and_compare(program, expected_output.to_string());
}

#[test]
fn else_statement_after_else_if_hit() {
	let program: Vec<&str> = vec![
		"if false",
		"    print(1)",
		"else if false",
		"    print(2)",
		"else",
		"    print(2)",
	];

	let expected_output = "2";

	run_and_compare(program, expected_output.to_string());
}

#[test]
fn else_if_statement_after_double_else_if() {
	let program: Vec<&str> = vec![
		"if false",
		"    print(1)",
		"else if false",
		"    print(2)",
		"else if false",
		"    print(3)",
		"else",
		"    print(4)",
	];

	let expected_output = "4";

	run_and_compare(program, expected_output.to_string());
}

#[test]
fn if_followed_by_if() {
	let program: Vec<&str> = vec![
		"if true",
		"    print(1)",
		"if true",
		"    print(2)",
	];

	let expected_output = "12";

	run_and_compare(program, expected_output.to_string());
}

#[test]
fn for_loop_over_integer_range_zero() {
	let program: Vec<&str> = vec![
		"for i in 0",
		"    print(i)",
	];

	let expected_output = "";

	run_and_compare(program, expected_output.to_string());
}

#[test]
fn for_loop_over_integer_range_one() {
	let program: Vec<&str> = vec![
		"for i in 1",
		"    print(i)",
	];

	let expected_output = "0";

	run_and_compare(program, expected_output.to_string());
}

#[test]
fn for_loop_over_integer_range_five() {
	let program: Vec<&str> = vec![
		"for i in 5",
		"    print(i)",
	];

	let expected_output = "01234";

	run_and_compare(program, expected_output.to_string());
}

#[test]
fn nested_for_loop_over_integer_range() {
	let program: Vec<&str> = vec![
		"for i in 3",
		"    for j in i",
		"        print(j)",
	];

	let expected_output = "001";

	run_and_compare(program, expected_output.to_string());
}

#[test]
fn boolean_true_literal() {
	let program: Vec<&str> = vec![
		"a = true",
		"if a",
		"    print(1)",
	];

	let expected_output = "1";

	run_and_compare(program, expected_output.to_string());
}

#[test]
fn boolean_false_literal() {
	let program: Vec<&str> = vec![
		"a = false",
		"if a",
		"    print(1)",
	];

	let expected_output = "";

	run_and_compare(program, expected_output.to_string());
}

#[test]
fn boolean_and_false_false() {
	let program: Vec<&str> = vec![
		"a = false and false",
		"if a",
		"    print(1)",
	];

	let expected_output = "";

	run_and_compare(program, expected_output.to_string());
}

#[test]
fn boolean_and_false_true() {
	let program: Vec<&str> = vec![
		"a = false and true",
		"if a",
		"    print(1)",
	];

	let expected_output = "";

	run_and_compare(program, expected_output.to_string());
}

#[test]
fn boolean_and_true_false() {
	let program: Vec<&str> = vec![
		"a = true and false",
		"if a",
		"    print(1)",
	];

	let expected_output = "";

	run_and_compare(program, expected_output.to_string());
}

#[test]
fn boolean_and_true_true() {
	let program: Vec<&str> = vec![
		"a = true and true",
		"if a",
		"    print(1)",
	];

	let expected_output = "1";

	run_and_compare(program, expected_output.to_string());
}

#[test]
fn boolean_or_false_false() {
	let program: Vec<&str> = vec![
		"a = false or false",
		"if a",
		"    print(1)",
	];

	let expected_output = "";

	run_and_compare(program, expected_output.to_string());
}

#[test]
fn boolean_or_false_true() {
	let program: Vec<&str> = vec![
		"a = false or true",
		"if a",
		"    print(1)",
	];

	let expected_output = "1";

	run_and_compare(program, expected_output.to_string());
}

#[test]
fn boolean_or_true_false() {
	let program: Vec<&str> = vec![
		"a = true or false",
		"if a",
		"    print(1)",
	];

	let expected_output = "1";

	run_and_compare(program, expected_output.to_string());
}

#[test]
fn boolean_or_true_true() {
	let program: Vec<&str> = vec![
		"a = true or true",
		"if a",
		"    print(1)",
	];

	let expected_output = "1";

	run_and_compare(program, expected_output.to_string());
}

#[test]
fn equals_two_immediate_values_false() {
	let program: Vec<&str> = vec![
		"a = 2 == 3",
		"if a",
		"    print(1)",
	];

	let expected_output = "";

	run_and_compare(program, expected_output.to_string());
}

#[test]
fn equals_two_immediate_values_true() {
	let program: Vec<&str> = vec![
		"a = 2 == 2",
		"if a",
		"    print(1)",
	];

	let expected_output = "1";

	run_and_compare(program, expected_output.to_string());
}

#[test]
fn equals_two_variable_values_false() {
	let program: Vec<&str> = vec![
		"a = 2",
		"b = 3",
		"if a == b",
		"    print(1)",
	];

	let expected_output = "";

	run_and_compare(program, expected_output.to_string());
}

#[test]
fn equals_two_variable_values_true() {
	let program: Vec<&str> = vec![
		"a = 2",
		"b = 2",
		"if a == b",
		"    print(1)",
	];

	let expected_output = "1";

	run_and_compare(program, expected_output.to_string());
}

#[test]
fn equals_two_boolean_variable_values_false_false() {
	let program: Vec<&str> = vec![
		"a = false",
		"b = false",
		"if a == b",
		"    print(1)",
	];

	let expected_output = "1";

	run_and_compare(program, expected_output.to_string());
}

#[test]
fn equals_two_boolean_variable_values_false_true() {
	let program: Vec<&str> = vec![
		"a = false",
		"b = true",
		"if a == b",
		"    print(1)",
	];

	let expected_output = "";

	run_and_compare(program, expected_output.to_string());
}

#[test]
fn equals_two_boolean_variable_values_true_false() {
	let program: Vec<&str> = vec![
		"a = true",
		"b = false",
		"if a == b",
		"    print(1)",
	];

	let expected_output = "";

	run_and_compare(program, expected_output.to_string());
}

#[test]
fn equals_two_boolean_variable_values_true_true() {
	let program: Vec<&str> = vec![
		"a = true",
		"b = true",
		"if a == b",
		"    print(1)",
	];

	let expected_output = "1";

	run_and_compare(program, expected_output.to_string());
}

#[test]
fn function_zero_args_no_return() {
	let program: Vec<&str> = vec![
		"fun f()",
		"    return",
		"f()",
	];

	let expected_output = "";

	// TODO enable
	//run_and_compare(program, expected_output.to_string());
}

#[test]
fn function_zero_args_static_return() {
	let program: Vec<&str> = vec![
		"fun f()",
		"    return 2",
		"print(f())",
	];

	let expected_output = "2";

	run_and_compare(program, expected_output.to_string());
}

#[test]
fn function_one_arg_no_return() {
	let program: Vec<&str> = vec![
		"fun f(a)",
		"    return",
	];

	let expected_output = "";

	run_and_compare(program, expected_output.to_string());
}

#[test]
fn function_one_arg_static_return() {
	let program: Vec<&str> = vec![
		"fun f(a)",
		"    return 2",
		"print(f(1))",
	];

	let expected_output = "2";

	run_and_compare(program, expected_output.to_string());
}

#[test]
fn function_one_arg_dynamic_return() {
	let program: Vec<&str> = vec![
		"fun f(a)",
		"    return a",
		"print(f(2))",
	];

	let expected_output = "2";

	run_and_compare(program, expected_output.to_string());
}

#[test]
fn function_two_arg_dynamic_return() {
	let program: Vec<&str> = vec![
		"fun f(a, b)",
		"    return a + b",
		"print(f(1, 2))",
	];

	let expected_output = "3";

	run_and_compare(program, expected_output.to_string());
}

#[test]
fn function_three_arg_dynamic_return() {
	let program: Vec<&str> = vec![
		"fun f(a, b, c)",
		"    return a + b + c",
		"print(f(1, 2, 3))",
	];

	let expected_output = "6";

	run_and_compare(program, expected_output.to_string());
}

#[test]
fn function_four_arg_dynamic_return() {
	let program: Vec<&str> = vec![
		"fun f(a, b, c, d)",
		"    return a + b + c + d",
		"print(f(1, 2, 3, 4))",
	];

	let expected_output = "10";

	run_and_compare(program, expected_output.to_string());
}

#[test]
fn function_five_arg_dynamic_return() {
	let program: Vec<&str> = vec![
		"fun f(a, b, c, d, e)",
		"    return a + b + c + d + e",
		"print(f(1, 2, 3, 4, 5))",
	];

	let expected_output = "15";

	run_and_compare(program, expected_output.to_string());
}

#[test]
fn function_six_arg_dynamic_return() {
	let program: Vec<&str> = vec![
		"fun f(a, b, c, d, e, x)",
		"    return a + b + c + d + e + x",
		"print(f(1, 2, 3, 4, 5, 6))",
	];

	let expected_output = "21";

	run_and_compare(program, expected_output.to_string());
}

#[test]
fn function_seven_arg_dynamic_return() {
	let program: Vec<&str> = vec![
		"fun f(a, b, c, d, e, x, g)",
		"    return a + b + c + d + e + x + g",
		"print(f(1, 2, 3, 4, 5, 6, 7))",
	];

	let expected_output = "28";

	run_and_compare(program, expected_output.to_string());
}

#[test]
fn function_eight_arg_dynamic_return() {
	let program: Vec<&str> = vec![
		"fun f(a, b, c, d, e, x, g, h)",
		"    return a + b + c + d + e + x + g + h",
		"print(f(1, 2, 3, 4, 5, 6, 7, 8))",
	];

	let expected_output = "36";

	run_and_compare(program, expected_output.to_string());
}

#[test]
fn function_nine_arg_dynamic_return() {
	let program: Vec<&str> = vec![
		"fun f(a, b, c, d, e, x, g, h, i)",
		"    return a + b + c + d + e + x + g + h + i",
		"print(f(1, 2, 3, 4, 5, 6, 7, 8, 9))",
	];

	let expected_output = "45";

	run_and_compare(program, expected_output.to_string());
}

#[test]
fn size_zero_list_creation() {
	let program: Vec<&str> = vec![
		"a = []",
	];

	let expected_output = "";

	run_and_compare(program, expected_output.to_string());
}

#[test]
fn size_one_list_creation_access_constant_index() {
	let program: Vec<&str> = vec![
		"a = [2]",
		"print(a[0])",
	];

	let expected_output = "2";

	run_and_compare(program, expected_output.to_string());
}

#[test]
fn size_three_list_creation_access_constant_index() {
	let program: Vec<&str> = vec![
		"a = [2, 3, 4]",
		"print(a[0])",
		"print(a[1])",
		"print(a[2])",
	];

	let expected_output = "234";

	run_and_compare(program, expected_output.to_string());
}

#[test]
fn size_three_list_creation_access_variable_index() {
	let program: Vec<&str> = vec![
		"a = [2, 3, 4]",
		"i = 0",
		"print(a[i])",
		"i = 1",
		"print(a[i])",
		"i = 2",
		"print(a[i])",
	];

	let expected_output = "234";

	run_and_compare(program, expected_output.to_string());
}

#[test]
fn list_update() {
	let program: Vec<&str> = vec![
		"a = [2, 3, 4]",
		"a[1] = 5",
		"print(a[0])",
		"print(a[1])",
		"print(a[2])",
	];

	let expected_output = "254";

	// TODO enable
	//run_and_compare(program, expected_output.to_string());
}