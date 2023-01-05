use nom::{
    bytes::complete::{tag, tag_no_case, take_till},
    character::complete::multispace1,
    multi::separated_list1,
    IResult,
};

#[derive(Debug)]
pub struct NodeSelector {
    pub path: Vec<String>,
    pub as_word: String,
    pub alias: String,
}

#[derive(Debug)]
pub struct GetStatement {
    pub get_word: String,
    pub node_selector: NodeSelector,
}

pub fn node_selector(s: &str) -> IResult<&str, NodeSelector> {
    let (s, path) = separated_list1(tag("/"), take_till(|c: char| !c.is_alphanumeric()))(s)?;
    let (s, _) = multispace1(s)?;
    let (s, as_word) = tag_no_case("as")(s)?;
    let (s, _) = multispace1(s)?;
    let (s, alias) = take_till(|c: char| !c.is_alphanumeric())(s)?;

    Ok((
        s,
        NodeSelector {
            path: path.iter().map(|p| p.to_string()).collect(),
            as_word: as_word.to_string(),
            alias: alias.to_string(),
        },
    ))
}

pub fn get_statement(s: &str) -> IResult<&str, GetStatement> {
    let (s, get_word) = tag_no_case("get")(s)?;
    let (s, _) = multispace1(s)?;
    let (s, node_selector) = node_selector(s)?;

    Ok((
        s,
        (GetStatement {
            get_word: get_word.to_string(),
            node_selector,
        }),
    ))
}
