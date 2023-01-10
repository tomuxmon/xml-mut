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

#[derive(Debug)]
pub enum ValueVariant {
    Selector(ValueSelector),
    LiteralString(String),
}

#[derive(Debug)]
pub struct ValueSelector {
    pub node_path: Vec<String>,
    pub ending: ValueSelectorEnding,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ValueSelectorEnding {
    AttributeName(String),
    NodeText,
}

#[derive(Debug)]
pub enum Predicate {
    NodeExists(PredicateNodeExists),
    Equals(PredicateEquals),
}

#[derive(Debug)]
pub struct PredicateNodeExists {
    pub exists_word: String,
    pub node_path: Vec<String>,
}

#[derive(Debug)]
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
