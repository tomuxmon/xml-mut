use roxmltree::*;
use xml_mut_impl::prelude::*;
use xml_mut_parse::prelude::*;

fn main() {
    // note: set p@version = p/version@>text should imply where exists p/version
    // since it is required in PredicateEquals.right_side

    let xml = r###"<ItemGroup>
    <doodles Version="sus"></doodles>
    <doodles Version="sus"> </doodles>
    <PackageReference Include="zuzu" Version="sus"> bleep <version>3.0</version> bloop </PackageReference>
    <PackageReference Include="zuzu" Version="suske"><version>5.0</version> bloop </PackageReference>
    <PackageReference Include="zizi"> <Version>3.0</Version> </PackageReference>
    <PackageReference Include="zizibottom"> 
        <Version>0.0</Version> 
    </PackageReference>
    <PackageReference><Version>4</Version></PackageReference>
    <PackageReference Include="zohan" Version="4.0" />
</ItemGroup>"###;

    let mutation_definition = r###"get ItemGroup/PackageReference as p
where exists p/version
set p@version = p/version@>text
delete p/version"###;

    // TODO: define how mullation is passed to the CLI
    // 1. isinlined as cli args
    // 2. a path to a mutation definition file
    // 3. a name of mutation defination file in a common folder ~/.xml-mut/

    // TODO: define how xml path is passed to the CLI
    // 1. a path to a single xml file
    // 2. a path to a folder of xml files
    // 3. shoul it drill down in to sub folders? (CLI arg for that)

    let (_, mutation) = mutation(mutation_definition).expect("could not parse mutation");
    let doc: Document = Document::parse(xml).expect("could not parse xml");

    let new_xml = if let Some(mutated_xml) = doc.mutate(&mutation) {
        mutated_xml
    } else {
        println!("nothing to be mutated");
        return;
    };

    println!("{new_xml}");
}
