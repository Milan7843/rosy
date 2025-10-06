use crate::parser;
use crate::parser::BaseExpr;
use crate::parser::RecExpr;

<<<<<<< Updated upstream
fn desugar(base_expressions: Vec<BaseExpr>) -> Result<Vec<BaseExpr>, String> {
    
}
=======
pub fn desugar(base_expressions: Vec<BaseExpr<()>>) -> Vec<BaseExpr<()>> {
    let mut desugared_expressions = Vec::new();

    for base_expr in base_expressions
    {
        let desugared_expr = desugar_base_expr(base_expr);
        desugared_expressions.extend(desugared_expr);
    }

    return desugared_expressions;
}

fn desugar_base_expr(base_expr: BaseExpr<()>) -> Vec<BaseExpr<()>> {
    match base_expr.data
    {
        parser::BaseExprData::PlusEqualsStatement { var_name, expr } =>
        {
            let var_name_len = var_name.len();
            let assignment = BaseExpr {
                data: parser::BaseExprData::VariableAssignment {
                    var_name: var_name.clone(),
                    expr: RecExpr {
                        data: parser::RecExprData::Add {
                            left: Box::new(RecExpr {
                                data: parser::RecExprData::Variable { name: var_name },
                                row: base_expr.row,
                                col_start: base_expr.col_start,
                                col_end: base_expr.col_start + var_name_len,
                                generic_data: (),
                            }),
                            right: Box::new(expr),
                        },
                        row: base_expr.row,
                        col_start: base_expr.col_start,
                        col_end: base_expr.col_end,
                        generic_data: (),
                    },
                },
                row: base_expr.row,
                col_start: base_expr.col_start,
                col_end: base_expr.col_end,
                generic_data: (),
            };
            return vec![assignment];
        }

        // Each of these constructs can contain multiple base expressions in their body,
        // so we need to recursively desugar those bodies.
        parser::BaseExprData::ForLoop {
            var_name,
            until,
            body,
        } =>
        {
            let mut desugared_expressions = Vec::new();

            for base_expr in body
            {
                let desugared_expr = desugar_base_expr(base_expr);
                desugared_expressions.extend(desugared_expr);
            }
            return vec![BaseExpr {
                data: parser::BaseExprData::ForLoop {
                    var_name,
                    until,
                    body: desugared_expressions,
                },
                row: base_expr.row,
                col_start: base_expr.col_start,
                col_end: base_expr.col_end,
                generic_data: (),
            }];
        }
        parser::BaseExprData::IfStatement {
            condition,
            body,
            else_statement,
        } =>
        {
            let mut desugared_expressions = Vec::new();

            for base_expr in body
            {
                let desugared_expr = desugar_base_expr(base_expr);
                desugared_expressions.extend(desugared_expr);
            }

            let desugared_else = match else_statement
            {
                Some(else_body) =>
                {
                    let desugared_else = desugar_base_expr(*else_body);
                    let desugared_else = Box::new(desugared_else[0].clone());
                    Some(desugared_else)
                }
                None => None,
            };

            return vec![BaseExpr {
                data: parser::BaseExprData::IfStatement {
                    condition,
                    body: desugared_expressions,
                    else_statement: desugared_else,
                },
                row: base_expr.row,
                col_start: base_expr.col_start,
                col_end: base_expr.col_end,
                generic_data: (),
            }];
        }
        parser::BaseExprData::ElseIfStatement {
            condition,
            body,
            else_statement,
        } =>
        {
            let mut desugared_expressions = Vec::new();

            for base_expr in body
            {
                let desugared_expr = desugar_base_expr(base_expr);
                desugared_expressions.extend(desugared_expr);
            }

            let desugared_else = match else_statement
            {
                Some(else_body) =>
                {
                    let desugared_else = desugar_base_expr(*else_body);
                    let desugared_else = Box::new(desugared_else[0].clone());
                    Some(desugared_else)
                }
                None => None,
            };

            return vec![BaseExpr {
                data: parser::BaseExprData::ElseIfStatement {
                    condition,
                    body: desugared_expressions,
                    else_statement: desugared_else,
                },
                row: base_expr.row,
                col_start: base_expr.col_start,
                col_end: base_expr.col_end,
                generic_data: (),
            }];
        }
        parser::BaseExprData::ElseStatement { body } =>
        {
            let mut desugared_expressions = Vec::new();

            for base_expr in body
            {
                let desugared_expr = desugar_base_expr(base_expr);
                desugared_expressions.extend(desugared_expr);
            }
            return vec![BaseExpr {
                data: parser::BaseExprData::ElseStatement {
                    body: desugared_expressions,
                },
                row: base_expr.row,
                col_start: base_expr.col_start,
                col_end: base_expr.col_end,
                generic_data: (),
            }];
        }
        parser::BaseExprData::FunctionDefinition {
            fun_name,
            args,
            body,
        } =>
        {
            let mut desugared_expressions = Vec::new();

            for base_expr in body
            {
                let desugared_expr = desugar_base_expr(base_expr);
                desugared_expressions.extend(desugared_expr);
            }
            return vec![BaseExpr {
                data: parser::BaseExprData::FunctionDefinition {
                    fun_name,
                    args,
                    body: desugared_expressions,
                },
                row: base_expr.row,
                col_start: base_expr.col_start,
                col_end: base_expr.col_end,
                generic_data: (),
            }];
        }
        _ =>
        {
            return vec![base_expr];
        }
    }
}
>>>>>>> Stashed changes
