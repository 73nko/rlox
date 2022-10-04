extern crate rlox;
use rlox::{
    error::LoxError,
    token::{Object, Token},
};

enum Expr {
    Binary(BinaryExpr),
    Grouping(GroupingExpr),
    Literal(LiteralExpr),
    Unary(UnaryExpr),
}

pub struct BinaryExpr {
    left: Box<Expr>,
    op: Token,
    right: Box<Expr>,
}
pub struct GroupingExpr {
    expression: Box<Expr>,
}

pub struct LiteralExpr {
    value: Object,
}

pub struct UnaryExpr {
    operator: Token,
    right: Box<Expr>,
}

pub trait ExprVisitor<T> {
    fn visit_binary(&self, expr: &BinaryExpr) -> Result<T, LoxError>;
    fn visit_grouping(&self, expr: &GroupingExpr) -> Result<T, LoxError>;
    fn visit_literal(&self, expr: &LiteralExpr) -> Result<T, LoxError>;
    fn visit_unary(&self, expr: &UnaryExpr) -> Result<T, LoxError>;
}

impl BinaryExpr {
    fn accept<T>(&self, visitor: &dyn ExprVisitor<T>) -> Result<T, LoxError> {
        visitor.visit_binary(self)
    }
}

impl GroupingExpr {
    fn accept<T>(&self, visitor: &dyn ExprVisitor<T>) -> Result<T, LoxError> {
        visitor.visit_grouping(self)
    }
}

impl LiteralExpr {
    fn accept<T>(&self, visitor: &dyn ExprVisitor<T>) -> Result<T, LoxError> {
        visitor.visit_literal(self)
    }
}

impl UnaryExpr {
    fn accept<T>(&self, visitor: &dyn ExprVisitor<T>) -> Result<T, LoxError> {
        visitor.visit_unary(self)
    }
}
