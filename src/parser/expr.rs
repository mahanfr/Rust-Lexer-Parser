use crate::lexer::Lexer;
pub fn get_expretion(lexer : &mut Lexer) -> Expr {
    todo!()
}

#[derive(Debug, PartialEq)]
pub enum Expr {
    ExprBinary,
    ExprUnary,
}

#[derive(Debug, PartialEq)]
pub struct ExprPath {}

#[derive(Debug, PartialEq)]
pub struct ExprAssign {
    pub left: ExprPath,
    pub right: Expr,
}

