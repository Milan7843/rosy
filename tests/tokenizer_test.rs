use rosy::tokenizer;
use tokenizer::SymbolType;
use tokenizer::Token;
use tokenizer::TokenLine;

fn compare(actual: Result<Vec<TokenLine>, String>, expected: Vec<TokenLine>) {
    match actual {
        Ok(tokens) => assert_eq!(tokens, expected),
        Err(e) => panic!("{}", e),
    }
}

fn compare_linewise(actual: Result<Vec<TokenLine>, String>, expected: Vec<TokenLine>) {
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
        Err(e) => panic!("{}", e),
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
            tokens: Vec::from([Token::Variable {
                name: String::from("a"),
            }]),
            indentation: 0,
        },
        TokenLine {
            tokens: Vec::from([Token::Variable {
                name: String::from("multiple_words"),
            }]),
            indentation: 0,
        },
        TokenLine {
            tokens: Vec::from([Token::Variable {
                name: String::from("CapITalS"),
            }]),
            indentation: 0,
        },
        TokenLine {
            tokens: Vec::from([Token::Variable {
                name: String::from("a1"),
            }]),
            indentation: 0,
        },
        TokenLine {
            tokens: Vec::from([Token::Variable {
                name: String::from("a1b34nh_4"),
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
            tokens: Vec::from([
                Token::Number { number: 2 },
                Token::Symbol {
                    symbol_type: SymbolType::Plus,
                },
                Token::Number { number: 3 },
            ]),
            indentation: 0,
        },
        TokenLine {
            tokens: Vec::from([
                Token::Number { number: 2 },
                Token::Symbol {
                    symbol_type: SymbolType::Plus,
                },
                Token::Number { number: 3 },
                Token::Symbol {
                    symbol_type: SymbolType::Star,
                },
                Token::Number { number: 4 },
            ]),
            indentation: 0,
        },
        TokenLine {
            tokens: Vec::from([
                Token::Number { number: 23 },
                Token::Symbol {
                    symbol_type: SymbolType::Plus,
                },
                Token::Number { number: 28 },
                Token::Symbol {
                    symbol_type: SymbolType::Slash,
                },
                Token::Number { number: 15 },
                Token::Symbol {
                    symbol_type: SymbolType::Star,
                },
                Token::Number { number: 7 },
                Token::Symbol {
                    symbol_type: SymbolType::Minus,
                },
                Token::Number { number: 13 },
                Token::Symbol {
                    symbol_type: SymbolType::Hat,
                },
                Token::Number { number: 2 },
            ]),
            indentation: 0,
        },
        TokenLine {
            tokens: Vec::from([
                Token::Number { number: 23 },
                Token::Symbol {
                    symbol_type: SymbolType::Plus,
                },
                Token::Number { number: 28 },
                Token::Symbol {
                    symbol_type: SymbolType::Slash,
                },
                Token::Number { number: 15 },
                Token::Symbol {
                    symbol_type: SymbolType::Star,
                },
                Token::Number { number: 7 },
                Token::Symbol {
                    symbol_type: SymbolType::Minus,
                },
                Token::Number { number: 13 },
                Token::Symbol {
                    symbol_type: SymbolType::Hat,
                },
                Token::Number { number: 2 },
            ]),
            indentation: 0,
        },
        TokenLine {
            tokens: Vec::from([
                Token::Variable {
                    name: String::from("var"),
                },
                Token::Symbol {
                    symbol_type: SymbolType::Equals,
                },
                Token::Number { number: 15 },
            ]),
            indentation: 0,
        },
        TokenLine {
            tokens: Vec::from([
                Token::Variable {
                    name: String::from("var2"),
                },
                Token::Symbol {
                    symbol_type: SymbolType::Equals,
                },
                Token::Symbol {
                    symbol_type: SymbolType::ParenthesisOpen,
                },
                Token::Number { number: 15 },
                Token::Symbol {
                    symbol_type: SymbolType::Plus,
                },
                Token::Number { number: 16 },
                Token::Symbol {
                    symbol_type: SymbolType::Slash,
                },
                Token::Number { number: 4 },
                Token::Symbol {
                    symbol_type: SymbolType::ParenthesisClosed,
                },
                Token::Symbol {
                    symbol_type: SymbolType::Hat,
                },
                Token::Number { number: 2 },
                Token::Symbol {
                    symbol_type: SymbolType::Star,
                },
                Token::Number { number: 4 },
                Token::Symbol {
                    symbol_type: SymbolType::Slash,
                },
                Token::Number { number: 2 },
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
            tokens: Vec::from([Token::Symbol {
                symbol_type: SymbolType::True,
            }]),
            indentation: 0,
        },
        TokenLine {
            tokens: Vec::from([Token::Symbol {
                symbol_type: SymbolType::False,
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
                Token::Symbol {
                    symbol_type: SymbolType::If,
                },
                Token::Number { number: 2 },
                Token::Symbol {
                    symbol_type: SymbolType::Plus,
                },
                Token::Number { number: 3 },
            ]),
            indentation: 0,
        },
        TokenLine {
            tokens: Vec::from([
                Token::Variable {
                    name: String::from("var"),
                },
                Token::Symbol {
                    symbol_type: SymbolType::Equals,
                },
                Token::Number { number: 15 },
            ]),
            indentation: 1,
        },
        TokenLine {
            tokens: Vec::from([
                Token::Symbol {
                    symbol_type: SymbolType::If,
                },
                Token::Number { number: 2 },
                Token::Symbol {
                    symbol_type: SymbolType::Plus,
                },
                Token::Number { number: 3 },
                Token::Symbol {
                    symbol_type: SymbolType::Star,
                },
                Token::Number { number: 4 },
            ]),
            indentation: 0,
        },
        TokenLine {
            tokens: Vec::from([
                Token::Variable {
                    name: String::from("var"),
                },
                Token::Symbol {
                    symbol_type: SymbolType::Equals,
                },
                Token::Number { number: 15 },
            ]),
            indentation: 1,
        },
        TokenLine {
            tokens: Vec::from([
                Token::Variable {
                    name: String::from("var2"),
                },
                Token::Symbol {
                    symbol_type: SymbolType::Equals,
                },
                Token::Number { number: 15 },
            ]),
            indentation: 1,
        },
        TokenLine {
            tokens: Vec::from([
                Token::Symbol {
                    symbol_type: SymbolType::If,
                },
                Token::Number { number: 2 },
                Token::Symbol {
                    symbol_type: SymbolType::Plus,
                },
                Token::Number { number: 3 },
                Token::Symbol {
                    symbol_type: SymbolType::Star,
                },
                Token::Number { number: 4 },
            ]),
            indentation: 0,
        },
        TokenLine {
            tokens: Vec::from([
                Token::Variable {
                    name: String::from("var"),
                },
                Token::Symbol {
                    symbol_type: SymbolType::Equals,
                },
                Token::Number { number: 15 },
            ]),
            indentation: 1,
        },
        TokenLine {
            tokens: Vec::from([
                Token::Variable {
                    name: String::from("var2"),
                },
                Token::Symbol {
                    symbol_type: SymbolType::Equals,
                },
                Token::Number { number: 15 },
            ]),
            indentation: 1,
        },
        TokenLine {
            tokens: Vec::from([
                Token::Variable {
                    name: String::from("var3"),
                },
                Token::Symbol {
                    symbol_type: SymbolType::Equals,
                },
                Token::Number { number: 15 },
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
                Token::Symbol {
                    symbol_type: SymbolType::If,
                },
                Token::Number { number: 2 },
                Token::Symbol {
                    symbol_type: SymbolType::Plus,
                },
                Token::Number { number: 3 },
            ]),
            indentation: 0,
        },
        TokenLine {
            tokens: Vec::from([
                Token::Variable {
                    name: String::from("var"),
                },
                Token::Symbol {
                    symbol_type: SymbolType::Equals,
                },
                Token::Number { number: 15 },
            ]),
            indentation: 1,
        },
        TokenLine {
            tokens: Vec::from([Token::Symbol {
                symbol_type: SymbolType::Else,
            }]),
            indentation: 0,
        },
        TokenLine {
            tokens: Vec::from([
                Token::Variable {
                    name: String::from("var"),
                },
                Token::Symbol {
                    symbol_type: SymbolType::Equals,
                },
                Token::Number { number: 16 },
            ]),
            indentation: 1,
        },
        TokenLine {
            tokens: Vec::from([
                Token::Symbol {
                    symbol_type: SymbolType::If,
                },
                Token::Number { number: 2 },
                Token::Symbol {
                    symbol_type: SymbolType::Plus,
                },
                Token::Number { number: 3 },
            ]),
            indentation: 0,
        },
        TokenLine {
            tokens: Vec::from([
                Token::Variable {
                    name: String::from("var"),
                },
                Token::Symbol {
                    symbol_type: SymbolType::Equals,
                },
                Token::Number { number: 15 },
            ]),
            indentation: 1,
        },
        TokenLine {
            tokens: Vec::from([
                Token::Symbol {
                    symbol_type: SymbolType::Else,
                },
                Token::Symbol {
                    symbol_type: SymbolType::If,
                },
                Token::Number { number: 2 },
                Token::Symbol {
                    symbol_type: SymbolType::Plus,
                },
                Token::Number { number: 3 },
                Token::Symbol {
                    symbol_type: SymbolType::Star,
                },
                Token::Number { number: 4 },
            ]),
            indentation: 0,
        },
        TokenLine {
            tokens: Vec::from([
                Token::Variable {
                    name: String::from("var"),
                },
                Token::Symbol {
                    symbol_type: SymbolType::Equals,
                },
                Token::Number { number: 17 },
            ]),
            indentation: 1,
        },
        TokenLine {
            tokens: Vec::from([
                Token::Symbol {
                    symbol_type: SymbolType::Else,
                },
                Token::Symbol {
                    symbol_type: SymbolType::If,
                },
                Token::Number { number: 2 },
                Token::Symbol {
                    symbol_type: SymbolType::Plus,
                },
                Token::Number { number: 3 },
                Token::Symbol {
                    symbol_type: SymbolType::Star,
                },
                Token::Number { number: 4 },
            ]),
            indentation: 0,
        },
        TokenLine {
            tokens: Vec::from([
                Token::Variable {
                    name: String::from("var"),
                },
                Token::Symbol {
                    symbol_type: SymbolType::Equals,
                },
                Token::Number { number: 18 },
            ]),
            indentation: 1,
        },
        TokenLine {
            tokens: Vec::from([Token::Symbol {
                symbol_type: SymbolType::Else,
            }]),
            indentation: 0,
        },
        TokenLine {
            tokens: Vec::from([
                Token::Variable {
                    name: String::from("var"),
                },
                Token::Symbol {
                    symbol_type: SymbolType::Equals,
                },
                Token::Number { number: 19 },
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
        "fun add(a, b, c)",
        "    return a + b + c",
        "fun add(a, b, c, d)",
        "    return a + b + c + d",
    ]);
    let tokens = tokenizer::tokenize(program);

    let expected = Vec::from([
        TokenLine {
            tokens: Vec::from([
                Token::Symbol {
                    symbol_type: SymbolType::Fun,
                },
                Token::Variable {
                    name: String::from("add"),
                },
                Token::Symbol {
                    symbol_type: SymbolType::ParenthesisOpen,
                },
                Token::Symbol {
                    symbol_type: SymbolType::ParenthesisClosed,
                },
            ]),
            indentation: 0,
        },
        TokenLine {
            tokens: Vec::from([
                Token::Symbol {
                    symbol_type: SymbolType::Return,
                },
                Token::Number { number: 0 },
            ]),
            indentation: 1,
        },
        TokenLine {
            tokens: Vec::from([
                Token::Symbol {
                    symbol_type: SymbolType::Fun,
                },
                Token::Variable {
                    name: String::from("add"),
                },
                Token::Symbol {
                    symbol_type: SymbolType::ParenthesisOpen,
                },
                Token::Variable {
                    name: String::from("a"),
                },
                Token::Symbol {
                    symbol_type: SymbolType::ParenthesisClosed,
                },
            ]),
            indentation: 0,
        },
        TokenLine {
            tokens: Vec::from([
                Token::Symbol {
                    symbol_type: SymbolType::Return,
                },
                Token::Variable {
                    name: String::from("a"),
                },
            ]),
            indentation: 1,
        },
        TokenLine {
            tokens: Vec::from([
                Token::Symbol {
                    symbol_type: SymbolType::Fun,
                },
                Token::Variable {
                    name: String::from("add"),
                },
                Token::Symbol {
                    symbol_type: SymbolType::ParenthesisOpen,
                },
                Token::Variable {
                    name: String::from("a"),
                },
                Token::Symbol {
                    symbol_type: SymbolType::Comma,
                },
                Token::Variable {
                    name: String::from("b"),
                },
                Token::Symbol {
                    symbol_type: SymbolType::ParenthesisClosed,
                },
            ]),
            indentation: 0,
        },
        TokenLine {
            tokens: Vec::from([
                Token::Symbol {
                    symbol_type: SymbolType::Return,
                },
                Token::Variable {
                    name: String::from("a"),
                },
                Token::Symbol {
                    symbol_type: SymbolType::Plus,
                },
                Token::Variable {
                    name: String::from("b"),
                },
            ]),
            indentation: 1,
        },
        TokenLine {
            tokens: Vec::from([
                Token::Symbol {
                    symbol_type: SymbolType::Fun,
                },
                Token::Variable {
                    name: String::from("add"),
                },
                Token::Symbol {
                    symbol_type: SymbolType::ParenthesisOpen,
                },
                Token::Variable {
                    name: String::from("a"),
                },
                Token::Symbol {
                    symbol_type: SymbolType::Comma,
                },
                Token::Variable {
                    name: String::from("b"),
                },
                Token::Symbol {
                    symbol_type: SymbolType::Comma,
                },
                Token::Variable {
                    name: String::from("c"),
                },
                Token::Symbol {
                    symbol_type: SymbolType::ParenthesisClosed,
                },
            ]),
            indentation: 0,
        },
        TokenLine {
            tokens: Vec::from([
                Token::Symbol {
                    symbol_type: SymbolType::Return,
                },
                Token::Variable {
                    name: String::from("a"),
                },
                Token::Symbol {
                    symbol_type: SymbolType::Plus,
                },
                Token::Variable {
                    name: String::from("b"),
                },
                Token::Symbol {
                    symbol_type: SymbolType::Plus,
                },
                Token::Variable {
                    name: String::from("c"),
                },
            ]),
            indentation: 1,
        },
        TokenLine {
            tokens: Vec::from([
                Token::Symbol {
                    symbol_type: SymbolType::Fun,
                },
                Token::Variable {
                    name: String::from("add"),
                },
                Token::Symbol {
                    symbol_type: SymbolType::ParenthesisOpen,
                },
                Token::Variable {
                    name: String::from("a"),
                },
                Token::Symbol {
                    symbol_type: SymbolType::Comma,
                },
                Token::Variable {
                    name: String::from("b"),
                },
                Token::Symbol {
                    symbol_type: SymbolType::Comma,
                },
                Token::Variable {
                    name: String::from("c"),
                },
                Token::Symbol {
                    symbol_type: SymbolType::Comma,
                },
                Token::Variable {
                    name: String::from("d"),
                },
                Token::Symbol {
                    symbol_type: SymbolType::ParenthesisClosed,
                },
            ]),
            indentation: 0,
        },
        TokenLine {
            tokens: Vec::from([
                Token::Symbol {
                    symbol_type: SymbolType::Return,
                },
                Token::Variable {
                    name: String::from("a"),
                },
                Token::Symbol {
                    symbol_type: SymbolType::Plus,
                },
                Token::Variable {
                    name: String::from("b"),
                },
                Token::Symbol {
                    symbol_type: SymbolType::Plus,
                },
                Token::Variable {
                    name: String::from("c"),
                },
                Token::Symbol {
                    symbol_type: SymbolType::Plus,
                },
                Token::Variable {
                    name: String::from("d"),
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
            Token::Variable {
                name: String::from("a"),
            },
            Token::Symbol {
                symbol_type: SymbolType::ParenthesisOpen,
            },
            Token::String {
                value: String::from("hi"),
            },
            Token::Symbol {
                symbol_type: SymbolType::ParenthesisClosed,
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
