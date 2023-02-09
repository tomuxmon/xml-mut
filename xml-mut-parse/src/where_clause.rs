use crate::prelude::*;
use nom::{
    bytes::complete::{tag, tag_no_case, take_till},
    character::complete::multispace1,
    combinator::opt,
    multi::separated_list1,
    IResult,
};

pub fn value_selector_ending(s: &str) -> IResult<&str, SelectorEnding> {
    // TODO: revisit and stabilize on maybe:
    // p@name or v@>text // current impl -> missing tail; also looks ugly as hell
    // p.name or v.text() or v.tail() // programmer friendly looks like property / method access
    // p/@name or p/text() or p/tail() // somewhar XPath complient
    let (s, _) = tag("@")(s)?;
    let (s, text_tag) = opt(tag(">text"))(s)?;

    Ok(if text_tag.is_some() {
        (s, SelectorEnding::NodeText)
    } else {
        let (s, attr_name) = take_till(|c: char| !c.is_alphanumeric())(s)?;
        (s, SelectorEnding::AttributeName(attr_name))
    })
}

pub fn value_selector(s: &str) -> IResult<&str, ValueSelector> {
    // TODO: revisit and stabilize on maybe:
    // some/path // current impl -> missing node index selector ; might be confusing since it is similar to XPath yet it is not
    // some[0]/path[1] // looks more like XPath and [0] could be ommited and minimized to current implementyed form
    // just use XPath // would require additional lib and owuld couple with presumably complicated XPath syntax (not needed here?)
    let (s, node_path) = separated_list1(tag("/"), take_till(|c: char| !c.is_alphanumeric()))(s)?;
    let (s, ending) = value_selector_ending(s)?;

    Ok((s, ValueSelector { node_path, ending }))
}

pub fn predicate_node_exists(s: &str) -> IResult<&str, PredicateNodeExists> {
    let (s, exists_word) = tag_no_case("exists")(s)?;
    let (s, _) = multispace1(s)?;
    let (s, node_path) = separated_list1(tag("/"), take_till(|c: char| !c.is_alphanumeric()))(s)?;

    Ok((
        s,
        PredicateNodeExists {
            exists_word,
            node_path,
        },
    ))
}

pub fn predicate_equals(s: &str) -> IResult<&str, PredicateEquals> {
    let (s, left_side) = value_selector(s)?;
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
