use std::path::PathBuf;

use crate::interpreter;
use crate::parser;
use crate::tokenizer;
use crate::tokenizer::Error;

pub fn run_pipeline_from_path(path: &std::path::PathBuf) -> Result<interpreter::Terminal, Error> {
    // Read the file into a big string
    let content = std::fs::read_to_string(path).expect("could not read file");

    // Split the string into lines and make an iterator over them
    let lines_iterator = content.split("\n");
    let lines: Vec<&str> = lines_iterator.collect();

    return run_pipeline(lines);
}

pub fn run_pipeline(lines: Vec<&str>) -> Result<interpreter::Terminal, Error> {
    let base_expressions: Vec<parser::BaseExpr> = match parser::parse_strings(lines) {
        Ok(base_expressions) => base_expressions,
        Err(error_message) => return Err(error_message),
    };

    parser::print_expressions(&base_expressions);

    let output_terminal = match interpreter::interpret(base_expressions) {
        Ok(output_terminal) => output_terminal,
        Err(error_message) => return Err(error_message),
    };

    return Ok(output_terminal);
}
