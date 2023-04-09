use roxmltree::Document;
use std::fs;
use xml_mut_data::{Mutation, Statement};
use xml_mut_impl::prelude::DocumentExt;
use xml_mut_parse::prelude::*;

#[test]
fn package_ref_version_mutation() {
    let xml_path = "tests/package_ref_version/input.xml";
    let xml_mut_path = "tests/package_ref_version/mutation.xmlmut";
    let xml_expected_path = "tests/package_ref_version/output.xml";

    let xml = fs::read_to_string(xml_path).expect("xml file should exist");
    let xut = fs::read_to_string(xml_mut_path).expect("xml mutation file should exist");
    let xml_expected = fs::read_to_string(xml_expected_path).expect("xml output file should exist");
    let doc = Document::parse(xml.as_str()).expect("should be avalid xml");

    let (non_parsed, ref statements) =
        statements(xut.as_str()).expect("could not parse statements");

    if !non_parsed.is_empty() {
        panic!("non_parsed should be empty but is {:#?}", non_parsed);
    }

    let mutations = &statements
        .iter()
        .filter_map(|s| match s {
            Statement::Mutation(rep) => Some(rep),
            _ => None,
        })
        .collect::<Vec<&Mutation>>();

    let replacers = &doc.get_replacers_all(mutations);
    let xml_new = doc
        .apply_extended(replacers)
        .expect("apply should not fail");
    assert_eq!(xml_expected, xml_new);
}
