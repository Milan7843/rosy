use std::fmt::format;

use crate::parser;
use crate::parser::BaseExpr;
use crate::parser::BaseExprData;
use crate::parser::RecExpr;
use crate::parser::RecExprData;
use crate::tokenizer::Error;
use crate::typechecker::Type;
use crate::typechecker::FunctionType;

pub fn uniquify(base_expressions: &mut Vec<BaseExpr<()>>) {
    let mut counter = 0;
    let mut env = std::collections::HashMap::new();

    for base_expr in base_expressions.iter_mut()
    {
        uniquify_base_expr(base_expr, &mut counter, &mut env);
    }
}

fn uniquify_base_expr(base_expr: &mut BaseExpr<()>, counter: &mut usize, env: &mut std::collections::HashMap<String, String>) {
    match &mut base_expr.data
    {
        _ =>
        {
            unimplemented!("Only RecExpr is implemented in uniquify_base_expr");
        }
    }
}