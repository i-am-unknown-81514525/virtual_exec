use crate::tokenizer;
use vir_py_rs_type::ast::core as final_ast;
use crate::error::ParseError;

fn convert_expr(expr: tokenizer::Expr) -> final_ast::Node<final_ast::Expr> {
    let mut final_expr = final_ast::Expr::from(expr.left);

    for (op, right_atom) in expr.rights {
        let right_expr = final_ast::Expr::from(right_atom);
        final_expr = final_ast::Expr::BinaryOp {
            left: Box::new(final_ast::Node { kind: final_expr, span: None }),
            op,
            right: Box::new(final_ast::Node { kind: right_expr, span: None }),
        };
    }
    
    final_ast::Node { kind: final_expr, span: None }
}

impl From<tokenizer::Atom> for final_ast::Expr {
    fn from(atom: tokenizer::Atom) -> Self {
        match atom {
            tokenizer::Atom::Literal(l) => final_ast::Expr::Literal(l),
            tokenizer::Atom::Variable(v) => final_ast::Expr::Variable(v),
        }
    }
}


pub fn parse(source: &str) -> std::result::Result<final_ast::Node<final_ast::Expr>, ParseError> {
    let intermediate_expr: tokenizer::Expr = syn::parse_str(source).map_err(ParseError::SynParseError)?;
    Ok(convert_expr(intermediate_expr))
}
