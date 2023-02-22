use crate::get_statement::node_path;
use nom::{bytes::complete::tag_no_case, character::complete::multispace1, IResult};
use xml_mut_data::DeleteStatement;

pub fn delete_statement(s: &str) -> IResult<&str, DeleteStatement> {
    let (s, delete_word) = tag_no_case("delete")(s)?;
    let (s, _) = multispace1(s)?;
    let (s, node_path) = node_path(s)?;

    Ok((
        s,
        DeleteStatement {
            delete_word,
            node_path,
        },
    ))
}
