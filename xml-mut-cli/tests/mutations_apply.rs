use roxmltree::Document;
use std::fs;
use xml_mut_data::{Mutation, Statement};
use xml_mut_impl::prelude::DocumentExt;
use xml_mut_parse::prelude::*;

fn with_input_expect_xml_mutation_output(
    xml_input_path: &str,
    xml_mut_path: &str,
    xml_output_path: &str,
) {
    let xml_string = fs::read_to_string(xml_input_path).expect("xml file should exist");
    let xml_doc = Document::parse(xml_string.as_str()).expect("should be avalid xml");

    let xml_expected_string =
        fs::read_to_string(xml_output_path).expect("xml output file should exist");

    let xml_mut_string = fs::read_to_string(xml_mut_path).expect("xml mutation file should exist");
    let (non_parsed, ref grammar) =
        xml_mut_grammar(xml_mut_string.as_str()).expect("could not parse statements");

    if !non_parsed.is_empty() {
        panic!("non_parsed should be empty but is {:#?}", non_parsed);
    }

    let mutations = &grammar
        .statements
        .iter()
        .filter_map(|s| match s {
            Statement::Mutation(rep) => Some(rep),
            _ => None,
        })
        .collect::<Vec<&Mutation>>();

    let replacers = &xml_doc.get_replacers_all(mutations);
    let xml_new_string = xml_doc
        .apply(replacers)
        .expect("apply should not fail");

    // fs::write(xml_output_path, xml_new_string.clone()).expect("nu nesamone");

    assert_eq!(xml_expected_string, xml_new_string);
}

#[test]
fn package_ref_version_mutation() {
    with_input_expect_xml_mutation_output(
        "tests/package_ref_version/in.xml",
        "tests/package_ref_version/mut.xmlmut",
        "tests/package_ref_version/out.xml",
    );
}

#[test]
fn package_ref_to_project_ref_mutation() {
    with_input_expect_xml_mutation_output(
        "tests/package_ref_to_project_ref/in.xml",
        "tests/package_ref_to_project_ref/mut.xmlmut",
        "tests/package_ref_to_project_ref/out.xml",
    );
}

#[test]
fn set_text_sub_node_exists_mutation() {
    with_input_expect_xml_mutation_output(
        "tests/set_text_sub_node_exists/in.xml",
        "tests/set_text_sub_node_exists/mut.xmlmut",
        "tests/set_text_sub_node_exists/out.xml",
    );
}

#[test]
fn package_ref_version_single_line() {
    with_input_expect_xml_mutation_output(
        "tests/package_ref_version_single_line/in.xml",
        "tests/package_ref_version_single_line/mut.xmlmut",
        "tests/package_ref_version_single_line/out.xml",
    );
}
