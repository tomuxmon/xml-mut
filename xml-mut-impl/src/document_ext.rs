use crate::prelude::{Mutable, Replacer};
use roxmltree::Document;
use xml_mut_data::Mutation;

pub trait DocumentExt {
    fn get_replacers(&self, mutation: &Mutation) -> Vec<Replacer>;
    fn get_replacers_all(&self, mutations: Vec<Mutation>) -> Vec<Replacer> {
        mutations
            .iter()
            .flat_map(|m| self.get_replacers(m))
            .collect()
    }
    fn apply(&self, replacers: Vec<Replacer>) -> Option<String>;
}

impl<'input> DocumentExt for Document<'input> {
    fn get_replacers(&self, mutation: &Mutation) -> Vec<Replacer> {
        self.descendants()
            .filter(|n| n.is_fit(mutation))
            .flat_map(|n| n.get_replacers(mutation))
            .collect()
    }

    fn apply(&self, replacers: Vec<Replacer>) -> Option<String> {
        if replacers.is_empty() {
            return None;
        }

        // TODO: validate replacer.bounds.is_empty()
        // TODO: validate replacer overlaps

        let new_len = usize::try_from(
            i32::try_from(self.input_text().len()).unwrap_or(0)
                + replacers.iter().map(|r| r.len_diff()).sum::<i32>(),
        )
        .unwrap_or(self.input_text().len());

        let mut new_xml = String::with_capacity(new_len);
        let mut offset = 0;
        let mut replacers_sorted = replacers.to_vec();
        replacers_sorted.sort_by(|a, b| a.bounds_cmp(b));

        for replacer in replacers_sorted {
            new_xml.push_str(&self.input_text()[offset..replacer.bounds.start]);
            new_xml.push_str(&replacer.replacement);
            offset = replacer.bounds.end;
        }
        new_xml.push_str(&self.input_text()[offset..self.input_text().len()]);

        Some(new_xml)
    }
}
