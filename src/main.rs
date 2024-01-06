use clap::{Parser, Subcommand};
use std::path::PathBuf;
use memmap2::Mmap;
use std::fs::File;
use dwat::Dwarf;

#[derive(Parser)]
struct CmdArgs {
    #[clap(subcommand)]
    commands: Commands

}

#[derive(Subcommand)]
enum Commands {
    /// Find and display a single struct
    Lookup {
        /// Path to the DWARF file
        #[clap(help = "The path to the file containing DWARF info.")]
        dwarf_file: PathBuf,

        /// The name of the struct to lookup
        #[clap(help = "The name of the struct to lookup.")]
        name: String,
    },
    /// Find and display all structs
    Dump {
        /// Path to the DWARF file
        #[clap(help = "The path to the file containing DWARF info.")]
        dwarf_file: PathBuf,
    },
}

fn main() -> anyhow::Result<()> {
    let args = CmdArgs::parse();

    match args.commands {
        Commands::Lookup { dwarf_file, name } => {
            let file = File::open(dwarf_file)?;
            let mmap = unsafe { Mmap::map(&file) }?;

            let mut dwarf = Dwarf::load(&*mmap)?;

            let res = dwarf.lookup_type::<dwat::Struct>(name.clone())?;
            if let Some(struc) = res {
                println!("{}", struc.to_string(&dwarf)?);
                std::process::exit(0);
            } else {
                println!("Could not find struct: {name}");
                std::process::exit(1);
            }
        },
        Commands::Dump { dwarf_file } => {
            let file = File::open(dwarf_file)?;
            let mmap = unsafe { Mmap::map(&file) }?;

            let dwarf = Dwarf::load(&*mmap)?;

            let struct_map = dwarf.get_fg_named_structs_map()?;

            for struc in struct_map.values() {
                println!("{}", struc.to_string(&dwarf)?)
            }
            std::process::exit(0)
        }
    };
}
