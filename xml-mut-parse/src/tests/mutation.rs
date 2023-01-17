use crate::prelude::*;

#[test]
fn parse_mutation_1() {
    let fragment = r###"get ItemGroup/PackageRef as p
where exists p/version
set p@version = p/version@>text
delete p/version"###;

    let (_, w) = mutation(fragment).expect("could not parse mutation");
    assert_eq!(w.get.get_word, "get".to_string());
    assert_eq!(w.get.node_selector.alias, "p".to_string());
    assert_eq!(w.get.node_selector.path.len(), 2);
    assert_eq!(w.get.node_selector.path[0], "ItemGroup".to_string());
    assert_eq!(w.get.node_selector.path[1], "PackageRef".to_string());
    assert_eq!(w.get.node_selector.as_word, "as".to_string());

    assert_eq!(w.where_clause.where_word, "where".to_string());
    assert_eq!(w.where_clause.predicates.len(), 1);
    assert_eq!(
        w.where_clause.predicates[0],
        Predicate::NodeExists(PredicateNodeExists {
            exists_word: "exists".to_string(),
            node_path: vec!["p".to_string(), "version".to_string()]
        })
    );
    assert!(w.set.is_some());
    assert!(w.delete.is_some());
}
