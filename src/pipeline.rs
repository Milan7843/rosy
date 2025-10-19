use std::path;
use std::path::PathBuf;

use crate::desugarer;
use crate::interpreter;
use crate::parser;
use crate::tokenizer;
use crate::tokenizer::Error;
use crate::typechecker;
use crate::uniquify;
use crate::livenessanalysis;
use crate::compiler;
use crate::assembler;
use crate::exewriter;
use crate::optimiser;

pub fn run_typecheck_pipeline_from_path(path: &std::path::PathBuf) -> Result<String, String> {
    // Read the file into a big string
    let content = std::fs::read_to_string(path).expect("could not read file");

    // Split the string into lines and make an iterator over them
    let lines_iterator = content.split("\n");
    let lines: Vec<&str> = lines_iterator.collect();

    return run_typecheck_pipeline(lines);
}

pub fn run_typecheck_pipeline(lines: Vec<&str>) -> Result<String, String> {
    let lines_copy = lines.clone();
    let base_expressions: Vec<parser::BaseExpr<()>> = match parser::parse_strings(lines) {
        Ok(base_expressions) => base_expressions,
        Err(error) => {
            print_error(&error, &lines_copy);
            return Err(String::new());
        }
    };

    let desugared_base_expressions = desugarer::desugar(base_expressions);

    match typechecker::type_check_program(desugared_base_expressions, true) {
        Ok(_) => {}
        Err(error) => {
            print_error(&error, &lines_copy);
            return Err(String::new());
        }
    }

    return Ok("Typecheck passed".to_string());
}

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
    let base_expressions: Vec<parser::BaseExpr<()>> = match parser::parse_strings(lines) {
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

pub fn run_compilation_pipeline_from_path(path: &std::path::PathBuf, output_path: &std::path::PathBuf) -> Result<(), String> {
    // Read the file into a big string
    let content = std::fs::read_to_string(path).expect("could not read file");

    // Split the string into lines and make an iterator over them
    let lines_iterator = content.split("\n");
    let lines: Vec<&str> = lines_iterator.collect();

    return run_compilation_pipeline(lines, output_path);
}

pub fn run_compilation_pipeline(lines: Vec<&str>, output_path: &std::path::PathBuf) -> Result<(), String> {
    let lines_copy = lines.clone();
    let base_expressions: Vec<parser::BaseExpr<()>> = match parser::parse_strings(lines) {
        Ok(base_expressions) => base_expressions,
        Err(error) => {
            print_error(&error, &lines_copy);
            return Err(String::new());
        }
    };

    let desugared_base_expressions = desugarer::desugar(base_expressions);


    let mut typed_program =
        match typechecker::type_check_program(desugared_base_expressions.clone(), false) {
            Ok(typed_program) => typed_program,
            Err(error) => {
                print_error(&error, &lines_copy);
                return Err(String::new());
            }
        };

    //print!("Typed program:\n{:#?}\n", typed_program);

    // Perform uniquification
    uniquify::uniquify(&mut typed_program);

    //print!("Uniquified program:\n{:#?}\n", typed_program);

    let assembly = match compiler::compile(typed_program) {
        Ok(assembly) => assembly,
        Err(error) => {
            print_error(&error, &lines_copy);
            return Err(String::new());
        }
    };

    let optimised_assembly = optimiser::optimise_assembly(&assembly);

    let (mut machine_code, syscalls_to_resolve, starting_point) = assembler::assemble_program(optimised_assembly);

    println!("Machine code ({} bytes):", machine_code.len());
    for byte in &machine_code {
        print!("{:02X} ", byte);
    }

    match exewriter::write_exe_file(&output_path, &mut machine_code, &syscalls_to_resolve, starting_point) {
        Ok(_) => println!("\nCompiled to {}", output_path.display()),
        Err(err) => println!("Error writing exe file: {}", err),
    }

    return Ok(());
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
        Error::TypeError {
            message,
            expected,
            found,
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
                "Type error: {} (line {}, col {})",
                message,
                row + 1,
                col_start + 1
            );
            println!("Expected type: {:?}", expected);
            println!("Found type: {:?}", found);
        }
    }
}
