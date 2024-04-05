# CLI Usage Examples

When using CLI you need to supply a path to your XML mutation file. A single XML mutation file can contain multiple mutation definitions. Next, you decide if you want to specify XML files one by one and use the [include command](#include-command) or scan the directory and use the [scan command](#scan-command). You can consult how to use CLI with a `help` call.

```bash
xml-mut --help
```

## include command

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

## scan command

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
