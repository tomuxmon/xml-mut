#[cfg(test)]
use super::*;

#[test]
fn parse_node_selector_path1() {
    let fragment = "Packageref as ref";
    let (_, b) = node_selector(fragment).expect("could not parse node selector");
    assert_eq!(b.alias, "ref");
    assert_eq!(b.as_word, "as");
    assert_eq!(b.path.len(), 1);
    assert_eq!(b.path[0], "Packageref");
}

#[test]
fn parse_node_selector_path2() {
    let fragment = "ItemGroup/PackageRef AS p";
    let (_, b) = node_selector(fragment).expect("could not parse node selector");
    assert_eq!(b.alias, "p");
    assert_eq!(b.as_word, "AS");
    assert_eq!(b.path.len(), 2);
    assert_eq!(b.path[0], "ItemGroup");
    assert_eq!(b.path[1], "PackageRef");
}

#[test]
fn parse_get_statement() {
    let fragment = "GeT ItemGroup/PackageRef AS IGPR";
    let (_, b) = get_statement(fragment).expect("could not parse get statement");
    assert_eq!(b.get_word, "GeT");
    assert_eq!(b.node_selector.alias, "IGPR");
    assert_eq!(b.node_selector.as_word, "AS");
    assert_eq!(b.node_selector.path.len(), 2);
    assert_eq!(b.node_selector.path[0], "ItemGroup");
    assert_eq!(b.node_selector.path[1], "PackageRef");
}
