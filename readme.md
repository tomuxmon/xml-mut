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

## CLI Usage Example

// TODO

## Architexture

// TODO

## Is it stable

it is still version `0.0.0`. But you can try it our and report any issues you had.

## License

- MIT License ([LICENSE-MIT](LICENSE-MIT) or [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or [http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0))
