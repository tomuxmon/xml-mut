use crate::cli::MutCli;
use clap::Parser;
use roxmltree::*;
use std::fs;
use xml_mut_data::{Mutation, Statement};
use xml_mut_impl::prelude::*;
use xml_mut_parse::prelude::*;

mod cli;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // TODO: a name of mutation defination file in a common folder ~/.xml-mut/

    let mut_cli = MutCli::parse();
    
    // println!("MutCli parsed: {mut_cli:?}");

    let xml_paths = mut_cli.scan();

    // println!("xml_paths parsed: {xml_paths:?}");

    for xml_path in xml_paths.into_iter() {
        let xml = fs::read_to_string(xml_path.clone())?;
        let doc: Document = Document::parse(xml.as_str())?;
        let mut replacers: Vec<Replacer> = vec![];

        let xut = fs::read_to_string(mut_cli.xml_mut_path.clone())?;
        let (_, ref statements) = statements(xut.as_str()).expect("could not parse statements");
        let mutations = statements
            .iter()
            .filter_map(|s| match s {
                Statement::Mutation(rep) => Some(rep),
                _ => None,
            })
            .collect::<Vec<&Mutation>>();

        replacers.append(&mut doc.get_replacers_all(mutations));

        if let Some(new_xml) = doc.apply(replacers) {
            // NOTE: reparse resulting xml to validate it.
            let new_doc = match Document::parse(&new_xml) {
                Ok(d) => d,
                Err(error) => {
                    print!("xml is invalid after applying replacers: {error}");
                    return Err(Box::new(error));
                }
            };

            // NOTE: post processing
            let trim_replacers = new_doc.get_end_tag_trim_replacers();
            let final_text = if let Some(n) = new_doc.apply(trim_replacers) {
                n
            } else {
                new_xml
            };

            fs::write(xml_path, final_text).expect("could not write xml");
        } else {
            println!("no changes");
        }
    }

    Ok(())
}
