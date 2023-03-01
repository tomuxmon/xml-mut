use crate::prelude::*;
use nom::{
    bytes::complete::{tag, take_until},
    character::complete::{multispace0, multispace1},
    combinator::opt,
    multi::separated_list1,
    sequence::delimited,
    IResult,
};
use xml_mut_data::Statement;

fn block_comment(s: &str) -> IResult<&str, &str> {
    let (s, comment) = delimited(tag("/*"), take_until("*/"), tag("*/"))(s)?;
    Ok((s, comment))
}

// TODO: impl line comment

pub fn statement(s: &str) -> IResult<&str, Statement> {
    let (s, comment) = opt(block_comment)(s)?;
    if let Some(comment) = comment {
        return Ok((s, Statement::Comment(comment)));
    }
    let (s, res) = mutation(s)?;
    Ok((s, Statement::Mutation(res)))
}

pub fn statements(s: &str) -> IResult<&str, Vec<Statement>> {
    let (s, _) = multispace0(s)?;
    let (s, res) = separated_list1(multispace1, statement)(s)?;
    let (s, _) = multispace0(s)?;
    Ok((s, res))
}
