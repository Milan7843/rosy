// in tests/your_tests.rs
use assert_cmd::Command;
use rosy::pipeline::run_compilation_pipeline;

// Feature test checklist
// - Addition:
//   - Two integers
//   - Two variables
//   - Variable and integer
//   - Integer and variable
// - Subtraction:
//   - Two integers
//   - Two variables
//   - Variable and integer
//   - Integer and variable
// - Multiplication:
//   - Two integers
//   - Two variables
//   - Variable and integer
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
//   - Print statement with variable
//   - Print statement with complex math expression
//   - Print statement with boolean
//   - Print statement with string
//   - Print statement with array
// - If statements:
//   - If statement with true condition
//   - If statement with false condition
//   - If statement with variable condition
//   - If statement with complex condition
//   - If else statement with true condition
//   - If else statement with false condition
//   - Several chained else if statements
//   - Else statement not hit
//   - Else statement hit
//   - Nested if statements
// - For loops:
//   - For loop over range of integers
//   - For loop over array
//   - Nested for loops
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
	let output_path = std::path::PathBuf::from("target/debug/output.exe");

	match run_compilation_pipeline(program, &output_path) {
		Ok(_) => {}
		Err(err) => panic!("Pipeline failed: {}", err),
	}

	let mut cmd = Command::cargo_bin("output").unwrap();
	cmd.assert()
	   .success()
	   .stdout(expected_output);
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
fn simple_print_with_variable_multiplication() {
	let program: Vec<&str> = vec![
		"a = 1",
		"b = 2",
		"c = a * b",
		"print(c)"
	];

	let expected_output = "3";

	run_and_compare(program, expected_output.to_string());
}

#[test]
fn print_with_variable_multiplication() {
	let program: Vec<&str> = vec![
		"a = 1",
		"b = 2",
		"print(a * b)"
	];

	let expected_output = "3";

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
fn print_with_variable_subtraction() {
	let program: Vec<&str> = vec![
		"a = 4",
		"b = 1",
		"print(a * b)"
	];

	let expected_output = "3";

	run_and_compare(program, expected_output.to_string());
}

