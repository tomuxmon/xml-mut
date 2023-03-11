use crate::get_statement::node_path;
use nom::{bytes::complete::tag_no_case, character::complete::multispace1, IResult};
use xml_mut_data::DeleteClause;

pub fn delete_statement(s: &str) -> IResult<&str, DeleteClause> {
    let (s, delete_word) = tag_no_case("delete")(s)?;
    let (s, _) = multispace1(s)?;
    let (s, node_path) = node_path(s)?;

    Ok((
        s,
        DeleteClause {
            delete_word,
            node_path,
        },
    ))
}
