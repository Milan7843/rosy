use crate::parser;
use crate::parser::BaseExpr;
use crate::parser::RecExpr;

enum Value {
    Number(i32),
    Bool(bool),
    String(String),
    Function {
        name: String,
        args: Vec<String>,
        body: Vec<BaseExpr>,
    },
}

struct Binding {
    name: String,
    value: Value
}

struct Scope {
    bindings: Vec<Binding>
}

struct Environment {
    scopes: Vec<Scope>
}

fn interpret(base_expressions: Vec<BaseExpr>) -> Result<String, String> {

}

fn interpret_rec(base_expressions: &[BaseExpr]) -> Result<String, String> {

}

fn interpret_expr(expr: RecExpr, env: &mut Environment) -> Result<Option<Value>, String> {
    match expr {
        RecExpr::Variable { name }
    }
}

fn find_in_env(name: &String, env: &mut Environment) {

}