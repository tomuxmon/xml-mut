#[derive(Debug)]
pub struct NodeSelector {
    pub path: Vec<String>,
    pub as_word: String,
    pub alias: String,
}

#[derive(Debug)]
pub struct GetStatement {
    pub get_word: String,
    pub node_selector: NodeSelector,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ValueVariant {
    Selector(ValueSelector),
    LiteralString(String),
}

#[derive(Debug, PartialEq, Eq)]
pub struct ValueSelector {
    pub node_path: Vec<String>,
    pub ending: ValueSelectorEnding,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ValueSelectorEnding {
    AttributeName(String),
    NodeText,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Predicate {
    NodeExists(PredicateNodeExists),
    Equals(PredicateEquals),
}

#[derive(Debug, PartialEq, Eq)]
pub struct PredicateNodeExists {
    pub exists_word: String,
    pub node_path: Vec<String>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct PredicateEquals {
    pub left_side: ValueSelector,
    pub right_side: String,
}

#[derive(Debug)]
pub struct WhereClause {
    pub where_word: String,
    pub predicates: Vec<Predicate>,
}

#[derive(Debug)]
pub struct ValueAssignment {
    pub left_side: ValueSelector,
    pub right_side: ValueVariant,
}

#[derive(Debug)]
pub struct SetStatement {
    pub set_word: String,
    pub assignments: Vec<ValueAssignment>,
}

// TODO: extract Vec<String> as NodePath struct

// TODO: expect multiple node paths or value selectors in delete statrement

// TODO: non desttructive parse of delete statement node path or value selector
// (if not value selector just use node path)

#[derive(Debug)]
pub struct DeleteStatement {
    pub delete_word: String,
    pub node_path: Vec<String>,
}

#[derive(Debug)]
pub struct Mutation {
    pub get: GetStatement,
    pub where_clause: WhereClause,
    pub set: Option<SetStatement>,
    pub delete: Option<DeleteStatement>,
}
