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

Now that you know the language lets put it into use with some CLI. You can install the CLI tool using cargo install. In case you have not installed the Rust language yet [do it](https://www.rust-lang.org/tools/install) first.

```bash
 cargo install xml-mut-cli --git https://github.com/tomuxmon/xml-mut
```

Some day it willl be mature enough, and we will be able to ommit the `--git` part.

## CLI Usage Examples

When using CLI you need to supply a path to your XML mutation file. A single XML mutation file can contain multiple mutation definitions. Next, you decide if you want to specify XML files one by one and use the [include command](#include-command) or scan the directory and use the [scan command](#scan-command). You can consult how to use CLI with a `help` call.

```bash
xml-mut --help
```

### include command

You can consult how to use the include command with a `help` call:

```bash
xml-mut include --help
```

The usage is as follows:

```bash
xml-mut <XML_MUT_PATH> include --xml-path <XML_PATH>
```

Here `<XML_MUT_PATH>` is a path to your XML mutation file. You can give it `.xmlmut` extension but it is not mandatory so far. `--xml-path` or `-x` argument can be repeated allowing you to include multiple XML files to be mutated. So a call could look something like this:

```bash
xml-mut ~/pref-version-fix.xmlmut include -x ~/code/awesome.csproj -x ~/code/amazing.fsproj
```

### scan command

You can consult how to use the `scan` command with a `help` call:

```bash
xml-mut scan --help
```

The usage is as follows:

```bash
xml-mut <XML_MUT_PATH> scan --extension <EXTENSION> <BASE_PATH>
```

Here `<XML_MUT_PATH>` is a path to your XML mutation file. You can give it `.xmlmut` extension but it is not mandatory so far. `--extension` or `-e` allows specifying what file extensions to include when scanning the directory. You can specify multiple extensions. `<BASE_PATH>` defines a path you want to scan. So a call could look something like this:

```bash
xml-mut ~/pref-version-fix.xmlmut scan -e csproj -e fsproj ~/code
```

## Architexture

There are 4 crates so far. [xml-mut-data](xml-mut-data/) - is where all data structures of the XML mutation language reside. [xml-mut-parse](xml-mut-parse/) crate uses [nom](https://github.com/rust-bakery/nom) to parse XML mutation definition. [xml-mut-xot](xml-mut-xot/) uses [xot](https://github.com/faassen/xot) (XML Object Tree) and extends it to be able to process XML with mutation definitions. [xml-mut-cli](xml-mut-cli/) uses [clap](https://github.com/clap-rs/clap) to combine both XML mutation parsing and read-only XML extensions to process XML with mutation definitions. All of this is just in a proof of concept stage and will likely change in the future.

## Is it stable

it is still version `0.1.0`. But it is as stable as [xot](https://github.com/faassen/xot) is regarding producing valid xml.

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
