use dwat::format::print_struct;
use dwat::Dwarf;
use std::fs::File;
use memmap2::Mmap;

fn main() -> anyhow::Result<()> {
    let mut args = std::env::args().skip(1);
    let struct_name = args.next().unwrap_or_else(|| {
        eprintln!("Usage: lookup <struct_name> <path>");
        std::process::exit(1);
    }).to_string();
    let path = args.next().unwrap_or_else(|| {
        eprintln!("Usage: lookup <struct_name> <path>");
        std::process::exit(1);
    });

    let file = File::open(path).unwrap();
    let mmap = unsafe { Mmap::map(&file) }.unwrap();
    let mut dwarf = Dwarf::parse(&*mmap)?;

    // some good test cases:
    // compat_rusage
    // file_system_type
    // trace_event_raw_itimer_expire
    // sev_config
    // mca_config
    // ntb_ctrl_regs

    let found = dwarf.lookup_item::<dwat::Struct>(struct_name)?;
    if let Some(found) = found {
        print_struct(&dwarf, found)?;
    }

    Ok(())
}
