use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::sync::Arc;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Type {
    Function(Arc<Type>, Arc<Type>),
    Number,
    Variable(usize),
}

impl Type {
    pub fn new_variable() -> Type {
        Type::Variable(rand::random())
    }

    pub fn substitute(&self, substitutions: &HashMap<usize, Type>) -> Type {
        match self {
            Self::Function(argument_type, result_type) => Self::Function(
                argument_type.substitute(substitutions).into(),
                result_type.substitute(substitutions).into(),
            ),
            Self::Number => Self::Number,
            Self::Variable(id) => substitutions.get(id).unwrap_or_else(|| self).clone(),
        }
    }
}

impl Display for Type {
    fn fmt(&self, formatter: &mut Formatter) -> std::fmt::Result {
        match self {
            Self::Function(argument, result) => write!(formatter, "{} -> {}", argument, result),
            Self::Number => write!(formatter, "Number"),
            Self::Variable(id) => write!(formatter, "<{}>", &format!("{:04x}", id)[..4]),
        }
    }
}
