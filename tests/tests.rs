use std::{io::Write, path::PathBuf, process::Command};
use dwat::Dwarf;
use std::fs::File;
use memmap2::Mmap;
use tempfile::TempDir;

use dwat::prelude::*;

fn compile(source: &str) -> anyhow::Result<(TempDir, PathBuf)> {
    let tmp_dir = TempDir::new()?;
    let src_path = tmp_dir.path().join("src.c");

    {
        let mut tmp_file = File::create(&src_path)?;
        tmp_file.write_all(source.as_bytes())?;
    }

    let out_path = tmp_dir.path().join("bin");
    let output = Command::new("gcc")
        .arg(&src_path)
        .arg("-gdwarf-5") // TODO: Allow this to be configurable, env var maybe
        .arg("-o")
        .arg(&out_path)
        .output()?;

    if !output.status.success() {
        panic!("gcc failed: {}", String::from_utf8_lossy(&output.stderr));
    }

    Ok((tmp_dir, out_path))
}

const SIMPLE: &str = "
struct simple {
    unsigned long long s;
};
int main() {
    struct simple s;
}";


#[test]
fn simple_struct() -> anyhow::Result<()> {
    let (_tmpdir, path) = compile(SIMPLE)?;

    let file = File::open(&path)?;
    let mmap = unsafe { Mmap::map(&file) }?;
    let dwarf = Dwarf::load(&*mmap)?;

    let found = dwarf.lookup_type::<dwat::Struct>("simple".to_string())?;
    assert!(found.is_some());

    let found = found.unwrap();
    assert!(found.members(&dwarf)?.len() == 1);

    let byte_size = found.byte_size(&dwarf)?;
    assert!(byte_size == 8);

    Ok(())
}

const PADDED: &str = "
struct padded {
    unsigned int ui;
    unsigned long long ull;
};
int main() {
    struct padded p;
}";

#[test]
fn padded_struct() -> anyhow::Result<()> {
    let (_tmpdir, path) = compile(PADDED)?;

    let file = File::open(&path)?;
    let mmap = unsafe { Mmap::map(&file) }?;
    let dwarf = Dwarf::load(&*mmap)?;

    let found = dwarf.lookup_type::<dwat::Struct>("padded".to_string())?;
    assert!(found.is_some());

    let found = found.unwrap();
    assert!(found.members(&dwarf)?.len() == 2);

    // Expect padding on the int to push the size from 12 to 16
    let byte_size = found.byte_size(&dwarf)?;
    assert!(byte_size == 16);

    let offsets = found.members(&dwarf)?.into_iter().map(|memb| {
        memb.offset(&dwarf)
    }).collect::<Vec<_>>();

    if let Ok(first_offset) = offsets[0] {
        assert!(first_offset == 0);
    } else {
        panic!("failed to get first offset");
    }

    if let Ok(second_offset) = offsets[1] {
        assert!(second_offset == 8);
    } else {
        panic!("failed to get second offset");
    }

    Ok(())
}

const PACKED: &str = "
struct packed {
    unsigned int ui;
    unsigned long long ull;
} __attribute__((packed));
int main() {
    struct packed p;
}";

#[test]
fn packed_struct() -> anyhow::Result<()> {
    let (_tmpdir, path) = compile(PACKED)?;

    let file = File::open(&path)?;
    let mmap = unsafe { Mmap::map(&file) }?;
    let dwarf = Dwarf::load(&*mmap)?;

    let found = dwarf.lookup_type::<dwat::Struct>("packed".to_string())?;
    assert!(found.is_some());

    let found = found.unwrap();
    assert!(found.members(&dwarf)?.len() == 2);

    // Expect packing to smoosh the long and int against eachother
    let byte_size = found.byte_size(&dwarf)?;
    assert!(byte_size == 12);

    let offsets = found.members(&dwarf)?.into_iter().map(|memb| {
        memb.offset(&dwarf)
    }).collect::<Vec<_>>();

    if let Ok(first_offset) = offsets[0] {
        assert!(first_offset == 0);
    } else {
        panic!("failed to get first offset");
    }

    if let Ok(second_offset) = offsets[1] {
        assert!(second_offset == 4);
    } else {
        panic!("failed to get second offset");
    }

    Ok(())
}
