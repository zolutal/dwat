/// Example of doing variable analysis to all variables that are unions
use dwat::prelude::*;
use dwat::Dwarf;
use std::fs::File;
use memmap2::Mmap;

fn main() -> anyhow::Result<()> {
    let mut args = std::env::args().skip(1);
    let path = args.next().unwrap_or_else(|| {
        eprintln!("Usage: vars <path>");
        std::process::exit(1);
    });

    let file = File::open(path).unwrap();
    let mmap = unsafe { Mmap::map(&file) }?;
    let dwarf = Dwarf::load(&*mmap)?;

    let vars = dwarf.get_named_types::<dwat::Variable>()?;

    // find all variables that are of type union
    // then print the union
    for (name, var) in vars.into_iter() {
        let typ = var.get_type(&dwarf)?;
        if let dwat::Type::Union(u) = typ {
            println!("{} : {}", name, u.to_string(&dwarf)?)
        }
    };

    Ok(())
}
