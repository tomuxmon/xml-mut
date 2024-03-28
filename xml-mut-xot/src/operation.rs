use xot::Node;

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
    pub node_path: Vec<String>,
    pub sub_op: SubOperation,
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
    pub node: Node,
    pub name: String,
    pub value: String,
}
pub struct OpRemoveAttribute {
    pub node: Node,
    pub name: String,
}
pub struct OpSetText {
    pub node: Node,
    pub value: String,
}

pub struct OpPrependText {
    pub node: Node,
    pub value: String,
}

pub struct OpSetTextAfter {
    pub node: Node,
    pub value: String,
}

pub struct OpSetName {
    pub node: Node,
    pub name: String,
}

pub struct OpDeleteNode {
    pub node: Node,
}
