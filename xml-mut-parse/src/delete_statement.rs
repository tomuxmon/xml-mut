use crate::prelude::*;
use nom::{
    bytes::complete::{tag, tag_no_case, take_till},
    character::complete::multispace1,
    multi::separated_list1,
    IResult,
};

pub fn delete_statement(s: &str) -> IResult<&str, DeleteStatement> {
    let (s, delete_word) = tag_no_case("delete")(s)?;
    let (s, _) = multispace1(s)?;
    let (s, node_path) = separated_list1(tag("/"), take_till(|c: char| !c.is_alphanumeric()))(s)?;

    Ok((
        s,
        DeleteStatement {
            delete_word: delete_word.to_string(),
            node_path: node_path.iter().map(|p| p.to_string()).collect(),
        },
    ))
}
