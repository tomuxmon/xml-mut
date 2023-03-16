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

    let xut = fs::read_to_string(mut_cli.xml_mut_path.clone())?;

    let (_, ref statements) = statements(xut.as_str()).expect("could not parse statements");
    let mutations = &statements
        .iter()
        .filter_map(|s| match s {
            Statement::Mutation(rep) => Some(rep),
            _ => None,
        })
        .collect::<Vec<&Mutation>>();

    for xml_path in mut_cli.scan().iter() {
        let xml = fs::read_to_string(xml_path.clone())?;
        let doc: Document = Document::parse(xml.as_str())?;
        let replacers = &doc.get_replacers_all(mutations);

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
            let trim_replacers = &new_doc.get_end_tag_trim_replacers();
            let final_text = if let Some(n) = new_doc.apply(trim_replacers) {
                n
            } else {
                new_xml
            };

            let replacers_len = replacers.len() + trim_replacers.len();

            match fs::write(xml_path, final_text) {
                Ok(_) => println!("{:?} - updated with {} replaces", xml_path, replacers_len),
                Err(err) => println!(
                    "{:?} - updated with {} replaces, BUT failed writing to disk: {}",
                    xml_path, replacers_len, err
                ),
            }
        } else {
            println!("{:?} - no changes", xml_path);
        }
    }

    Ok(())
}
