use crate::ast::expressions_ast::ExpAst;
use std::sync::Arc;

pub struct ContextVar {
    pub parent_contexts: Vec<Arc<ContextVar>>,
    pub _type: ExpAst,
}
