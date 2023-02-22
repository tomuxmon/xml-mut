use nom::{
    bytes::complete::{tag, tag_no_case, take_till},
    character::complete::multispace1,
    multi::separated_list1,
    IResult,
};
use xml_mut_data::{GetStatement, NodePath};

// TODO: instead of alphanumeric use standard defined characters
// https://www.w3.org/TR/xml/#NT-NameStartChar
pub fn node_path(s: &str) -> IResult<&str, NodePath> {
    let (s, path) = separated_list1(tag("/"), take_till(|c: char| !c.is_alphanumeric()))(s)?;
    Ok((s, NodePath { path }))
}

pub fn get_statement(s: &str) -> IResult<&str, GetStatement> {
    let (s, get_word) = tag_no_case("get")(s)?;
    let (s, _) = multispace1(s)?;
    let (s, node_selector) = node_path(s)?;

    Ok((
        s,
        (GetStatement {
            get_word,
            node_selector,
        }),
    ))
}
