use crate::prelude::{Mutable, Replacer};
use roxmltree::Document;
use xml_mut_parse::prelude::Mutation;

pub trait DocumentExt {
    fn mutate(&self, mutation: &Mutation) -> Option<String>;
}

impl<'input> DocumentExt for Document<'input> {
    fn mutate(&self, mutation: &Mutation) -> Option<String> {
        let mut replacers: Vec<Replacer> = vec![];
        for node in self.descendants().filter(|n| n.is_fit(mutation)) {
            match node.apply(mutation) {
                Ok(replacer) => replacers.push(replacer),
                Err(error) => println!("{error:?}"), // todo: better error handling
            }
        }

        if replacers.is_empty() {
            return None;
        }

        let mut new_xml = String::with_capacity(self.input_text().len());
        let mut offset = 0;
        for replacer in replacers {
            new_xml.push_str(&self.input_text()[offset..replacer.bounds.start]);
            new_xml.push_str(&replacer.replacement);
            offset = replacer.bounds.end;
        }
        new_xml.push_str(&self.input_text()[offset..self.input_text().len()]);
        Some(new_xml)
    }
}
