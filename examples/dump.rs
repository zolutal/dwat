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

    let dwarf = Dwarf::load(&*mmap)?;
    let struct_map = dwarf.get_fg_named_structs_map()?;

    for (key, struc) in struct_map.into_iter() {
        let members = struc.members(&dwarf)?.len();
        println!("{}\t{}", key.name, members);
    };

    Ok(())
}
