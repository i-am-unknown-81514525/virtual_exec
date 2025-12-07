use syn::parse::{Parse, ParseStream, Result};
use syn::{braced, parenthesized, Ident, Lit, Token};
use vir_py_rs_type::ast::core as final_ast;


pub enum Stmt {
    Expr(Expr),
    // TODO: Let, If, For, etc.
}

pub struct Expr {
    pub left: Atom,
    pub rights: Vec<(final_ast::BinaryOperator, Atom)>,
}

pub enum Atom {
    Literal(final_ast::Literal),
    Variable(String),
    // TODO: UnaryOp, Call, etc.
}


impl Parse for Expr {
    fn parse(input: ParseStream) -> Result<Self> {
        let left = input.parse()?;
        let mut rights = Vec::new();
        while input.peek(Token![+]) || input.peek(Token![*]) { // simplified for now
            let op = if input.peek(Token![+]) {
                input.parse::<Token![+]>()?;
                final_ast::BinaryOperator::Add
            } else {
                input.parse::<Token![*]>()?;
                final_ast::BinaryOperator::Multiply
            };
            let right = input.parse()?;
            rights.push((op, right));
        }
        Ok(Expr { left, rights })
    }
}

impl Parse for Atom {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.peek(Lit) {
            let lit: Lit = input.parse()?;
            let final_lit = match lit {
                Lit::Int(i) => final_ast::Literal::Int(i.base10_parse()?),
                Lit::Float(f) => final_ast::Literal::Float(f.base10_parse()?),
                Lit::Str(s) => final_ast::Literal::String(s.value()),
                Lit::Bool(b) => final_ast::Literal::Bool(b.value),
                _ => return Err(input.error("unsupported literal type")),
            };
            Ok(Atom::Literal(final_lit))
        } else if input.peek(Ident) {
            let ident: Ident = input.parse()?;
            Ok(Atom::Variable(ident.to_string()))
        } else {
            Err(input.error("expected a literal or an identifier"))
        }
    }
}
