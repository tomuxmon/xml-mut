use crate::{prelude::comma_surounded_mulispace01, where_clause::path_variant};
use nom::{
    bytes::complete::tag_no_case, character::complete::multispace1, multi::separated_list1, IResult,
};
use xml_mut_data::DeleteClause;

pub fn delete_statement(s: &str) -> IResult<&str, DeleteClause> {
    let (s, delete_word) = tag_no_case("delete")(s)?;
    let (s, _) = multispace1(s)?;
    let (s, targets) = separated_list1(comma_surounded_mulispace01, path_variant)(s)?;

    Ok((
        s,
        DeleteClause {
            delete_word,
            targets,
        },
    ))
}
