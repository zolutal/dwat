/// Example of doing variable analysis to find those which
/// have types that are pointer arrays
use rshole_rewrite as rh;
use rh::Dwarf;
use std::fs::File;
use memmap2::Mmap;

fn main() -> anyhow::Result<()> {
    let path = "/home/jmill/kernel-junk/kernel-dbg/vmlinux";
    let file = File::open(path).unwrap();
    let mmap = unsafe { Mmap::map(&file) }?;

    let dwarf = Dwarf::parse(&*mmap)?;

    let vars: Vec<(String, rh::Variable)> = dwarf.get_named_variables()?;

    for (name, var) in vars.into_iter() {
        let typ = var.get_type(&dwarf)?;
        if let Some(rh::MemberType::Array(a)) = typ {
            if let Some(rh::MemberType::Pointer(_p)) = a.get_type(&dwarf)? {
                println!("{}", name);
            }
        }
    };

    Ok(())
}
