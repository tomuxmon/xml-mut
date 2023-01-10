use crate::prelude::*;
use nom::{
    bytes::complete::{tag_no_case, take_till},
    character::complete::{char, multispace1},
    combinator::opt,
    multi::separated_list1,
    sequence::delimited,
    IResult,
};

pub fn literal_quoted_string(s: &str) -> IResult<&str, String> {
    let (s, res) = delimited(char('\"'), take_till(|c| c == '\"'), char('\"'))(s)?;
    Ok((s, res.to_string()))
}

pub fn value_variant(s: &str) -> IResult<&str, ValueVariant> {
    let (s, maybe_p_node_exists) = opt(value_selector)(s)?;
    Ok(if let Some(p_node_exists) = maybe_p_node_exists {
        (s, ValueVariant::Selector(p_node_exists))
    } else {
        let (s, p_equals) = literal_quoted_string(s)?;
        (s, ValueVariant::LiteralString(p_equals))
    })
}

pub fn value_assignment(s: &str) -> IResult<&str, ValueAssignment> {
    let (s, left_side) = value_selector(s)?;
    let (s, _) = multispace1(s)?;
    let (s, _) = tag_no_case("=")(s)?;
    let (s, _) = multispace1(s)?;
    let (s, right_side) = value_variant(s)?;
    Ok((
        s,
        ValueAssignment {
            left_side,
            right_side,
        },
    ))
}

fn comma_surounded_mulispace1(s: &str) -> IResult<&str, &str> {
    let (s, _) = multispace1(s)?;
    let (s, and_word) = tag_no_case(",")(s)?;
    let (s, _) = multispace1(s)?;

    Ok((s, and_word))
}

pub fn set_statement(s: &str) -> IResult<&str, SetStatement> {
    let (s, set_word) = tag_no_case("set")(s)?;
    let (s, _) = multispace1(s)?;
    let (s, assignments) = separated_list1(comma_surounded_mulispace1, value_assignment)(s)?;
    Ok((
        s,
        SetStatement {
            set_word: set_word.to_string(),
            assignments,
        },
    ))
}
