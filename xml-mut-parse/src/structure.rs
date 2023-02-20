// TODO: move structs into a separate crate
// Allows separate parsers to be implemented
// Allows xml document extensions(xml-mut-impl) to be independent of parser implementation (xml-mut-parse)

#[derive(Debug, Clone)]
pub struct NodeSelector<'a> {
    pub path: Vec<&'a str>,
    pub as_word: &'a str,
    pub alias: &'a str,
}

#[derive(Debug, Clone)]
pub struct GetStatement<'a> {
    pub get_word: &'a str,
    pub node_selector: NodeSelector<'a>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ValueVariant<'a> {
    Selector(ValueSelector<'a>),
    LiteralString(&'a str),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ValueSelector<'a> {
    pub node_path: Vec<&'a str>,
    pub source: ValueSource<'a>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ValueSource<'a> {
    Attribute(&'a str),
    Text,
    Tail,
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

#[derive(Debug, Clone)]
pub struct WhereClause<'a> {
    pub where_word: &'a str,
    pub predicates: Vec<Predicate<'a>>,
}

#[derive(Debug, Clone)]
pub struct ValueAssignment<'a> {
    pub target: ValueSelector<'a>,
    pub source: ValueVariant<'a>,
}

#[derive(Debug, Clone)]
pub struct SetStatement<'a> {
    pub set_word: &'a str,
    pub assignments: Vec<ValueAssignment<'a>>,
}

// TODO: extract Vec<String> as NodePath struct

// TODO: expect multiple node paths or value selectors in delete statrement

// TODO: non desttructive parse of delete statement node path or value selector
// (if not value selector just use node path)

#[derive(Debug, Clone)]
pub struct DeleteStatement<'a> {
    pub delete_word: &'a str,
    pub node_path: Vec<&'a str>,
}

#[derive(Debug, Clone)]
pub struct Mutation<'a> {
    pub get: GetStatement<'a>,
    pub where_clause: WhereClause<'a>,
    pub set: Option<SetStatement<'a>>,
    pub delete: Option<DeleteStatement<'a>>,
}
