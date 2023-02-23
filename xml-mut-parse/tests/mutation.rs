use xml_mut_data::{NodePath, Predicate, PredicateNodeExists};
use xml_mut_parse::prelude::*;

#[test]
fn parse_mutation_1() {
    let fragment = r###"get ItemGroup/PackageRef
where exists version
set [@version] = version[text]
delete version"###;

    let (_, w) = mutation(fragment).expect("could not parse mutation");
    assert_eq!(w.get.get_word, "get");
    assert_eq!(w.get.node_selector.path.len(), 2);
    assert_eq!(w.get.node_selector.path[0], "ItemGroup");
    assert_eq!(w.get.node_selector.path[1], "PackageRef");

    assert!(w.where_clause.is_some());
    let where_clause = w.where_clause.unwrap();

    assert_eq!(where_clause.where_word, "where");
    assert_eq!(where_clause.predicates.len(), 1);
    assert_eq!(
        where_clause.predicates[0],
        Predicate::NodeExists(PredicateNodeExists {
            exists_word: "exists",
            node_path: NodePath {
                path: vec!["version"]
            }
        })
    );
    assert!(w.set.is_some());
    assert!(w.delete.is_some());
}

#[test]
fn parse_mutation_2() {
    let fragment = r###"get ItemGroup/PackageReference
    set [@Version] = Version[text]
    delete Version"###;

    let (_, w) = mutation(fragment).expect("could not parse mutation");
    assert_eq!(w.get.get_word, "get");
    assert_eq!(w.get.node_selector.path.len(), 2);
    assert_eq!(w.get.node_selector.path[0], "ItemGroup");
    assert_eq!(w.get.node_selector.path[1], "PackageReference");

    assert!(w.where_clause.is_none());
    assert!(w.set.is_some());
    assert!(w.delete.is_some());
}
