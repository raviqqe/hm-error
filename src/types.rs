use std::collections::{HashMap, HashSet};
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

    pub fn variables(&self) -> HashSet<usize> {
        match self {
            Self::Function(argument_type, result_type) => {
                let mut variables = argument_type.variables();
                variables.extend(result_type.variables());
                variables
            }
            Self::Number => Default::default(),
            Self::Variable(id) => vec![*id].into_iter().collect(),
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TypeScheme(pub HashSet<usize>, pub Type);

impl TypeScheme {
    pub fn free_variables(&self) -> HashSet<usize> {
        let TypeScheme(varaiables, type_) = self;

        type_.variables().difference(varaiables).cloned().collect()
    }

    pub fn instance(&self) -> Type {
        let TypeScheme(variables, type_) = self;

        type_.substitute(
            &variables
                .iter()
                .map(|id| (*id, Type::new_variable()))
                .collect(),
        )
    }
}

impl Display for TypeScheme {
    fn fmt(&self, formatter: &mut Formatter) -> std::fmt::Result {
        let TypeScheme(variables, type_) = self;

        if variables.is_empty() {
            write!(formatter, "{}", type_)
        } else {
            write!(
                formatter,
                "\\{}. {}",
                variables
                    .iter()
                    .map(|id| format!("{}", Type::Variable(*id)))
                    .collect::<Vec<_>>()
                    .join(", "),
                type_
            )
        }
    }
}
