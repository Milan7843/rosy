use rosy::parser::{self, BaseExpr};

#[test]
fn simple_variable() {
    let program = Vec::from([
        "a_b",
        "long_variable",
        "var"
    ]);
    let expressions = parser::parse_strings(program);
    let expected = Vec::from([
        BaseExpr::Simple { expr: parser::RecExpr::Variable { name: String::from("a_b") } },
        BaseExpr::Simple { expr: parser::RecExpr::Variable { name: String::from("long_variable") } },
        BaseExpr::Simple { expr: parser::RecExpr::Variable { name: String::from("var") } }
    ]);

    match expressions {
        Ok(expressions) => assert_eq!(expressions, expected),
        Err(e) => panic!("{}", e)
    }
}

#[test]
fn simple_integer() {
    let program = Vec::from([
        "0",
        "1",
        "12",
        "234589374"
    ]);
    let expressions = parser::parse_strings(program);
    let expected = Vec::from([
        BaseExpr::Simple { expr: parser::RecExpr::Number { number: 0 } },
        BaseExpr::Simple { expr: parser::RecExpr::Number { number: 1 } },
        BaseExpr::Simple { expr: parser::RecExpr::Number { number: 12 } },
        BaseExpr::Simple { expr: parser::RecExpr::Number { number: 234589374 } }
    ]);

    match expressions {
        Ok(expressions) => assert_eq!(expressions, expected),
        Err(e) => panic!("{}", e)
    }
}

#[test]
fn simple_string() {
    let program = Vec::from([
        "\"blah\"",
        "\"fun in for loop  { } () (*)^)*& _+-=    spaces\"",
    ]);
    let expressions = parser::parse_strings(program);
    let expected = Vec::from([
        BaseExpr::Simple { expr: parser::RecExpr::String { value: String::from("blah") } },
        BaseExpr::Simple { expr: parser::RecExpr::String { value: String::from("fun in for loop  { } () (*)^)*& _+-=    spaces") } },
    ]);

    match expressions {
        Ok(expressions) => assert_eq!(expressions, expected),
        Err(e) => panic!("{}", e)
    }
}
