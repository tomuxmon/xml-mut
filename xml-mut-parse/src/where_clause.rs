use nom::{
    branch::alt,
    bytes::complete::{tag, tag_no_case, take_till},
    character::complete::{alpha1, alphanumeric1, digit1, multispace1},
    multi::separated_list1,
    IResult,
};

#[derive(Debug)]
pub struct ValueSelector {
    pub node_path: Vec<String>,
    pub ending: ValueSelectorEnding,
}

#[derive(Debug)]
pub enum ValueSelectorEnding {
    AttributeName(String),
    NodeText,
}

#[derive(Debug)]
pub enum Predicate {
    NodeExists(Vec<String>),
    ValueEquals(ValueSelector, String),
}

#[derive(Debug)]
pub struct WhereClause {
    pub where_word: String,
    pub predicates: Predicate,
}

pub fn value_selector_ending(s: &str) -> IResult<&str, ValueSelectorEnding> {
    // p@name or v@>text
    let (s, _) = tag("@")(s)?;
    let (s, value) = alt((alphanumeric1, tag(">text")))(s)?;
    
    Ok((
        s,
        if value == ">text" {
            ValueSelectorEnding::NodeText
        } else {
            ValueSelectorEnding::AttributeName(value.to_string())
        },
    ))
}

pub fn value_selector(s: &str) -> IResult<&str, ValueSelector> {
    let (s, path) = separated_list1(tag("/"), take_till(|c: char| !c.is_alphanumeric()))(s)?;
    let (s, ending) = value_selector_ending(s)?;

    Ok((
        s,
        ValueSelector {
            node_path: path.iter().map(|p| p.to_string()).collect(),
            ending,
        },
    ))
}
