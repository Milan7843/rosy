use clap::Parser;
use rosy::parser;
use rosy::tokenizer;
use rosy::interpreter;

// Language features:
/*
BaseExpr:
- Expr
- Variable assignment: [var_name] = Expr
- if statement:
    if Expr
        BaseExpr+
- else-if statement:
    else if Expr
        BaseExpr+
- else statement:
    else
        BaseExpr+
- for loop:
    for [var_name] in Expr
        BaseExpr+
- Function definition:
    fun [fun_name](arg*)
        BaseExpr+
- Struct:
    struct [struct_name]
        [var_name]*
- Return statement: return
- Break statement: break

Expr:
- Addition: Expr + Expr
- Subtraction: Expr - Expr
- Multiplication: Expr * Expr
- Division: Expr / Expr
- Negative number: - Expr
- Variables: [var_name]
- Integer numbers
- Strings: "[str]"
- False: false
- True: true
- Or operator: Expr or Expr
- And operator: Expr and Expr
- Equals operator: Expr == Expr
- struct access: [struct_name].[var_name]
- function call: [fun_name](arg*)

Default functions:
- print(String)
- print(Integer)
- print(Boolean)
*/

// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    // The path to the file to read
    path: std::path::PathBuf,
}

pub fn main() {
    let args = Cli::parse();

    println!("path: {:?}", args.path);

    match run_pipeline(&args.path) {
        Ok(message) => println!("{message}"),
        Err(err) => println!("{err}"),
    }
}

pub fn run_pipeline(path: &std::path::PathBuf) -> Result<String, String> {
    let base_expressions = match parser::parse(path) {
        Ok(base_expressions) => base_expressions,
        Err(error_message) => return Err(error_message),
    };

    parser::print_expressions(&base_expressions);

    let message = match interpreter::interpret(base_expressions) {
        Ok(message) => message,
        Err(error_message) => return Err(error_message),
    };

    return Ok(String::from("All went well"));
}
