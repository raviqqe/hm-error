use crate::ast::*;
use crate::types::{RawType, Type};
use std::collections::HashMap;

#[derive(Debug)]
pub struct InferenceError;

pub fn infer_type(expression: &Expression) -> Result<Type, InferenceError> {
    let (_, type_) = infer(&Default::default(), expression)?;

    Ok(type_)
}

// TODO Fix the type of substitutions. Replace it with Vec<(usize, Type)>?
fn infer(
    environment: &HashMap<String, Type>,
    expression: &Expression,
) -> Result<(HashMap<usize, Type>, Type), InferenceError> {
    Ok(match expression {
        Expression::Application(function, argument) => {
            let (mut substitutions, function_type) = infer(&environment, &function)?;
            let (other_substitutions, argument_type) = infer(&environment, &argument)?;

            substitutions.extend(other_substitutions);

            let result_type = Type::new_variable();

            substitutions.extend(unify(
                &function_type,
                &RawType::Function(argument_type.into(), result_type.clone().into()).into(),
            )?);

            let result_type = result_type.substitute(&substitutions);

            (substitutions, result_type)
        }
        Expression::Lambda(variable, expression) => {
            let argument_type = Type::new_variable();

            let mut environment = environment.clone();
            environment.insert(variable.clone(), argument_type.clone());

            let (substitutions, result_type) = infer(&environment, &expression)?;
            let function_type = RawType::Function(argument_type.into(), result_type.into())
                .substitute(&substitutions);

            (substitutions, function_type.into())
        }
        Expression::Let(variable, bound_expression, expression) => {
            let (mut substitutions, type_) = infer(&environment, &bound_expression)?;

            let mut environment = environment.clone();
            environment.insert(variable.clone(), type_);

            let (other_substitutions, type_) = infer(&environment, &expression)?;

            substitutions.extend(other_substitutions);
            (substitutions.clone(), type_.substitute(&substitutions))
        }
        Expression::Number(_) => (Default::default(), RawType::Number.into()),
        Expression::Variable(variable) => (
            Default::default(),
            environment.get(variable).ok_or(InferenceError)?.clone(),
        ),
    })
}

fn unify(one: &Type, other: &Type) -> Result<HashMap<usize, Type>, InferenceError> {
    Ok(match (one, other) {
        (Type::Variable(id), other) | (other, Type::Variable(id)) => {
            vec![(*id, other.clone())].into_iter().collect()
        }
        (Type::Raw(RawType::Number, _), Type::Raw(RawType::Number, _)) => Default::default(),
        (
            Type::Raw(RawType::Function(one_argument, one_result), _),
            Type::Raw(RawType::Function(other_argument, other_result), _),
        ) => {
            let mut substitutions = unify(one_argument, other_argument)?;

            substitutions.extend(unify(
                &one_result.substitute(&substitutions),
                &other_result.substitute(&substitutions),
            )?);

            substitutions
        }
        _ => return Err(InferenceError),
    })
}
