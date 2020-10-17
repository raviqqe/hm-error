use crate::ast::*;
use crate::types::{Type, TypeScheme};
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct InferenceError;

pub fn infer_type_scheme(expression: &Expression) -> Result<TypeScheme, InferenceError> {
    let (_, type_) = infer(&Default::default(), expression)?;

    Ok(TypeScheme(type_.variables(), type_))
}

// TODO Fix the type of substitutions. Replace it with Vec<(usize, Type)>?
fn infer(
    environment: &HashMap<String, TypeScheme>,
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
                &Type::Function(argument_type.into(), result_type.clone().into()),
            )?);

            let result_type = result_type.substitute(&substitutions);

            (substitutions, result_type)
        }
        Expression::Lambda(variable, expression) => {
            let argument_type = Type::new_variable();

            let mut environment = environment.clone();
            environment.insert(
                variable.clone(),
                TypeScheme(Default::default(), argument_type.clone()),
            );

            let (substitutions, result_type) = infer(&environment, &expression)?;
            let function_type =
                Type::Function(argument_type.into(), result_type.into()).substitute(&substitutions);

            (substitutions, function_type)
        }
        Expression::Let(variable, bound_expression, expression) => {
            let mut environment = environment.clone();

            // Assume a concrete type of the variable for recursive types.
            environment.insert(
                variable.clone(),
                TypeScheme(Default::default(), Type::new_variable()),
            );

            let (mut substitutions, type_) = infer(&environment, &bound_expression)?;

            // Use the inferred type to construct the actual type which is possibly recursive.
            let type_scheme = TypeScheme(
                type_
                    .variables()
                    .difference(&calculate_free_variables_in_environment(&environment))
                    .cloned()
                    .collect(),
                type_,
            );

            environment.insert(variable.clone(), type_scheme);

            let (other_substitutions, type_) = infer(&environment, &expression)?;

            substitutions.extend(other_substitutions);

            (substitutions.clone(), type_.substitute(&substitutions))
        }
        Expression::Number(_) => (Default::default(), Type::Number),
        Expression::Variable(variable) => (
            Default::default(),
            environment.get(variable).ok_or(InferenceError)?.instance(),
        ),
    })
}

fn unify(one: &Type, other: &Type) -> Result<HashMap<usize, Type>, InferenceError> {
    Ok(match (one, other) {
        (Type::Variable(id), other) | (other, Type::Variable(id)) => {
            vec![(*id, other.clone())].into_iter().collect()
        }
        (Type::Number, Type::Number) => Default::default(),
        (
            Type::Function(one_argument, one_result),
            Type::Function(other_argument, other_result),
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

fn calculate_free_variables_in_environment(
    environment: &HashMap<String, TypeScheme>,
) -> HashSet<usize> {
    let mut variables = HashSet::new();

    for type_scheme in environment.values() {
        variables.extend(type_scheme.free_variables());
    }

    variables
}
