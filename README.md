# dwat

A fairly performant library intended to make DWARF (v4/v5) debugging information more accessible. 

My focus so far has been on making the type information (specifically structs) present in DWARF info easier to work with, so functionality related to that is largely what is implemented at this point.

**Current Features**:
- Get a dictionary of all structs
- Get a list of all variables
- Basic formating of parsed struct information to C style struct definitions

**TODO**:
- test array lengths more thoroughly
- get return types of subprocedures
- support printing more than just structs (some more generic formatting function?)
- make size information on types accessible
- many other things


# Examples

There are several examples in the `examples` directory. The following will roughly describe the `lookup` example in that directory.

---

The first step of using the library is to load the file containing DWARF info into memory, and then invoke `Dwarf::parse`:

```rust
    let file = File::open(path)?;
    let mmap = unsafe { Mmap::map(&file) }?;

    let dwarf = Dwarf::parse(&*mmap)?;
```

You can then, for example, use the `lookup_struct` function on the Dwarf object to get an object representing a struct in the dwarf information with the specified name:

```rust
    let found = dwarf.lookup_struct(struct_name.to_string())?;
```

A struct object can be printed as a C style definition by invoking the `dwat::format::print_struct` function:

```rust
    if let Some(found) = found {
        print_struct(&dwarf, found)?;
    }
```

This is the result for the `ntb_ctrl_regs` struct found in the Linux kernel:

```
┌──(zolutal@ubuntu)-[~/repos/dwat]
└─$ cargo run --release --example=lookup ntb_ctrl_regs ./vmlinux
    Finished release [optimized] target(s) in 0.03s
     Running `target/release/examples/lookup ntb_ctrl_regs ./vmlinux`
struct ntb_ctrl_regs {
        u32 partition_status;
        u32 partition_op;
        u32 partition_ctrl;
        u32 bar_setup;
        u32 bar_error;
        u16 lut_table_entries;
        u16 lut_table_offset;
        u32 lut_error;
        u16 req_id_table_size;
        u16 req_id_table_offset;
        u32 req_id_error;
        u32 reserved1[7];
        struct {
                u32 ctl;
                u32 win_size;
                u64 xlate_addr;
        } bar_entry[6];
        struct {
                u32 win_size;
                u32 reserved[3];
        } bar_ext_entry[6];
        u32 reserved2[192];
        u32 req_id_table[];
        u32 reserved3[256];
        u64 lut_entry[];
};
```
