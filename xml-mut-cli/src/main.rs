use crate::cli::MutCli;
use clap::Parser;
use std::fs;
use xml_mut_data::{Mutation, Statement};
use xml_mut_parse::prelude::*;
use xml_mut_xot::prelude::Valueable;
use xot::Xot;

mod cli;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // TODO: a name of mutation defination file in a common folder ~/.xml-mut/

    let mut_cli = MutCli::parse();

    let xut = fs::read_to_string(mut_cli.xml_mut_path.clone())?;

    let (non_parsed, ref grammar) =
        xml_mut_grammar(xut.as_str()).expect("could not parse statements");

    let mutations = &grammar
        .statements
        .iter()
        .filter_map(|s| match s {
            Statement::Mutation(rep) => Some(rep),
            _ => None,
        })
        .collect::<Vec<&Mutation>>();

    if !non_parsed.is_empty() {
        println!(
            "Parsed xml mut out of '{:?}', but there is still remainder left: \n{}",
            mut_cli.xml_mut_path, non_parsed
        );
    }

    for xml_path in mut_cli.scan().iter() {
        let xml = fs::read_to_string(xml_path.clone())?;
        let mut xot = Xot::new();
        let root = xot.parse(xml.as_str())?;
        let doc_element_node = xot.document_element(root)?;
        xot.get_operations_all(doc_element_node, mutations)?;
        let mut xml_file = fs::File::open(xml_path.clone())?;
        xot.serialize_xml_write(Default::default(), root, &mut xml_file)?;
        // TODO: count number of mutations applied
        println!("{:?} - updated", xml_path);
    }

    Ok(())
}

// fn get_nodes_as_text(
//     paths: Vec<std::path::PathBuf>,
//     mutations: &[&Mutation],
// ) -> Result<String, Box<dyn std::error::Error>> {
//     let mut nodes: Vec<String> = Vec::new();

//     for xml_path in paths {
//         let xml = fs::read_to_string(xml_path.clone())?;
//         let doc: Document = Document::parse(xml.as_str())?;
//         let mut found = doc.get_fit_nodes_all(mutations);
//         println!("{} found in {:?}", found.len(), xml_path,);
//         nodes.append(&mut found);
//     }

//     Ok(nodes
//         .iter()
//         .unique_by(|p| *p)
//         .sorted_by(|a, b| Ord::cmp(&a, &b))
//         .join("\n"))
// }
