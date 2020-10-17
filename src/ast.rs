use std::fmt::{Display, Formatter};
use std::sync::Arc;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Expression {
    Application(Arc<Self>, Arc<Self>),
    Lambda(String, Arc<Self>),
    Let(String, Arc<Self>, Arc<Self>),
    Number(isize),
    Variable(String),
}

impl Display for Expression {
    fn fmt(&self, formatter: &mut Formatter) -> std::fmt::Result {
        match self {
            Self::Application(function, argument) => write!(formatter, "{} {}", function, argument),
            Self::Lambda(variable, expression) => {
                write!(formatter, "\\{}. {}", variable, expression)
            }
            Self::Let(variable, bound_expression, expression) => write!(
                formatter,
                "let {} = {} in {}",
                variable, bound_expression, expression
            ),
            Self::Number(number) => write!(formatter, "{}", number),
            Self::Variable(name) => write!(formatter, "{}", name),
        }
    }
}

pub fn app(
    function: impl Into<Arc<Expression>>,
    argument: impl Into<Arc<Expression>>,
) -> Expression {
    Expression::Application(function.into(), argument.into())
}

pub fn lambda(variable: impl Into<String>, expression: impl Into<Arc<Expression>>) -> Expression {
    Expression::Lambda(variable.into(), expression.into())
}

pub fn let_(
    variable: impl Into<String>,
    bound_expression: impl Into<Arc<Expression>>,
    expression: impl Into<Arc<Expression>>,
) -> Expression {
    Expression::Let(variable.into(), bound_expression.into(), expression.into())
}

pub fn num(number: isize) -> Expression {
    Expression::Number(number)
}

pub fn var(name: impl Into<String>) -> Expression {
    Expression::Variable(name.into())
}
