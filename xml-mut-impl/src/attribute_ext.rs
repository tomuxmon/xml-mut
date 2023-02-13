use roxmltree::Attribute;
use std::ops::Range;

pub trait AttributeExtensions {
    fn get_bounds(&self) -> Range<usize>;
    fn get_value_bounds(&self) -> Range<usize>;
}

// TODO: write tests.
impl<'a, 'input: 'a> AttributeExtensions for Attribute<'a, 'input> {
    fn get_bounds(&self) -> Range<usize> {
        self.position()..self.position() + self.name().len() + self.value().len() + 3
    }

    fn get_value_bounds(&self) -> Range<usize> {
        self.position() + self.name().len() + 2
            ..self.position() + self.name().len() + self.value().len() + 2
    }
}
