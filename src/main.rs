use clap::{Parser, Subcommand};
use std::path::PathBuf;
use dwat::prelude::*;
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

        /// Add comments containing '/* size | offset */' for struct members
        #[clap(long, action, help = "Prints sizes and offsets of struct \
                                     fields.")]
        verbose: bool,
    },
    /// Find and display all structs
    Dump {
        /// Path to the DWARF file
        #[clap(help = "The path to the file containing DWARF info.")]
        dwarf_file: PathBuf,

        /// Add comments containing '/* size | offset */' for struct members
        #[clap(long, action, help = "Prints sizes and offsets of struct \
                                     fields.")]
        verbose: bool,

        /// Use only name for determining uniqueness of structs
        #[clap(long, action, help = "Find unique structs by name only, faster \
                                     but misses cases where multiple structs \
                                     are declared with the same name")]
        fast: bool
    },
}

fn main() -> anyhow::Result<()> {
    let args = CmdArgs::parse();

    match args.commands {
        Commands::Lookup { dwarf_file, name, verbose } => {
            let file = File::open(dwarf_file)?;
            let mmap = &*unsafe { Mmap::map(&file) }?;

            let dwarf = Dwarf::load(mmap)?;

            let verbosity: u8 = verbose.into();

            let res = dwarf.lookup_type::<dwat::Struct>(name.clone())?;
            if let Some(struc) = res {
                println!("{}", struc.to_string_verbose(&dwarf, verbosity)?);
                std::process::exit(0);
            } else {
                println!("Could not find struct: {name}");
                std::process::exit(1);
            }
        },
        Commands::Dump { dwarf_file, verbose, fast } => {
            let file = File::open(dwarf_file)?;
            let mmap = unsafe { Mmap::map(&file) }?;

            let dwarf = Dwarf::load(&*mmap)?;

            let verbosity: u8 = verbose.into();

            if fast {
                let map = dwarf.get_named_types_map::<dwat::Struct>()?;
                for struc in map.values() {
                    println!("{}", struc.to_string_verbose(&dwarf, verbosity)?)
                }
            } else {
                let map = dwarf.get_fg_named_structs_map()?;
                for struc in map.values() {
                    println!("{}", struc.to_string_verbose(&dwarf, verbosity)?)
                }
            };
            std::process::exit(0)
        }
    };
}
