use dwat::format::print_struct;
use dwat::Dwarf;

use std::fs::File;
use memmap2::Mmap;

fn main() -> anyhow::Result<()> {
    let mut args = std::env::args().skip(1);
    let path = args.next().unwrap_or_else(|| {
        eprintln!("Usage: dump_verbose <path>");
        std::process::exit(1);
    });

    let file = File::open(path)?;
    let mmap = unsafe { Mmap::map(&file) }?;
    let dwarf = Dwarf::parse(&*mmap)?;

    let struct_map = dwarf.get_named_structs()?;

    for (_, dwstruct) in struct_map.into_iter() {
        print_struct(&dwarf, dwstruct)?;
    }

    Ok(())
}
