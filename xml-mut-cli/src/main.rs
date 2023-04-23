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
        let doc: Document = Document::parse(xml.as_str())?;
        let replacers = &doc.get_replacers_all(mutations);

        match doc.apply(replacers) {
            Ok(final_xml) => match fs::write(xml_path, final_xml) {
                Ok(_) => println!("{:?} - updated with {} replaces", xml_path, replacers.len()),
                Err(io_err) => println!(
                    "{:?} - updated with {} replaces, BUT failed writing to disk: {}",
                    xml_path,
                    replacers.len(),
                    io_err
                ),
            },
            Err(err) => match err {
                ReplaceError::NoChange => println!("{:?} - no changes", xml_path),
                ReplaceError::ReplacerOverlap(r1, r2) => println!(
                    "{:?} - replacer {:?} overlaps with replacer {:?}",
                    xml_path, r1, r2
                ),
                ReplaceError::GeneratedXmlInvalid(xml_err) => {
                    println!("{:?} - generated xml is invalid: {}", xml_path, xml_err)
                }
            },
        };
    }

    Ok(())
}
