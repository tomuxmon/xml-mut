use crate::prelude::{Mutable, Replacer};
use roxmltree::Document;
use xml_mut_data::Mutation;

pub trait DocumentExt {
    fn get_replacers(&self, mutation: &Mutation) -> Vec<Replacer>;
    fn get_replacers_all(&self, mutations: Vec<Mutation>) -> Vec<Replacer> {
        let mut replacers: Vec<Replacer> = vec![];
        for mutation in mutations {
            replacers.append(&mut self.get_replacers(&mutation));
        }
        replacers
    }
    fn replace_with(&self, replacers: Vec<Replacer>) -> Option<String>;
}

impl<'input> DocumentExt for Document<'input> {
    fn get_replacers(&self, mutation: &Mutation) -> Vec<Replacer> {
        let mut replacers: Vec<Replacer> = vec![];
        for node in self.descendants().filter(|n| n.is_fit(mutation)) {
            match node.apply(mutation) {
                Ok(replacer) => replacers.push(replacer),
                Err(error) => println!("{error:?}"), // todo: better error handling
            }
        }
        replacers
    }

    fn replace_with(&self, replacers: Vec<Replacer>) -> Option<String> {
        let mut new_xml = String::with_capacity(self.input_text().len());
        let mut offset = 0;
        let mut replacers_sorted = replacers.to_vec();
        replacers_sorted.sort_by(|a, b| a.bounds_cmp(b));

        for replacer in replacers_sorted.iter() {
            new_xml.push_str(&self.input_text()[offset..replacer.bounds.start]);
            new_xml.push_str(&replacer.replacement);
            offset = replacer.bounds.end;
        }
        new_xml.push_str(&self.input_text()[offset..self.input_text().len()]);

        if replacers.is_empty() {
            return None;
        }

        Some(new_xml)
    }
}
