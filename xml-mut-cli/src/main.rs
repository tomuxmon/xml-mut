use roxmltree::*;
use xml_mut_parse::prelude::*;

fn main() {
    let mutation_definition = r###"get ItemGroup/PackageReference as p
where exists p/version
set p@version = p/version@>text
delete p/version"###;

    let xml = r###"<ItemGroup>

    <PackageReference Include="Microsoft.CodeAnalysis.Analyzers">
        <version>3.3.3</version>
    </PackageReference>

    <PackageReference Include="Microsoft.CodeAnalysis.BannedApiAnalyzers">
    <version>3.3.3</version>
    </PackageReference>

    <PackageReference Include="Microsoft.CodeAnalysis.CSharp.Workspaces" Version="4.4.0" />

</ItemGroup>"###;

    let (_, mutation) = mutation(mutation_definition).expect("could not parse mutation");
    println!("{:?}", mutation);

    let doc = Document::parse(xml).unwrap();

    for node in doc
        .descendants()
        .filter(|n| n.has_tag_name(mutation.get.node_selector.path[0].as_str()))
    {
        println!("{:?}", node);
    }

    println!("Hello, world!");
}
