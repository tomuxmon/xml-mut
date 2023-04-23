use crate::prelude::{Mutable, NodeExtensions, ReplaceError, Replacer, Valueable};
use roxmltree::{Document, Node};
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
    fn apply_internal(&self, replacers: &[Replacer]) -> Result<String, ReplaceError>;
    fn apply(&self, replacers: &[Replacer]) -> Result<String, ReplaceError>;
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
        fn is_fit(node: &Node) -> bool {
            node.is_element()
                && (node
                    .get_bounds(&ValueSelector::Text)
                    .map_or(false, |b| b.is_empty())
                    || (node.first_element_child().is_none()
                        && node.text().map_or(false, |t| t.trim().is_empty())))
        }

        self.descendants()
            .filter(is_fit)
            .map(|n| Replacer {
                bounds: n.get_tag_end_position()..n.range().end,
                replacement: "/>".to_string(),
            })
            .collect()
    }
    // TODO: Option -> Result
    // TODO: String -> Self (or Box<Self> at least?)
    fn apply_internal(&self, replacers: &[Replacer]) -> Result<String, ReplaceError> {
        if replacers.is_empty() {
            return Err(ReplaceError::NoChange);
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

        Ok(new_xml)
    }

    fn apply(&self, replacers: &[Replacer]) -> Result<String, ReplaceError> {
        let new_xml = self.apply_internal(replacers)?;
        let new_doc = match Document::parse(&new_xml) {
            Ok(d) => d,
            Err(error) => return Err(ReplaceError::GeneratedXmlInvalid(error)),
        };
        // NOTE: post processing
        let trim_replacers = &new_doc.get_end_tag_trim_replacers();
        if let Ok(n) = new_doc.apply_internal(trim_replacers) {
            Ok(n)
        } else {
            Ok(new_xml)
        }
    }
}

#[cfg(test)]
mod tests {
    use roxmltree::Document;

    use super::*;

    #[test]
    fn get_end_tag_trim_replacers_01() {
        let xml = r###"<A b="zuzu"/>"###;
        let doc = Document::parse(xml).expect("could not parse xml");
        let replacement = doc.get_end_tag_trim_replacers();
        assert_eq!(replacement.len(), 0);
    }

    #[test]
    fn get_end_tag_trim_replacers_02() {
        let xml = r###"<A b="zuzu"></A>"###;
        let doc = Document::parse(xml).expect("could not parse xml");
        let replacement = doc.get_end_tag_trim_replacers();
        assert_eq!(replacement.len(), 1);
        assert_eq!(
            replacement[0],
            Replacer {
                bounds: 11..16,
                replacement: "/>".to_string()
            }
        );
    }

    #[test]
    fn get_end_tag_trim_replacers_03() {
        let xml = r###"<A b="zuzu">     
        </A>"###;
        let doc = Document::parse(xml).expect("could not parse xml");
        let replacement = doc.get_end_tag_trim_replacers();
        assert_eq!(replacement.len(), 1);
        assert_eq!(
            replacement[0],
            Replacer {
                bounds: 11..30,
                replacement: "/>".to_string()
            }
        );
    }
}
