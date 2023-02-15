use xml_mut_parse::prelude::*;

#[test]
fn parse_delete_statement() {
    let fragment = "delete ItemGroup/PackageRef/fomo";
    let (_, b) = delete_statement(fragment).expect("could not parse delete statement");
    assert_eq!(b.delete_word, "delete");
    assert_eq!(b.node_path.len(), 3);
    assert_eq!(b.node_path[0], "ItemGroup");
    assert_eq!(b.node_path[1], "PackageRef");
    assert_eq!(b.node_path[2], "fomo");    
}