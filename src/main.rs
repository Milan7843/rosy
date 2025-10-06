use clap::Parser;
use rosy::interpreter;
use rosy::parser;
use rosy::pipeline;
use rosy::tokenizer;

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

<<<<<<< Updated upstream
    match pipeline::run_pipeline_from_path(&args.path) {
        Ok(terminal) => {}
        Err(err) => println!("{err}"),
=======
    match args.command
    {
        Command::Run { path } => match pipeline::run_pipeline_from_path(&path)
        {
            Ok(_) =>
            {}
            Err(err) => println!("{err}"),
        },
        Command::Compile { path } =>
        {
            match pipeline::run_compilation_pipeline_from_path(&path)
            {
                Ok(_) =>
                {}
                Err(err) => println!("{err}"),
            }
            //exewriter::write_exe_file(&path.with_extension("exe")).unwrap();
            //println!("Compiled to {}", path.with_extension("exe").display());
        }
        Command::Typecheck { path } => match pipeline::run_typecheck_pipeline_from_path(&path)
        {
            Ok(_) => println!("Typecheck passed"),
            Err(err) => println!("Typecheck error: {err}"),
        },
        Command::Debug { path: _ } =>
        {}
>>>>>>> Stashed changes
    }
}
