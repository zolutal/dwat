/// Example of doing variable analysis to find those which
/// have types that are pointer arrays
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
    let dwarf = Dwarf::parse(&*mmap)?;

    let vars = dwarf.get_named_items::<dwat::Variable>()?;

    // find all variables that are of type union
    // then print the union
    for (name, var) in vars.into_iter() {
        let typ = var.get_type(&dwarf)?;
        if let Some(dwat::MemberType::Union(u)) = typ {
            println!("{} : {}", name, u.to_string(&dwarf)?)
        }
    };

    Ok(())
}
