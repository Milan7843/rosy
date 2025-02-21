use std::path::PathBuf;

use crate::interpreter;
use crate::parser;
use crate::tokenizer;
use crate::tokenizer::Error;

pub fn run_pipeline_from_path(path: &std::path::PathBuf) -> Result<interpreter::Terminal, String> {
    // Read the file into a big string
    let content = std::fs::read_to_string(path).expect("could not read file");

    // Split the string into lines and make an iterator over them
    let lines_iterator = content.split("\n");
    let lines: Vec<&str> = lines_iterator.collect();

    return run_pipeline(lines);
}

pub fn run_pipeline(lines: Vec<&str>) -> Result<interpreter::Terminal, String> {
    let lines_copy = lines.clone();
    let base_expressions: Vec<parser::BaseExpr> = match parser::parse_strings(lines) {
        Ok(base_expressions) => base_expressions,
        Err(error) => {
            print_error(&error, &lines_copy);
            return Err(String::new());
        }
    };

    let output_terminal = match interpreter::interpret(base_expressions) {
        Ok(output_terminal) => output_terminal,
        Err(error) => {
            print_error(&error, &lines_copy);
            return Err(String::new());
        }
    };

    return Ok(output_terminal);
}

pub fn print_error(error: &Error, lines: &Vec<&str>) {
    match error {
        Error::SimpleError { message } => {
            println!("Error: {}", message);
        }
        Error::LocationError {
            message,
            row,
            col_start,
            col_end,
        } => {
            println!("{}", lines[*row as usize]);
            println!(
                "{}{}",
                " ".repeat(*col_start as usize),
                "^".repeat(*col_end as usize - *col_start as usize)
            );
            println!(
                "Error: {} (line {}, col {})",
                message,
                row + 1,
                col_start + 1
            );
        }
    }
}
