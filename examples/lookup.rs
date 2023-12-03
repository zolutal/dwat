use dwat::format::print_struct;
use dwat::Dwarf;
use std::fs::File;
use memmap2::Mmap;

fn main() -> anyhow::Result<()> {
    let mut args = std::env::args().skip(1);
    let struct_name = args.next().unwrap_or_else(|| {
        eprintln!("Usage: lookup <struct_name> <path>");
        std::process::exit(1);
    });
    let path = args.next().unwrap_or_else(|| {
        eprintln!("Usage: lookup <struct_name> <path>");
        std::process::exit(1);
    });

    let file = File::open(path).unwrap();
    let mmap = unsafe { Mmap::map(&file) }.unwrap();
    let mut parser = Dwarf::parse(&*mmap)?;

    // potential test cases:
    //let found = parser.lookup_struct("compat_rusage".to_string())?;
    //let found = parser.lookup_struct("file_system_type".to_string())?;
    //let found = parser.lookup_struct("trace_event_raw_itimer_expire".to_string())?;
    //let found = parser.lookup_struct("sev_config".to_string())?;
    //let found = parser.lookup_struct("mca_config".to_string())?;
    //let found = parser.lookup_struct("ntb_ctrl_regs".to_string())?;

    let found = parser.lookup_struct(struct_name.to_string())?;
    if let Some(found) = found {
        print_struct(&parser, found)?;
    }

    Ok(())
}
