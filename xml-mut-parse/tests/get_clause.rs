use xml_mut_parse::prelude::*;

#[test]
fn parse_node_selector_path1() {
    let fragment = "Packageref";
    let (_, b) = node_path(fragment).expect("could not parse node selector");
    assert_eq!(b.path.len(), 1);
    assert_eq!(b.path[0], "Packageref");
}

#[test]
fn parse_node_selector_path2() {
    let fragment = "ItemGroup/PackageRef";
    let (_, b) = node_path(fragment).expect("could not parse node selector");
    assert_eq!(b.path.len(), 2);
    assert_eq!(b.path[0], "ItemGroup");
    assert_eq!(b.path[1], "PackageRef");
}

#[test]
fn parse_get_statement() {
    let fragment = "GeT ItemGroup/PackageRef";
    let (_, b) = get_clause(fragment).expect("could not parse get statement");
    assert_eq!(b.get_word, "GeT");
    assert_eq!(b.node_selector.path.len(), 2);
    assert_eq!(b.node_selector.path[0], "ItemGroup");
    assert_eq!(b.node_selector.path[1], "PackageRef");
}
