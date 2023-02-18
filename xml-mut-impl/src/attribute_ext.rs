use roxmltree::Attribute;
use std::ops::Range;

pub trait AttributeExtensions {
    fn range(&self) -> Range<usize>;
    fn value_range(&self) -> Range<usize>;
}

// TODO: write tests.
impl<'a, 'input: 'a> AttributeExtensions for Attribute<'a, 'input> {
    fn range(&self) -> Range<usize> {
        self.position()..self.position() + self.name().len() + self.value().len() + 3
    }

    fn value_range(&self) -> Range<usize> {
        self.position() + self.name().len() + 2
            ..self.position() + self.name().len() + self.value().len() + 2
    }
}
