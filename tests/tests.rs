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

    std::thread::sleep(std::time::Duration::from_secs(3));

    let out_path = tmp_dir.path().join("bin");
    let output = Command::new("gcc")
        .arg(&src_path)
        .arg("-g")
        .arg("-o")
        .arg(&out_path)
        .output()?;

    if !output.status.success() {
        eprintln!("gcc failed: {}", String::from_utf8_lossy(&output.stderr));
        return Err(anyhow::anyhow!("Compilation failed"));
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
    println!("{:?}", path.to_str());

    let file = File::open(&path)?;
    let mmap = unsafe { Mmap::map(&file) }?;
    let mut dwarf = Dwarf::parse(&*mmap)?;

    let found = dwarf.lookup_item::<dwat::Struct>("simple".to_string())?;
    assert!(found.is_some());

    let found = found.unwrap();
    assert!(found.members(&dwarf)?.len() == 1);

    let byte_size = found.byte_size(&dwarf)?;
    assert!(byte_size.is_some());
    assert!(byte_size.unwrap() == 8);

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
    println!("{:?}", path.to_str());

    let file = File::open(&path)?;
    let mmap = unsafe { Mmap::map(&file) }?;
    let mut dwarf = Dwarf::parse(&*mmap)?;

    let found = dwarf.lookup_item::<dwat::Struct>("padded".to_string())?;
    assert!(found.is_some());

    let found = found.unwrap();
    assert!(found.members(&dwarf)?.len() == 2);

    // Expect padding on the int to push the size from 12 to 16
    let byte_size = found.byte_size(&dwarf)?;
    assert!(byte_size.is_some());
    assert!(byte_size.unwrap() == 16);

    let offsets = found.members(&dwarf)?.into_iter().map(|memb| {
        memb.member_location(&dwarf)
    }).collect::<Vec<_>>();

    if let Ok(Some(first_offset)) = offsets[0] {
        assert!(first_offset == 0);
    } else {
        panic!("failed to get first offset");
    }

    if let Ok(Some(second_offset)) = offsets[1] {
        assert!(second_offset == 8);
    } else {
        panic!("failed to get second offset");
    }

    Ok(())
}
