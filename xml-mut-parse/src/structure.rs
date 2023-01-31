#[derive(Debug)]
pub struct NodeSelector<'a> {
    pub path: Vec<&'a str>,
    pub as_word: &'a str,
    pub alias: &'a str,
}

#[derive(Debug)]
pub struct GetStatement<'a> {
    pub get_word: &'a str,
    pub node_selector: NodeSelector<'a>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ValueVariant<'a> {
    Selector(ValueSelector<'a>),
    LiteralString(&'a str),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ValueSelector<'a> {
    pub node_path: Vec<&'a str>,
    pub ending: SelectorEnding<'a>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum SelectorEnding<'a> {
    AttributeName(&'a str),
    NodeText,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Predicate<'a> {
    NodeExists(PredicateNodeExists<'a>),
    Equals(PredicateEquals<'a>),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct PredicateNodeExists<'a> {
    pub exists_word: &'a str,
    pub node_path: Vec<&'a str>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct PredicateEquals<'a> {
    pub left_side: ValueSelector<'a>,
    pub right_side: &'a str,
}

#[derive(Debug)]
pub struct WhereClause<'a> {
    pub where_word: &'a str,
    pub predicates: Vec<Predicate<'a>>,
}

#[derive(Debug)]
pub struct ValueAssignment<'a> {
    pub left_side: ValueSelector<'a>,
    pub right_side: ValueVariant<'a>,
}

#[derive(Debug)]
pub struct SetStatement<'a> {
    pub set_word: &'a str,
    pub assignments: Vec<ValueAssignment<'a>>,
}

// TODO: extract Vec<String> as NodePath struct

// TODO: expect multiple node paths or value selectors in delete statrement

// TODO: non desttructive parse of delete statement node path or value selector
// (if not value selector just use node path)

#[derive(Debug)]
pub struct DeleteStatement<'a> {
    pub delete_word: &'a str,
    pub node_path: Vec<&'a str>,
}

#[derive(Debug)]
pub struct Mutation<'a> {
    pub get: GetStatement<'a>,
    pub where_clause: WhereClause<'a>,
    pub set: Option<SetStatement<'a>>,
    pub delete: Option<DeleteStatement<'a>>,
}
