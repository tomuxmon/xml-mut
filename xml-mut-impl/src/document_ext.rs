use crate::prelude::{Mutable, NodeExtensions, Replacer, Valueable};
use roxmltree::Document;
use xml_mut_data::{Mutation, ValueSelector};

pub trait DocumentExt {
    fn get_replacers(&self, mutation: &Mutation) -> Vec<Replacer>;
    fn get_replacers_all(&self, mutations: &[&Mutation]) -> Vec<Replacer> {
        mutations
            .iter()
            .flat_map(|m| self.get_replacers(m))
            .collect()
    }
    fn get_end_tag_trim_replacers(&self) -> Vec<Replacer>;
    fn apply(&self, replacers: &[Replacer]) -> Option<String>;
}

impl<'input> DocumentExt for Document<'input> {
    fn get_replacers(&self, mutation: &Mutation) -> Vec<Replacer> {
        self.descendants()
            .filter(|n| n.is_fit(mutation))
            .flat_map(|n| n.get_replacers(mutation))
            .collect()
    }

    fn get_end_tag_trim_replacers(&self) -> Vec<Replacer> {
        // from: <a></a> -> from tag_end_pos to tag_end_pos + tag_name_len + 4;;
        // to: <a/>
        self.descendants()
            .filter(|n| {
                n.is_element()
                    && (n.get_bounds(&ValueSelector::Text).is_none()
                        || (n.first_element_child().is_none()
                            && n.text().map_or(true, |t| t.trim().is_empty())))
            })
            .map(|n| Replacer {
                bounds: n.get_tag_end_position()..n.range().end,
                replacement: "/>".to_string(),
            })
            .collect()
    }

    fn apply(&self, replacers: &[Replacer]) -> Option<String> {
        if replacers.is_empty() {
            return None;
        }

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
