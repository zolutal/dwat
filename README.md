# dwat

A fairly performant library intended to make DWARF (v4/v5) debugging information more accessible.

My focus so far has been on making the type information (specifically structs) present in DWARF info easier to work with, so functionality related to that is largely what is implemented at this point.

**Current Features**:
- Get a list of types by name
- Get a map of types by name
- Recursive formating of parsed struct and union information to C style definitions
- Get members of structs/unions
- Get underlying types of modifiers (volatile/const/etc...)
- Get byte size information for types
- Get bit sizes for bit field struct members

# Examples

There are several examples in the `examples` directory, the following will roughly describe the `lookup` example.

---


The first step of using the library is to load the file containing DWARF info into memory, and then invoke `Dwarf::parse`:

```rust
    let file = File::open(path)?;
    let mmap = unsafe { Mmap::map(&file) }?;

    let dwarf = Dwarf::parse(&*mmap)?;
```

The dwarf object has a `lookup_item` method that can be used to lookup any type implementing the `Tagged` trait by name, in this case a struct will be searched for:

```rust
    let found = dwarf.lookup_item::<dwat::Struct>(struct_name)?;
```

A struct object can be converted to a C style definition String by invoking the `to_string` function:

```rust
    if let Some(found) = found {
        println!("{}", found.to_string(&dwarf)?);
    }
```

This is the result for the `ntb_ctrl_regs` struct found in the Linux kernel which is formatted well despite its moderate complexity:

```
┌──(zolutal@ubuntu)-[~/repos/dwat]
└─$ cargo run --release --example=lookup ntb_ctrl_regs ./vmlinux
Finished release [optimized] target(s) in 0.01s
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
    u32 req_id_table[512];
    u32 reserved3[256];
    u64 lut_entry[512];
};
```

It is also possible to format structs in a verbose mode:

```rust
    if let Some(found) = found {
        println!("{}", found.to_string_verbose(&dwarf, 1)?);
    }
```


Which will add size and offset information, similar to the output of other tools (pahole, gdb, etc...)

```
struct ntb_ctrl_regs {
    u32 partition_status;                       	/*     4 |     0 */
    u32 partition_op;                           	/*     4 |     4 */
    u32 partition_ctrl;                         	/*     4 |     8 */
    u32 bar_setup;                              	/*     4 |    12 */
    u32 bar_error;                              	/*     4 |    16 */
    u16 lut_table_entries;                      	/*     2 |    20 */
    u16 lut_table_offset;                       	/*     2 |    22 */
    u32 lut_error;                              	/*     4 |    24 */
    u16 req_id_table_size;                      	/*     2 |    28 */
    u16 req_id_table_offset;                    	/*     2 |    30 */
    u32 req_id_error;                           	/*     4 |    32 */
    u32 reserved1[7];                           	/*    28 |    36 */
    struct {
        u32 ctl;                                	/*     4 |    64 */
        u32 win_size;                           	/*     4 |    68 */
        u64 xlate_addr;                         	/*     8 |    72 */
    } bar_entry[6];                             	/*    96 |    64 */
    struct {
        u32 win_size;                           	/*     4 |   160 */
        u32 reserved[3];                        	/*    12 |   164 */
    } bar_ext_entry[6];                         	/*    96 |   160 */
    u32 reserved2[192];                         	/*   768 |   256 */
    u32 req_id_table[512];                      	/*  2048 |  1024 */
    u32 reserved3[256];                         	/*  1024 |  3072 */
    u64 lut_entry[512];                         	/*  4096 |  4096 */

    /* total size: 8192 */
};
```

