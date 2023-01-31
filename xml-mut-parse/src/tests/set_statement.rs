use crate::prelude::*;

#[test]
fn parse_literal_quoted_string() {
    let fragment = "\"lso  g123645789**--// lomon\"";
    let (_, b) = literal_quoted_string(fragment).expect("could not parse quoted string");
    assert_eq!(b, "lso  g123645789**--// lomon");
}

#[test]
fn parse_value_variant_1() {
    let fragment = "r/tron@morka";
    let (_, b) = value_variant(fragment).expect("could not parse value variant");
    if let ValueVariant::Selector(_) = b {
    } else {
        panic!("could not parse value variant selector");
    }
}

#[test]
fn parse_value_assignment_1() {
    let fragment = "r/tron@morka = \"true\"";
    let (_, b) = value_assignment(fragment).expect("could not parse value assignment");
    assert_eq!(
        b.left_side.ending,
        SelectorEnding::AttributeName("morka")
    );
    assert_eq!(b.left_side.node_path.len(), 2);
    assert_eq!(b.left_side.node_path[0], "r");
    assert_eq!(b.left_side.node_path[1], "tron");
    assert_eq!(
        b.right_side,
        ValueVariant::LiteralString("true")
    );
}

#[test]
fn parse_value_assignment_2() {
    let fragment = "r/tron@morka = r/balbon@>text";
    let (_, b) = value_assignment(fragment).expect("could not parse value assignment");
    assert_eq!(
        b.left_side.ending,
        SelectorEnding::AttributeName("morka")
    );
    assert_eq!(b.left_side.node_path.len(), 2);
    assert_eq!(b.left_side.node_path[0], "r");
    assert_eq!(b.left_side.node_path[1], "tron");
    if let ValueVariant::Selector(_) = b.right_side {
    } else {
        panic!("could not parse value assignment right side as a value selector");
    }
}

#[test]
fn parse_set_statement_1() {
    let fragment = "SET r/gho@>text = r/tron@morka, r/tron@morka = \"true\"";
    let (rem, b) = set_statement(fragment).expect("could not parse set statement");
    assert_eq!(rem, "");
    assert_eq!(b.set_word, "SET".to_string());
    assert_eq!(b.assignments.len(), 2);
}
