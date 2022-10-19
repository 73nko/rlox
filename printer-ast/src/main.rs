use rlox::{
    error::LoxError,
    expr::{BinaryExpr, Expr, ExprVisitor, GroupingExpr, LiteralExpr, UnaryExpr},
    token::{Object, Token},
    token_type::TokenType,
};

struct AstPrinter;

impl AstPrinter {
    fn print(&self, expr: &Expr) -> Result<String, LoxError> {
        expr.accept(self)
    }

    fn parenthesize(&self, name: &str, exprs: &[&Expr]) -> Result<String, LoxError> {
        let mut builder = name.to_string();

        for expr in exprs {
            builder = format!("{builder} ({})", expr.accept(self)?);
        }

        Ok(builder)
    }
}

impl ExprVisitor<String> for AstPrinter {
    fn visit_binary_expr(self: &AstPrinter, expr: &BinaryExpr) -> Result<String, LoxError> {
        self.parenthesize(&expr.operator.lexeme, &[&expr.left, &expr.right])
    }

    fn visit_grouping_expr(self: &AstPrinter, expr: &GroupingExpr) -> Result<String, LoxError> {
        self.parenthesize("group", &[&expr.expression])
    }

    fn visit_literal_expr(self: &AstPrinter, expr: &LiteralExpr) -> Result<String, LoxError> {
        if let Some(value) = &expr.value {
            Ok(value.to_string())
        } else {
            Ok("nil".to_string())
        }
    }

    fn visit_unary_expr(self: &AstPrinter, expr: &UnaryExpr) -> Result<String, LoxError> {
        self.parenthesize(&expr.operator.lexeme, &[&expr.right])
    }
}

fn main() {
    let expression = Expr::Binary(BinaryExpr {
        left: Box::new(Expr::Unary(UnaryExpr {
            operator: Token {
                ttype: TokenType::Minus,
                lexeme: "-".to_string(),
                literal: None,
                line: 1,
            },
            right: Box::new(Expr::Literal(LiteralExpr {
                value: Some(Object::Num(123.0)),
            })),
        })),
        operator: Token {
            ttype: TokenType::Star,
            lexeme: "*".to_string(),
            literal: None,
            line: 1,
        },
        right: Box::new(Expr::Grouping(GroupingExpr {
            expression: Box::new(Expr::Literal(LiteralExpr {
                value: Some(Object::Num(45.67)),
            })),
        })),
    });

    let printer = AstPrinter {};

    let result = printer.print(&expression).unwrap();

    print!("{}", result);
}
