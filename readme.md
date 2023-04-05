# xml mut

xml mut (XML mutation) - a simple XML mutation definition language resembling SQL. Define your simple XML transformations the easy and readable way.

## Example

Lets say you have some simple XML, but you are still not happy and you would like to simplify it a bit more. In the bellow XML you know that `PackageReference` sub node's `Version` text could just be placed directly as an attribute of `PackageReference`. But you are too lazy to do it manually 😒.

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

Here we are saying we need to `get` xml node having a path of `Project`, `ItemGroup` and `PackageReference` (`PackageReference` node will be the target of the mutation). For that node we set `Version` attribute (`[@Version]`) with value from the `Version` sub node text (`Version[text]`). Since the `Version` sub node is no longer needed we delete it. Applying an XML mutation we get XML shown below 🥧.

```xml
<Project>
    <ItemGroup>
        <PackageReference Include="System.Text.Json" Version="7.0.2"/>
        <PackageReference Include="Mono.Cecil" Version="0.11.4"/>
    </ItemGroup>
</Project>
```

Your eyes hurt much less looking into XML above 😎.

## Mutation syntax

The syntax reminds SQL. It is plain and simple. It all starts with `GET`.

### GET

[Get syntax](xml-mut-parse/src/get_clause.rs) is expressed like shown below:

```sql
GET {node_path}
```

`GET` is mandatory clause containing only the path of the node you intend to mutate. A simple example below:

```sql
GET Candy/Sweet
```

Here it will try to find all `Sweet` XML nodes having a parent node `Candy`. So if we had XML as below, we would match on 2 nodes. `<Sweet name="Caramel"/>` and `<Sweet name="Lolipop" />`

```xml
<KidsJoy>
    <Candy>
        <Sweet name="Caramel"/>
        <Sweet name="Lolipop" />
        <Salty name="Lacris" />
    </Candy>
    <Sweet name="Potato"/>
</KidsJoy>
```

`<Sweet name="Potato"/>` does not match because it does not have a parent of `Candy`.

### WHERE

```sql
WHERE {predicates}
```

[Where clause](xml-mut-parse/src/where_clause.rs) allows filtering down desired nodes when node name match is not enough. You can have multiple predicates and you have to separate them with `and`.

```sql
{predicate} and {predicate} and {predicate} ...
```

There are 2 kinds of predicates. `EXISTS` and `EQUALS`.

```sql
{predicate} ::= {predicate_exists} | {predicate_equals}
```

#### Exists

Exists predicate syntax is expressed like shown below:

```sql
{predicate_exists} ::= EXISTS {node_path}
```

A simple example where clause with exists predicate:

```sql
WHERE EXISTS Sprinkles/Round
```

Here it will require that the node would contain sub node `Sprinkles` and then `Sprinkles` would contain sub node `Round`. This is similar to how `GET` works but instead of requiring path up to the node (parent path) it requires a path down the node (child path). So if we would combine with previous `GET` we would get this:

```sql
GET Candy/Sweet
WHERE EXISTS Sprinkles/Round
```

and if we had XML like below we would match on 1 node `<Sweet name="Lolipop">`.

```xml
<KidsJoy>
    <Candy>
        <Sweet name="Caramel" />
        <Sweet name="Lolipop">
            <Sprinkles>
                <Round />
            </Sprinkles>
        </Sweet>
        <Salty name="Lacris" />
    </Candy>
    <Sweet name="Potato"/>
</KidsJoy>
```

#### Equals

equals predicate syntax is expressed like shown below:

```sql
{predicate_equals} ::= {value_path} == {value_variant}
{value_path} ::= {node_path}{value_selector}
{value_variant} ::= {value_path} | {value_literal}
{value_literal} ::= {dis_just_a_string}
```

Oh boy it is so unreadable above. Lets look at a simple example of where clause with equals predicate:

```sql
WHERE Sprinkles/Round[@color] == "pink"
```

Here we are saying that our desired node should have children path `Sprinkles/Round`. That is it should have `Sprinkles` sub node and that `Sprinkles` sub node should contain `Round` sub node. Then we say that this `Round` node should have an attribute `color` with literal value `pink`. Let us combine our previous `GET` with this where clause and explore.

```sql
GET Candy/Sweet
WHERE Sprinkles/Round[@color] == "pink"
```

Here we say that we want `Sweet` node with parent node `Candy` and with child node path `Sprinkles/Round` where `Round` attribute `color` is equal to `pink`. So the below XML would match on 1 node `<Sweet name="Caramel">`.

```xml
<KidsJoy>
    <Candy>
        <Sweet name="Caramel">
            <Sprinkles>
                <Round color="pink"/>
            </Sprinkles>
        </Sweet>
        <Sweet name="Lolipop">
            <Sprinkles>
                <Round color="jade"/>
            </Sprinkles>
        </Sweet>
    </Candy>
</KidsJoy>
```

Ofcourse you can always match on direct node attributes.

```sql
GET Candy/Sweet
WHERE [@name] == "Lolipop"
```

this will instead pick `<Sweet name="Lolipop">` node from the XML above. To see what kind of value selector are possible refer to [value selectors](#value-selectors) section.

#### Mixing and matching

You can include as many predicates as you need so where clause like below is valid.

```sql
WHERE [@name] == "Lolipop" 
    and EXISTS Sprinkles
    and EXISTS Filler
```

### SET

Set is where mutation part begins. [set clause](xml-mut-parse/src/set_clause.rs) syntax is expressed like shown below:

```sql
SET {value_assignment}, {value_assignment}, ...
{value_assignment} ::= {value_path} = {value_variant}
```

The only difference form `EQUALS` predicate is it uses a single `=` sign to denote the assignment (and confuse people). The main differnece is how it behaves. Lets look at some examples.

```sql
SET [@name] = "ToughCaramel"
```

Here we are saying we need want to set nodes `name` attribute with literal value `ToughCaramel`.

If we had a full mutation like blow.

```sql
GET Candy/Sweet
WHERE [@name] == "Caramel"
SET [@name] = "ToughCaramel"
```

and would apply it to XML like below.

```xml
<KidsJoy>
    <Candy>
        <Sweet name="Caramel"/>
        <Sweet name="Lolipop" />
    </Candy>
</KidsJoy>
```

we would get the following.

```xml
<KidsJoy>
    <Candy>
        <Sweet name="ToughCaramel"/>
        <Sweet name="Lolipop" />
    </Candy>
</KidsJoy>
```

A simple syntax for a simple task.

### Value selectors

You might notice that both `equals` and `value assignment` end with squere bracket indexer `[]`. Currently it supports 4 types of value selectors.

#### Attribute

Attribute selector starts with `@` sign and looks like this: `[@name]`. Here we are saying we want to pick a value from nodes name attribute. Example in a where clause:

```sql
GET Project/ItemGroup/PackageReference
WHERE [@Include] == "Mono.Cecil"
```

if we had XML like below:

```xml
<Project>
    <ItemGroup>
        <PackageReference Include="System.Text.Json" Version="7.0.2"/>
        <PackageReference Include="Mono.Cecil" Version="0.11.4"/>
    </ItemGroup>
</Project>
```

A single node `<PackageReference Include="Mono.Cecil" Version="0.11.4"/>` would match the predicate.

#### Text

Text selector must always contain `text` literal inside. it laways looks like this: `[text]`. Here we are saying we want to pick nodes text as a value. Example in a set clause:

```sql
GET Project/ItemGroup/PackageReference
SET [@Version] = Version[text]
```

so if we had XML like below:

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

Applying the mutation would result in xml like below:

```xml
<Project>
    <ItemGroup>
        <PackageReference Include="System.Text.Json" Version="7.0.2">
            <Version>7.0.2</Version>
        </PackageReference>
        <PackageReference Include="Mono.Cecil" Version="0.11.4">
            <Version>0.11.4</Version>
        </PackageReference>
    </ItemGroup>
</Project>
```

Here we are missing a `delete` so most probably that is not an intended result. But it demonstrates the transfer of text value from `Version` sub node to `Version` attribute.

#### Tail

like with `text`, `tail` will always look the same: `[tail]`. Here we are saying we want to pick nodes tail as a value. Nodes tail is actually a text after nodes closing tag. Example tail usage below.

```sql
GET Project/ItemGroup
SET PackageReference[tail] = ""
```

If we had XML like below.

```xml
<Project>
    <ItemGroup>
        <PackageReference Include="System.Text.Json"/>
        <PackageReference Include="Mono.Cecil"/>
    </ItemGroup>
</Project>
```

and would apply the mutation the result would be like below.

```xml
<Project>
    <ItemGroup>
        <PackageReference Include="System.Text.Json"/><PackageReference Include="Mono.Cecil"/>
    </ItemGroup>
</Project>
```

#### Name

`name` will also always look the same: `[name]`. It get nodes name as a value. Example name usage below.

```sql
GET Project/ItemGroup/PackageReference
SET [name] = "ProjectReference"
```

If we had XML like below.

```xml
<Project>
    <ItemGroup>
        <PackageReference Include="System.Text.Json"/>
        <PackageReference Include="Mono.Cecil"/>
    </ItemGroup>
</Project>
```

and would apply the mutation the result would be like below.

```xml
<Project>
    <ItemGroup>
        <ProjectReference Include="System.Text.Json"/>
        <ProjectReference Include="Mono.Cecil"/>
    </ItemGroup>
</Project>
```

### DELETE

```sql
DELETE {path_variant}, {path_variant}, ...
```

[Delete clause](xml-mut-parse/src/delete_clause.rs) allows removing parts of an XML. A path variant can be either a node path (target a specific XML to be removed) or a value selector (target a [specific value](#value-selectors) to be removed). An example was already presented in the begining.

```sql
DELETE Version
```

Here we are saying that we want to delete the `Version` sub node. So if we had a full Mutation like below.

```sql
GET Project/ItemGroup/PackageReference
DELETE Version
```

And applied it to the XML like below.

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

The result would be like the one below.

```xml
<Project>
    <ItemGroup>
        <PackageReference Include="System.Text.Json"/>
        <PackageReference Include="Mono.Cecil"/>
    </ItemGroup>
</Project>
```

## CLI Usage Example

When using CLI you need to suply a path to your XML mutation file. A single XML mutation file can contain multiple mutation definitions. Next you decide if you want to specify XML files one by one and use [include command](#include-command) or scan the directory and use [scan command](#scan-command). You can consult how to use CLI with a `help` call.

```bash
xml-mut-cli --help
```

### include command

You can consult how to use include command with a `help` call:

```bash
xml-mut-cli include --help
```

The usage is as follows:

```bash
xml-mut-cli <XML_MUT_PATH> include --xml-path <XML_PATH>
```

Here `<XML_MUT_PATH>` is a path to your XML mutation file. You can give it `.xmlmut` extension but it is not mandatory so far. `--xml-path` or `-x` argument can be repeated allowing you to include multiple XML files to be mutated. So a call could look something like this:

```bash
xml-mut-cli ~/pref-version-fix.xmlmut include -x ~/code/awesome.csproj -x ~/code/amazing.fsproj
```

### scan command

You can consult how to use `scan` command with a `help` call:

```bash
xml-mut-cli scan --help
```

The usage is as follows:

```bash
xml-mut-cli <XML_MUT_PATH> scan --extension <EXTENSION> <BASE_PATH>
```

Here `<XML_MUT_PATH>` is a path to your XML mutation file. You can give it `.xmlmut` extension but it is not mandatory so far. `--extension` or `-e` allows specifying what file extensions to include when scaning directory. You can specify multiple extensions. `<BASE_PATH>` defines a path you want to scan. So a call could look something like this:

```bash
xml-mut-cli ~/pref-version-fix.xmlmut scan -e csproj -e fsproj ~/code
```

## Architexture

There are 4 crates so far. [xml-mut-data](xml-mut-data/) - is where all data structures of the XML mutation language reside. [xml-mut-parse](xml-mut-parse/) crate uses [nom](https://github.com/rust-bakery/nom) to parse XML mutation definition. [xml-mut-impl](xml-mut-impl/) uses [roxmltree](https://github.com/RazrFalcon/roxmltree) (read only xml tree) and extends it to be able to process XML with mutation definitions. [xml-mut-cli](xml-mut-cli/) uses [clap](https://github.com/clap-rs/clap) to combine both XML mutation parsing and read only xml extensions to process XML with mutation definitions. All of this is just in a proof of concept stage and will likely change in the future.

## Is it stable

it is still version `0.0.0`. But you can try it our and report any issues you had.

## But tell me why

Currently the only option to transform your XML is using [XSLT](https://www.w3.org/TR/xslt-30/). Most of the time it is an overkill. So for simple transformations simple readable definitions should be enough. XSLT is nothing but readable. So xml-mut is trying to bring the simplicity of SQL (only the simple parts 😅) into XML transformation land.

## License

- MIT License ([LICENSE-MIT](LICENSE-MIT) or [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or [http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0))
