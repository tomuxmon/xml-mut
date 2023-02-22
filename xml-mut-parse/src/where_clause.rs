use crate::get_statement::node_path;
use nom::{
    bytes::complete::{tag, tag_no_case, take_till},
    character::complete::multispace1,
    combinator::opt,
    multi::separated_list1,
    IResult,
};
use xml_mut_data::{
    Predicate, PredicateEquals, PredicateNodeExists, ValuePath, ValueSource, WhereClause,
};

pub fn value_source(s: &str) -> IResult<&str, ValueSource> {
    let (s, _) = tag("[")(s)?;
    let (s, at) = opt(tag("@"))(s)?;
    let (s, name) = take_till(|c: char| c == ']')(s)?;
    let (s, _) = tag("]")(s)?;

    Ok(if at.is_some() {
        (s, ValueSource::Attribute(name))
    } else {
        match name {
            "text" => (s, ValueSource::Text),
            "tail" => (s, ValueSource::Tail),
            _ => {
                return Err(nom::Err::Error(nom::error::Error {
                    code: nom::error::ErrorKind::Tag,
                    input: name,
                }))
            }
        }
    })
}

pub fn value_path(s: &str) -> IResult<&str, ValuePath> {
    let (s, node_path) = node_path(s)?;
    let (s, source) = value_source(s)?;
    Ok((s, ValuePath { node_path, source }))
}

pub fn predicate_node_exists(s: &str) -> IResult<&str, PredicateNodeExists> {
    let (s, exists_word) = tag_no_case("exists")(s)?;
    let (s, _) = multispace1(s)?;
    let (s, node_path) = node_path(s)?;

    Ok((
        s,
        PredicateNodeExists {
            exists_word,
            node_path,
        },
    ))
}

pub fn predicate_equals(s: &str) -> IResult<&str, PredicateEquals> {
    let (s, left_side) = value_path(s)?;
    let (s, _) = multispace1(s)?;
    let (s, _) = tag("==")(s)?;
    let (s, _) = multispace1(s)?;
    let (s, right_side) = take_till(|c: char| !c.is_alphanumeric())(s)?;

    Ok((
        s,
        PredicateEquals {
            left_side,
            right_side,
        },
    ))
}

pub fn predicate(s: &str) -> IResult<&str, Predicate> {
    let (s, maybe_p_node_exists) = opt(predicate_node_exists)(s)?;

    Ok(if let Some(p_node_exists) = maybe_p_node_exists {
        (s, Predicate::NodeExists(p_node_exists))
    } else {
        let (s, p_equals) = predicate_equals(s)?;
        (s, Predicate::Equals(p_equals))
    })
}

fn and_surounded_mulispace1(s: &str) -> IResult<&str, &str> {
    let (s, _) = multispace1(s)?;
    let (s, and_word) = tag_no_case("and")(s)?;
    let (s, _) = multispace1(s)?;

    Ok((s, and_word))
}

pub fn where_clause(s: &str) -> IResult<&str, WhereClause> {
    let (s, where_word) = tag_no_case("where")(s)?;
    let (s, _) = multispace1(s)?;
    let (s, predicates) = separated_list1(and_surounded_mulispace1, predicate)(s)?;

    Ok((
        s,
        WhereClause {
            where_word,
            predicates,
        },
    ))
}
