use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::sync::Arc;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Type {
    Raw(RawType, bool),
    Variable(usize),
}

impl Type {
    pub fn new_variable() -> Type {
        Self::Variable(rand::random())
    }

    pub fn substitute(&self, substitutions: &HashMap<usize, Type>) -> Type {
        match self {
            Self::Raw(type_, error) => Self::Raw(type_.substitute(substitutions), *error),
            Self::Variable(id) => substitutions.get(id).unwrap_or_else(|| self).clone(),
        }
    }
}

impl Display for Type {
    fn fmt(&self, formatter: &mut Formatter) -> std::fmt::Result {
        match self {
            Self::Raw(type_, error) => {
                if *error {
                    write!(formatter, "({})?", type_)
                } else {
                    write!(formatter, "{}", type_)
                }
            }
            Self::Variable(id) => write!(formatter, "<{}>", &format!("{:04x}", id)[..4]),
        }
    }
}

impl From<RawType> for Type {
    fn from(type_: RawType) -> Self {
        Self::Raw(type_, false)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RawType {
    Error,
    Function(Arc<Type>, Arc<Type>),
    Number,
}

impl RawType {
    pub fn substitute(&self, substitutions: &HashMap<usize, Type>) -> Self {
        match self {
            Self::Error => Self::Error,
            Self::Function(argument_type, result_type) => Self::Function(
                argument_type.substitute(substitutions).into(),
                result_type.substitute(substitutions).into(),
            ),
            Self::Number => Self::Number,
        }
    }
}

impl Display for RawType {
    fn fmt(&self, formatter: &mut Formatter) -> std::fmt::Result {
        match self {
            Self::Error => write!(formatter, "Error"),
            Self::Function(argument, result) => write!(formatter, "{} -> {}", argument, result),
            Self::Number => write!(formatter, "Number"),
        }
    }
}
