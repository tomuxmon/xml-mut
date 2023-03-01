use bpaf::*;
use roxmltree::*;
use std::fs;
use xml_mut_data::{Mutation, Statement};
use xml_mut_impl::prelude::*;
use xml_mut_parse::prelude::*;

fn main() {
    // note: set p@version = p/version@>text should imply where exists p/version
    // since it is required in PredicateEquals.right_side

    //     let xml = r###"<ItemGroup>
    //     <doodles Version="sus"></doodles>
    //     <doodles Version="sus"> </doodles>
    //     <PackageReference Include="zuzu" Version="sus"> bleep <version>3.0</version> bloop </PackageReference>
    //     <PackageReference Include="zuzu" Version="suske"><version>5.0</version> bloop </PackageReference>
    //     <PackageReference Include="zizi"> <Version>3.0</Version> </PackageReference>
    //     <PackageReference Include="zizibottom">
    //         <Version>0.0</Version>
    //     </PackageReference>
    //     <PackageReference><Version>4</Version></PackageReference>
    //     <PackageReference Include="zohan" Version="4.0" />
    // </ItemGroup>"###;

    //     let mutation_definition = r###"get ItemGroup/PackageReference as p
    // where exists p/version
    // set p@version = p/version@>text
    // delete p/version"###;

    // let (_, mutation) = mutation_surounded_multispace0(mutation_definition).expect("could not parse mutation");
    // let doc: Document = Document::parse(xml).expect("could not parse xml");

    // TODO: define how mullation is passed to the CLI
    // 1. isinlined as cli args
    // 2. a path to a mutation definition file
    // 3. a name of mutation defination file in a common folder ~/.xml-mut/
    // 4. can one apply multiple mutations at once with a single call?

    // TODO: define how xml path is passed to the CLI
    // 1. a path to a single xml file
    // 2. a path to a folder of xml files
    // 3. shoul it drill down in to sub folders? (CLI arg for that)

    // relative path
    // absolute path
    // multiple paths
    // single path pattern


    let xml_paths = xml_path_many();
    let xut_paths = xut_path_many();

    let parser = construct!(MutationSet {
        xml_paths,
        xut_paths
    });

    let speed_and_distance = parser
        .to_options()
        .descr("transforms xml files using xut definitions");

    let opts = speed_and_distance.run();

    for xml_path in opts.xml_paths.clone().into_iter() {
        let xml = fs::read_to_string(xml_path.clone()).expect("invalid xml path");
        let doc: Document = Document::parse(xml.as_str()).expect("could not parse xml");
        let mut replacers: Vec<Replacer> = vec![];
        for xut_path in opts.xut_paths.clone().into_iter() {
            let xut = fs::read_to_string(xut_path).expect("invalid xut path");
            let (_, statements) = statements(xut.as_str()).expect("could not parse mutation");
            let mutations = statements
                .into_iter()
                .filter_map(|s| match s {
                    Statement::Mutation(rep) => Some(rep),
                    _ => None,
                })
                .collect::<Vec<Mutation>>();

            replacers.append(&mut doc.get_replacers_all(mutations))
        }
        if let Some(new_xml) = doc.replace_with(replacers) {
            fs::write(xml_path, new_xml).expect("could not write xml");
        } else {
            println!("no changes");
        }
    }
}

#[derive(Clone, Debug)]
struct MutationSet {
    /// unbla
    xml_paths: Vec<String>,
    /// nbla
    xut_paths: Vec<String>,
}

fn xml_path() -> impl Parser<String> {
    long("xml-path")
        .short('p')
        .help("path of an xml file")
        .argument::<String>("PATH")
}

fn xml_path_many() -> impl Parser<Vec<String>> {
    xml_path().many()
}

fn xut_path() -> impl Parser<String> {
    long("xut-path")
        .short('u')
        .help("path of a mutation file, usually with a .xut file extension")
        .argument::<String>("PATH")
}

fn xut_path_many() -> impl Parser<Vec<String>> {
    xut_path().many()
}
