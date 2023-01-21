use crate::ast::expressions_ast::ExpAst;
use std::sync::Arc;

pub struct Judgment {
    expression: Arc<ExpAst>,
    _type: Arc<ExpAst>,
}
