# XML mut

XML mut (XML mutation) - a simple XML mutation definition language resembling SQL. Define your simple XML transformations in an easy and readable way.

## Example

Let us say you have some simple XML, but you are still not happy and you would like to simplify it a bit more. In the bellow XML, you know that the `PackageReference` sub-node's `Version` text could just be placed directly as an attribute of `PackageReference`. But you are too lazy to do it manually 😒.

```xml
<Project>
    <ItemGroup>
        <PackageReference Include="System.Text.Json">
            <Version>7.0.2</Version>
        </PackageReference>
        <PackageReference Include="Mono.Cecil">
            <Version>0.11.4</Version>
        </PackageReference>
    </ItemGroup>
</Project>
```

So instead you use xml-mut to do the work for you. You have seen some SQL in the past, and this task should be trivial. You brace yourself 💪 and write a simple XML mutation.

```sql
GET Project/ItemGroup/PackageReference
SET [@Version] = Version[text]
DELETE Version
```

Here we are saying we need to `get` an XML node having a path of `Project`, `ItemGroup`, and `PackageReference` (`PackageReference` node will be the target of the mutation). For that node, we set the `Version` attribute (`[@Version]`) with a value from the `Version` sub-node text (`Version[text]`). Since the `Version` sub-node is no longer needed we delete it. Applying an XML mutation we get XML shown below 🥧.

```xml
<Project>
    <ItemGroup>
        <PackageReference Include="System.Text.Json" Version="7.0.2"/>
        <PackageReference Include="Mono.Cecil" Version="0.11.4"/>
    </ItemGroup>
</Project>
```

Your eyes hurt much less looking into XML above 😎. The [syntax](syntax.md) reminds SQL. It is plain and simple.

## CLI Instalation

Lets put it into use with a [CLI](cli.md). You can install the tool using cargo install. In case you have not installed the Rust language yet [do it](https://www.rust-lang.org/tools/install) first.

```bash
 cargo install xml-mut-cli --git https://github.com/tomuxmon/xml-mut
```

You can consult how to use CLI with a `help` call.

```bash
xml-mut --help
```

Details are in the [cli docs](cli.md).

## Architexture

There are 4 crates so far. [xml-mut-data](xml-mut-data/) - is where all data structures of the XML mutation language reside. [xml-mut-parse](xml-mut-parse/) crate uses [nom](https://github.com/rust-bakery/nom) to parse XML mutation definition. [xml-mut-xot](xml-mut-xot/) uses [xot](https://github.com/faassen/xot) (XML Object Tree) and extends it to be able to process XML with mutation definitions. [xml-mut-cli](xml-mut-cli/) uses [clap](https://github.com/clap-rs/clap) to combine both XML mutation parsing and xot extensions to process XML with mutation definitions.

## Is it stable

It is as stable as [xot](https://github.com/faassen/xot) is regarding producing valid xml. Worth noting that [xml declaration will be lost](xml-mut-cli/tests/delete_with_preserve/in.xml) in a process of transformation and some white space inside xml tag will be removed.

## But tell me why

Currently, the only option to transform your XML is using [XSLT](https://www.w3.org/TR/xslt-30/). Most of the time it is overkill. Let us  compare with an example we had [in the beggining](#example). What we actually wanted is a "simple xslt to get version sub node and place it as an attribute of the node". It has been a long long time since I wrote XSLT. So instead we will [cheat and ask AI to write it for us](https://sl.bing.net/eg2eoXzw2dU).

```xml
<xsl:template match="node()">
    <xsl:copy>
        <xsl:apply-templates select="@*|node()"/>
    </xsl:copy>
</xsl:template>

<xsl:template match="version">
    <xsl:attribute name="version">
        <xsl:value-of select="."/>
    </xsl:attribute>
</xsl:template>
```

This does not look simple. We have to learn the complicated XSLT language and the syntax. Also, XSLT is same old XML. Instead, for simple transformations, simple readable definitions should be enough. Above XSLT could be expressed in a much simpler way with xml-mut below.

```sql
GET Project/ItemGroup/PackageReference
SET [@Version] = Version[text]
DELETE Version
```

So xml-mut is trying to bring the simplicity of SQL (only the simple parts 😅) into XML transformation land.

## License

- MIT License ([LICENSE-MIT](LICENSE-MIT) or [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or [http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0))
