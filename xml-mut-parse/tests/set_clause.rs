use xml_mut_data::{ValueSource, ValueVariant};
use xml_mut_parse::prelude::*;

#[test]
fn parse_literal_quoted_string() {
    let fragment = "\"lso  g123645789**--// lomon\"";
    let (_, b) = literal_quoted_string(fragment).expect("could not parse quoted string");
    assert_eq!(b, "lso  g123645789**--// lomon");
}

#[test]
fn parse_value_variant_1() {
    let fragment = "r/tron[@morka]";
    let (_, b) = value_variant(fragment).expect("could not parse value variant");
    if let ValueVariant::Selector(_) = b {
    } else {
        panic!("could not parse value variant selector");
    }
}

#[test]
fn parse_value_assignment_1() {
    let fragment = "r/tron[@morka] = \"true\"";
    let (_, b) = value_assignment(fragment).expect("could not parse value assignment");
    assert_eq!(b.target.source, ValueSource::Attribute("morka"));
    assert_eq!(b.target.node_path.len(), 2);
    assert_eq!(b.target.node_path[0], "r");
    assert_eq!(b.target.node_path[1], "tron");
    assert_eq!(b.source, ValueVariant::LiteralString("true"));
}

#[test]
fn parse_value_assignment_2() {
    let fragment = "r/tron[@morka] = r/balbon[text]";
    let (_, b) = value_assignment(fragment).expect("could not parse value assignment");
    assert_eq!(b.target.source, ValueSource::Attribute("morka"));
    assert_eq!(b.target.node_path.len(), 2);
    assert_eq!(b.target.node_path[0], "r");
    assert_eq!(b.target.node_path[1], "tron");
    if let ValueVariant::Selector(_) = b.source {
    } else {
        panic!("could not parse value assignment right side as a value selector");
    }
}

#[test]
fn parse_set_statement_1() {
    // instead: SET [@Version] = Version[text]
    let fragment = "SET r/gho[text] = r/tron[@morka], r/tron[@morka] = \"true\"";
    let (rem, b) = set_clause(fragment).expect("could not parse set statement");
    assert_eq!(rem, "");
    assert_eq!(b.set_word, "SET".to_string());
    assert_eq!(b.assignments.len(), 2);
}

//
#[test]
fn parse_set_statement_2() {
    let fragment = "set [@Version] = Version[text]";
    let (rem, b) = set_clause(fragment).expect("could not parse set statement");
    assert_eq!(rem, "");
    assert_eq!(b.set_word, "set".to_string());
    assert_eq!(b.assignments.len(), 1);
}

#[test]
fn parse_set_statement_3() {
    let fragment = "SET Version[text] = [@Version]";
    let (rem, b) = set_clause(fragment).expect("could not parse set statement");
    assert_eq!(rem, "");
    assert_eq!(b.set_word, "SET".to_string());
    assert_eq!(b.assignments.len(), 1);
}
