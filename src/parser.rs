use crate::tokenizer;
use crate::tokenizer::Error;
use crate::tokenizer::SymbolType;
use crate::tokenizer::Token;
use crate::tokenizer::TokenData;
use crate::tokenizer::TokenLine;
use std::f32::consts::{E, PI};

#[derive(PartialEq, Debug, Clone)]
pub struct BaseExpr {
    pub data: BaseExprData,
    pub row: usize,
    pub col_start: usize,
    pub col_end: usize,
}

#[derive(PartialEq, Debug, Clone)]
pub enum BaseExprData {
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
        condition: RecExpr,
        body: Vec<BaseExpr>,
        else_statement: Option<Box<BaseExpr>>,
    },
    ElseIfStatement {
        condition: RecExpr,
        body: Vec<BaseExpr>,
        else_statement: Option<Box<BaseExpr>>,
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

#[derive(PartialEq, Debug, Clone)]
pub struct RecExpr {
    pub data: RecExprData,
    pub row: usize,
    pub col_start: usize,
    pub col_end: usize,
}

#[derive(PartialEq, Debug, Clone)]
pub enum RecExprData {
    Variable {
        name: String,
    },
    Number {
        number: i32,
    },
    String {
        value: String,
    },
    Boolean {
        value: bool,
    },
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
    Not {
        right: Box<RecExpr>,
    },
    Equals {
        left: Box<RecExpr>,
        right: Box<RecExpr>,
    },
    NotEquals {
        left: Box<RecExpr>,
        right: Box<RecExpr>,
    },
    GreaterThan {
        left: Box<RecExpr>,
        right: Box<RecExpr>,
    },
    LessThan {
        left: Box<RecExpr>,
        right: Box<RecExpr>,
    },
    GreaterThanOrEqual {
        left: Box<RecExpr>,
        right: Box<RecExpr>,
    },
    LessThanOrEqual {
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
    List {
        elements: Vec<RecExpr>,
    },
    ListAccess {
        variable: String,
        index: Box<RecExpr>,
    },
}

// Generic expression, leaves out detail in e.g. operator specifics
#[derive(PartialEq, Clone)]
struct GenExpr {
    pub data: GenExprData,
    pub row: usize,
    pub col_start: usize,
    pub col_end: usize,
}

#[derive(PartialEq, Clone)]
enum GenExprData {
    Variable {
        name: String,
    },
    Number {
        number: i32,
    },
    String {
        value: String,
    },
    Boolean {
        value: bool,
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
    List {
        elements: Vec<GenExpr>,
    },
    ListAccess {
        variable: String,
        index: Box<GenExpr>,
    },
}

pub fn parse(path: &std::path::PathBuf) -> Result<Vec<BaseExpr>, Error> {
    // Read the file into a big string
    let content = std::fs::read_to_string(path).expect("could not read file");

    // Split the string into lines and make an iterator over them
    let lines_iterator = content.split("\n");
    let lines: Vec<&str> = lines_iterator.collect();

    return parse_strings(lines);
}

pub fn parse_strings(lines: Vec<&str>) -> Result<Vec<BaseExpr>, Error> {
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

    // Third, merge subsequent if statements
    let merged_base_expressions = match merge_if_statements(base_expressions) {
        Ok(base_expressions) => base_expressions,
        Err(error_message) => return Err(error_message),
    };

    return Ok(merged_base_expressions);
}

fn get_last_occurence(
    tokens: &[Token],
    match_on: Vec<SymbolType>,
) -> Result<(SymbolType, usize), Error> {
    let mut indentation_depth = 0;

    for (i, token) in tokens.iter().enumerate().rev() {
        // Keep track of the indentation depth
        match token.data {
            TokenData::Symbol {
                symbol_type: SymbolType::ParenthesisOpen,
            } => indentation_depth -= 1,
            TokenData::Symbol {
                symbol_type: SymbolType::ParenthesisClosed,
            } => indentation_depth += 1,
            TokenData::Symbol {
                symbol_type: SymbolType::SquareBracketOpen,
            } => indentation_depth -= 1,
            TokenData::Symbol {
                symbol_type: SymbolType::SquareBracketClosed,
            } => indentation_depth += 1,
            _ => {}
        }
        // We are only looking for top-level symbols,
        // so if we are inside a parenthesis, we skip
        if indentation_depth > 0 {
            continue;
        }

        for symbol_type in &match_on {
            if token.data
                == (TokenData::Symbol {
                    symbol_type: symbol_type.clone(),
                })
            {
                // Special case handling: difference between unary and binary minus
                if symbol_type == &SymbolType::Minus {
                    if i == 0 {
                        return Err(Error::SimpleError {
                            // if its the last token, it must be unary
                            message: format!("No occurances found"),
                        });
                    }
                    match tokens[i - 1].data {
                        TokenData::Symbol {
                            symbol_type: SymbolType::ParenthesisOpen,
                        }
                        | TokenData::Symbol {
                            symbol_type: SymbolType::Comma,
                        }
                        | TokenData::Symbol {
                            symbol_type: SymbolType::Equals,
                        }
                        | TokenData::Symbol {
                            symbol_type: SymbolType::Plus,
                        }
                        | TokenData::Symbol {
                            symbol_type: SymbolType::Minus,
                        }
                        | TokenData::Symbol {
                            symbol_type: SymbolType::Star,
                        }
                        | TokenData::Symbol {
                            symbol_type: SymbolType::Slash,
                        }
                        | TokenData::Symbol {
                            symbol_type: SymbolType::Hat,
                        }
                        | TokenData::Symbol {
                            symbol_type: SymbolType::Or,
                        }
                        | TokenData::Symbol {
                            symbol_type: SymbolType::And,
                        }
                        | TokenData::Symbol {
                            symbol_type: SymbolType::EqualsEquals,
                        }
                        | TokenData::Symbol {
                            symbol_type: SymbolType::NotEquals,
                        }
                        | TokenData::Symbol {
                            symbol_type: SymbolType::GreaterThan,
                        }
                        | TokenData::Symbol {
                            symbol_type: SymbolType::LessThan,
                        }
                        | TokenData::Symbol {
                            symbol_type: SymbolType::GreaterThanOrEqual,
                        }
                        | TokenData::Symbol {
                            symbol_type: SymbolType::LessThanOrEqual,
                        } => continue,
                        _ => return Ok((symbol_type.clone(), i)),
                    }
                }

                return Ok((symbol_type.clone(), i));
            }
        }
    }

    return Err(Error::SimpleError {
        message: format!("No occurances found"),
    });
}

fn get_expression(tokens: &[Token]) -> Result<RecExpr, Error> {
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

fn generic_expression_to_recursive_expression(gen_expr: GenExpr) -> Result<RecExpr, Error> {
    let data = match gen_expr.data {
        GenExprData::Variable { name } => RecExprData::Variable { name },
        GenExprData::Number { number } => RecExprData::Number { number },
        GenExprData::String { value } => RecExprData::String { value },
        GenExprData::Boolean { value } => RecExprData::Boolean { value },
        GenExprData::UnaryOp { operator, operand } => match operator {
            SymbolType::Minus => match generic_expression_to_recursive_expression(*operand) {
                Ok(operand_expr) => RecExprData::Minus {
                    right: Box::new(operand_expr),
                },
                Err(e) => return Err(e),
            },
            SymbolType::Not => match generic_expression_to_recursive_expression(*operand) {
                Ok(operand_expr) => RecExprData::Not {
                    right: Box::new(operand_expr),
                },
                Err(e) => return Err(e),
            },
            _ => {
                return Err(Error::LocationError {
                    message: format!(
                        "Invalid unary operator: {}",
                        tokenizer::get_symbol_from_type(&operator)
                    ),
                    row: gen_expr.row,
                    col_start: gen_expr.col_start,
                    col_end: gen_expr.col_end,
                });
            }
        },
        GenExprData::BinaryOp {
            left_operand,
            operator,
            right_operand,
        } => match operator {
            SymbolType::Plus => {
                match (
                    generic_expression_to_recursive_expression(*left_operand),
                    generic_expression_to_recursive_expression(*right_operand),
                ) {
                    (Ok(left_expr), Ok(right_expr)) => RecExprData::Add {
                        left: Box::new(left_expr),
                        right: Box::new(right_expr),
                    },
                    (Err(e), _) => return Err(e),
                    (_, Err(e)) => return Err(e),
                }
            }
            SymbolType::Minus => {
                match (
                    generic_expression_to_recursive_expression(*left_operand),
                    generic_expression_to_recursive_expression(*right_operand),
                ) {
                    (Ok(left_expr), Ok(right_expr)) => RecExprData::Subtract {
                        left: Box::new(left_expr),
                        right: Box::new(right_expr),
                    },
                    (Err(e), _) => return Err(e),
                    (_, Err(e)) => return Err(e),
                }
            }
            SymbolType::Star => {
                match (
                    generic_expression_to_recursive_expression(*left_operand),
                    generic_expression_to_recursive_expression(*right_operand),
                ) {
                    (Ok(left_expr), Ok(right_expr)) => RecExprData::Multiply {
                        left: Box::new(left_expr),
                        right: Box::new(right_expr),
                    },
                    (Err(e), _) => return Err(e),
                    (_, Err(e)) => return Err(e),
                }
            }
            SymbolType::Slash => {
                match (
                    generic_expression_to_recursive_expression(*left_operand),
                    generic_expression_to_recursive_expression(*right_operand),
                ) {
                    (Ok(left_expr), Ok(right_expr)) => RecExprData::Divide {
                        left: Box::new(left_expr),
                        right: Box::new(right_expr),
                    },
                    (Err(e), _) => return Err(e),
                    (_, Err(e)) => return Err(e),
                }
            }
            SymbolType::Hat => {
                match (
                    generic_expression_to_recursive_expression(*left_operand),
                    generic_expression_to_recursive_expression(*right_operand),
                ) {
                    (Ok(left_expr), Ok(right_expr)) => RecExprData::Power {
                        left: Box::new(left_expr),
                        right: Box::new(right_expr),
                    },
                    (Err(e), _) => return Err(e),
                    (_, Err(e)) => return Err(e),
                }
            }
            SymbolType::Or => {
                match (
                    generic_expression_to_recursive_expression(*left_operand),
                    generic_expression_to_recursive_expression(*right_operand),
                ) {
                    (Ok(left_expr), Ok(right_expr)) => RecExprData::Or {
                        left: Box::new(left_expr),
                        right: Box::new(right_expr),
                    },
                    (Err(e), _) => return Err(e),
                    (_, Err(e)) => return Err(e),
                }
            }
            SymbolType::And => {
                match (
                    generic_expression_to_recursive_expression(*left_operand),
                    generic_expression_to_recursive_expression(*right_operand),
                ) {
                    (Ok(left_expr), Ok(right_expr)) => RecExprData::And {
                        left: Box::new(left_expr),
                        right: Box::new(right_expr),
                    },
                    (Err(e), _) => return Err(e),
                    (_, Err(e)) => return Err(e),
                }
            }
            SymbolType::EqualsEquals => {
                match (
                    generic_expression_to_recursive_expression(*left_operand),
                    generic_expression_to_recursive_expression(*right_operand),
                ) {
                    (Ok(left_expr), Ok(right_expr)) => RecExprData::Equals {
                        left: Box::new(left_expr),
                        right: Box::new(right_expr),
                    },
                    (Err(e), _) => return Err(e),
                    (_, Err(e)) => return Err(e),
                }
            }
            SymbolType::NotEquals => {
                match (
                    generic_expression_to_recursive_expression(*left_operand),
                    generic_expression_to_recursive_expression(*right_operand),
                ) {
                    (Ok(left_expr), Ok(right_expr)) => RecExprData::NotEquals {
                        left: Box::new(left_expr),
                        right: Box::new(right_expr),
                    },
                    (Err(e), _) => return Err(e),
                    (_, Err(e)) => return Err(e),
                }
            }
            SymbolType::GreaterThan => {
                match (
                    generic_expression_to_recursive_expression(*left_operand),
                    generic_expression_to_recursive_expression(*right_operand),
                ) {
                    (Ok(left_expr), Ok(right_expr)) => RecExprData::GreaterThan {
                        left: Box::new(left_expr),
                        right: Box::new(right_expr),
                    },
                    (Err(e), _) => return Err(e),
                    (_, Err(e)) => return Err(e),
                }
            }
            SymbolType::GreaterThanOrEqual => {
                match (
                    generic_expression_to_recursive_expression(*left_operand),
                    generic_expression_to_recursive_expression(*right_operand),
                ) {
                    (Ok(left_expr), Ok(right_expr)) => RecExprData::GreaterThanOrEqual {
                        left: Box::new(left_expr),
                        right: Box::new(right_expr),
                    },
                    (Err(e), _) => return Err(e),
                    (_, Err(e)) => return Err(e),
                }
            }
            SymbolType::LessThan => {
                match (
                    generic_expression_to_recursive_expression(*left_operand),
                    generic_expression_to_recursive_expression(*right_operand),
                ) {
                    (Ok(left_expr), Ok(right_expr)) => RecExprData::LessThan {
                        left: Box::new(left_expr),
                        right: Box::new(right_expr),
                    },
                    (Err(e), _) => return Err(e),
                    (_, Err(e)) => return Err(e),
                }
            }
            SymbolType::LessThanOrEqual => {
                match (
                    generic_expression_to_recursive_expression(*left_operand),
                    generic_expression_to_recursive_expression(*right_operand),
                ) {
                    (Ok(left_expr), Ok(right_expr)) => RecExprData::LessThanOrEqual {
                        left: Box::new(left_expr),
                        right: Box::new(right_expr),
                    },
                    (Err(e), _) => return Err(e),
                    (_, Err(e)) => return Err(e),
                }
            }

            _ => {
                return Err(Error::LocationError {
                    message: format!(
                        "Invalid binary operator: {}",
                        tokenizer::get_symbol_from_type(&operator)
                    ),
                    row: gen_expr.row,
                    col_start: gen_expr.col_start,
                    col_end: gen_expr.col_end,
                });
            }
        },
        GenExprData::FunctionCall {
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

            RecExprData::FunctionCall {
                function_name,
                args: rec_expr_arguments,
            }
        }
        GenExprData::List { elements } => {
            let mut rec_expr_elements = Vec::new();
            for gen_element in elements {
                match generic_expression_to_recursive_expression(gen_element) {
                    Ok(rec_expr_element) => rec_expr_elements.push(rec_expr_element),
                    Err(e) => return Err(e),
                }
            }

            RecExprData::List {
                elements: rec_expr_elements,
            }
        }
        GenExprData::ListAccess { variable, index } => {
            match generic_expression_to_recursive_expression(*index) {
                Ok(rec_expr_index) => RecExprData::ListAccess {
                    variable,
                    index: Box::new(rec_expr_index),
                },
                Err(e) => return Err(e),
            }
        }
    };

    return Ok(RecExpr {
        data,
        row: gen_expr.row,
        col_start: gen_expr.col_start,
        col_end: gen_expr.col_end,
    });
}

fn get_generic_expression(tokens: &[Token]) -> Result<GenExpr, Error> {
    //let mut token_vec = Vec::from(tokens);
    //let root_token = parenthesize(&mut token_vec);

    let precedence_one = Vec::from([SymbolType::Or]);
    let precedence_two = Vec::from([SymbolType::And]);
    let precedence_three = Vec::from([SymbolType::EqualsEquals, SymbolType::NotEquals]);
    let precedence_four = Vec::from([
        SymbolType::GreaterThan,
        SymbolType::LessThan,
        SymbolType::GreaterThanOrEqual,
        SymbolType::LessThanOrEqual,
    ]);
    let precedence_five = Vec::from([SymbolType::Plus, SymbolType::Minus]);
    let precedence_six = Vec::from([SymbolType::Star, SymbolType::Slash]);
    let precedence_seven = Vec::from([SymbolType::Hat]);

    // Looking for the first lowest precedence operators
    if let Ok((symbol_type, index)) = get_last_occurence(tokens, precedence_one) {
        let left = get_generic_expression(&tokens[0..index]);
        let right = get_generic_expression(&tokens[index + 1..]);

        match (left, right) {
            (Ok(left_expr), Ok(right_expr)) => {
                let row = left_expr.row;
                let col_start = left_expr.col_start;
                let col_end = right_expr.col_end;
                return Ok(GenExpr {
                    data: GenExprData::BinaryOp {
                        left_operand: Box::new(left_expr),
                        operator: symbol_type,
                        right_operand: Box::new(right_expr),
                    },
                    row,
                    col_start,
                    col_end,
                });
            }
            (Err(e), _) => return Err(e),
            (_, Err(e)) => return Err(e),
        }
    }

    // Looking for the second lowest precedence operators
    if let Ok((symbol_type, index)) = get_last_occurence(tokens, precedence_two) {
        let left = get_generic_expression(&tokens[0..index]);
        let right = get_generic_expression(&tokens[index + 1..]);

        match (left, right) {
            (Ok(left_expr), Ok(right_expr)) => {
                let row = left_expr.row;
                let col_start = left_expr.col_start;
                let col_end = right_expr.col_end;
                return Ok(GenExpr {
                    data: GenExprData::BinaryOp {
                        left_operand: Box::new(left_expr),
                        operator: symbol_type,
                        right_operand: Box::new(right_expr),
                    },
                    row,
                    col_start,
                    col_end,
                });
            }
            (Err(e), _) => return Err(e),
            (_, Err(e)) => return Err(e),
        }
    }

    // Looking for the third lowest precedence operators
    if let Ok((symbol_type, index)) = get_last_occurence(tokens, precedence_three) {
        let left = get_generic_expression(&tokens[0..index]);
        let right = get_generic_expression(&tokens[index + 1..]);

        match (left, right) {
            (Ok(left_expr), Ok(right_expr)) => {
                let row = left_expr.row;
                let col_start = left_expr.col_start;
                let col_end = right_expr.col_end;
                return Ok(GenExpr {
                    data: GenExprData::BinaryOp {
                        left_operand: Box::new(left_expr),
                        operator: symbol_type,
                        right_operand: Box::new(right_expr),
                    },
                    row,
                    col_start,
                    col_end,
                });
            }
            (Err(e), _) => return Err(e),
            (_, Err(e)) => return Err(e),
        }
    }

    match tokens {
        [Token {
            data: TokenData::Symbol {
                symbol_type: SymbolType::Not,
            },
            row: row_not,
            col_start: col_start_not,
            ..
        }, rest @ ..] => {
            // not statement detected
            match get_generic_expression(&rest) {
                Ok(expr) => {
                    let expr_col_end = expr.col_end;
                    return Ok(GenExpr {
                        data: GenExprData::UnaryOp {
                            operator: SymbolType::Not,
                            operand: Box::new(expr),
                        },
                        row: *row_not,
                        col_start: *col_start_not,
                        col_end: expr_col_end,
                    });
                }
                Err(e) => return Err(e),
            }
        }
        _ => {}
    }

    // Looking for the fourth lowest precedence operators
    if let Ok((symbol_type, index)) = get_last_occurence(tokens, precedence_four) {
        let left = get_generic_expression(&tokens[0..index]);
        let right = get_generic_expression(&tokens[index + 1..]);

        match (left, right) {
            (Ok(left_expr), Ok(right_expr)) => {
                let row = left_expr.row;
                let col_start = left_expr.col_start;
                let col_end = right_expr.col_end;
                return Ok(GenExpr {
                    data: GenExprData::BinaryOp {
                        left_operand: Box::new(left_expr),
                        operator: symbol_type,
                        right_operand: Box::new(right_expr),
                    },
                    row,
                    col_start,
                    col_end,
                });
            }
            (Err(e), _) => return Err(e),
            (_, Err(e)) => return Err(e),
        }
    }

    // Looking for the fifth lowest precedence operators
    if let Ok((symbol_type, index)) = get_last_occurence(tokens, precedence_five) {
        let left = get_generic_expression(&tokens[0..index]);
        let right = get_generic_expression(&tokens[index + 1..]);

        match (left, right) {
            (Ok(left_expr), Ok(right_expr)) => {
                let row = left_expr.row;
                let col_start = left_expr.col_start;
                let col_end = right_expr.col_end;
                return Ok(GenExpr {
                    data: GenExprData::BinaryOp {
                        left_operand: Box::new(left_expr),
                        operator: symbol_type,
                        right_operand: Box::new(right_expr),
                    },
                    row,
                    col_start,
                    col_end,
                });
            }
            (Err(e), _) => return Err(e),
            (_, Err(e)) => return Err(e),
        }
    }

    // Looking for the sixth lowest precedence operators
    if let Ok((symbol_type, index)) = get_last_occurence(tokens, precedence_six) {
        let left = get_generic_expression(&tokens[0..index]);
        let right = get_generic_expression(&tokens[index + 1..]);

        match (left, right) {
            (Ok(left_expr), Ok(right_expr)) => {
                let row = left_expr.row;
                let col_start = left_expr.col_start;
                let col_end = right_expr.col_end;
                return Ok(GenExpr {
                    data: GenExprData::BinaryOp {
                        left_operand: Box::new(left_expr),
                        operator: symbol_type,
                        right_operand: Box::new(right_expr),
                    },
                    row,
                    col_start,
                    col_end,
                });
            }
            (Err(e), _) => return Err(e),
            (_, Err(e)) => return Err(e),
        }
    }

    match tokens {
        // negative unary operator
        [Token {
            data: TokenData::Symbol {
                symbol_type: SymbolType::Minus,
            },
            row: row_not,
            col_start: col_start_not,
            ..
        }, rest @ ..] => {
            // unary - statement detected
            match get_generic_expression(&rest) {
                Ok(expr) => {
                    let expr_col_end = expr.col_end;
                    return Ok(GenExpr {
                        data: GenExprData::UnaryOp {
                            operator: SymbolType::Minus,
                            operand: Box::new(expr),
                        },
                        row: *row_not,
                        col_start: *col_start_not,
                        col_end: expr_col_end,
                    });
                }
                Err(e) => return Err(e),
            }
        }
        _ => {}
    }

    // Looking for the seventh lowest precedence operators
    if let Ok((symbol_type, index)) = get_last_occurence(tokens, precedence_seven) {
        let left = get_generic_expression(&tokens[0..index]);
        let right = get_generic_expression(&tokens[index + 1..]);

        match (left, right) {
            (Ok(left_expr), Ok(right_expr)) => {
                let row = left_expr.row;
                let col_start = left_expr.col_start;
                let col_end = right_expr.col_end;
                return Ok(GenExpr {
                    data: GenExprData::BinaryOp {
                        left_operand: Box::new(left_expr),
                        operator: symbol_type,
                        right_operand: Box::new(right_expr),
                    },
                    row,
                    col_start,
                    col_end,
                });
            }
            (Err(e), _) => return Err(e),
            (_, Err(e)) => return Err(e),
        }
    }

    // No operators were found at the highest level, thus the expression must
    // be a single expression which we can match for

    match tokens {
        [Token {
            data: TokenData::Variable {
                name: function_name,
            },
            ..
        }, Token {
            data:
                TokenData::Symbol {
                    symbol_type: SymbolType::ParenthesisOpen,
                },
            ..
        }, rest @ ..]
            // Last token must be a closing parenthesis
            if rest.last().unwrap().data
                == TokenData::Symbol {
                    symbol_type: SymbolType::ParenthesisClosed,
                } =>
        {
            match read_function_parameters(rest) {
                Ok(arguments) => {
                    return Ok(GenExpr {
                        data: GenExprData::FunctionCall {
                            function_name: function_name.clone(),
                            arguments: arguments,
                        },
                        row: tokens[0].row,
                        col_start: tokens[0].col_start,
                        col_end: tokens[tokens.len() - 1].col_end,
                    })
                }
                Err(e) => return Err(e),
            }
            // Possible function call
        }

        // List [a, b, c]
        [Token {
            data:
                TokenData::Symbol {
                    symbol_type: SymbolType::SquareBracketOpen,
                },
            ..
        }, rest @ ..]
            // Last token must be a closing parenthesis
            if rest.last().unwrap().data
                == TokenData::Symbol {
                    symbol_type: SymbolType::SquareBracketClosed,
                } =>
        {
            match read_list_items(rest) {
                Ok(arguments) => {
                    return Ok(GenExpr {
                        data: GenExprData::List {
                            elements: arguments,
                        },
                        row: tokens[0].row,
                        col_start: tokens[0].col_start,
                        col_end: tokens[tokens.len() - 1].col_end,
                    })
                }
                Err(e) => return Err(e),
            }
        }

        // List access
        [Token {
            data: TokenData::Variable { name: variable_name },
            row : row_variable,
            col_start: col_start_variable,
            ..
        },
        Token {
            data:
                TokenData::Symbol {
                    symbol_type: SymbolType::SquareBracketOpen,
                },
            ..
        }, rest @ .., Token {
            data:
                TokenData::Symbol {
                    symbol_type: SymbolType::SquareBracketClosed,
                },
            col_end: col_end_parenthesis,
            ..
        }] =>
        {
            match get_generic_expression(&rest) {
                Ok(index) => {
                    return Ok(GenExpr {
                        data: GenExprData::ListAccess {
                            variable: variable_name.clone(),
                            index: Box::new(index),
                        },
                        row: *row_variable,
                        col_start: *col_start_variable,
                        col_end: *col_end_parenthesis,
                    })
                }
                Err(e) => return Err(e),
            }
        }

        // Parentheses with content
        [Token {
            data:
                TokenData::Symbol {
                    symbol_type: SymbolType::ParenthesisOpen,
                },
            col_start: col_start_parenthesis,
            ..
        }, content @ .., Token {
            data:
                TokenData::Symbol {
                    symbol_type: SymbolType::ParenthesisClosed,
                },
            col_end: col_end_parenthesis,
            ..
        }] => {
            // Parentheses detected
            match get_generic_expression(&content) {
                Ok(mut expr) => {
                    expr.col_start = *col_start_parenthesis;
                    expr.col_end = *col_end_parenthesis;

                    return Ok(expr);
                }
                Err(e) => return Err(e),
            }
        }
        // not statement
        [Token {
            data:
                TokenData::Symbol {
                    symbol_type: SymbolType::Not,
                },
            row: row_not,
            col_start: col_start_not,
            ..
        }, rest @ ..] => {
            // not statement detected
            match get_generic_expression(&rest) {
                Ok(expr) => {
                    let expr_col_end = expr.col_end;
                    return Ok(GenExpr {
                        data: GenExprData::UnaryOp {
                            operator: SymbolType::Not,
                            operand: Box::new(expr),
                        },
                        row: *row_not,
                        col_start: *col_start_not,
                        col_end: expr_col_end,
                    })
                }
                Err(e) => return Err(e),
            }
        }

        // Just a variable
        [Token {
            data: TokenData::Variable {
                name: variable_name,
            },
            ..
        }] => {
            return Ok(GenExpr {
                data: GenExprData::Variable {
                    name: variable_name.clone(),
                },
                row: tokens[0].row,
                col_start: tokens[0].col_start,
                col_end: tokens[0].col_end,
            })
        }

        // Just a number
        [Token {
            data: TokenData::Number { number },
            ..
        }] => {
            return Ok(GenExpr {
                data: GenExprData::Number { number: *number },
                row: tokens[0].row,
                col_start: tokens[0].col_start,
                col_end: tokens[0].col_end,
            })
        }

        // negative unary operator
        [Token {
            data:
                TokenData::Symbol {
                    symbol_type: SymbolType::Minus,
                },
            row: row_not,
            col_start: col_start_not,
            ..
        }, rest @ ..] => {
            // unary - statement detected
            match get_generic_expression(&rest) {
                Ok(expr) => {
                    let expr_col_end = expr.col_end;
                    return Ok(GenExpr {
                        data: GenExprData::UnaryOp {
                            operator: SymbolType::Minus,
                            operand: Box::new(expr),
                        },
                        row: *row_not,
                        col_start: *col_start_not,
                        col_end: expr_col_end,
                    })
                }
                Err(e) => return Err(e),
            }
        }

        // Just a string
        [Token {
            data: TokenData::String { value },
            ..
        }] => {
            return Ok(GenExpr {
                data: GenExprData::String {
                    value: value.clone(),
                },
                row: tokens[0].row,
                col_start: tokens[0].col_start,
                col_end: tokens[0].col_end,
            })
        }

        // True
        [Token {
            data: TokenData::Symbol {
                symbol_type: SymbolType::True,
            },
            ..
        }] => {
            return Ok(GenExpr {
                data: GenExprData::Boolean { value: true },
                row: tokens[0].row,
                col_start: tokens[0].col_start,
                col_end: tokens[0].col_end,
            })
        }

        // False
        [Token {
            data: TokenData::Symbol {
                symbol_type: SymbolType::False,
            },
            ..
        }] => {
            return Ok(GenExpr {
                data: GenExprData::Boolean { value: false },
                row: tokens[0].row,
                col_start: tokens[0].col_start,
                col_end: tokens[0].col_end,
            })
        }

        [first, .., last] => {
            return Err(Error::LocationError { message: format!("No expression found"), row: first.row, col_start: first.col_start, col_end: last.col_end })
        }
        [first, ..] => {
            return Err(Error::LocationError { message: format!("No expression found"), row: first.row, col_start: first.col_start, col_end: first.col_end })
        }
        [] => {
            return Err(Error::SimpleError { message: format!("No expression found") })
        }
    }
}

fn read_function_parameters(line: &[Token]) -> Result<Vec<GenExpr>, Error> {
    let mut parameters: Vec<GenExpr> = Vec::new();

    match read_function_parameters_rec(line, &mut parameters) {
        Ok(_) => return Ok(parameters),
        Err(_) => {
            return Err(Error::LocationError {
                message: format!("Could not find a valid function call"),
                row: line[0].row,
                col_start: line[0].col_start,
                col_end: line[line.len() - 1].col_end - 1,
            })
        }
    }
}

fn read_function_parameters_rec(
    line: &[Token],
    parameters: &mut Vec<GenExpr>,
) -> Result<String, Error> {
    // Attempt to read a function parameter by trying to find a valid expression looking at each comma

    match read_function_parameter(line) {
        Ok((None, _)) => return Ok(String::from("Success")),
        Ok((Some(parameter), rest)) => {
            parameters.push(parameter);

            return read_function_parameters_rec(rest, parameters);
        }
        Err(e) => return Err(e),
    }
}

fn read_function_parameter(line: &[Token]) -> Result<(Option<GenExpr>, &[Token]), Error> {
    // Attempt to read a function parameter by trying to find a valid expression looking at each comma
    match line {
        // Found the end of the function parameters, stopping now
        [Token {
            data:
                TokenData::Symbol {
                    symbol_type: SymbolType::ParenthesisClosed,
                },
            ..
        }, rest @ ..] => return Ok((None, rest)),
        _ => {
            if line.len() <= 1 {
                return Err(Error::SimpleError {
                    message: format!("Could not find a valid function call"),
                });
            }

            let mut parenthesis_depth = 1;
            match line[0].data {
                TokenData::Symbol {
                    symbol_type: SymbolType::ParenthesisOpen,
                } => parenthesis_depth += 1,
                TokenData::Symbol {
                    symbol_type: SymbolType::SquareBracketOpen,
                } => parenthesis_depth += 1,
                _ => {}
            }
            for i in 1..line.len() {
                match line[i].data {
                    TokenData::Symbol {
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
                    }
                    TokenData::Symbol {
                        symbol_type: SymbolType::ParenthesisOpen,
                    } => parenthesis_depth += 1,
                    TokenData::Symbol {
                        symbol_type: SymbolType::SquareBracketOpen,
                    } => parenthesis_depth += 1,
                    TokenData::Symbol {
                        symbol_type: SymbolType::SquareBracketClosed,
                    } => parenthesis_depth -= 1,
                    TokenData::Symbol {
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
            return Err(Error::SimpleError {
                message: format!("Could not find a valid function call"),
            });
        }
    }
}

fn read_list_items(line: &[Token]) -> Result<Vec<GenExpr>, Error> {
    let mut items: Vec<GenExpr> = Vec::new();

    match read_list_items_rec(line, &mut items) {
        Ok(_) => return Ok(items),
        Err(e) => {
            return Err(Error::LocationError {
                message: format!("Could not find a valid list"),
                row: line[0].row,
                col_start: line[0].col_start,
                col_end: line[line.len() - 1].col_end - 1,
            })
        }
    }
}

fn read_list_items_rec(line: &[Token], items: &mut Vec<GenExpr>) -> Result<String, Error> {
    match read_list_item(line) {
        Ok((None, _)) => return Ok(String::from("Succcess")),
        Ok((Some(item), rest)) => {
            items.push(item);

            return read_list_items_rec(rest, items);
        }
        Err(e) => return Err(e),
    }
}

fn read_list_item(line: &[Token]) -> Result<(Option<GenExpr>, &[Token]), Error> {
    // Attempt to read a list item by trying to find a valid expression looking at each comma
    match line {
        // Found the end of the list items, stopping now
        [Token {
            data:
                TokenData::Symbol {
                    symbol_type: SymbolType::SquareBracketClosed,
                },
            ..
        }, rest @ ..] => return Ok((None, rest)),
        _ => {
            if line.len() <= 1 {
                return Err(Error::SimpleError {
                    message: format!("Could not find a valid list"),
                });
            }

            let mut parenthesis_depth = 1;
            match line[0].data {
                TokenData::Symbol {
                    symbol_type: SymbolType::ParenthesisOpen,
                } => parenthesis_depth += 1,
                TokenData::Symbol {
                    symbol_type: SymbolType::SquareBracketOpen,
                } => parenthesis_depth += 1,
                _ => {}
            }
            for i in 1..line.len() {
                match line[i].data {
                    TokenData::Symbol {
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
                    }
                    TokenData::Symbol {
                        symbol_type: SymbolType::ParenthesisOpen,
                    } => parenthesis_depth += 1,
                    TokenData::Symbol {
                        symbol_type: SymbolType::SquareBracketOpen,
                    } => parenthesis_depth += 1,
                    TokenData::Symbol {
                        symbol_type: SymbolType::ParenthesisClosed,
                    } => parenthesis_depth -= 1,
                    TokenData::Symbol {
                        symbol_type: SymbolType::SquareBracketClosed,
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
            return Err(Error::SimpleError {
                message: format!("Could not find a valid list"),
            });
        }
    }
}

fn add_to_if_statement(
    if_statement: &mut BaseExpr,
    else_statement_to_add: BaseExpr,
) -> Result<String, Error> {
    match &mut if_statement.data {
        BaseExprData::IfStatement { else_statement, .. }
        | BaseExprData::ElseIfStatement { else_statement, .. } => match else_statement {
            Some(embedded_if_statement) => {
                return add_to_if_statement(embedded_if_statement, else_statement_to_add);
            }
            None => {
                *else_statement = Some(Box::new(else_statement_to_add));
                return Ok(String::from("OK"));
            }
        },
        _ => {
            return Err(Error::LocationError {
                message: format!("Could not find if statement to add else statement to"),
                row: else_statement_to_add.row,
                col_start: else_statement_to_add.col_start,
                col_end: else_statement_to_add.col_end,
            });
        }
    }
}

fn merge_if_statements(base_expressions: Vec<BaseExpr>) -> Result<Vec<BaseExpr>, Error> {
    let mut merged_statements = Vec::new();

    // This can probably be done without copying every single item...

    for base_expression in base_expressions {
        match base_expression.data {
            BaseExprData::IfStatement {
                condition,
                body,
                else_statement,
            } => {
                // Recursively merge if statements in the body
                let merged_body = match merge_if_statements(body) {
                    Ok(body) => body,
                    Err(e) => return Err(e),
                };

                merged_statements.push(BaseExpr {
                    data: BaseExprData::IfStatement {
                        condition: condition,
                        body: merged_body,
                        else_statement: else_statement,
                    },
                    row: base_expression.row,
                    col_start: base_expression.col_start,
                    col_end: base_expression.col_end,
                });
            }
            BaseExprData::ElseIfStatement {
                condition, body, ..
            } => {
                // Recursively merge if statements in the body
                let merged_body = match merge_if_statements(body) {
                    Ok(body) => body,
                    Err(e) => return Err(e),
                };

                match merged_statements.last_mut() {
                    Some(
                        upper_if_statement @ BaseExpr {
                            data: BaseExprData::IfStatement { .. },
                            ..
                        },
                    ) => {
                        match add_to_if_statement(
                            upper_if_statement,
                            BaseExpr {
                                data: BaseExprData::ElseIfStatement {
                                    condition,
                                    body: merged_body,
                                    else_statement: None,
                                },
                                row: base_expression.row,
                                col_start: base_expression.col_start,
                                col_end: base_expression.col_end,
                            },
                        ) {
                            Ok(_) => {}
                            Err(e) => return Err(e),
                        }
                    }
                    _ => {
                        return Err(Error::LocationError {
                            message: format!(
                                "Could not find if statement to add else-if statement to"
                            ),
                            row: base_expression.row,
                            col_start: base_expression.col_start,
                            col_end: base_expression.col_end,
                        });
                    }
                }
            }
            BaseExprData::ElseStatement { body } => {
                // Recursively merge if statements in the body
                let merged_body = match merge_if_statements(body) {
                    Ok(body) => body,
                    Err(e) => return Err(e),
                };

                match &mut merged_statements.last_mut() {
                    Some(
                        upper_if_statement @ BaseExpr {
                            data: BaseExprData::IfStatement { .. },
                            ..
                        },
                    ) => {
                        match add_to_if_statement(
                            upper_if_statement,
                            BaseExpr {
                                data: BaseExprData::ElseStatement { body: merged_body },
                                row: base_expression.row,
                                col_start: base_expression.col_start,
                                col_end: base_expression.col_end,
                            },
                        ) {
                            Ok(_) => {}
                            Err(e) => return Err(e),
                        }
                    }
                    _ => {
                        return Err(Error::LocationError {
                            message: format!(
                                "Could not find if statement to add else statement to"
                            ),
                            row: base_expression.row,
                            col_start: base_expression.col_start,
                            col_end: base_expression.col_end,
                        });
                    }
                }
            }
            BaseExprData::ForLoop {
                var_name,
                until,
                body,
            } => {
                // Recursively merge if statements in the body
                let merged_body = match merge_if_statements(body) {
                    Ok(body) => body,
                    Err(e) => return Err(e),
                };

                merged_statements.push(BaseExpr {
                    data: BaseExprData::ForLoop {
                        var_name: var_name,
                        until: until,
                        body: merged_body,
                    },
                    row: base_expression.row,
                    col_start: base_expression.col_start,
                    col_end: base_expression.col_end,
                });
            }
            BaseExprData::FunctionDefinition {
                fun_name,
                args,
                body,
            } => {
                // Recursively merge if statements in the body
                let merged_body = match merge_if_statements(body) {
                    Ok(body) => body,
                    Err(e) => return Err(e),
                };

                merged_statements.push(BaseExpr {
                    data: BaseExprData::FunctionDefinition {
                        fun_name: fun_name,
                        args: args,
                        body: merged_body,
                    },
                    row: base_expression.row,
                    col_start: base_expression.col_start,
                    col_end: base_expression.col_end,
                });
            }
            other => {
                merged_statements.push(BaseExpr {
                    data: other,
                    ..base_expression
                });
            }
        }
    }

    return Ok(merged_statements);
}

fn get_base_expressions(token_lines: &Vec<TokenLine>) -> Result<Vec<BaseExpr>, Error> {
    let mut line_iterator = token_lines.iter().peekable();

    return get_base_expressions_with_indentation(&mut line_iterator, 0);
}

fn get_base_expressions_with_indentation(
    token_lines_iter: &mut std::iter::Peekable<std::slice::Iter<'_, TokenLine>>,
    indentation: usize,
) -> Result<Vec<BaseExpr>, Error> {
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

fn get_base_expression(
    token_lines_iter: &mut std::iter::Peekable<std::slice::Iter<'_, TokenLine>>,
) -> Result<BaseExpr, Error> {
    let Some(token_line) = token_lines_iter.next() else {
        return Err(Error::SimpleError {
            message: format!("No more lines found"),
        });
    };

    let tokens = &token_line.tokens;
    let (row, col_start, col_end) = match &tokens[..] {
        [first, .., last] => (first.row, first.col_start, last.col_end),
        [only_one] => (only_one.row, only_one.col_start, only_one.col_end),
        _ => {
            return Err(Error::SimpleError {
                message: format!("No tokens found"),
            });
        }
    };

    let data: BaseExprData = match &tokens[..] {
        [Token {
            data: TokenData::Variable { name },
            ..
        }, Token {
            data: TokenData::Symbol {
                symbol_type: SymbolType::Equals,
            },
            ..
        }, rest @ ..] => {
            let expression = match get_expression(rest) {
                Ok(expression) => expression,
                Err(error_message) => return Err(error_message),
            };
            BaseExprData::VariableAssignment {
                var_name: name.clone(),
                expr: expression,
            }
        }
        [Token {
            data: TokenData::Variable { name },
            ..
        }, Token {
            data:
                TokenData::Symbol {
                    symbol_type: SymbolType::PlusEquals,
                },
            ..
        }, rest @ ..] => {
            let expression = match get_expression(rest) {
                Ok(expression) => expression,
                Err(error_message) => return Err(error_message),
            };
            BaseExprData::PlusEqualsStatement {
                var_name: name.clone(),
                expr: expression,
            }
        }
        [Token {
            data: TokenData::Symbol {
                symbol_type: SymbolType::If,
            },
            ..
        }, rest @ ..] => {
            let condition = match get_expression(rest) {
                Ok(expression) => expression,
                Err(error_message) => return Err(error_message),
            };

            let body = match get_base_expressions_with_indentation(
                token_lines_iter,
                token_line.indentation + 1,
            ) {
                Ok(body) => body,
                Err(e) => return Err(e),
            };

            BaseExprData::IfStatement {
                condition,
                body,
                else_statement: None,
            }
        }
        [Token {
            data: TokenData::Symbol {
                symbol_type: SymbolType::Else,
            },
            ..
        }, Token {
            data: TokenData::Symbol {
                symbol_type: SymbolType::If,
            },
            ..
        }, rest @ ..] => {
            let condition = match get_expression(rest) {
                Ok(expression) => expression,
                Err(error_message) => return Err(error_message),
            };

            let body = match get_base_expressions_with_indentation(
                token_lines_iter,
                token_line.indentation + 1,
            ) {
                Ok(body) => body,
                Err(e) => return Err(e),
            };

            BaseExprData::ElseIfStatement {
                condition,
                body,
                else_statement: None,
            }
        }
        [Token {
            data: TokenData::Symbol {
                symbol_type: SymbolType::Else,
            },
            ..
        }, rest @ ..] => {
            match rest {
                [first, .., last] => {
                    return Err(Error::LocationError {
                        message: format!("Unexpected extra tokens on else statement"),
                        row: first.row,
                        col_start: first.col_start,
                        col_end: last.col_end,
                    });
                }
                [only_one] => {
                    return Err(Error::LocationError {
                        message: format!("Unexpected extra tokens on else statement"),
                        row: only_one.row,
                        col_start: only_one.col_start,
                        col_end: only_one.col_end,
                    });
                }
                _ => {}
            }

            let body = match get_base_expressions_with_indentation(
                token_lines_iter,
                token_line.indentation + 1,
            ) {
                Ok(body) => body,
                Err(e) => return Err(e),
            };

            BaseExprData::ElseStatement { body }
        }
        [Token {
            data: TokenData::Symbol {
                symbol_type: SymbolType::Break,
            },
            ..
        }, rest @ ..] => {
            match rest {
                [first, .., last] => {
                    return Err(Error::LocationError {
                        message: format!("Unexpected extra tokens on else statement"),
                        row: first.row,
                        col_start: first.col_start,
                        col_end: last.col_end,
                    });
                }
                [only_one] => {
                    return Err(Error::LocationError {
                        message: format!("Unexpected extra tokens on else statement"),
                        row: only_one.row,
                        col_start: only_one.col_start,
                        col_end: only_one.col_end,
                    });
                }
                _ => {}
            }

            BaseExprData::Break
        }
        [Token {
            data: TokenData::Symbol {
                symbol_type: SymbolType::For,
            },
            ..
        }, Token {
            data: TokenData::Variable {
                name: variable_name,
            },
            ..
        }, Token {
            data: TokenData::Symbol {
                symbol_type: SymbolType::In,
            },
            ..
        }, rest @ ..] => {
            let range = match get_expression(rest) {
                Ok(expression) => expression,
                Err(error_message) => return Err(error_message),
            };

            let body = match get_base_expressions_with_indentation(
                token_lines_iter,
                token_line.indentation + 1,
            ) {
                Ok(body) => body,
                Err(e) => return Err(e),
            };

            BaseExprData::ForLoop {
                var_name: variable_name.clone(),
                until: range,
                body: body,
            }
        }
        [Token {
            data: TokenData::Symbol {
                symbol_type: SymbolType::Fun,
            },
            row,
            col_start,
            ..
        }, Token {
            data: TokenData::Variable {
                name: function_name,
            },
            ..
        }, Token {
            data:
                TokenData::Symbol {
                    symbol_type: SymbolType::ParenthesisOpen,
                },
            col_end,
            ..
        }, rest @ ..] => {
            let parameters = match parse_function_parameters(rest) {
                Ok(parameters) => parameters,
                Err(_) => match rest {
                    [.., last] => {
                        return Err(Error::LocationError {
                            message: format!("Invalid function parameters"),
                            row: *row,
                            col_start: *col_start,
                            col_end: last.col_end,
                        })
                    }
                    _ => {
                        return Err(Error::LocationError {
                            message: format!("Invalid function parameters"),
                            row: *row,
                            col_start: *col_start,
                            col_end: *col_end,
                        })
                    }
                },
            };

            let body = match get_base_expressions_with_indentation(
                token_lines_iter,
                token_line.indentation + 1,
            ) {
                Ok(body) => body,
                Err(e) => return Err(e),
            };

            BaseExprData::FunctionDefinition {
                fun_name: function_name.clone(),
                args: parameters,
                body: body,
            }
        }
        [Token {
            data: TokenData::Symbol {
                symbol_type: SymbolType::Return,
            },
            ..
        }, rest @ ..] => {
            if rest.len() == 0 {
                BaseExprData::Return { return_value: None }
            } else {
                let expression = match get_expression(rest) {
                    Ok(expression) => expression,
                    Err(error_message) => return Err(error_message),
                };
                BaseExprData::Return {
                    return_value: Some(expression),
                }
            }
        }
        rest @ _ => {
            let expression = match get_expression(rest) {
                Ok(expression) => expression,
                Err(error_message) => return Err(error_message),
            };
            BaseExprData::Simple { expr: expression }
        }
    };

    return Ok(BaseExpr {
        data,
        row,
        col_start,
        col_end,
    });
}

fn parse_function_parameters(tokens: &[Token]) -> Result<Vec<String>, Error> {
    match tokens {
        [Token {
            data: TokenData::Variable {
                name: parameter_name,
            },
            ..
        }, Token {
            data: TokenData::Symbol {
                symbol_type: SymbolType::Comma,
            },
            ..
        }, rest @ ..] => match parse_function_parameters(rest) {
            Ok(mut other_parameters) => {
                other_parameters.insert(0, parameter_name.clone());
                return Ok(other_parameters);
            }
            Err(e) => return Err(e),
        },

        [Token {
            data: TokenData::Variable {
                name: parameter_name,
            },
            ..
        }, Token {
            data:
                TokenData::Symbol {
                    symbol_type: SymbolType::ParenthesisClosed,
                },
            ..
        }] => {
            return Ok(vec![parameter_name.clone()]);
        }

        // Closing bracket
        [Token {
            data:
                TokenData::Symbol {
                    symbol_type: SymbolType::ParenthesisClosed,
                },
            ..
        }, rest @ ..] => {
            match rest {
                [first, .., last] => {
                    return Err(Error::LocationError {
                        message: format!("Unexpected extra tokens on else statement"),
                        row: first.row,
                        col_start: first.col_start,
                        col_end: last.col_end,
                    });
                }
                [only_one] => {
                    return Err(Error::LocationError {
                        message: format!("Unexpected extra tokens on else statement"),
                        row: only_one.row,
                        col_start: only_one.col_start,
                        col_end: only_one.col_end,
                    });
                }
                _ => {}
            }

            return Ok(Vec::new());
        }
        _ => {
            return Err(Error::SimpleError {
                message: format!("Invalid function parameter definition"),
            })
        }
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
    match &expression.data {
        BaseExprData::Simple { expr } => print_recursive_expression(expr),
        BaseExprData::VariableAssignment { var_name, expr } => {
            print!("VarAssign({var_name:?}, ");
            print_recursive_expression(expr);
            print!(")");
        }
        BaseExprData::PlusEqualsStatement { var_name, expr } => {
            print!("PlusEquals({var_name:?}, ");
            print_recursive_expression(expr);
            print!(")");
        }
        BaseExprData::IfStatement {
            condition, body, ..
        } => {
            print!("IfSt(");
            print_recursive_expression(condition);
            print!(")\n");
            for expr in body {
                print_expression(expr, indentation + 1);
            }
        }
        BaseExprData::ElseIfStatement {
            condition, body, ..
        } => {
            print!("ElseIfSt(");
            print_recursive_expression(condition);
            print!(")");
            for expr in body {
                print_expression(expr, indentation + 1);
            }
        }
        BaseExprData::ElseStatement { body } => {
            print!("ElseSt(");
            for expr in body {
                print_expression(expr, indentation + 1);
            }
            print!(")");
        }
        BaseExprData::ForLoop {
            var_name,
            until,
            body,
        } => {
            print!("For({var_name:?} in ");
            print_recursive_expression(until);
            print!("\n");
            for expr in body {
                print_expression(expr, indentation + 1);
            }
            print!(")");
        }
        BaseExprData::FunctionDefinition {
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
                print_expression(expr, indentation + 1);
            }
        }
        BaseExprData::Return { return_value } => {
            print!("Return(");
            match return_value {
                Some(expr) => print_recursive_expression(expr),
                None => print!(""),
            }
            print!(")")
        }
        BaseExprData::Break => print!("break"),
    }
}

fn print_recursive_expression(expression: &RecExpr) {
    match &expression.data {
        RecExprData::Variable { name } => print!("Var({name:?})"),
        RecExprData::Number { number } => print!("Num({number})"),
        RecExprData::String { value } => print!("Str({value:?})"),
        RecExprData::Boolean { value } => print!("Bool({value})"),
        RecExprData::Assign {
            variable_name,
            right,
        } => {
            print!("Var({variable_name:?}) = ");
            print_recursive_expression(&*right);
        }
        RecExprData::Add { left, right } => {
            print!("(");
            print_recursive_expression(&*left);
            print!(" + ");
            print_recursive_expression(&*right);
            print!(")")
        }
        RecExprData::Subtract { left, right } => {
            print!("(");
            print_recursive_expression(&*left);
            print!(" - ");
            print_recursive_expression(&*right);
            print!(")");
        }
        RecExprData::Multiply { left, right } => {
            print!("(");
            print_recursive_expression(&*left);
            print!(" * ");
            print_recursive_expression(&*right);
            print!(")");
        }
        RecExprData::Divide { left, right } => {
            print!("(");
            print_recursive_expression(&*left);
            print!(" / ");
            print_recursive_expression(&*right);
            print!(")");
        }
        RecExprData::Power { left, right } => {
            print!("(");
            print_recursive_expression(&*left);
            print!(" ^ ");
            print_recursive_expression(&*right);
            print!(")");
        }
        RecExprData::Minus { right } => {
            print!("(");
            print!("- ");
            print_recursive_expression(&*right);
            print!(")");
        }
        RecExprData::Or { left, right } => {
            print!("(");
            print_recursive_expression(&*left);
            print!(" or ");
            print_recursive_expression(&*right);
            print!(")");
        }
        RecExprData::And { left, right } => {
            print!("(");
            print_recursive_expression(&*left);
            print!(" and ");
            print_recursive_expression(&*right);
            print!(")");
        }
        RecExprData::Not { right } => {
            print!("(");
            print!("not ");
            print_recursive_expression(&*right);
            print!(")");
        }
        RecExprData::Equals { left, right } => {
            print!("(");
            print_recursive_expression(&*left);
            print!(" == ");
            print_recursive_expression(&*right);
            print!(")");
        }
        RecExprData::NotEquals { left, right } => {
            print!("(");
            print_recursive_expression(&*left);
            print!(" != ");
            print_recursive_expression(&*right);
            print!(")");
        }
        RecExprData::LessThan { left, right } => {
            print!("(");
            print_recursive_expression(&*left);
            print!(" < ");
            print_recursive_expression(&*right);
            print!(")");
        }
        RecExprData::LessThanOrEqual { left, right } => {
            print!("(");
            print_recursive_expression(&*left);
            print!(" <= ");
            print_recursive_expression(&*right);
            print!(")");
        }
        RecExprData::GreaterThan { left, right } => {
            print!("(");
            print_recursive_expression(&*left);
            print!(" > ");
            print_recursive_expression(&*right);
            print!(")");
        }
        RecExprData::GreaterThanOrEqual { left, right } => {
            print!("(");
            print_recursive_expression(&*left);
            print!(" >= ");
            print_recursive_expression(&*right);
            print!(")");
        }
        RecExprData::Access { object, variable } => {
            print!("{object:?}.{variable:?}");
        }
        RecExprData::FunctionCall {
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
        RecExprData::List { elements } => {
            print!("[");
            for element in elements {
                print_recursive_expression(element);
                print!(", ");
            }
            print!("]");
        }
        RecExprData::ListAccess { variable, index } => {
            print!("{variable:?}[");
            print_recursive_expression(index);
            print!("]");
        }
    }
}
