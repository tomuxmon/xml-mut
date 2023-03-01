use std::ops::Deref;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NodePath<'a> {
    pub path: Vec<&'a str>,
}

impl<'a> Deref for NodePath<'a> {
    type Target = Vec<&'a str>;

    fn deref(&self) -> &Self::Target {
        &self.path
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetStatement<'a> {
    pub get_word: &'a str,
    pub node_selector: NodePath<'a>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValueVariant<'a> {
    Selector(ValuePath<'a>),
    LiteralString(&'a str),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValuePath<'a> {
    pub node_path: NodePath<'a>,
    pub source: ValueSource<'a>,
}

// TODO: add name
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValueSource<'a> {
    Attribute(&'a str),
    Text,
    Tail,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Predicate<'a> {
    NodeExists(PredicateNodeExists<'a>),
    Equals(PredicateEquals<'a>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PredicateNodeExists<'a> {
    pub exists_word: &'a str,
    pub node_path: NodePath<'a>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PredicateEquals<'a> {
    pub left_side: ValuePath<'a>,
    pub right_side: ValueVariant<'a>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhereClause<'a> {
    pub where_word: &'a str,
    pub predicates: Vec<Predicate<'a>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValueAssignment<'a> {
    pub target: ValuePath<'a>,
    pub source: ValueVariant<'a>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SetStatement<'a> {
    pub set_word: &'a str,
    pub assignments: Vec<ValueAssignment<'a>>,
}

impl<'a> SetStatement<'a> {
    pub fn imply_predicates(&self) -> Vec<Predicate<'a>> {
        let mut predicates = Vec::new();
        for assignment in &self.assignments {
            if !assignment.target.node_path.is_empty() {
                predicates.push(Predicate::NodeExists(PredicateNodeExists {
                    node_path: assignment.target.node_path.clone(),
                    exists_word: "exists",
                }));
            }
            if let ValueVariant::Selector(value_path) = &assignment.source {
                if !value_path.node_path.is_empty() {
                    predicates.push(Predicate::NodeExists(PredicateNodeExists {
                        node_path: value_path.node_path.clone(),
                        exists_word: "exists",
                    }));
                }
            }
        }
        predicates
    }
}

// TODO: expect multiple node paths or value selectors in delete statrement
// TODO: non desttructive parse of delete statement node path or value selector
// (if not value selector just use node path)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeleteStatement<'a> {
    pub delete_word: &'a str,
    pub node_path: NodePath<'a>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Mutation<'a> {
    pub get: GetStatement<'a>,
    pub where_clause: Option<WhereClause<'a>>,
    pub set: Option<SetStatement<'a>>,
    pub delete: Option<DeleteStatement<'a>>,
}

pub enum Statement<'a> {
    Mutation(Mutation<'a>),
    Comment(&'a str),
}