/// Example of doing variable analysis to find those which
/// have types that are pointer arrays
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

    for (name, var) in vars.into_iter() {
        let typ = var.get_type(&dwarf)?;
        if let Some(dwat::MemberType::Array(a)) = typ {
            if let Some(dwat::MemberType::Pointer(_p)) = a.get_type(&dwarf)? {
                println!("{}", name);
            }
        }
    };

    Ok(())
}
