use crate::typechecker::Type;

#[derive(PartialEq, Debug)]
pub enum Error {
    LocationError {
        message: String,
        row: usize,
        col_start: usize,
        col_end: usize,
    },
    SimpleError {
        message: String,
    },
    TypeError {
        message: String,
        expected: Type,
        found: Type,
        row: usize,
        col_start: usize,
        col_end: usize,
    },
}

#[derive(PartialEq)]
enum CharType {
    Space,
    Symbol,
    Number,
    Variable,
}

fn get_char_type(c: char) -> CharType {
    match c {
        ' ' => CharType::Space,
        '0'..='9' => CharType::Number,
        _ if RESERVED_SYMBOLS.contains(&c) => CharType::Symbol,
        _ => CharType::Variable,
    }
}

#[derive(PartialEq, Clone, Debug)]
pub enum SymbolType {
    Equals,
    Plus,
    Minus,
    Star,
    Slash,
    Hat,
    Dot,
    Comma,
    ParenthesisOpen,
    ParenthesisClosed,
    SquareBracketOpen,
    SquareBracketClosed,
    EqualsEquals,
    NotEquals,
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
    Or,
    And,
    Not,
    For,
    In,
    If,
    Else,
    Fun,
    QuotationMark,
    Return,
    Break,
    PlusEquals,
    True,
    False,
}

#[derive(PartialEq, Clone, Debug)]
pub struct Token {
    pub data: TokenData,
    pub row: usize,
    pub col_start: usize,
    pub col_end: usize,
}

#[derive(PartialEq, Clone, Debug)]
pub enum TokenData {
    Variable { name: String },
    Symbol { symbol_type: SymbolType },
    Number { number: i32 },
    String { value: String },
}

#[derive(PartialEq, Debug)]
pub struct TokenLine {
    pub tokens: Vec<Token>,
    pub indentation: usize,
}

static RESERVED_SYMBOLS: [char; 16] = [
    '=', '+', '-', '*', '/', '^', '.', ',', '(', ')', '"', '<', '>', '!', '[', ']',
];
static BINARY_OPERATORS: [&str; 9] = ["+", "-", "*", "/", "^", ".", "==", "or", "and"];

fn get_symbol_type(symbol: &String) -> Result<SymbolType, Error> {
    match symbol {
        s if s == "=" => Ok(SymbolType::Equals),
        s if s == "-" => Ok(SymbolType::Minus),
        s if s == "+" => Ok(SymbolType::Plus),
        s if s == "*" => Ok(SymbolType::Star),
        s if s == "/" => Ok(SymbolType::Slash),
        s if s == "^" => Ok(SymbolType::Hat),
        s if s == "." => Ok(SymbolType::Dot),
        s if s == "," => Ok(SymbolType::Comma),
        s if s == "(" => Ok(SymbolType::ParenthesisOpen),
        s if s == ")" => Ok(SymbolType::ParenthesisClosed),
        s if s == "[" => Ok(SymbolType::SquareBracketOpen),
        s if s == "]" => Ok(SymbolType::SquareBracketClosed),
        s if s == "==" => Ok(SymbolType::EqualsEquals),
        s if s == "!=" => Ok(SymbolType::NotEquals),
        s if s == ">" => Ok(SymbolType::GreaterThan),
        s if s == ">=" => Ok(SymbolType::GreaterThanOrEqual),
        s if s == "<" => Ok(SymbolType::LessThan),
        s if s == "<=" => Ok(SymbolType::LessThanOrEqual),
        s if s == "or" => Ok(SymbolType::Or),
        s if s == "and" => Ok(SymbolType::And),
        s if s == "not" => Ok(SymbolType::Not),
        s if s == "for" => Ok(SymbolType::For),
        s if s == "in" => Ok(SymbolType::In),
        s if s == "if" => Ok(SymbolType::If),
        s if s == "else" => Ok(SymbolType::Else),
        s if s == "fun" => Ok(SymbolType::Fun),
        s if s == "\"" => Ok(SymbolType::QuotationMark),
        s if s == "return" => Ok(SymbolType::Return),
        s if s == "break" => Ok(SymbolType::Break),
        s if s == "+=" => Ok(SymbolType::PlusEquals),
        s if s == "true" => Ok(SymbolType::True),
        s if s == "false" => Ok(SymbolType::False),
        _ => Err(Error::SimpleError {
            message: format!("{} is not a Symbol", symbol),
        }),
    }
}

pub fn get_symbol_from_type(symbol_type: &SymbolType) -> String {
    match symbol_type {
        SymbolType::Equals => String::from("="),
        SymbolType::Minus => String::from("-"),
        SymbolType::Plus => String::from("+"),
        SymbolType::Star => String::from("*"),
        SymbolType::Slash => String::from("/"),
        SymbolType::Hat => String::from("^"),
        SymbolType::Dot => String::from("."),
        SymbolType::Comma => String::from(","),
        SymbolType::ParenthesisOpen => String::from("("),
        SymbolType::ParenthesisClosed => String::from(")"),
        SymbolType::SquareBracketOpen => String::from("["),
        SymbolType::SquareBracketClosed => String::from("]"),
        SymbolType::EqualsEquals => String::from("=="),
        SymbolType::NotEquals => String::from("!="),
        SymbolType::GreaterThan => String::from(">"),
        SymbolType::GreaterThanOrEqual => String::from(">="),
        SymbolType::LessThan => String::from("<"),
        SymbolType::LessThanOrEqual => String::from("<="),
        SymbolType::Or => String::from("or"),
        SymbolType::And => String::from("and"),
        SymbolType::Not => String::from("not"),
        SymbolType::For => String::from("for"),
        SymbolType::In => String::from("in"),
        SymbolType::If => String::from("if"),
        SymbolType::Else => String::from("else"),
        SymbolType::Fun => String::from("fun"),
        SymbolType::QuotationMark => String::from("\""),
        SymbolType::Return => String::from("return"),
        SymbolType::Break => String::from("break"),
        SymbolType::PlusEquals => String::from("+="),
        SymbolType::True => String::from("true"),
        SymbolType::False => String::from("false"),
    }
}

fn is_symbol(symbol: &String) -> bool {
    match get_symbol_type(symbol) {
        Ok(_) => true,
        _ => false,
    }
}

fn separate_symbols(
    symbol: &str,
    row_index: usize,
    start_column: usize,
) -> Result<Vec<Token>, Error> {
    let mut symbols: Vec<Token> = Vec::new();

    if symbol.len() == 0 {
        return Ok(symbols);
    }

    for i in (1..=symbol.len()).rev() {
        let left_side = String::from(&symbol[0..i]);

        match get_symbol_type(&left_side) {
            Ok(symbol_type) => {
                symbols.push(Token {
                    data: TokenData::Symbol { symbol_type },
                    row: row_index,
                    col_start: start_column,
                    col_end: start_column + i,
                });
                match separate_symbols(&symbol[i..], row_index, start_column + i) {
                    Ok(mut rest_symbols) => {
                        symbols.append(&mut rest_symbols);
                        return Ok(symbols);
                    }
                    Err(_) => continue,
                }
            }
            Err(_) => continue,
        }
    }

    return Err(Error::SimpleError {
        message: format!("No symbol combination found for {}", symbol),
    });
}

fn count_indentation(line: &String, line_index: usize) -> Result<usize, Error> {
    let indentation_spaces = 4;
    let mut indentation = 0;
    if line.len() == 0 {
        return Ok(0);
    }

    // Check for tab indentation
    if line.chars().nth(0) == Some('\t') {
        for c in line.chars() {
            if c == '\t' {
                indentation += 1;
            } else {
                return Ok(indentation);
            }
        }
    }

    // Use space indentation
    let mut leading_spaces: usize = 0;
    for c in line.chars() {
        if c == ' ' {
            leading_spaces += 1;
        } else {
            break;
        }
    }

    if leading_spaces % indentation_spaces == 0 {
        return Ok(leading_spaces / indentation_spaces);
    }

    return Err(Error::LocationError {
        message: format!("Invalid indentation"),
        row: line_index,
        col_start: 0,
        col_end: leading_spaces,
    });
}

pub fn tokenize(lines: Vec<&str>) -> Result<Vec<TokenLine>, Error> {
    let mut cleaned_lines: Vec<String> = Vec::new();
    let mut line_indices: Vec<usize> = Vec::new();

    for (line_index, line) in lines.iter().enumerate() {
        let mut line_cleaned = line.replace("\r", "");
        // Removing empty lines
        if line_cleaned.replace(" ", "").replace("\t", "").len() == 0 {
            continue;
        }
        line_cleaned = line_cleaned.replace("\t", "    ");
        cleaned_lines.push(line_cleaned);
        line_indices.push(line_index);
    }

    let mut token_lines: Vec<TokenLine> = Vec::new();

    for (line_index, line) in line_indices.iter().zip(cleaned_lines.iter()) {
        let indentation = match count_indentation(&line, *line_index) {
            Ok(indentation) => indentation,
            Err(error_message) => return Err(error_message),
        };

        let mut token_line: TokenLine = TokenLine {
            tokens: Vec::new(),
            indentation: indentation,
        };

        let mut in_number = false;
        let mut current_number = 0;
        let mut in_string = false;
        let mut current_string = String::new();
        let mut in_variable = false;
        let mut current_variable = String::new();
        let mut in_symbol = false;
        let mut current_symbol = String::new();
        let mut current_token_start = 0;

        for (current_column, c) in line.chars().enumerate() {
            let char_type: CharType = get_char_type(c);

            if in_string {
                match get_symbol_type(&String::from(c)) {
                    // Found the second quotation mark
                    Ok(SymbolType::QuotationMark) => {
                        token_line.tokens.push(Token {
                            data: TokenData::String {
                                value: current_string.clone(),
                            },
                            row: *line_index,
                            col_start: current_token_start,
                            col_end: current_column + 1,
                        });

                        in_string = false;
                        current_string = String::new();
                    }
                    _ => {
                        current_string.push(c);
                    }
                }

                continue;
            }

            // If we move out of a number
            if in_number && char_type != CharType::Number {
                token_line.tokens.push(Token {
                    data: TokenData::Number {
                        number: current_number,
                    },
                    row: *line_index,
                    col_start: current_token_start,
                    col_end: current_column,
                });
                current_number = 0;
                in_number = false;
            }

            // If we move out of a variable
            if in_variable && char_type != CharType::Variable && char_type != CharType::Number {
                // The string might be a symbol so we check for that
                match get_symbol_type(&current_variable) {
                    // String was a symbol
                    Ok(symbol_type) => token_line.tokens.push(Token {
                        data: TokenData::Symbol { symbol_type },
                        row: *line_index,
                        col_start: current_token_start,
                        col_end: current_column,
                    }),

                    // String was just a variable
                    Err(_) => token_line.tokens.push(Token {
                        data: TokenData::Variable {
                            name: current_variable.clone(),
                        },
                        row: *line_index,
                        col_start: current_token_start,
                        col_end: current_column,
                    }),
                }
                current_variable = String::new();
                in_variable = false;
            }

            // If we move out of a symbol
            if in_symbol && char_type != CharType::Symbol {
                match get_symbol_type(&current_symbol) {
                    Ok(symbol_type) => token_line.tokens.push(Token {
                        data: TokenData::Symbol { symbol_type },
                        row: *line_index,
                        col_start: current_token_start,
                        col_end: current_column,
                    }),
                    Err(_) => {
                        match separate_symbols(&current_symbol, *line_index, current_token_start) {
                            Ok(symbols_separated) => {
                                for symbol in symbols_separated {
                                    token_line.tokens.push(symbol);
                                }
                            }
                            Err(_) => {
                                return Err(Error::LocationError {
                                    message: format!("Invalid symbol: {}", current_symbol),
                                    row: *line_index,
                                    col_start: current_token_start,
                                    col_end: current_column,
                                });
                            }
                        }
                    }
                }
                current_symbol = String::new();
                in_symbol = false;
            }

            match char_type {
                // Spaces are just skipped, but kept initially to distinguish between e.g. 'if k' and 'ifk'
                CharType::Space => {}

                CharType::Symbol => {
                    match get_symbol_type(&String::from(c)) {
                        Ok(SymbolType::QuotationMark) => {
                            // Save current symbol
                            if in_symbol {
                                match get_symbol_type(&current_symbol) {
                                    Ok(symbol_type) => token_line.tokens.push(Token {
                                        data: TokenData::Symbol { symbol_type },
                                        row: *line_index,
                                        col_start: current_token_start,
                                        col_end: current_column,
                                    }),
                                    Err(_) => {
                                        return Err(Error::LocationError {
                                            message: format!("Invalid symbol: {}", current_symbol),
                                            row: *line_index,
                                            col_start: current_token_start,
                                            col_end: current_column,
                                        });
                                    }
                                }
                            }

                            in_symbol = false;
                            in_string = true;
                            current_symbol = String::new();
                            current_token_start = current_column;
                        }
                        _ => {
                            // If we just entered a symbol, keep track of the location
                            if !in_symbol {
                                current_token_start = current_column;
                            }

                            in_symbol = true;
                            current_symbol.push(c);
                        }
                    }
                }

                CharType::Number => {
                    // It is possible to add numbers to variables, but variables may not start with a number
                    if in_variable {
                        current_variable.push(c);
                        continue;
                    }

                    // If we just entered a number, keep track of the location
                    if !in_number {
                        current_token_start = current_column;
                    }

                    in_number = true;
                    if let Some(number) = c.to_digit(10) {
                        current_number = current_number * 10 + number as i32;
                    }
                }

                CharType::Variable => {
                    // If we just entered a variable, keep track of the location
                    if !in_variable {
                        current_token_start = current_column;
                    }

                    in_variable = true;
                    current_variable.push(c);
                }
            }
        }

        // If we are still in a number at the end
        if in_number {
            token_line.tokens.push(Token {
                data: TokenData::Number {
                    number: current_number,
                },
                row: *line_index,
                col_start: current_token_start,
                col_end: line.len(),
            });
        }

        // If we are still in a variable at the end
        if in_variable {
            // The string might be a symbol so we check for that
            match get_symbol_type(&current_variable) {
                // String was a symbol
                Ok(symbol_type) => token_line.tokens.push(Token {
                    data: TokenData::Symbol { symbol_type },
                    row: *line_index,
                    col_start: current_token_start,
                    col_end: line.len(),
                }),

                // String was just a variable
                Err(_) => token_line.tokens.push(Token {
                    data: TokenData::Variable {
                        name: current_variable.clone(),
                    },
                    row: *line_index,
                    col_start: current_token_start,
                    col_end: line.len(),
                }),
            }
        }

        // If we are still in a symbol at the end
        if in_symbol {
            match get_symbol_type(&current_symbol) {
                Ok(symbol_type) => token_line.tokens.push(Token {
                    data: TokenData::Symbol { symbol_type },
                    row: *line_index,
                    col_start: current_token_start,
                    col_end: line.len(),
                }),
                Err(_) => {
                    match separate_symbols(&current_symbol, *line_index, current_token_start) {
                        Ok(symbols_separated) => {
                            for symbol in symbols_separated {
                                token_line.tokens.push(symbol);
                            }
                        }
                        Err(_) => {
                            return Err(Error::LocationError {
                                message: format!("Invalid symbol: {}", current_symbol),
                                row: *line_index,
                                col_start: current_token_start,
                                col_end: line.len(),
                            });
                        }
                    }
                }
            }
        }

        token_lines.push(token_line);
    }

    return Ok(token_lines);
}

pub fn print_token_lines(token_lines: &Vec<TokenLine>) {
    for token_line in token_lines {
        print_tokens(token_line);
        print!("\n")
    }
}

pub fn print_tokens(token_line: &TokenLine) {
    for token in &token_line.tokens {
        print_token(token);
        print!(" ")
    }
    print!("\n")
}

pub fn print_token(token: &Token) {
    match &token.data {
        TokenData::Variable { name } => print!("Var({name:?})"),
        TokenData::Number { number } => print!("Num({number})"),
        TokenData::String { value } => print!("Str({value:?})"),
        TokenData::Symbol { symbol_type } => print!("Sym{}", get_symbol_from_type(symbol_type)),
    }
}
