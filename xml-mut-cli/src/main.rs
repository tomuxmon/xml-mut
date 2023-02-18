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

    let (_, mutation) = mutation(mutation_definition).expect("could not parse mutation");
    let doc: Document = Document::parse(xml).expect("could not parse xml");

    let mut replacers: Vec<Replacer> = vec![];
    for node in doc.descendants().filter(|n| n.is_fit(&mutation)) {
        match node.apply(&mutation) {
            Ok(replacer) => replacers.push(replacer),
            Err(error) => println!("{error:?}"), // todo: better error handling
        }
    }

    let mut new_xml = String::with_capacity(xml.len());
    let mut offset = 0;
    for replacer in replacers {
        new_xml.push_str(&doc.input_text()[offset..replacer.bounds.start]);
        new_xml.push_str(&replacer.replacement);
        offset = replacer.bounds.end;
    }
    new_xml.push_str(&doc.input_text()[offset..doc.input_text().len()]);

    println!("{new_xml}");
}
