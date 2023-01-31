use crate::prelude::*;

#[test]
fn parse_value_selector_path1() {

    // TODO: value selector could also include descendant node index to pick:
    // zomb[0]/tron[0]@>text
    // TODO: using value selector with no index should result in error when 
    // more than one descendant is found
    // tron@>text -> error when more then one tron sub node exists
    // OR "tron@>text" should be considered equivalent to "tron[0]@>text"

    let fragment = "žomb/tronš@>text";
    let (_, b) = value_selector(fragment).expect("could not parse value selector");
    assert_eq!(b.ending, SelectorEnding::NodeText);
    assert_eq!(b.node_path.len(), 2);
    assert_eq!(b.node_path[0], "žomb");
    assert_eq!(b.node_path[1], "tronš");
}

#[test]
fn parse_value_selector_path2() {
    let fragment = "r/tron@morka";
    let (_, b) = value_selector(fragment).expect("could not parse value selector");
    assert_eq!(
        b.ending,
        SelectorEnding::AttributeName("morka")
    );
    assert_eq!(b.node_path.len(), 2);
    assert_eq!(b.node_path[0], "r");
    assert_eq!(b.node_path[1], "tron");
}

#[test]
fn parse_value_selector_ending_1() {
    let fragment = "@>text";
    let (_, b) =
        value_selector_ending(fragment).expect("could not parse node text value selector ending");
    assert_eq!(b, SelectorEnding::NodeText);
}

#[test]
fn parse_value_selector_ending_2() {
    let fragment = "@version";
    let (_, b) = value_selector_ending(fragment)
        .expect("could not parse attribute name value selector ending");
    assert_eq!(b, SelectorEnding::AttributeName("version"));
}

#[test]
fn parse_predicate_node_exists_1() {
    let fragment = "exists version";
    let (_, b) = predicate_node_exists(fragment).expect("could not parse predicate node exists");
    assert_eq!(b.exists_word, "exists");
    assert_eq!(b.node_path.len(), 1);
    assert_eq!(b.node_path[0], "version");
}

#[test]
fn parse_predicate_node_exists_2() {
    let fragment = "exists ItemGroup/ReferenceŲ/verŠion";
    let (_, b) = predicate_node_exists(fragment).expect("could not parse predicate node exists");
    assert_eq!(b.exists_word, "exists");
    assert_eq!(b.node_path.len(), 3);
    assert_eq!(b.node_path[0], "ItemGroup");
    assert_eq!(b.node_path[1], "ReferenceŲ");
    assert_eq!(b.node_path[2], "verŠion");
}

#[test]
fn parse_predicate_equals_1() {
    let fragment = "r/tron@morka == baranka";
    let (_, b) = predicate_equals(fragment).expect("could not parse predicate node exists");
    assert_eq!(b.left_side.node_path.len(), 2);
    assert_eq!(b.left_side.node_path[0], "r");
    assert_eq!(b.left_side.node_path[1], "tron");
    assert_eq!(
        b.left_side.ending,
        SelectorEnding::AttributeName("morka")
    );
    assert_eq!(b.right_side, "baranka");
}

#[test]
fn parse_predicate_1() {
    let fragment = "r/tron@morka == baranka";
    let (_, b) = predicate(fragment).expect("could not parse predicate node exists");
    if let Predicate::Equals(_) = b {
    } else {
        panic!("could not parse predicate equals");
    }
}

#[test]
fn parse_predicate_2() {
    let fragment = "exists r/tron";
    let (_, b) = predicate(fragment).expect("could not parse predicate node exists");
    if let Predicate::NodeExists(_) = b {
    } else {
        panic!("could not parse predicate node exists");
    }
}

#[test]
fn parse_where_clause_1() {
    let fragment = "where exists r/tron and exists morka and r/tron@morka == baranka";
    let (_, w) = where_clause(fragment).expect("could not parse where clause");
    assert_eq!(w.where_word, "where");
    assert_eq!(w.predicates.len(), 3);
}

#[test]
fn parse_where_clause_2() {
    let fragment = "WhErE r/tron@morka == baranka";
    let (_, w) = where_clause(fragment).expect("could not parse where clause");
    assert_eq!(w.where_word, "WhErE");
    assert_eq!(w.predicates.len(), 1);
}