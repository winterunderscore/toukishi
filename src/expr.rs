use super::lexer::{ Token, Literal };

pub trait Visitor<T> {
	fn visit_binary(&mut self, left: &Expr, operator: &Token, right: &Expr) -> T;
	fn visit_grouping(&mut self, expression: &Expr) -> T;
	fn visit_literal(&mut self, value: &Literal) -> T;
	fn visit_unary(&mut self, operator: &Token, right: &Expr) -> T;
}

pub trait Acceptor<T> {
	fn accept(&self, visitor: &mut dyn Visitor<T>) -> T;
}

pub enum Expr {
	Binary {
		left: Box<Expr>,
		operator: Token,
		right: Box<Expr>,
	},
	Grouping {
		expression: Box<Expr>,
	},
	Literal {
		value: Literal,
	},
	Unary {
		operator: Token,
		right: Box<Expr>,
	},
}

impl <T> Acceptor<T> for Expr {
	fn accept(&self, visitor: &mut dyn Visitor<T>) -> T {
		match self {
			Expr::Binary { left, operator, right } => visitor.visit_binary(left, operator, right),
			Expr::Grouping { expression } => visitor.visit_grouping(expression),
			Expr::Literal { value } => visitor.visit_literal(value),
			Expr::Unary { operator, right } => visitor.visit_unary(operator, right),
		}
	}
}
