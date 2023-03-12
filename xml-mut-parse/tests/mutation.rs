use xml_mut_data::{
    GetClause, Mutation, NodePath, PathVariant, Predicate, PredicateExists, SetClause,
    ValueAssignment, ValuePath, ValueSelector, ValueVariant,
};
use xml_mut_parse::prelude::*;

#[test]
fn parse_mutation_1() {
    let fragment = r###"get ItemGroup/PackageRef
where exists version
set [@version] = version[text]
delete version"###;

    let (_, w) = mutation(fragment).expect("could not parse mutation");
    assert_eq!(w.get_clause.get_word, "get");
    assert_eq!(w.get_clause.node_selector.path.len(), 2);
    assert_eq!(w.get_clause.node_selector.path[0], "ItemGroup");
    assert_eq!(w.get_clause.node_selector.path[1], "PackageRef");

    assert!(w.where_clause.is_some());
    let where_clause = w.where_clause.unwrap();

    assert_eq!(where_clause.where_word, "where");
    assert_eq!(where_clause.predicates.len(), 1);
    assert_eq!(
        where_clause.predicates[0],
        Predicate::Exists(PredicateExists {
            exists_word: "exists",
            path: PathVariant::Node(NodePath {
                path: vec!["version"]
            }),
        })
    );
    assert!(w.set_clause.is_some());
    assert!(w.delete_clause.is_some());
}

#[test]
fn parse_mutation_2() {
    let fragment = r###"get ItemGroup/PackageReference
    set [@Version] = Version[text]
    delete Version"###;

    let (_, w) = mutation(fragment).expect("could not parse mutation");
    assert_eq!(w.get_clause.get_word, "get");
    assert_eq!(w.get_clause.node_selector.path.len(), 2);
    assert_eq!(w.get_clause.node_selector.path[0], "ItemGroup");
    assert_eq!(w.get_clause.node_selector.path[1], "PackageReference");

    assert!(w.where_clause.is_none());
    assert!(w.set_clause.is_some());
    assert!(w.delete_clause.is_some());
}

#[test]
fn parse_mutation_3() {
    let fragment = r###"GET ItemGroup/PackageReference
SET Version[text] = [@Version]"###;

    let (_, w) = mutation(fragment).expect("could not parse mutation");

    assert_eq!(
        w,
        Mutation {
            get_clause: GetClause {
                get_word: "GET",
                node_selector: NodePath {
                    path: vec!["ItemGroup", "PackageReference"]
                }
            },
            where_clause: None,
            set_clause: Some(SetClause {
                set_word: "SET",
                assignments: vec![ValueAssignment {
                    target: ValuePath {
                        node_path: NodePath {
                            path: vec!["Version"]
                        },
                        selector: ValueSelector::Text,
                    },
                    source: ValueVariant::Selector(ValuePath {
                        node_path: NodePath { path: vec![] },
                        selector: ValueSelector::Attribute("Version"),
                    })
                }]
            }),
            delete_clause: None
        }
    );
}
