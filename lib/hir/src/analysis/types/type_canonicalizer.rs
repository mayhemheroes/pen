use super::{type_resolver, TypeError};
use crate::types::*;
use std::collections::{BTreeMap, BTreeSet};

// Canonicalize a type deeply.
pub fn canonicalize(type_: &Type, types: &BTreeMap<String, Type>) -> Result<Type, TypeError> {
    Ok(match &type_ {
        Type::Function(function) => Function::new(
            function
                .arguments()
                .iter()
                .map(|type_| canonicalize(type_, types))
                .collect::<Result<_, _>>()?,
            canonicalize(function.result(), types)?,
            function.position().clone(),
        )
        .into(),
        Type::List(list) => List::new(
            canonicalize(list.element(), types)?,
            list.position().clone(),
        )
        .into(),
        Type::Union(union) => canonicalize_union(union, types)?,
        Type::Any(_)
        | Type::Boolean(_)
        | Type::Record(_)
        | Type::None(_)
        | Type::Number(_)
        | Type::String(_) => type_.clone(),
        Type::Reference(reference) => {
            canonicalize(&type_resolver::resolve(reference, types)?, types)?
        }
    })
}

pub fn canonicalize_function(
    type_: &Type,
    types: &BTreeMap<String, Type>,
) -> Result<Option<Function>, TypeError> {
    Ok(canonicalize(type_, types)?.into_function())
}

pub fn canonicalize_list(
    type_: &Type,
    types: &BTreeMap<String, Type>,
) -> Result<Option<List>, TypeError> {
    Ok(canonicalize(type_, types)?.into_list())
}

pub fn canonicalize_record(
    type_: &Type,
    types: &BTreeMap<String, Type>,
) -> Result<Option<Record>, TypeError> {
    Ok(canonicalize(type_, types)?.into_record())
}

fn canonicalize_union(union: &Union, types: &BTreeMap<String, Type>) -> Result<Type, TypeError> {
    Ok(collect_types(&union.clone().into(), types)?
        .into_iter()
        .reduce(|one, other| {
            if one.is_any() {
                one
            } else if other.is_any() {
                other
            } else {
                Union::new(one, other, union.position().clone()).into()
            }
        })
        .unwrap())
}

fn collect_types(
    type_: &Type,
    types: &BTreeMap<String, Type>,
) -> Result<BTreeSet<Type>, TypeError> {
    Ok(match type_ {
        Type::Union(union) => collect_types(union.lhs(), types)?
            .into_iter()
            .chain(collect_types(union.rhs(), types)?)
            .collect(),
        Type::Any(_)
        | Type::Boolean(_)
        | Type::Function(_)
        | Type::Record(_)
        | Type::List(_)
        | Type::None(_)
        | Type::Number(_)
        | Type::String(_) => vec![canonicalize(type_, types)?].into_iter().collect(),
        Type::Reference(reference) => {
            collect_types(&type_resolver::resolve(reference, types)?, types)?
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use position::{test::PositionFake, Position};

    #[test]
    fn canonicalize_number() {
        assert_eq!(
            canonicalize(&Number::new(Position::fake()).into(), &Default::default(),),
            Ok(Number::new(Position::fake()).into())
        );
    }

    #[test]
    fn canonicalize_union_of_numbers() {
        assert_eq!(
            canonicalize(
                &Union::new(
                    Number::new(Position::fake()),
                    Number::new(Position::fake()),
                    Position::fake()
                )
                .into(),
                &Default::default(),
            ),
            Ok(Number::new(Position::fake()).into())
        );
    }

    #[test]
    fn canonicalize_union_of_3_types() {
        assert_eq!(
            canonicalize(
                &Union::new(
                    Number::new(Position::fake()),
                    Union::new(
                        Boolean::new(Position::fake()),
                        None::new(Position::fake()),
                        Position::fake()
                    ),
                    Position::fake()
                )
                .into(),
                &Default::default(),
            ),
            Ok(Union::new(
                Union::new(
                    Boolean::new(Position::fake()),
                    None::new(Position::fake()),
                    Position::fake()
                ),
                Number::new(Position::fake()),
                Position::fake()
            )
            .into())
        );
    }

    #[test]
    fn canonicalize_union_of_function_argument() {
        assert_eq!(
            canonicalize(
                &Function::new(
                    vec![Union::new(
                        Number::new(Position::fake()),
                        Number::new(Position::fake()),
                        Position::fake()
                    )
                    .into()],
                    None::new(Position::fake()),
                    Position::fake(),
                )
                .into(),
                &Default::default(),
            ),
            Ok(Function::new(
                vec![Number::new(Position::fake()).into()],
                None::new(Position::fake()),
                Position::fake(),
            )
            .into())
        );
    }

    #[test]
    fn canonicalize_union_of_function_result() {
        assert_eq!(
            canonicalize(
                &Function::new(
                    vec![],
                    Union::new(
                        Number::new(Position::fake()),
                        Number::new(Position::fake()),
                        Position::fake()
                    ),
                    Position::fake(),
                )
                .into(),
                &Default::default(),
            ),
            Ok(Function::new(vec![], Number::new(Position::fake()), Position::fake(),).into())
        );
    }

    #[test]
    fn canonicalize_union_of_list_element() {
        assert_eq!(
            canonicalize(
                &List::new(
                    Union::new(
                        Number::new(Position::fake()),
                        Number::new(Position::fake()),
                        Position::fake()
                    ),
                    Position::fake(),
                )
                .into(),
                &Default::default(),
            ),
            Ok(List::new(Number::new(Position::fake()), Position::fake(),).into())
        );
    }

    #[test]
    fn canonicalize_union_with_any() {
        assert_eq!(
            canonicalize(
                &Union::new(
                    Number::new(Position::fake()),
                    Any::new(Position::fake()),
                    Position::fake()
                )
                .into(),
                &Default::default(),
            ),
            Ok(Any::new(Position::fake()).into())
        );
    }

    #[test]
    fn canonicalize_reference() {
        assert_eq!(
            canonicalize(
                &Reference::new("t", Position::fake()).into(),
                &vec![("t".into(), Number::new(Position::fake()).into())]
                    .into_iter()
                    .collect(),
            ),
            Ok(Number::new(Position::fake()).into())
        );
    }

    #[test]
    fn canonicalize_union_in_function_in_union() {
        assert_eq!(
            canonicalize(
                &Union::new(
                    Function::new(
                        vec![],
                        Union::new(
                            None::new(Position::fake()),
                            None::new(Position::fake()),
                            Position::fake()
                        ),
                        Position::fake()
                    ),
                    None::new(Position::fake()),
                    Position::fake()
                )
                .into(),
                &vec![("t".into(), Number::new(Position::fake()).into())]
                    .into_iter()
                    .collect(),
            ),
            Ok(Union::new(
                Function::new(vec![], None::new(Position::fake()), Position::fake()),
                None::new(Position::fake()),
                Position::fake()
            )
            .into())
        );
    }
}