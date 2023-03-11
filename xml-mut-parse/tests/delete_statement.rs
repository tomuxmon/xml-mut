use xml_mut_data::{NodePath, PathVariant};
use xml_mut_parse::prelude::*;

#[test]
fn parse_delete_statement() {
    let fragment = "delete ItemGroup/PackageRef/fomo";
    let (_, b) = delete_statement(fragment).expect("could not parse delete statement");
    assert_eq!(b.delete_word, "delete");
    assert_eq!(b.targets.len(), 1);

    assert_eq!(
        b.targets[0],
        PathVariant::Path(NodePath {
            path: vec!["ItemGroup", "PackageRef", "fomo"]
        })
    );
}
