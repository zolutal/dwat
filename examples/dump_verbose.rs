use memmap2::Mmap;
use std::fs::File;
use dwat::Dwarf;


fn main() -> anyhow::Result<()> {
    let mut args = std::env::args().skip(1);
    let path = args.next().unwrap_or_else(|| {
        eprintln!("Usage: dump_verbose <path>");
        std::process::exit(1);
    });

    let file = File::open(path)?;
    let mmap = unsafe { Mmap::map(&file) }?;
    let dwarf = Dwarf::parse(&*mmap)?;

    let struct_map = dwarf.get_named_items_map::<dwat::Struct>()?;

    for (_, struc) in struct_map.into_iter() {
        println!("{}", struc.to_string(&dwarf)?);
    }

    Ok(())
}
