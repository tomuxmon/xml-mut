use crate::prelude::*;

#[test]
fn parse_value_selector_path1() {
    let fragment = "žomb/tronš@>text";
    let (_, b) = value_selector(fragment).expect("could not parse value selector");
    assert_eq!(b.ending, ValueSelectorEnding::NodeText);
    assert_eq!(b.node_path.len(), 2);
    assert_eq!(b.node_path[0], "žomb".to_string());
    assert_eq!(b.node_path[1], "tronš");
}

#[test]
fn parse_value_selector_path2() {
    let fragment = "r/tron@morka";
    let (_, b) = value_selector(fragment).expect("could not parse value selector");
    assert_eq!(b.ending, ValueSelectorEnding::AttributeName("morka".to_string()));
    assert_eq!(b.node_path.len(), 2);
    assert_eq!(b.node_path[0], "r".to_string());
    assert_eq!(b.node_path[1], "tron");
}

#[test]
fn parse_value_selector_ending_1() {
    let fragment = "@>text";
    let (_, b) = value_selector_ending(fragment).expect("could not parse node text value selector ending");
    assert_eq!(b, ValueSelectorEnding::NodeText);
}

#[test]
fn parse_value_selector_ending_2() {
    let fragment = "@version";
    let (_, b) = value_selector_ending(fragment).expect("could not parse attribute name value selector ending");
    assert_eq!(b, ValueSelectorEnding::AttributeName("version".to_string()));
}
