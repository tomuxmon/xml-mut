use roxmltree::Document;
use std::{env, fs, path::PathBuf};
use xml_mut_data::{Mutation, Statement};
use xml_mut_impl::prelude::DocumentExt;
use xml_mut_parse::prelude::*;

fn with_input_expect_xml_mutation_output(
    common_test_path: &str,
    xml_input_path: &str,
    xml_mut_path: &str,
    xml_output_path: &str,
) {
    let mut common_path = env::current_dir().expect("could not get the current dir");
    if !common_path.ends_with("xml-mut-cli"){
        common_path.push("xml-mut-cli")
    }
    common_path.push("tests");
    common_path.push(common_test_path);

    let mut xml_input_path_buf = common_path.clone();
    xml_input_path_buf.push(xml_input_path);

    let mut xml_output_path_buf = common_path.clone();
    xml_output_path_buf.push(xml_output_path);

    let mut xml_mut_path_buf = common_path.clone();
    xml_mut_path_buf.push(xml_mut_path);

    let xml_string = fs::read_to_string(xml_input_path_buf).expect("xml file should exist");

    let xml_doc = Document::parse(xml_string.as_str()).expect("should be avalid xml");

    let xml_expected_string =
        fs::read_to_string(xml_output_path_buf).expect("xml output file should exist");

    let xml_mut_string = fs::read_to_string(xml_mut_path_buf).expect("xml mutation file should exist");
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
    let xml_new_string = xml_doc.apply(replacers).expect("apply should not fail");

    // fs::write(xml_output_path, xml_new_string.clone()).expect("nu nesamone");

    assert_eq!(xml_expected_string, xml_new_string);
}

#[test]
fn package_ref_version_mutation() {
    with_input_expect_xml_mutation_output("package_ref_version", "in.xml", "mut.xmlmut", "out.xml");
}

#[test]
fn package_ref_to_project_ref_mutation() {
    with_input_expect_xml_mutation_output(
        "package_ref_to_project_ref",
        "in.xml",
        "mut.xmlmut",
        "out.xml",
    );
}

#[test]
fn package_ref_to_project_ref_sub_node_mutation() {
    with_input_expect_xml_mutation_output(
        "package_ref_to_project_ref_sub_node",
        "in.xml",
        "mut.xmlmut",
        "out.xml",
    );
}

#[test]
fn set_text_sub_node_exists_mutation() {
    with_input_expect_xml_mutation_output(
        "set_text_sub_node_exists",
        "in.xml",
        "mut.xmlmut",
        "out.xml",
    );
}

#[test]
fn package_ref_version_single_line() {
    with_input_expect_xml_mutation_output(
        "package_ref_version_single_line",
        "in.xml",
        "mut.xmlmut",
        "out.xml",
    );
}
