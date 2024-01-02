use std::fs::File;
use memmap2::Mmap;

use dwat::prelude::*;
use dwat::Dwarf;

fn main() -> anyhow::Result<()> {
    let mut args = std::env::args().skip(1);
    let path = args.next().unwrap_or_else(|| {
        eprintln!("Usage: dump <path>");
        std::process::exit(1);
    });

    let file = File::open(path)?;
    let mmap = unsafe { Mmap::map(&file) }?;

    let dwarf = Dwarf::parse(&*mmap)?;
    let struct_map = dwarf.get_named_items_map::<dwat::Struct>()?;

    for (name, dwstruct) in struct_map.into_iter() {
        let members = dwstruct.members(&dwarf)?;
        println!("{}\t{}", name, members.len());
    };

    Ok(())
}
