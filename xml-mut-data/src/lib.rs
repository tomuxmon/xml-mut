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

/// A selector that could be target of a single value or a delete.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValueSelector<'a> {
    /// Nodes attribute
    Attribute(&'a str),
    /// Nodes inner text
    Text,
    /// Text after nodes end tag
    Tail,
    /// Nodes name, opssibly located in 2 places if the node has a closing tag
    /// you can not delete name hence it isinvalid
    /// rename should have aseparate syntax
    #[deprecated]
    Name,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValuePath<'a> {
    pub node_path: NodePath<'a>,
    pub selector: ValueSelector<'a>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValueVariant<'a> {
    Selector(ValuePath<'a>),
    LiteralString(&'a str),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PathVariant<'a> {
    Node(NodePath<'a>),
    Value(ValuePath<'a>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValueAssignment<'a> {
    pub target: ValuePath<'a>,
    pub source: ValueVariant<'a>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PredicateExists<'a> {
    pub exists_word: &'a str,
    pub path: PathVariant<'a>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PredicateEquals<'a> {
    pub left_side: ValuePath<'a>,
    pub right_side: ValueVariant<'a>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Predicate<'a> {
    Exists(PredicateExists<'a>),
    Equals(PredicateEquals<'a>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetClause<'a> {
    pub get_word: &'a str,
    pub node_selector: NodePath<'a>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhereClause<'a> {
    pub where_word: &'a str,
    pub predicates: Vec<Predicate<'a>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SetClause<'a> {
    pub set_word: &'a str,
    pub assignments: Vec<ValueAssignment<'a>>,
}

impl<'a> SetClause<'a> {
    pub fn imply_predicates(&self) -> Vec<Predicate<'a>> {
        let mut predicates = Vec::new();
        for assignment in &self.assignments {
            // NOTE: assignment.target.node_path could possibly
            // not exist and it would be constructed
            // no need to imply predicate on it

            if let ValueVariant::Selector(value_path) = &assignment.source {
                if !value_path.node_path.is_empty() {
                    predicates.push(Predicate::Exists(PredicateExists {
                        exists_word: "exists",
                        path: PathVariant::Value(value_path.clone()),
                    }));
                }
            }
        }
        predicates
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeleteClause<'a> {
    pub delete_word: &'a str,
    pub targets: Vec<PathVariant<'a>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Mutation<'a> {
    pub get_clause: GetClause<'a>,
    pub where_clause: Option<WhereClause<'a>>,
    pub set_clause: Option<SetClause<'a>>,
    pub delete_clause: Option<DeleteClause<'a>>,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Statement<'a> {
    Mutation(Mutation<'a>),
    Comment(&'a str),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct XmlMutGrammar<'a> {
    pub statements: Vec<Statement<'a>>,
}
