use roxmltree::Attribute;
use std::ops::Range;

pub trait AttributeExtensions {
    fn range(&self) -> Range<usize>;
    fn value_range(&self) -> Range<usize>;
}

impl<'a, 'input: 'a> AttributeExtensions for Attribute<'a, 'input> {
    fn range(&self) -> Range<usize> {
        self.position()..self.position() + self.name().len() + self.value().len() + 3
    }

    fn value_range(&self) -> Range<usize> {
        self.position() + self.name().len() + 2
            ..self.position() + self.name().len() + self.value().len() + 2
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use roxmltree::Document;

    #[test]
    fn range_01() {
        let xml = r###"<A b="zuzu" />"###;
        let doc = Document::parse(xml).expect("could not parse xml");
        let atribute = doc
            .root()
            .first_child()
            .expect("first child should be A")
            .attribute_node("b")
            .expect("should be an attribute");

        let range = atribute.range();
        let value_range = atribute.value_range();

        assert_eq!(range, 3..11);
        assert_eq!(&doc.input_text()[range], "b=\"zuzu\"");

        assert_eq!(value_range, 6..10);
        assert_eq!(&doc.input_text()[value_range], "zuzu");
    }

    #[test]
    fn range_02() {
        let xml = r###"<A b="_žuƒu" />"###;
        let doc = Document::parse(xml).expect("could not parse xml");
        let atribute = doc
            .root()
            .first_child()
            .expect("first child should be A")
            .attribute_node("b")
            .expect("should be an attribute");

        let range = atribute.range();
        let value_range = atribute.value_range();

        assert_eq!(range, 3..14);
        assert_eq!(&doc.input_text()[range], "b=\"_žuƒu\"");

        assert_eq!(value_range, 6..13);
        assert_eq!(&doc.input_text()[value_range], "_žuƒu");
    }
}
