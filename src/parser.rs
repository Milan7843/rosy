use crate::tokenizer;
use crate::tokenizer::TokenLine;
use crate::tokenizer::Token;
use crate::tokenizer::SymbolType;
use std::f32::consts::{E, PI};


#[derive(PartialEq, Debug)]
pub enum BaseExpr {
    Simple {
        expr: RecExpr,
    },
    VariableAssignment {
        var_name: String,
        expr: RecExpr,
    },
    PlusEqualsStatement {
        var_name: String,
        expr: RecExpr,
    },
    IfStatement {
        clause: RecExpr,
        body: Vec<BaseExpr>,
    },
    ElseIfStatement {
        clause: RecExpr,
        body: Vec<BaseExpr>,
    },
    ElseStatement {
        body: Vec<BaseExpr>,
    },
    ForLoop {
        var_name: String,
        until: RecExpr,
        body: Vec<BaseExpr>,
    },
    FunctionDefinition {
        fun_name: String,
        args: Vec<String>,
        body: Vec<BaseExpr>,
    },
    Return {
        return_value: Option<RecExpr>,
    },
    Break,
}

#[derive(PartialEq, Debug)]
pub enum RecExpr {
    Variable {
        name: String,
    },
    Number {
        number: i32,
    },
    String {
        value: String,
    },
    False,
    True,
    Assign {
        variable_name: String,
        right: Box<RecExpr>,
    },
    Add {
        left: Box<RecExpr>,
        right: Box<RecExpr>,
    },
    Subtract {
        left: Box<RecExpr>,
        right: Box<RecExpr>,
    },
    Multiply {
        left: Box<RecExpr>,
        right: Box<RecExpr>,
    },
    Divide {
        left: Box<RecExpr>,
        right: Box<RecExpr>,
    },
    Power {
        left: Box<RecExpr>,
        right: Box<RecExpr>,
    },
    Minus {
        right: Box<RecExpr>,
    },
    Or {
        left: Box<RecExpr>,
        right: Box<RecExpr>,
    },
    And {
        left: Box<RecExpr>,
        right: Box<RecExpr>,
    },
    Equals {
        left: Box<RecExpr>,
        right: Box<RecExpr>,
    },
    Access {
        object: String,
        variable: String,
    },
    FunctionCall {
        function_name: String,
        args: Vec<RecExpr>,
    },
}

// Generic expression, leaves out detail in e.g. operator specifics
#[derive(PartialEq, Clone)]
enum GenExpr {
    Variable {
        name: String,
    },
    Number {
        number: i32,
    },
    String {
        value: String,
    },
    UnaryOp {
        operator: SymbolType,
        operand: Box<GenExpr>,
    },
    BinaryOp {
        left_operand: Box<GenExpr>,
        operator: SymbolType,
        right_operand: Box<GenExpr>,
    },
    FunctionCall {
        function_name: String,
        arguments: Vec<GenExpr>,
    },
}

pub fn parse(path: &std::path::PathBuf) -> Result<Vec<BaseExpr>, String> {
    // Read the file into a big string
    let content = std::fs::read_to_string(path).expect("could not read file");

    // Split the string into lines and make an iterator over them
    let lines_iterator = content.split("\n");
    let lines: Vec<&str> = lines_iterator.collect();

    return parse_strings(lines);
}

pub fn parse_strings(lines: Vec<&str>) -> Result<Vec<BaseExpr>, String> {
    // First: tokenize the lines
    let token_lines = match tokenizer::tokenize(lines) {
        Ok(token_lines) => token_lines,
        Err(error_message) => return Err(error_message),
    };

    // Second, parse the token lines into a list of base expresssions
    let base_expressions = match get_base_expressions(&token_lines) {
        Ok(tokens) => tokens,
        Err(error_message) => return Err(error_message),
    };

    return Ok(base_expressions);
}

fn get_first_occurence(
    tokens: &[Token],
    match_on: Vec<SymbolType>,
) -> Result<(SymbolType, usize), String> {
    for (i, token) in tokens.iter().enumerate() {
        for symbol_type in &match_on {
            if *token
                == (Token::Symbol {
                    symbol_type: symbol_type.clone(),
                })
            {
                return Ok((symbol_type.clone(), i));
            }
        }
    }

    return Err(String::from("No occurances found"));
}

fn get_expression(tokens: &[Token]) -> Result<RecExpr, String> {
    // First we get the generic expressions
    match get_generic_expression(tokens) {
        // And then convert the generic expression to a recursive expression
        Ok(gen_expr) => match generic_expression_to_recursive_expression(gen_expr) {
            Ok(rec_expr) => Ok(rec_expr),
            Err(e) => return Err(e),
        },
        Err(e) => return Err(e),
    }
}

fn generic_expression_to_recursive_expression(gen_expr: GenExpr) -> Result<RecExpr, String> {
    match gen_expr {
        GenExpr::Variable { name } => return Ok(RecExpr::Variable { name }),
        GenExpr::Number { number } => return Ok(RecExpr::Number { number }),
        GenExpr::String { value } => return Ok(RecExpr::String { value }),
        GenExpr::UnaryOp { operator, operand } => match operator {
            SymbolType::Minus => match generic_expression_to_recursive_expression(*operand) {
                Ok(operand_expr) => {
                    return Ok(RecExpr::Minus {
                        right: Box::new(operand_expr),
                    })
                }
                Err(e) => return Err(e),
            },
            _ => return Err(String::from("Invalid unary operator")),
        },
        GenExpr::BinaryOp {
            left_operand,
            operator,
            right_operand,
        } => match operator {
            SymbolType::Plus => {
                match (
                    generic_expression_to_recursive_expression(*left_operand),
                    generic_expression_to_recursive_expression(*right_operand),
                ) {
                    (Ok(left_expr), Ok(right_expr)) => {
                        return Ok(RecExpr::Add {
                            left: Box::new(left_expr),
                            right: Box::new(right_expr),
                        })
                    }
                    _ => return Err(String::from("Error reading subexpressions")),
                }
            }
            SymbolType::Minus => {
                match (
                    generic_expression_to_recursive_expression(*left_operand),
                    generic_expression_to_recursive_expression(*right_operand),
                ) {
                    (Ok(left_expr), Ok(right_expr)) => {
                        return Ok(RecExpr::Subtract {
                            left: Box::new(left_expr),
                            right: Box::new(right_expr),
                        })
                    }
                    _ => return Err(String::from("Error reading subexpressions")),
                }
            }
            SymbolType::Star => {
                match (
                    generic_expression_to_recursive_expression(*left_operand),
                    generic_expression_to_recursive_expression(*right_operand),
                ) {
                    (Ok(left_expr), Ok(right_expr)) => {
                        return Ok(RecExpr::Multiply {
                            left: Box::new(left_expr),
                            right: Box::new(right_expr),
                        })
                    }
                    _ => return Err(String::from("Error reading subexpressions")),
                }
            }
            SymbolType::Slash => {
                match (
                    generic_expression_to_recursive_expression(*left_operand),
                    generic_expression_to_recursive_expression(*right_operand),
                ) {
                    (Ok(left_expr), Ok(right_expr)) => {
                        return Ok(RecExpr::Divide {
                            left: Box::new(left_expr),
                            right: Box::new(right_expr),
                        })
                    }
                    _ => return Err(String::from("Error reading subexpressions")),
                }
            }
            SymbolType::Hat => {
                match (
                    generic_expression_to_recursive_expression(*left_operand),
                    generic_expression_to_recursive_expression(*right_operand),
                ) {
                    (Ok(left_expr), Ok(right_expr)) => {
                        return Ok(RecExpr::Power {
                            left: Box::new(left_expr),
                            right: Box::new(right_expr),
                        })
                    }
                    _ => return Err(String::from("Error reading subexpressions")),
                }
            }
            SymbolType::Or => {
                match (
                    generic_expression_to_recursive_expression(*left_operand),
                    generic_expression_to_recursive_expression(*right_operand),
                ) {
                    (Ok(left_expr), Ok(right_expr)) => {
                        return Ok(RecExpr::Or {
                            left: Box::new(left_expr),
                            right: Box::new(right_expr),
                        })
                    }
                    _ => return Err(String::from("Error reading subexpressions")),
                }
            }
            SymbolType::And => {
                match (
                    generic_expression_to_recursive_expression(*left_operand),
                    generic_expression_to_recursive_expression(*right_operand),
                ) {
                    (Ok(left_expr), Ok(right_expr)) => {
                        return Ok(RecExpr::And {
                            left: Box::new(left_expr),
                            right: Box::new(right_expr),
                        })
                    }
                    _ => return Err(String::from("Error reading subexpressions")),
                }
            }
            SymbolType::EqualsEquals => {
                match (
                    generic_expression_to_recursive_expression(*left_operand),
                    generic_expression_to_recursive_expression(*right_operand),
                ) {
                    (Ok(left_expr), Ok(right_expr)) => {
                        return Ok(RecExpr::Equals {
                            left: Box::new(left_expr),
                            right: Box::new(right_expr),
                        })
                    }
                    _ => return Err(String::from("Error reading subexpressions")),
                }
            }
            _ => return Err(String::from("Invalid binary operator")),
        },
        GenExpr::FunctionCall {
            function_name,
            arguments,
        } => {
            let mut rec_expr_arguments = Vec::new();
            for gen_argument in arguments {
                match generic_expression_to_recursive_expression(gen_argument) {
                    Ok(rec_expr_argument) => rec_expr_arguments.push(rec_expr_argument),
                    Err(e) => return Err(e),
                }
            }
            return Ok(RecExpr::FunctionCall {
                function_name,
                args: rec_expr_arguments,
            });
        }
    }
}

fn get_generic_expression(tokens: &[Token]) -> Result<GenExpr, String> {
    //let mut token_vec = Vec::from(tokens);
    //let root_token = parenthesize(&mut token_vec);

    let precedence_one = Vec::from([SymbolType::Plus, SymbolType::Minus]);
    let precedence_two = Vec::from([SymbolType::Star, SymbolType::Slash]);
    let precedence_three = Vec::from([SymbolType::Hat]);
    let precedence_four = Vec::from([SymbolType::EqualsEquals]);
    let precedence_five = Vec::from([SymbolType::And, SymbolType::Or]);

    // Looking for the first lowest precedence operators
    if let Ok((symbol_type, index)) = get_first_occurence(tokens, precedence_one) {
        let left = get_generic_expression(&tokens[0..index]);
        let right = get_generic_expression(&tokens[index + 1..]);

        match (left, right) {
            (Ok(left_expr), Ok(right_expr)) => {
                return Ok(GenExpr::BinaryOp {
                    left_operand: Box::new(left_expr),
                    operator: symbol_type,
                    right_operand: Box::new(right_expr),
                })
            }
            (Err(e), _) => return Err(e),
            (_, Err(e)) => return Err(e),
        }
    }

    // Looking for the second lowest precedence operators
    if let Ok((symbol_type, index)) = get_first_occurence(tokens, precedence_two) {
        let left = get_generic_expression(&tokens[0..index]);
        let right = get_generic_expression(&tokens[index + 1..]);

        match (left, right) {
            (Ok(left_expr), Ok(right_expr)) => {
                return Ok(GenExpr::BinaryOp {
                    left_operand: Box::new(left_expr),
                    operator: symbol_type,
                    right_operand: Box::new(right_expr),
                })
            }
            (Err(e), _) => return Err(e),
            (_, Err(e)) => return Err(e),
        }
    }

    // Looking for the third lowest precedence operators
    if let Ok((symbol_type, index)) = get_first_occurence(tokens, precedence_three) {
        let left = get_generic_expression(&tokens[0..index]);
        let right = get_generic_expression(&tokens[index + 1..]);

        match (left, right) {
            (Ok(left_expr), Ok(right_expr)) => {
                return Ok(GenExpr::BinaryOp {
                    left_operand: Box::new(left_expr),
                    operator: symbol_type,
                    right_operand: Box::new(right_expr),
                })
            }
            (Err(e), _) => return Err(e),
            (_, Err(e)) => return Err(e),
        }
    }

    // Looking for the fourth lowest precedence operators
    if let Ok((symbol_type, index)) = get_first_occurence(tokens, precedence_four) {
        let left = get_generic_expression(&tokens[0..index]);
        let right = get_generic_expression(&tokens[index + 1..]);

        match (left, right) {
            (Ok(left_expr), Ok(right_expr)) => {
                return Ok(GenExpr::BinaryOp {
                    left_operand: Box::new(left_expr),
                    operator: symbol_type,
                    right_operand: Box::new(right_expr),
                })
            }
            (Err(e), _) => return Err(e),
            (_, Err(e)) => return Err(e),
        }
    }

    // Looking for the fifth lowest precedence operators
    if let Ok((symbol_type, index)) = get_first_occurence(tokens, precedence_five) {
        let left = get_generic_expression(&tokens[0..index]);
        let right = get_generic_expression(&tokens[index + 1..]);

        match (left, right) {
            (Ok(left_expr), Ok(right_expr)) => {
                return Ok(GenExpr::BinaryOp {
                    left_operand: Box::new(left_expr),
                    operator: symbol_type,
                    right_operand: Box::new(right_expr),
                })
            }
            (Err(e), _) => return Err(e),
            (_, Err(e)) => return Err(e),
        }
    }

    // No operators were found at the highest level, thus the expression must
    // be a single expression which we can match for

    match tokens {
        [Token::Variable {
            name: function_name,
        }, Token::Symbol {
            symbol_type: SymbolType::ParenthesisOpen,
        }, rest @ ..]
            if rest.last()
                == Some(&Token::Symbol {
                    symbol_type: SymbolType::ParenthesisClosed,
                }) =>
        {
            match read_function_parameters(rest) {
                Ok(arguments) => {
                    return Ok(GenExpr::FunctionCall {
                        function_name: function_name.clone(),
                        arguments: arguments,
                    })
                }
                Err(e) => return Err(e),
            }
            // Possible function call
        }

        // Parentheses with content
        [Token::Symbol {
            symbol_type: SymbolType::ParenthesisOpen,
        }, content @ .., Token::Symbol {
            symbol_type: SymbolType::ParenthesisClosed,
        }] => {
            // Parentheses detected
            return get_generic_expression(&content);
        }

        // Just a variable
        [Token::Variable {
            name: variable_name,
        }] => {
            return Ok(GenExpr::Variable {
                name: variable_name.clone(),
            });
        }

        // Just a number
        [Token::Number { number }] => {
            return Ok(GenExpr::Number { number: *number });
        }

        // Just a string
        [Token::String { value }] => {
            return Ok(GenExpr::Variable {
                name: value.clone(),
            });
        }

        _ => return Err(String::from("No matching expression found")),
    }
}

fn read_function_parameters(line: &[Token]) -> Result<Vec<GenExpr>, String> {
    let mut parameters: Vec<GenExpr> = Vec::new();

    match read_function_parameters_rec(line, &mut parameters) {
        Ok(_) => return Ok(parameters),
        Err(e) => return Err(e),
    }
}

fn read_function_parameters_rec(
    line: &[Token],
    parameters: &mut Vec<GenExpr>,
) -> Result<String, String> {
    // Attempt to read a function parameter by trying to find a valid expression looking at each comma

    match read_function_parameter(line) {
        Ok((None, _)) => return Ok(String::from("Succcess")),
        Ok((Some(parameter), rest)) => {
            parameters.push(parameter);

            return read_function_parameters_rec(rest, parameters);
        }
        Err(e) => return Err(e),
    }
}

fn read_function_parameter(line: &[Token]) -> Result<(Option<GenExpr>, &[Token]), String> {
    // Attempt to read a function parameter by trying to find a valid expression looking at each comma
    match line {
        // Found the end of the function parameters, stopping now
        [Token::Symbol {
            symbol_type: SymbolType::ParenthesisClosed,
        }, rest @ ..] => {
            return Ok((None, rest));
        }
        _ => {
            if line.len() <= 1 {
                return Err(String::from("Could not find a valid function call"));
            }

            let mut parenthesis_depth = 1;
            for i in 1..line.len() {
                match line[i] {
                    Token::Symbol {
                        symbol_type: SymbolType::Comma,
                    } => {
                        // Check if we're in main body of the function call
                        if parenthesis_depth == 1 {

                            // Attempt to get an expression from all tokens up until this comma
                            match get_generic_expression(&line[0..i]) {
                                Ok(expr) => return Ok((Some(expr), &line[i + 1..])),
                                Err(_) => continue,
                            }
                        }
                    },
                    Token::Symbol {
                        symbol_type: SymbolType::ParenthesisOpen,
                    } => parenthesis_depth += 1,
                    Token::Symbol {
                        symbol_type: SymbolType::ParenthesisClosed,
                    } => {
                        parenthesis_depth -= 1;
                        if parenthesis_depth == 0 {
                            match get_generic_expression(&line[0..i]) {
                                Ok(expr) => return Ok((Some(expr), &line[i..])),
                                Err(e) => return Err(e),
                            }
                        }
                    }
                    _ => {}
                }
            }

            // No valid expression was found
            return Err(String::from("No valid function call was found"));
        }
    }
}

fn get_base_expressions(token_lines: &Vec<TokenLine>) -> Result<Vec<BaseExpr>, String> {

    let mut line_iterator = token_lines.iter().peekable();

    return get_base_expressions_with_indentation(&mut line_iterator, 0);
}

fn get_base_expressions_with_indentation(token_lines_iter: &mut std::iter::Peekable<std::slice::Iter<'_, TokenLine>>, indentation: i32) -> Result<Vec<BaseExpr>, String> {

    let mut expressions = Vec::new();

    while let Some(token_line) = token_lines_iter.peek() {
        // Stop when we find a line with lower indentation
        if token_line.indentation < indentation {
            return Ok(expressions);
        }

        match get_base_expression(token_lines_iter) {
            Ok(base_expr) => expressions.push(base_expr),
            Err(e) => return Err(e),
        }
    }

    return Ok(expressions);
}

fn get_base_expression(token_lines_iter: &mut std::iter::Peekable<std::slice::Iter<'_, TokenLine>>) -> Result<BaseExpr, String> {
    let Some(token_line) = token_lines_iter.next() else {
        return Err(String::from("No more lines found"));
    };

    let tokens = &token_line.tokens;

    match &tokens[..] {
        [Token::Variable { name }, Token::Symbol {
            symbol_type: SymbolType::Equals,
        }, rest @ ..] => {
            let expression = match get_expression(rest) {
                Ok(expression) => expression,
                Err(error_message) => return Err(error_message),
            };
            return Ok(BaseExpr::VariableAssignment {
                var_name: name.clone(),
                expr: expression,
            });
        }
        [Token::Variable { name }, Token::Symbol {
            symbol_type: SymbolType::PlusEquals,
        }, rest @ ..] => {
            let expression = match get_expression(rest) {
                Ok(expression) => expression,
                Err(error_message) => return Err(error_message),
            };
            return Ok(BaseExpr::PlusEqualsStatement {
                var_name: name.clone(),
                expr: expression,
            });
        }
        [Token::Symbol {
            symbol_type: SymbolType::If,
        }, rest @ ..] => {
            let clause = match get_expression(rest) {
                Ok(expression) => expression,
                Err(error_message) => return Err(error_message),
            };

            let body = match get_base_expressions_with_indentation(token_lines_iter, token_line.indentation + 1) {
                Ok(body) => body,
                Err(e) => return Err(e),
            };

            return Ok(BaseExpr::IfStatement {
                clause,
                body,
            });
        }
        [Token::Symbol {
            symbol_type: SymbolType::Else,
        }, Token::Symbol {
            symbol_type: SymbolType::If,
        }, rest @ ..] => {
            let clause = match get_expression(rest) {
                Ok(expression) => expression,
                Err(error_message) => return Err(error_message),
            };

            let body = match get_base_expressions_with_indentation(token_lines_iter, token_line.indentation + 1) {
                Ok(body) => body,
                Err(e) => return Err(e),
            };

            return Ok(BaseExpr::ElseIfStatement {
                clause,
                body,
            });
        }
        [Token::Symbol {
            symbol_type: SymbolType::Else,
        }, rest @ ..] => {
            if rest.len() > 0 {
                return Err(String::from("Unexpected extra tokens on else statement"));
            }

            let body = match get_base_expressions_with_indentation(token_lines_iter, token_line.indentation + 1) {
                Ok(body) => body,
                Err(e) => return Err(e),
            };

            return Ok(BaseExpr::ElseStatement { body });
        }
        [Token::Symbol {
            symbol_type: SymbolType::Break,
        }, rest @ ..] => {
            if rest.len() > 0 {
                return Err(String::from("Unexpected extra tokens on break statement"));
            }

            return Ok(BaseExpr::Break);
        }
        [Token::Symbol {
            symbol_type: SymbolType::For,
        }, Token::Variable {
            name: variable_name,
        }, Token::Symbol {
            symbol_type: SymbolType::In,
        }, rest @ ..] => {
            let range = match get_expression(rest) {
                Ok(expression) => expression,
                Err(error_message) => return Err(error_message),
            };

            let body = match get_base_expressions_with_indentation(token_lines_iter, token_line.indentation + 1) {
                Ok(body) => body,
                Err(e) => return Err(e),
            };

            return Ok(BaseExpr::ForLoop {
                var_name: variable_name.clone(),
                until: range,
                body: body,
            });
        }
        [Token::Symbol {
            symbol_type: SymbolType::Fun,
        }, Token::Variable {
            name: function_name,
        }, Token::Symbol {
            symbol_type: SymbolType::ParenthesisOpen,
        }, rest @ ..] => {
            let parameters = match parse_function_parameters(rest) {
                Ok(parameters) => parameters,
                Err(e) => return Err(e),
            };

            let body = match get_base_expressions_with_indentation(token_lines_iter, token_line.indentation + 1) {
                Ok(body) => body,
                Err(e) => return Err(e),
            };

            return Ok(BaseExpr::FunctionDefinition {
                fun_name: function_name.clone(),
                args: parameters,
                body: body,
            });
        }
        [Token::Symbol {
            symbol_type: SymbolType::Return,
        }, rest @ ..] => {
            if rest.len() == 0 {
                return Ok(BaseExpr::Return { return_value: None });
            }

            let expression = match get_expression(rest) {
                Ok(expression) => expression,
                Err(error_message) => return Err(error_message),
            };
            return Ok(BaseExpr::Return {
                return_value: Some(expression),
            });
        }
        rest @ _ => {
            let expression = match get_expression(rest) {
                Ok(expression) => expression,
                Err(error_message) => return Err(error_message),
            };
            return Ok(BaseExpr::Simple { expr: expression });
        }
    }
}

fn parse_function_parameters(tokens: &[Token]) -> Result<Vec<String>, String> {
    match tokens {
        [Token::Variable {
            name: parameter_name,
        }, Token::Symbol {
            symbol_type: SymbolType::Comma,
        }, rest @ ..] => match parse_function_parameters(rest) {
            Ok(mut other_parameters) => {
                other_parameters.insert(0, parameter_name.clone());
                return Ok(other_parameters);
            }
            Err(e) => return Err(e),
        },
        [Token::Variable {
            name: parameter_name,
        }, Token::Symbol {
            symbol_type: SymbolType::ParenthesisClosed,
        }] => return Ok(vec![parameter_name.clone()]),
        [Token::Symbol {
            symbol_type: SymbolType::ParenthesisClosed,
        }, rest @ ..] => {
            if rest.len() > 0 {
                return Err(String::from("Unexpected tokens after function definition"));
            }

            return Ok(Vec::new());
        }
        _ => return Err(String::from("Invalid function parameter definition")),
    }
}

/*
fn find_next_bracket(tokens: &[Token]) -> i32 {

    match tokens {
        [Token::Symbol { symbol_type: SymbolType::ParenthesisOpen }, rest @ ..] => {
            // Open bracket found: find next bracket and get the expression within it
        }
    }
}
*/

pub fn print_expressions(expressions: &Vec<BaseExpr>) {
    for expression in expressions {
        print_expression(expression, 0);
        print!("\n");
    }
}

fn print_indentation(indentation: i32) {
    for _ in 0..indentation {
        print!("  ")
    }
}

fn print_expression(expression: &BaseExpr, indentation: i32) {
    print_indentation(indentation);
    match expression {
        BaseExpr::Simple { expr } => print_recursive_expression(expr),
        BaseExpr::VariableAssignment { var_name, expr } => {
            print!("VarAssign({var_name:?}, ");
            print_recursive_expression(expr);
            print!(")");
        }
        BaseExpr::PlusEqualsStatement { var_name, expr } => {
            print!("PlusEquals({var_name:?}, ");
            print_recursive_expression(expr);
            print!(")");
        }
        BaseExpr::IfStatement { clause, body } => {
            print!("IfSt(");
            print_recursive_expression(clause);
            print!(")\n");
            for expr in body {
                print_expression(expr, indentation+1);
            }
        }
        BaseExpr::ElseIfStatement { clause, body } => {
            print!("ElseIfSt(");
            print_recursive_expression(clause);
            print!(")");
            for expr in body {
                print_expression(expr, indentation+1);
            }
        }
        BaseExpr::ElseStatement { body } => {
            print!("ElseSt(");
            for expr in body {
                print_expression(expr, indentation+1);
            }
            print!(")");
        }
        BaseExpr::ForLoop {
            var_name,
            until,
            body,
        } => {
            print!("For({var_name:?} in ");
            print_recursive_expression(until);
            print!("\n");
            for expr in body {
                print_expression(expr, indentation+1);
            }
            print!(")");
        }
        BaseExpr::FunctionDefinition {
            fun_name,
            args,
            body,
        } => {
            print!("fun {fun_name}(");
            for (i, arg) in args.iter().enumerate() {
                print!("{arg}");
                if i != args.len() - 1 {
                    print!(", ");
                }
            }
            print!(")\n");
            for expr in body {
                print_expression(expr, indentation+1);
            }
        }
        BaseExpr::Return { return_value } => {
            print!("Return(");
            match return_value {
                Some(expr) => print_recursive_expression(expr),
                None => print!(""),
            }
            print!(")")
        }
        BaseExpr::Break => print!("break"),
    }
}

fn print_recursive_expression(expression: &RecExpr) {
    match expression {
        RecExpr::Variable { name } => print!("Var({name:?})"),
        RecExpr::Number { number } => print!("Num({number})"),
        RecExpr::String { value } => print!("Str({value:?})"),
        RecExpr::False => print!("False"),
        RecExpr::True => print!("True"),
        RecExpr::Assign {
            variable_name,
            right,
        } => {
            print!("Var({variable_name:?}) = ");
            print_recursive_expression(&*right);
        }
        RecExpr::Add { left, right } => {
            print!("(");
            print_recursive_expression(&*left);
            print!(" + ");
            print_recursive_expression(&*right);
            print!(")")
        }
        RecExpr::Subtract { left, right } => {
            print!("(");
            print_recursive_expression(&*left);
            print!(" - ");
            print_recursive_expression(&*right);
            print!(")");
        }
        RecExpr::Multiply { left, right } => {
            print!("(");
            print_recursive_expression(&*left);
            print!(" * ");
            print_recursive_expression(&*right);
            print!(")");
        }
        RecExpr::Divide { left, right } => {
            print!("(");
            print_recursive_expression(&*left);
            print!(" / ");
            print_recursive_expression(&*right);
            print!(")");
        }
        RecExpr::Power { left, right } => {
            print!("(");
            print_recursive_expression(&*left);
            print!(" ^ ");
            print_recursive_expression(&*right);
            print!(")");
        }
        RecExpr::Minus { right } => {
            print!("(");
            print!("- ");
            print_recursive_expression(&*right);
            print!(")");
        }
        RecExpr::Or { left, right } => {
            print!("(");
            print_recursive_expression(&*left);
            print!(" or ");
            print_recursive_expression(&*right);
            print!(")");
        }
        RecExpr::And { left, right } => {
            print!("(");
            print_recursive_expression(&*left);
            print!(" and ");
            print_recursive_expression(&*right);
            print!(")");
        }
        RecExpr::Equals { left, right } => {
            print!("(");
            print_recursive_expression(&*left);
            print!(" == ");
            print_recursive_expression(&*right);
            print!(")");
        }
        RecExpr::Access { object, variable } => {
            print!("{object:?}.{variable:?}");
        }
        RecExpr::FunctionCall {
            function_name,
            args,
        } => {
            print!("Call({function_name:?} with (");
            for arg in args {
                print_recursive_expression(arg);
                print!(", ");
            }
            print!("))");
        }
    }
}
