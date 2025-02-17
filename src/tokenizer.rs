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

#[derive(PartialEq, Clone)]
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
    EqualsEquals,
    Or,
    And,
    For,
    In,
    If,
    Else,
    Fun,
    QuotationMark,
    Return,
    Break,
    PlusEquals,
}

#[derive(PartialEq, Clone)]
pub enum Token {
    Variable { name: String },
    Symbol { symbol_type: SymbolType },
    Number { number: i32 },
    String { value: String },
}

pub struct TokenLine {
    pub tokens: Vec<Token>,
    pub indentation: i32,
}

static RESERVED_SYMBOLS: [char; 11] = ['=', '+', '-', '*', '/', '^', '.', ',', '(', ')', '"'];
static BINARY_OPERATORS: [&str; 9] = ["+", "-", "*", "/", "^", ".", "==", "or", "and"];

fn get_symbol_type(symbol: &String) -> Result<SymbolType, String> {
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
        s if s == "==" => Ok(SymbolType::EqualsEquals),
        s if s == "or" => Ok(SymbolType::Or),
        s if s == "and" => Ok(SymbolType::And),
        s if s == "for" => Ok(SymbolType::For),
        s if s == "in" => Ok(SymbolType::In),
        s if s == "if" => Ok(SymbolType::If),
        s if s == "else" => Ok(SymbolType::Else),
        s if s == "fun" => Ok(SymbolType::Fun),
        s if s == "\"" => Ok(SymbolType::QuotationMark),
        s if s == "return" => Ok(SymbolType::Return),
        s if s == "break" => Ok(SymbolType::Break),
        s if s == "+=" => Ok(SymbolType::PlusEquals),
        _ => Err(String::from("String is not a Symbol")),
    }
}

fn get_symbol_from_type(symbol_type: &SymbolType) -> String {
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
        SymbolType::EqualsEquals => String::from("=="),
        SymbolType::Or => String::from("or"),
        SymbolType::And => String::from("and"),
        SymbolType::For => String::from("for"),
        SymbolType::In => String::from("in"),
        SymbolType::If => String::from("if"),
        SymbolType::Else => String::from("else"),
        SymbolType::Fun => String::from("fun"),
        SymbolType::QuotationMark => String::from("\""),
        SymbolType::Return => String::from("return"),
        SymbolType::Break => String::from("break"),
        SymbolType::PlusEquals => String::from("+="),
    }
}

fn is_symbol(symbol: &String) -> bool {
    match get_symbol_type(symbol) {
        Ok(_) => true,
        _ => false,
    }
}

fn separate_symbols(symbol: &str) -> Result<Vec<Token>, String> {
    let mut symbols: Vec<Token> = Vec::new();

    if symbol.len() == 0 {
        return Ok(symbols);
    }

    println!("called with {}", symbol);

    for i in (1..=symbol.len()).rev() {
        let left_side = String::from(&symbol[0..i]);

        println!("let side: {}, right side: {}", left_side, &symbol[i..]);
        match get_symbol_type(&left_side) {
            Ok(symbol_type) => {
                symbols.push(Token::Symbol { symbol_type });
                match separate_symbols(&symbol[i..]) {
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

    return Err(String::from("No symbol combination found"));
}

fn count_indentation(line: &String) -> Result<i32, String> {
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
    let mut leading_spaces = 0;
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

    return Err(String::from("Invalid indentation"));
}

pub fn tokenize(lines: Vec<&str>) -> Result<Vec<TokenLine>, String> {
    let mut cleaned_lines: Vec<String> = Vec::new();

    for line in lines {
        let mut line_cleaned = line.replace("\r", "");
        // Removing empty lines
        if line_cleaned.replace(" ", "").len() == 0 {
            continue;
        }
        line_cleaned = line_cleaned.replace("\t", "");
        println!("cleaned line: {line_cleaned:?}");
        cleaned_lines.push(line_cleaned);
    }

    let mut token_lines: Vec<TokenLine> = Vec::new();

    for line in cleaned_lines {
        let indentation = match count_indentation(&line) {
            Ok(indentation) => indentation,
            Err(error_message) => return Err(error_message),
        };

        let mut token_line: TokenLine = TokenLine { tokens: Vec::new(), indentation: indentation };

        let mut in_number = false;
        let mut current_number = 0;
        let mut in_string = false;
        let mut current_string = String::new();
        let mut in_variable = false;
        let mut current_variable = String::new();
        let mut in_symbol = false;
        let mut current_symbol = String::new();

        for c in line.chars() {
            let char_type: CharType = get_char_type(c);

            if in_string {
                match get_symbol_type(&String::from(c)) {
                    // Found the second quotation mark
                    Ok(SymbolType::QuotationMark) => {
                        token_line.tokens.push(Token::String {
                            value: current_string,
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
                token_line.tokens.push(Token::Number {
                    number: current_number,
                });
                current_number = 0;
                in_number = false;
            }

            // If we move out of a variable
            if in_variable && char_type != CharType::Variable {
                // The string might be a symbol so we check for that
                match get_symbol_type(&current_variable) {
                    // String was a symbol
                    Ok(symbol_type) => token_line.tokens.push(Token::Symbol { symbol_type }),

                    // String was just a variable
                    Err(_) => token_line.tokens.push(Token::Variable {
                        name: current_variable,
                    }),
                }
                current_variable = String::new();
                in_variable = false;
            }

            // If we move out of a symbol
            if in_symbol && char_type != CharType::Symbol {
                match get_symbol_type(&current_symbol) {
                    Ok(symbol_type) => token_line.tokens.push(Token::Symbol { symbol_type }),
                    Err(_) => match separate_symbols(&current_symbol) {
                        Ok(symbols_separated) => {
                            for symbol in symbols_separated {
                                token_line.tokens.push(symbol);
                            }
                        }
                        Err(_) => {
                            return Err(String::from("Found an invalid symbol: ") + &current_symbol)
                        }
                    },
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
                                    Ok(symbol_type) => {
                                        token_line.tokens.push(Token::Symbol { symbol_type })
                                    }
                                    Err(_) => {
                                        return Err(String::from("Found an invalid symbol: ")
                                            + &current_symbol)
                                    }
                                }
                            }

                            in_symbol = false;
                            in_string = true;
                        }
                        _ => {
                            in_symbol = true;
                            current_symbol.push(c);
                        }
                    }
                }

                CharType::Number => {
                    in_number = true;
                    if let Some(number) = c.to_digit(10) {
                        current_number = current_number * 10 + number as i32;
                    }
                }

                CharType::Variable => {
                    in_variable = true;
                    current_variable.push(c);
                }
            }
        }

        // If we are still in a number at the end
        if in_number {
            token_line.tokens.push(Token::Number {
                number: current_number,
            });
        }

        // If we are still in a variable at the end
        if in_variable {
            // The string might be a symbol so we check for that
            match get_symbol_type(&current_variable) {
                // String was a symbol
                Ok(symbol_type) => token_line.tokens.push(Token::Symbol { symbol_type }),

                // String was just a variable
                Err(_) => token_line.tokens.push(Token::Variable {
                    name: current_variable,
                }),
            }
        }

        // If we are still in a symbol at the end
        if in_symbol {
            match get_symbol_type(&current_symbol) {
                Ok(symbol_type) => token_line.tokens.push(Token::Symbol { symbol_type }),
                Err(_) => match separate_symbols(&current_symbol) {
                    Ok(symbols_separated) => {
                        for symbol in symbols_separated {
                            token_line.tokens.push(symbol);
                        }
                    }
                    Err(_) => {
                        return Err(String::from("Found an invalid symbol: ") + &current_symbol)
                    }
                },
            }
        }

        token_lines.push(token_line);
    }

    print_token_lines(&token_lines);
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
    match token {
        Token::Variable { name } => print!("Var({name:?})"),
        Token::Number { number } => print!("Num({number})"),
        Token::String { value } => print!("Str({value:?})"),
        Token::Symbol { symbol_type } => print!("Sym{}", get_symbol_from_type(symbol_type)),
    }
}