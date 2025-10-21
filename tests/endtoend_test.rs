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
//   - True literal
//   - False literal
//   - And operator
//   - Or operator
//   - Equals operator
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
//   - Function with no arguments and static return value
//   - Function with single argument and return value
//   - Function with arguments and return value
//   - Function with no arguments and no return value
//   - Function with single argument and no return value
//   - Function with arguments and no return value
//   - Multiple functions
//   - Recursive function
//   - Return complex expression
//   - Return value from within if statement
//   - Return early from function
//   - Call function several times

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
		"b = a - 2",
		"print(b)"
	];

	let expected_output = "2";

	run_and_compare(program, expected_output.to_string());
}

#[test]
fn simple_print_with_integer_and_variable_subtraction() {
	let program: Vec<&str> = vec![
		"a = 2",
		"b = 4 - a",
		"print(b)"
	];

	let expected_output = "2";

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

