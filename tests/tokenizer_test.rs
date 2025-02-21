use rosy::tokenizer;
use tokenizer::Error;
use tokenizer::SymbolType;
use tokenizer::Token;
use tokenizer::TokenData;
use tokenizer::TokenLine;

fn compare(actual: Result<Vec<TokenLine>, Error>, expected: Vec<TokenLine>) {
    match actual {
        Ok(tokens) => assert_eq!(tokens, expected),
        Err(_) => panic!("error"),
    }
}

fn compare_linewise(actual: Result<Vec<TokenLine>, Error>, expected: Vec<TokenLine>) {
    match actual {
        Ok(tokens) => {
            if tokens.len() != expected.len() {
                panic!(
                    "Expected and actual have differing lengths ({} and {})",
                    expected.len(),
                    tokens.len()
                );
            }

            let it = tokens.iter().zip(expected.iter());

            for (_, (act, exp)) in it.enumerate() {
                assert_eq!(act, exp);
            }
        }
        Err(_) => panic!("error"),
    }
}

#[test]
fn empty_lines_test() {
    #[rustfmt::skip]
    let program = Vec::from([
        "",
        " ",
        "\t",
        "\t\t",
        "    ",
        "        ",
    ]);
    let tokens = tokenizer::tokenize(program);

    let expected = Vec::new();

    compare_linewise(tokens, expected);
}

#[test]
fn variable_test() {
    #[rustfmt::skip]
    let program = Vec::from([
        "a",
        "multiple_words",
        "CapITalS",
        "a1",
        "a1b34nh_4",
    ]);
    let tokens = tokenizer::tokenize(program);

    let expected = Vec::from([
        TokenLine {
            tokens: Vec::from([Token {
                data: TokenData::Variable {
                    name: String::from("a"),
                },
                row: 0,
                col_start: 0,
                col_end: 1,
            }]),
            indentation: 0,
        },
        TokenLine {
            tokens: Vec::from([Token {
                data: TokenData::Variable {
                    name: String::from("multiple_words"),
                },
                row: 1,
                col_start: 0,
                col_end: 14,
            }]),
            indentation: 0,
        },
        TokenLine {
            tokens: Vec::from([Token {
                data: TokenData::Variable {
                    name: String::from("CapITalS"),
                },
                row: 2,
                col_start: 0,
                col_end: 8,
            }]),
            indentation: 0,
        },
        TokenLine {
            tokens: Vec::from([Token {
                data: TokenData::Variable {
                    name: String::from("a1"),
                },
                row: 3,
                col_start: 0,
                col_end: 2,
            }]),
            indentation: 0,
        },
        TokenLine {
            tokens: Vec::from([Token {
                data: TokenData::Variable {
                    name: String::from("a1b34nh_4"),
                },
                row: 4,
                col_start: 0,
                col_end: 9,
            }]),
            indentation: 0,
        },
    ]);

    compare_linewise(tokens, expected);
}

#[test]
fn arithmetic_test() {
    #[rustfmt::skip]
    let program = Vec::from([
        "2 + 3",
        "2 + 3 * 4",
        "23 + 28 / 15 * 7 - 13 ^ 2",
        "23+28/15*7-13^2",
        "var = 15",
        "var2 = (15 + 16 / 4) ^2 * 4 / 2",
    ]);
    let tokens = tokenizer::tokenize(program);

    let expected = Vec::from([
        TokenLine {
            #[rustfmt::skip]
            tokens: Vec::from([
                Token { data: TokenData::Number { number: 2 }, row: 0, col_start: 0, col_end: 1 },
                Token { data: TokenData::Symbol { symbol_type: SymbolType::Plus }, row: 0, col_start: 2, col_end: 3 },
                Token { data: TokenData::Number { number: 3 }, row: 0, col_start: 4, col_end: 5 },
            ]),
            indentation: 0,
        },
        TokenLine {
            #[rustfmt::skip]
            tokens: Vec::from([
                Token { data: TokenData::Number { number: 2 }, row: 1, col_start: 0, col_end: 1 },
                Token { data: TokenData::Symbol { symbol_type: SymbolType::Plus }, row: 1, col_start: 2, col_end: 3 },
                Token { data: TokenData::Number { number: 3 }, row: 1, col_start: 4, col_end: 5 },
                Token { data: TokenData::Symbol { symbol_type: SymbolType::Star }, row: 1, col_start: 6, col_end: 7 },
                Token { data: TokenData::Number { number: 4 }, row: 1, col_start: 8, col_end: 9 },
            ]),
            indentation: 0,
        },
        TokenLine {
            #[rustfmt::skip]
            tokens: Vec::from([
                Token { data: TokenData::Number { number: 23 }, row: 2, col_start: 0, col_end: 2 },
                Token { data: TokenData::Symbol { symbol_type: SymbolType::Plus }, row: 2, col_start: 3, col_end: 4 },
                Token { data: TokenData::Number { number: 28 }, row: 2, col_start: 5, col_end: 7 },
                Token { data: TokenData::Symbol { symbol_type: SymbolType::Slash }, row: 2, col_start: 8, col_end: 9 },
                Token { data: TokenData::Number { number: 15 }, row: 2, col_start: 10, col_end: 12 },
                Token { data: TokenData::Symbol { symbol_type: SymbolType::Star }, row: 2, col_start: 13, col_end: 14 },
                Token { data: TokenData::Number { number: 7 }, row: 2, col_start: 15, col_end: 16 },
                Token { data: TokenData::Symbol { symbol_type: SymbolType::Minus }, row: 2, col_start: 17, col_end: 18 },
                Token { data: TokenData::Number { number: 13 }, row: 2, col_start: 19, col_end: 21 },
                Token { data: TokenData::Symbol { symbol_type: SymbolType::Hat }, row: 2, col_start: 22, col_end: 23 },
                Token { data: TokenData::Number { number: 2 }, row: 2, col_start: 24, col_end: 25 },
            ]),
            indentation: 0,
        },
        TokenLine {
            #[rustfmt::skip]
            tokens: Vec::from([
                Token { data: TokenData::Number { number: 23 }, row: 3, col_start: 0, col_end: 2 },
                Token { data: TokenData::Symbol { symbol_type: SymbolType::Plus }, row: 3, col_start: 2, col_end: 3 },
                Token { data: TokenData::Number { number: 28 }, row: 3, col_start: 3, col_end: 5 },
                Token { data: TokenData::Symbol { symbol_type: SymbolType::Slash }, row: 3, col_start: 5, col_end: 6 },
                Token { data: TokenData::Number { number: 15 }, row: 3, col_start: 6, col_end: 8 },
                Token { data: TokenData::Symbol { symbol_type: SymbolType::Star }, row: 3, col_start: 8, col_end: 9 },
                Token { data: TokenData::Number { number: 7 }, row: 3, col_start: 9, col_end: 10 },
                Token { data: TokenData::Symbol { symbol_type: SymbolType::Minus }, row: 3, col_start: 10, col_end: 11 },
                Token { data: TokenData::Number { number: 13 }, row: 3, col_start: 11, col_end: 13 },
                Token { data: TokenData::Symbol { symbol_type: SymbolType::Hat }, row: 3, col_start: 13, col_end: 14 },
                Token { data: TokenData::Number { number: 2 }, row: 3, col_start: 14, col_end: 15 },
            ]),
            indentation: 0,
        },
        TokenLine {
            #[rustfmt::skip]
            tokens: Vec::from([
                Token { data: TokenData::Variable { name: String::from("var") }, row: 4, col_start: 0, col_end: 3 },
                Token { data: TokenData::Symbol { symbol_type: SymbolType::Equals }, row: 4, col_start: 4, col_end: 5 },
                Token { data: TokenData::Number { number: 15 }, row: 4, col_start: 6, col_end: 8 },
            ]),
            indentation: 0,
        },
        TokenLine {
            #[rustfmt::skip]
            tokens: Vec::from([
                Token { data: TokenData::Variable { name: String::from("var2") }, row: 5, col_start: 0, col_end: 4 },
                Token { data: TokenData::Symbol { symbol_type: SymbolType::Equals }, row: 5, col_start: 5, col_end: 6 },
                Token { data: TokenData::Symbol { symbol_type: SymbolType::ParenthesisOpen }, row: 5, col_start: 7, col_end: 8 },
                Token { data: TokenData::Number { number: 15 }, row: 5, col_start: 8, col_end: 10 },
                Token { data: TokenData::Symbol { symbol_type: SymbolType::Plus }, row: 5, col_start: 11, col_end: 12 },
                Token { data: TokenData::Number { number: 16 }, row: 5, col_start: 13, col_end: 15 },
                Token { data: TokenData::Symbol { symbol_type: SymbolType::Slash }, row: 5, col_start: 16, col_end: 17 },
                Token { data: TokenData::Number { number: 4 }, row: 5, col_start: 18, col_end: 19 },
                Token { data: TokenData::Symbol { symbol_type: SymbolType::ParenthesisClosed }, row: 5, col_start: 19, col_end: 20 },
                Token { data: TokenData::Symbol { symbol_type: SymbolType::Hat }, row: 5, col_start: 21, col_end: 22 },
                Token { data: TokenData::Number { number: 2 }, row: 5, col_start: 22, col_end: 23 },
                Token { data: TokenData::Symbol { symbol_type: SymbolType::Star }, row: 5, col_start: 24, col_end: 25 },
                Token { data: TokenData::Number { number: 4 }, row: 5, col_start: 26, col_end: 27 },
                Token { data: TokenData::Symbol { symbol_type: SymbolType::Slash }, row: 5, col_start: 28, col_end: 29 },
                Token { data: TokenData::Number { number: 2 }, row: 5, col_start: 30, col_end: 31 },
            ]),
            indentation: 0,
        },
    ]);

    compare_linewise(tokens, expected);
}

#[test]
fn boolean_test() {
    #[rustfmt::skip]
    let program = Vec::from([
        "true",
        "false",
    ]);
    let tokens = tokenizer::tokenize(program);

    let expected = Vec::from([
        TokenLine {
            tokens: Vec::from([Token {
                data: TokenData::Symbol {
                    symbol_type: SymbolType::True,
                },
                row: 0,
                col_start: 0,
                col_end: 4,
            }]),
            indentation: 0,
        },
        TokenLine {
            tokens: Vec::from([Token {
                data: TokenData::Symbol {
                    symbol_type: SymbolType::False,
                },
                row: 1,
                col_start: 0,
                col_end: 5,
            }]),
            indentation: 0,
        },
    ]);

    compare_linewise(tokens, expected);
}

#[test]
fn if_statement_test() {
    // If statement syntax:
    // if <condition>
    //     <statements>

    #[rustfmt::skip]
    let program = Vec::from([
        "if 2 + 3",
        "    var = 15",
        "if 2 + 3 * 4",
        "    var = 15",
        "    var2 = 15",
        "if 2 + 3 * 4",
        "    var = 15",
        "    var2 = 15",
        "    var3 = 15",
    ]);
    let tokens = tokenizer::tokenize(program);

    let expected = Vec::from([
        TokenLine {
            tokens: Vec::from([
                Token {
                    data: TokenData::Symbol {
                        symbol_type: SymbolType::If,
                    },
                    row: 0,
                    col_start: 0,
                    col_end: 2,
                },
                Token {
                    data: TokenData::Number { number: 2 },
                    row: 0,
                    col_start: 3,
                    col_end: 4,
                },
                Token {
                    data: TokenData::Symbol {
                        symbol_type: SymbolType::Plus,
                    },
                    row: 0,
                    col_start: 5,
                    col_end: 6,
                },
                Token {
                    data: TokenData::Number { number: 3 },
                    row: 0,
                    col_start: 7,
                    col_end: 8,
                },
            ]),
            indentation: 0,
        },
        TokenLine {
            tokens: Vec::from([
                Token {
                    data: TokenData::Variable {
                        name: String::from("var"),
                    },
                    row: 1,
                    col_start: 4,
                    col_end: 7,
                },
                Token {
                    data: TokenData::Symbol {
                        symbol_type: SymbolType::Equals,
                    },
                    row: 1,
                    col_start: 8,
                    col_end: 9,
                },
                Token {
                    data: TokenData::Number { number: 15 },
                    row: 1,
                    col_start: 10,
                    col_end: 12,
                },
            ]),
            indentation: 1,
        },
        TokenLine {
            tokens: Vec::from([
                Token {
                    data: TokenData::Symbol {
                        symbol_type: SymbolType::If,
                    },
                    row: 2,
                    col_start: 0,
                    col_end: 2,
                },
                Token {
                    data: TokenData::Number { number: 2 },
                    row: 2,
                    col_start: 3,
                    col_end: 4,
                },
                Token {
                    data: TokenData::Symbol {
                        symbol_type: SymbolType::Plus,
                    },
                    row: 2,
                    col_start: 5,
                    col_end: 6,
                },
                Token {
                    data: TokenData::Number { number: 3 },
                    row: 2,
                    col_start: 7,
                    col_end: 8,
                },
                Token {
                    data: TokenData::Symbol {
                        symbol_type: SymbolType::Star,
                    },
                    row: 2,
                    col_start: 9,
                    col_end: 10,
                },
                Token {
                    data: TokenData::Number { number: 4 },
                    row: 2,
                    col_start: 11,
                    col_end: 12,
                },
            ]),
            indentation: 0,
        },
        TokenLine {
            tokens: Vec::from([
                Token {
                    data: TokenData::Variable {
                        name: String::from("var"),
                    },
                    row: 3,
                    col_start: 4,
                    col_end: 7,
                },
                Token {
                    data: TokenData::Symbol {
                        symbol_type: SymbolType::Equals,
                    },
                    row: 3,
                    col_start: 8,
                    col_end: 9,
                },
                Token {
                    data: TokenData::Number { number: 15 },
                    row: 3,
                    col_start: 10,
                    col_end: 12,
                },
            ]),
            indentation: 1,
        },
        TokenLine {
            tokens: Vec::from([
                Token {
                    data: TokenData::Variable {
                        name: String::from("var2"),
                    },
                    row: 4,
                    col_start: 4,
                    col_end: 8,
                },
                Token {
                    data: TokenData::Symbol {
                        symbol_type: SymbolType::Equals,
                    },
                    row: 4,
                    col_start: 9,
                    col_end: 10,
                },
                Token {
                    data: TokenData::Number { number: 15 },
                    row: 4,
                    col_start: 11,
                    col_end: 13,
                },
            ]),
            indentation: 1,
        },
        TokenLine {
            tokens: Vec::from([
                Token {
                    data: TokenData::Symbol {
                        symbol_type: SymbolType::If,
                    },
                    row: 5,
                    col_start: 0,
                    col_end: 2,
                },
                Token {
                    data: TokenData::Number { number: 2 },
                    row: 5,
                    col_start: 3,
                    col_end: 4,
                },
                Token {
                    data: TokenData::Symbol {
                        symbol_type: SymbolType::Plus,
                    },
                    row: 5,
                    col_start: 5,
                    col_end: 6,
                },
                Token {
                    data: TokenData::Number { number: 3 },
                    row: 5,
                    col_start: 7,
                    col_end: 8,
                },
                Token {
                    data: TokenData::Symbol {
                        symbol_type: SymbolType::Star,
                    },
                    row: 5,
                    col_start: 9,
                    col_end: 10,
                },
                Token {
                    data: TokenData::Number { number: 4 },
                    row: 5,
                    col_start: 11,
                    col_end: 12,
                },
            ]),
            indentation: 0,
        },
        TokenLine {
            tokens: Vec::from([
                Token {
                    data: TokenData::Variable {
                        name: String::from("var"),
                    },
                    row: 6,
                    col_start: 4,
                    col_end: 7,
                },
                Token {
                    data: TokenData::Symbol {
                        symbol_type: SymbolType::Equals,
                    },
                    row: 6,
                    col_start: 8,
                    col_end: 9,
                },
                Token {
                    data: TokenData::Number { number: 15 },
                    row: 6,
                    col_start: 10,
                    col_end: 12,
                },
            ]),
            indentation: 1,
        },
        TokenLine {
            tokens: Vec::from([
                Token {
                    data: TokenData::Variable {
                        name: String::from("var2"),
                    },
                    row: 7,
                    col_start: 4,
                    col_end: 8,
                },
                Token {
                    data: TokenData::Symbol {
                        symbol_type: SymbolType::Equals,
                    },
                    row: 7,
                    col_start: 9,
                    col_end: 10,
                },
                Token {
                    data: TokenData::Number { number: 15 },
                    row: 7,
                    col_start: 11,
                    col_end: 13,
                },
            ]),
            indentation: 1,
        },
        TokenLine {
            tokens: Vec::from([
                Token {
                    data: TokenData::Variable {
                        name: String::from("var3"),
                    },
                    row: 8,
                    col_start: 4,
                    col_end: 8,
                },
                Token {
                    data: TokenData::Symbol {
                        symbol_type: SymbolType::Equals,
                    },
                    row: 8,
                    col_start: 9,
                    col_end: 10,
                },
                Token {
                    data: TokenData::Number { number: 15 },
                    row: 8,
                    col_start: 11,
                    col_end: 13,
                },
            ]),
            indentation: 1,
        },
    ]);

    compare(tokens, expected);
}

#[test]
fn else_and_else_if_test() {
    // If statement syntax:
    // if <condition>
    //     <statements>
    // else
    //     <statements>
    // else if <condition>
    //     <statements>

    #[rustfmt::skip]
    let program = Vec::from([
        "if 2 + 3",
        "    var = 15",
        "else",
        "    var = 16",
        "if 2 + 3",
        "    var = 15",
        "else if 2 + 3 * 4",
        "    var = 17",
        "else if 2 + 3 * 4",
        "    var = 18",
        "else",
        "    var = 19",
    ]);
    let tokens = tokenizer::tokenize(program);

    let expected = Vec::from([
        TokenLine {
            tokens: Vec::from([
                Token {
                    data: TokenData::Symbol {
                        symbol_type: SymbolType::If,
                    },
                    row: 0,
                    col_start: 0,
                    col_end: 2,
                },
                Token {
                    data: TokenData::Number { number: 2 },
                    row: 0,
                    col_start: 3,
                    col_end: 4,
                },
                Token {
                    data: TokenData::Symbol {
                        symbol_type: SymbolType::Plus,
                    },
                    row: 0,
                    col_start: 5,
                    col_end: 6,
                },
                Token {
                    data: TokenData::Number { number: 3 },
                    row: 0,
                    col_start: 7,
                    col_end: 8,
                },
            ]),
            indentation: 0,
        },
        TokenLine {
            tokens: Vec::from([
                Token {
                    data: TokenData::Variable {
                        name: String::from("var"),
                    },
                    row: 1,
                    col_start: 4,
                    col_end: 7,
                },
                Token {
                    data: TokenData::Symbol {
                        symbol_type: SymbolType::Equals,
                    },
                    row: 1,
                    col_start: 8,
                    col_end: 9,
                },
                Token {
                    data: TokenData::Number { number: 15 },
                    row: 1,
                    col_start: 10,
                    col_end: 12,
                },
            ]),
            indentation: 1,
        },
        TokenLine {
            tokens: Vec::from([Token {
                data: TokenData::Symbol {
                    symbol_type: SymbolType::Else,
                },
                row: 2,
                col_start: 0,
                col_end: 4,
            }]),
            indentation: 0,
        },
        TokenLine {
            tokens: Vec::from([
                Token {
                    data: TokenData::Variable {
                        name: String::from("var"),
                    },
                    row: 3,
                    col_start: 4,
                    col_end: 7,
                },
                Token {
                    data: TokenData::Symbol {
                        symbol_type: SymbolType::Equals,
                    },
                    row: 3,
                    col_start: 8,
                    col_end: 9,
                },
                Token {
                    data: TokenData::Number { number: 16 },
                    row: 3,
                    col_start: 10,
                    col_end: 12,
                },
            ]),
            indentation: 1,
        },
        TokenLine {
            tokens: Vec::from([
                Token {
                    data: TokenData::Symbol {
                        symbol_type: SymbolType::If,
                    },
                    row: 4,
                    col_start: 0,
                    col_end: 2,
                },
                Token {
                    data: TokenData::Number { number: 2 },
                    row: 4,
                    col_start: 3,
                    col_end: 4,
                },
                Token {
                    data: TokenData::Symbol {
                        symbol_type: SymbolType::Plus,
                    },
                    row: 4,
                    col_start: 5,
                    col_end: 6,
                },
                Token {
                    data: TokenData::Number { number: 3 },
                    row: 4,
                    col_start: 7,
                    col_end: 8,
                },
            ]),
            indentation: 0,
        },
        TokenLine {
            tokens: Vec::from([
                Token {
                    data: TokenData::Variable {
                        name: String::from("var"),
                    },
                    row: 5,
                    col_start: 4,
                    col_end: 7,
                },
                Token {
                    data: TokenData::Symbol {
                        symbol_type: SymbolType::Equals,
                    },
                    row: 5,
                    col_start: 8,
                    col_end: 9,
                },
                Token {
                    data: TokenData::Number { number: 15 },
                    row: 5,
                    col_start: 10,
                    col_end: 12,
                },
            ]),
            indentation: 1,
        },
        TokenLine {
            tokens: Vec::from([
                Token {
                    data: TokenData::Symbol {
                        symbol_type: SymbolType::Else,
                    },
                    row: 6,
                    col_start: 0,
                    col_end: 4,
                },
                Token {
                    data: TokenData::Symbol {
                        symbol_type: SymbolType::If,
                    },
                    row: 6,
                    col_start: 5,
                    col_end: 7,
                },
                Token {
                    data: TokenData::Number { number: 2 },
                    row: 6,
                    col_start: 8,
                    col_end: 9,
                },
                Token {
                    data: TokenData::Symbol {
                        symbol_type: SymbolType::Plus,
                    },
                    row: 6,
                    col_start: 10,
                    col_end: 11,
                },
                Token {
                    data: TokenData::Number { number: 3 },
                    row: 6,
                    col_start: 12,
                    col_end: 13,
                },
                Token {
                    data: TokenData::Symbol {
                        symbol_type: SymbolType::Star,
                    },
                    row: 6,
                    col_start: 14,
                    col_end: 15,
                },
                Token {
                    data: TokenData::Number { number: 4 },
                    row: 6,
                    col_start: 16,
                    col_end: 17,
                },
            ]),
            indentation: 0,
        },
        TokenLine {
            tokens: Vec::from([
                Token {
                    data: TokenData::Variable {
                        name: String::from("var"),
                    },
                    row: 7,
                    col_start: 4,
                    col_end: 7,
                },
                Token {
                    data: TokenData::Symbol {
                        symbol_type: SymbolType::Equals,
                    },
                    row: 7,
                    col_start: 8,
                    col_end: 9,
                },
                Token {
                    data: TokenData::Number { number: 17 },
                    row: 7,
                    col_start: 10,
                    col_end: 12,
                },
            ]),
            indentation: 1,
        },
        TokenLine {
            tokens: Vec::from([
                Token {
                    data: TokenData::Symbol {
                        symbol_type: SymbolType::Else,
                    },
                    row: 8,
                    col_start: 0,
                    col_end: 4,
                },
                Token {
                    data: TokenData::Symbol {
                        symbol_type: SymbolType::If,
                    },
                    row: 8,
                    col_start: 5,
                    col_end: 7,
                },
                Token {
                    data: TokenData::Number { number: 2 },
                    row: 8,
                    col_start: 8,
                    col_end: 9,
                },
                Token {
                    data: TokenData::Symbol {
                        symbol_type: SymbolType::Plus,
                    },
                    row: 8,
                    col_start: 10,
                    col_end: 11,
                },
                Token {
                    data: TokenData::Number { number: 3 },
                    row: 8,
                    col_start: 12,
                    col_end: 13,
                },
                Token {
                    data: TokenData::Symbol {
                        symbol_type: SymbolType::Star,
                    },
                    row: 8,
                    col_start: 14,
                    col_end: 15,
                },
                Token {
                    data: TokenData::Number { number: 4 },
                    row: 8,
                    col_start: 16,
                    col_end: 17,
                },
            ]),
            indentation: 0,
        },
        TokenLine {
            tokens: Vec::from([
                Token {
                    data: TokenData::Variable {
                        name: String::from("var"),
                    },
                    row: 9,
                    col_start: 4,
                    col_end: 7,
                },
                Token {
                    data: TokenData::Symbol {
                        symbol_type: SymbolType::Equals,
                    },
                    row: 9,
                    col_start: 8,
                    col_end: 9,
                },
                Token {
                    data: TokenData::Number { number: 18 },
                    row: 9,
                    col_start: 10,
                    col_end: 12,
                },
            ]),
            indentation: 1,
        },
        TokenLine {
            tokens: Vec::from([Token {
                data: TokenData::Symbol {
                    symbol_type: SymbolType::Else,
                },
                row: 10,
                col_start: 0,
                col_end: 4,
            }]),
            indentation: 0,
        },
        TokenLine {
            tokens: Vec::from([
                Token {
                    data: TokenData::Variable {
                        name: String::from("var"),
                    },
                    row: 11,
                    col_start: 4,
                    col_end: 7,
                },
                Token {
                    data: TokenData::Symbol {
                        symbol_type: SymbolType::Equals,
                    },
                    row: 11,
                    col_start: 8,
                    col_end: 9,
                },
                Token {
                    data: TokenData::Number { number: 19 },
                    row: 11,
                    col_start: 10,
                    col_end: 12,
                },
            ]),
            indentation: 1,
        },
    ]);

    compare(tokens, expected);
}

#[test]
fn function_def_test() {
    #[rustfmt::skip]
    let program = Vec::from([
        "fun add()",
        "    return 0",
        "fun add(a)",
        "    return a",
        "fun add(a, b)",
        "    return a + b",
        "fun add(a, b, c, d)",
        "    return a + b + c + d",
    ]);
    let tokens = tokenizer::tokenize(program);

    let expected = Vec::from([
        TokenLine {
            tokens: Vec::from([
                Token {
                    data: TokenData::Symbol {
                        symbol_type: SymbolType::Fun,
                    },
                    row: 0,
                    col_start: 0,
                    col_end: 3,
                },
                Token {
                    data: TokenData::Variable {
                        name: String::from("add"),
                    },
                    row: 0,
                    col_start: 4,
                    col_end: 7,
                },
                Token {
                    data: TokenData::Symbol {
                        symbol_type: SymbolType::ParenthesisOpen,
                    },
                    row: 0,
                    col_start: 7,
                    col_end: 8,
                },
                Token {
                    data: TokenData::Symbol {
                        symbol_type: SymbolType::ParenthesisClosed,
                    },
                    row: 0,
                    col_start: 8,
                    col_end: 9,
                },
            ]),
            indentation: 0,
        },
        TokenLine {
            tokens: Vec::from([
                Token {
                    data: TokenData::Symbol {
                        symbol_type: SymbolType::Return,
                    },
                    row: 1,
                    col_start: 4,
                    col_end: 10,
                },
                Token {
                    data: TokenData::Number { number: 0 },
                    row: 1,
                    col_start: 11,
                    col_end: 12,
                },
            ]),
            indentation: 1,
        },
        TokenLine {
            tokens: Vec::from([
                Token {
                    data: TokenData::Symbol {
                        symbol_type: SymbolType::Fun,
                    },
                    row: 2,
                    col_start: 0,
                    col_end: 3,
                },
                Token {
                    data: TokenData::Variable {
                        name: String::from("add"),
                    },
                    row: 2,
                    col_start: 4,
                    col_end: 7,
                },
                Token {
                    data: TokenData::Symbol {
                        symbol_type: SymbolType::ParenthesisOpen,
                    },
                    row: 2,
                    col_start: 7,
                    col_end: 8,
                },
                Token {
                    data: TokenData::Variable {
                        name: String::from("a"),
                    },
                    row: 2,
                    col_start: 8,
                    col_end: 9,
                },
                Token {
                    data: TokenData::Symbol {
                        symbol_type: SymbolType::ParenthesisClosed,
                    },
                    row: 2,
                    col_start: 9,
                    col_end: 10,
                },
            ]),
            indentation: 0,
        },
        TokenLine {
            tokens: Vec::from([
                Token {
                    data: TokenData::Symbol {
                        symbol_type: SymbolType::Return,
                    },
                    row: 3,
                    col_start: 4,
                    col_end: 10,
                },
                Token {
                    data: TokenData::Variable {
                        name: String::from("a"),
                    },
                    row: 3,
                    col_start: 11,
                    col_end: 12,
                },
            ]),
            indentation: 1,
        },
        TokenLine {
            tokens: Vec::from([
                Token {
                    data: TokenData::Symbol {
                        symbol_type: SymbolType::Fun,
                    },
                    row: 4,
                    col_start: 0,
                    col_end: 3,
                },
                Token {
                    data: TokenData::Variable {
                        name: String::from("add"),
                    },
                    row: 4,
                    col_start: 4,
                    col_end: 7,
                },
                Token {
                    data: TokenData::Symbol {
                        symbol_type: SymbolType::ParenthesisOpen,
                    },
                    row: 4,
                    col_start: 7,
                    col_end: 8,
                },
                Token {
                    data: TokenData::Variable {
                        name: String::from("a"),
                    },
                    row: 4,
                    col_start: 8,
                    col_end: 9,
                },
                Token {
                    data: TokenData::Symbol {
                        symbol_type: SymbolType::Comma,
                    },
                    row: 4,
                    col_start: 9,
                    col_end: 10,
                },
                Token {
                    data: TokenData::Variable {
                        name: String::from("b"),
                    },
                    row: 4,
                    col_start: 11,
                    col_end: 12,
                },
                Token {
                    data: TokenData::Symbol {
                        symbol_type: SymbolType::ParenthesisClosed,
                    },
                    row: 4,
                    col_start: 12,
                    col_end: 13,
                },
            ]),
            indentation: 0,
        },
        TokenLine {
            tokens: Vec::from([
                Token {
                    data: TokenData::Symbol {
                        symbol_type: SymbolType::Return,
                    },
                    row: 5,
                    col_start: 4,
                    col_end: 10,
                },
                Token {
                    data: TokenData::Variable {
                        name: String::from("a"),
                    },
                    row: 5,
                    col_start: 11,
                    col_end: 12,
                },
                Token {
                    data: TokenData::Symbol {
                        symbol_type: SymbolType::Plus,
                    },
                    row: 5,
                    col_start: 13,
                    col_end: 14,
                },
                Token {
                    data: TokenData::Variable {
                        name: String::from("b"),
                    },
                    row: 5,
                    col_start: 15,
                    col_end: 16,
                },
            ]),
            indentation: 1,
        },
        TokenLine {
            tokens: Vec::from([
                Token {
                    data: TokenData::Symbol {
                        symbol_type: SymbolType::Fun,
                    },
                    row: 6,
                    col_start: 0,
                    col_end: 3,
                },
                Token {
                    data: TokenData::Variable {
                        name: String::from("add"),
                    },
                    row: 6,
                    col_start: 4,
                    col_end: 7,
                },
                Token {
                    data: TokenData::Symbol {
                        symbol_type: SymbolType::ParenthesisOpen,
                    },
                    row: 6,
                    col_start: 7,
                    col_end: 8,
                },
                Token {
                    data: TokenData::Variable {
                        name: String::from("a"),
                    },
                    row: 6,
                    col_start: 8,
                    col_end: 9,
                },
                Token {
                    data: TokenData::Symbol {
                        symbol_type: SymbolType::Comma,
                    },
                    row: 6,
                    col_start: 9,
                    col_end: 10,
                },
                Token {
                    data: TokenData::Variable {
                        name: String::from("b"),
                    },
                    row: 6,
                    col_start: 11,
                    col_end: 12,
                },
                Token {
                    data: TokenData::Symbol {
                        symbol_type: SymbolType::Comma,
                    },
                    row: 6,
                    col_start: 12,
                    col_end: 13,
                },
                Token {
                    data: TokenData::Variable {
                        name: String::from("c"),
                    },
                    row: 6,
                    col_start: 14,
                    col_end: 15,
                },
                Token {
                    data: TokenData::Symbol {
                        symbol_type: SymbolType::Comma,
                    },
                    row: 6,
                    col_start: 15,
                    col_end: 16,
                },
                Token {
                    data: TokenData::Variable {
                        name: String::from("d"),
                    },
                    row: 6,
                    col_start: 17,
                    col_end: 18,
                },
                Token {
                    data: TokenData::Symbol {
                        symbol_type: SymbolType::ParenthesisClosed,
                    },
                    row: 6,
                    col_start: 18,
                    col_end: 19,
                },
            ]),
            indentation: 0,
        },
        TokenLine {
            tokens: Vec::from([
                Token {
                    data: TokenData::Symbol {
                        symbol_type: SymbolType::Return,
                    },
                    row: 7,
                    col_start: 4,
                    col_end: 10,
                },
                Token {
                    data: TokenData::Variable {
                        name: String::from("a"),
                    },
                    row: 7,
                    col_start: 11,
                    col_end: 12,
                },
                Token {
                    data: TokenData::Symbol {
                        symbol_type: SymbolType::Plus,
                    },
                    row: 7,
                    col_start: 13,
                    col_end: 14,
                },
                Token {
                    data: TokenData::Variable {
                        name: String::from("b"),
                    },
                    row: 7,
                    col_start: 15,
                    col_end: 16,
                },
                Token {
                    data: TokenData::Symbol {
                        symbol_type: SymbolType::Plus,
                    },
                    row: 7,
                    col_start: 17,
                    col_end: 18,
                },
                Token {
                    data: TokenData::Variable {
                        name: String::from("c"),
                    },
                    row: 7,
                    col_start: 19,
                    col_end: 20,
                },
                Token {
                    data: TokenData::Symbol {
                        symbol_type: SymbolType::Plus,
                    },
                    row: 7,
                    col_start: 21,
                    col_end: 22,
                },
                Token {
                    data: TokenData::Variable {
                        name: String::from("d"),
                    },
                    row: 7,
                    col_start: 23,
                    col_end: 24,
                },
            ]),
            indentation: 1,
        },
    ]);

    compare(tokens, expected);
}

#[test]
fn function_calls_test() {
    #[rustfmt::skip]
    let program = Vec::from([
        "a(\"hi\")",
    ]);
    let tokens = tokenizer::tokenize(program);

    let expected = Vec::from([TokenLine {
        tokens: Vec::from([
            Token {
                data: TokenData::Variable {
                    name: String::from("a"),
                },
                row: 0,
                col_start: 0,
                col_end: 1,
            },
            Token {
                data: TokenData::Symbol {
                    symbol_type: SymbolType::ParenthesisOpen,
                },
                row: 0,
                col_start: 1,
                col_end: 2,
            },
            Token {
                data: TokenData::String {
                    value: String::from("hi"),
                },
                row: 0,
                col_start: 2,
                col_end: 6,
            },
            Token {
                data: TokenData::Symbol {
                    symbol_type: SymbolType::ParenthesisClosed,
                },
                row: 0,
                col_start: 6,
                col_end: 7,
            },
        ]),
        indentation: 0,
    }]);

    compare(tokens, expected);
}

#[test]
fn full_test() {
    let program = Vec::from([""]);
    let tokens = tokenizer::tokenize(program);
}
