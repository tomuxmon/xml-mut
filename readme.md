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

```sql
GET {node_path}
```

`get` is mandatory clause containing only the path of the node you intend to mutate. A simple example below:

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

Where clause allows filtering down desired nodes when node name match is not enough. You can have multiple predicates and you have to separate them with `and`.

```sql
{predicate} and {predicate} and {predicate} ...
```

There are 2 kinds of predicates. `EXISTS` and `EQUALS`.

```sql
{predicate} ::= {predicate_exists} | {predicate_equals}
```

#### Exists

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

this will instead pick `<Sweet name="Lolipop">` node from the XML above.

#### Mixing and matching

You can include as many predicates as you need so where clause like below is valid.

```sql
WHERE [@name] == "Lolipop" 
    and EXISTS Sprinkles
    and EXISTS Filler
```

### SET

// tood

### DELETE

// tood

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

## License

- MIT License ([LICENSE-MIT](LICENSE-MIT) or [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or [http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0))
