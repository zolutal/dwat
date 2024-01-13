# dwat

A fairly performant library intended to make DWARF (v4/v5) debugging information more accessible.

My focus so far has been on making the type information (specifically structs) present in DWARF info easier to work with, so functionality related to that is largely what is implemented at this point.

**Current Features**:
- Get a list of types by name
- Get a map of types by name
- Lookup types by name
- Formating of parsed struct and union information to C-style definitions
- Get members of structs/unions
- Get underlying types of modifiers (volatile/const/etc...)
- Get byte size information for types
- Get bit sizes for bit field struct members

# CLI

Though `dwat` is primarily meant to be a library, a basic cli is included:

```
Usage: dwat <COMMAND>

Commands:
  lookup  Find and display a single struct
  dump    Find and display all structs
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

# Python bindings

`dwat` has python bindings! The documentation can be found here: https://zolutal.github.io/dwat/

# Examples

There are several examples in the `examples` directory that are worth checking out.

# Usage

The first step of using the library is to load the file containing DWARF info into memory, then invoke `Dwarf::load`:

```rust
    let file = File::open(path)?;
    let mmap = unsafe { Mmap::map(&file) }?;

    let dwarf = Dwarf::load(&*mmap)?;
```

The dwarf object has a `lookup_type` method that can be used to lookup any type implementing the `Tagged` trait by name, in this case a struct will be searched for:

```rust
    let found = dwarf.lookup_type::<dwat::Struct>(struct_name)?;
```

Struct members can then be retrieved by calling `.members()` which returns a Vector of `Member` structs.

```rust
    let members = struc.members(&dwarf)?;
```

A struct object can be converted to a C-style definition String by invoking the `to_string` function:

```rust
    if let Some(found) = found {
        println!("{}", found.to_string(&dwarf)?);
    }
```
