use memmap2::Mmap;
use std::fs::File;

use dwat::prelude::*;
use dwat::Dwarf;

fn main() -> anyhow::Result<()> {
    let mut args = std::env::args().skip(1);
    let path = args.next().unwrap_or_else(|| {
        eprintln!("Usage: dump_verbose <path> [verbosity-level]");
        std::process::exit(1);
    });
    let verbosity = args.next().unwrap_or_else(|| {
        "0".to_string()
    });

    let verbosity = verbosity.parse::<u8>()?;

    let file = File::open(path)?;
    let mmap = unsafe { Mmap::map(&file) }?;
    let dwarf = Dwarf::load(&*mmap)?;

    let struct_map = dwarf.get_fg_named_structs_map()?;

    for (_, struc) in struct_map.into_iter() {
        println!("{}", struc.to_string_verbose(&dwarf, verbosity)?);
    }

    Ok(())
}
