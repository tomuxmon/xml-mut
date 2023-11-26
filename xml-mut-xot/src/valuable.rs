use crate::prelude::{Error, Fitable, NodeExtensions};
use xml_mut_data::{
    Mutation, PathVariant, ValueAssignment, ValuePath, ValueSelector, ValueVariant,
};
use xot::{Node, Xot};

pub trait Valueable {
    fn get_value(&self, node: Node, selector: &ValueSelector) -> Option<&str>;
    fn get_child_value(&self, node: Node, path: &ValuePath) -> Option<&str>;
    fn get_value_of<'a>(&'a self, node: Node, variant: &'a ValueVariant) -> Option<&str> {
        match variant {
            ValueVariant::Selector(path) => self.get_child_value(node, path),
            ValueVariant::LiteralString(val) => Some(val),
        }
    }
    fn add_sub_tree(&self, node: Node, path: &ValuePath, value: String)
        -> Result<Operation, Error>;
    fn assign(&self, node: Node, assignment: &ValueAssignment) -> Result<Operation, Error>;
    fn delete(&self, node: Node, path_variant: &PathVariant) -> Result<Operation, Error>;
    fn get_operations(&self, node: Node, mutation: &Mutation) -> Result<Vec<Operation>, Error>;
    fn get_operations_all(
        &self,
        node: Node,
        mutations: &[&Mutation],
    ) -> Result<Vec<Operation>, Error>;
    fn apply(&mut self, operation: &Operation) -> Result<(), Error>;
    fn apply_all(&mut self, operations: &[Operation]) -> Result<(), Error>;
}

impl Valueable for Xot {
    fn get_value(&self, node: Node, selector: &ValueSelector) -> Option<&str> {
        match selector {
            ValueSelector::Attribute(name) => self.element(node)?.get_attribute(self.name(name)?),
            ValueSelector::Text => self.first_child(node).and_then(|t| self.text_str(t)),
            ValueSelector::Tail => self.next_sibling(node).and_then(|t| self.text_str(t)),
            ValueSelector::Name => self.get_name(node),
        }
    }

    fn get_child_value(&self, node: Node, path: &ValuePath) -> Option<&str> {
        self.find_first_child_element(node, &path.node_path)
            .and_then(|n| self.get_value(n, &path.selector))
    }

    // TODO: specialized  AssignError and map it in call site
    fn assign(&self, node: Node, assignment: &ValueAssignment) -> Result<Operation, Error> {
        let value = self
            .get_value_of(node, &assignment.source)
            .ok_or(
                // NOTE: there is no predicate ensuring attribute existance yet
                Error::AssignmentSourceValueNotFound(format!(
                    "Node {:?} does not contain a sub node at {:?}.",
                    self.get_name(node).unwrap_or("noname"),
                    &assignment.source
                )),
            )?
            .to_string();

        // TODO: instead of self.find_first_child_element
        // use enumerable and assign for all matches
        let node = match self.find_first_child_element(node, &assignment.target.node_path) {
            Some(n) => n,
            None => {
                return self.add_sub_tree(node, &assignment.target, value);
            }
        };

        let op = match assignment.target.selector {
            ValueSelector::Attribute(name) => Operation::SetAttribute(OpSetAttribute {
                node,
                name: name.to_string(),
                value,
            }),
            ValueSelector::Text => {
                if let Some(node) = self.text_node(node) {
                    Operation::SetText(OpSetText { node, value })
                } else {
                    Operation::PrependText(OpPrependText { node, value })
                }
            }
            ValueSelector::Tail => {
                if let Some(node) = self.tail_text_node(node) {
                    Operation::SetText(OpSetText { node, value })
                } else {
                    Operation::SetTextAfter(OpSetTextAfter { node, value })
                }
            }
            ValueSelector::Name => Operation::SetName(OpSetName { node, name: value }),
        };

        Ok(op)
    }

    fn add_sub_tree(
        &self,
        node: Node,
        path: &ValuePath,
        value: String,
    ) -> Result<Operation, Error> {
        let mut node = node;
        let mut path_vec = path.node_path.to_vec();
        path_vec.reverse();

        loop {
            if let Some(name) = path_vec.pop() {
                if let Some(child_node) = self
                    .children(node)
                    .find(|n| self.is_element_with_name(*n, name))
                {
                    node = child_node;
                } else {
                    // depleated, can create sub tree from this point
                    path_vec.push(name);
                    path_vec.reverse();
                    break;
                }
            } else {
                return Err(Error::NothingToAdd);
            }
        }

        if ValueSelector::Name == path.selector && path_vec.pop().is_some() {
            path_vec.push(value.as_str());
        }

        let node_path: Vec<String> = path_vec.iter().map(|s| s.to_string()).collect();

        let sub_op = match path.selector {
            ValueSelector::Attribute(name) => SubOperation::AddAttribute(name.to_string(), value),
            ValueSelector::Text => SubOperation::AddText(value),
            ValueSelector::Tail => SubOperation::AddTailText(value),
            ValueSelector::Name => SubOperation::None,
        };

        Ok(Operation::AddSubTree(OpAddSubTree {
            node,
            node_path,
            sub_op,
        }))
    }

    // TODO: specialized  DeleteError and map it in call site
    fn delete(&self, node: Node, path_variant: &PathVariant) -> Result<Operation, Error> {
        let (path, maybe_source) = match path_variant {
            PathVariant::Value(v) => (&v.node_path, Some(&v.selector)),
            PathVariant::Node(p) => (p, None),
        };

        let node = self
            .find_first_child_element(node, path)
            .ok_or(Error::DeleteNothing(
                "Delete target node not found.".to_string(),
            ))?;

        if let Some(source) = maybe_source {
            if *source == ValueSelector::Name {
                Err(Error::DeleteNameIsInvalid)?;
            }

            let value = "".to_string();

            let op = match source {
                ValueSelector::Attribute(name) => {
                    let name = name.to_string();
                    Operation::RemoveAttribute(OpRemoveAttribute { name, node })
                }
                ValueSelector::Text => {
                    let node = self.text_node(node).ok_or(Error::DeleteNothing(
                        "Delete text node not found.".to_string(),
                    ))?;
                    Operation::SetText(OpSetText { node, value })
                }
                ValueSelector::Tail => {
                    let node = self.tail_text_node(node).ok_or(Error::DeleteNothing(
                        "Delete tail text node not found.".to_string(),
                    ))?;
                    Operation::SetText(OpSetText { node, value })
                }
                ValueSelector::Name => {
                    unreachable!("already checked and returned Error::DeleteNameIsInvalid")
                }
            };

            return Ok(op);
        }

        Ok(Operation::DeleteNode(OpDeleteNode { node }))
    }

    fn get_operations(&self, node: Node, mutation: &Mutation) -> Result<Vec<Operation>, Error> {
        let mut ops = vec![];

        if let Some(set_clause) = mutation.set_clause.clone() {
            for ref assignment in set_clause.assignments.into_iter() {
                ops.push(self.assign(node, assignment)?);
            }
        }
        if let Some(ref delete_clause) = mutation.delete_clause.clone() {
            for path_var in &delete_clause.targets {
                ops.push(self.delete(node, path_var)?);
            }
        }

        Ok(ops)
    }

    fn get_operations_all(
        &self,
        node: Node,
        mutations: &[&Mutation],
    ) -> Result<Vec<Operation>, Error> {
        let mut operations: Vec<Operation> = vec![];
        for &mutation in mutations {
            let nodes: Vec<Node> = self.descendants(node).collect();
            for node in nodes {
                if self.is_fit(node, mutation) {
                    let mut ops = self.get_operations(node, mutation)?;
                    operations.append(&mut ops);
                }
            }
        }

        Ok(operations)
    }

    fn apply(&mut self, operation: &Operation) -> Result<(), Error> {
        match operation {
            Operation::AddSubTree(op) => {
                let mut node = op.node;
                for name in &op.node_path {
                    let name_id = self.add_name(name.as_str());
                    let element_node = self.new_element(name_id);
                    self.append(node, element_node).map_err(Error::XotError)?;
                    node = element_node;
                }

                match &op.sub_op {
                    SubOperation::None => (),
                    SubOperation::AddAttribute(name, value) => {
                        let attribute_name = self.add_name(name.as_str());
                        let element = self.element_mut(node).unwrap();
                        element.set_attribute(attribute_name, value);
                    }
                    SubOperation::AddText(value) => {
                        self.append_text(node, value.as_str())
                            .map_err(Error::XotError)?;
                    }
                    SubOperation::AddTailText(value) => {
                        let text_node = self.new_text(value.as_str());
                        self.insert_after(node, text_node)
                            .map_err(Error::XotError)?;
                    }
                }
            }
            Operation::SetAttribute(op) => {
                let name_id = self.add_name(op.name.as_str());
                let element = self.element_mut(op.node).ok_or(Error::NotAnElement)?;
                element.set_attribute(name_id, op.value.clone())
            }
            Operation::RemoveAttribute(op) => {
                let name_id = self
                    .name(op.name.as_str())
                    .ok_or(Error::NameNotFound(op.name.clone()))?;
                let element = self.element_mut(op.node).ok_or(Error::NotAnElement)?;
                element.remove_attribute(name_id);
            }
            Operation::SetText(op) => {
                let text = self.text_mut(op.node).ok_or(Error::NotATextNode)?;
                text.set(op.value.clone());
            }
            Operation::PrependText(op) => {
                let text = self.new_text(op.value.as_str());
                self.prepend(op.node, text).map_err(Error::XotError)?;
            }
            Operation::SetTextAfter(op) => {
                let text = self.new_text(op.value.as_str());
                self.insert_after(op.node, text).map_err(Error::XotError)?;
            }
            Operation::SetName(op) => {
                let name_id = self.add_name(op.name.as_str());
                let element = self.element_mut(op.node).ok_or(Error::NotAnElement)?;
                element.set_name(name_id);
            }
            Operation::DeleteNode(op) => {
                self.remove(op.node).map_err(Error::XotError)?;
            }
        }

        Ok(())
    }

    fn apply_all(&mut self, operations: &[Operation]) -> Result<(), Error> {
        for op in operations {
            self.apply(op)?;
        }
        Ok(())
    }
}

pub enum Operation {
    AddSubTree(OpAddSubTree),
    SetAttribute(OpSetAttribute),
    RemoveAttribute(OpRemoveAttribute),
    SetText(OpSetText),
    PrependText(OpPrependText),
    SetTextAfter(OpSetTextAfter),
    SetName(OpSetName),
    DeleteNode(OpDeleteNode),
}

pub struct OpAddSubTree {
    pub node: Node,
    node_path: Vec<String>,
    sub_op: SubOperation,
}

pub enum SubOperation {
    None,
    /// Name and value of the attribute to be added
    AddAttribute(String, String),
    /// Value of the text to be added
    AddText(String),
    /// Value of the tail text to be inserted after
    AddTailText(String),
}

pub struct OpSetAttribute {
    node: Node,
    name: String,
    value: String,
}
pub struct OpRemoveAttribute {
    node: Node,
    name: String,
}
pub struct OpSetText {
    node: Node,
    value: String,
}

pub struct OpPrependText {
    node: Node,
    value: String,
}

pub struct OpSetTextAfter {
    node: Node,
    value: String,
}

pub struct OpSetName {
    node: Node,
    name: String,
}

pub struct OpDeleteNode {
    node: Node,
}
