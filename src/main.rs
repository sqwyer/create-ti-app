#![allow(unused_results,unused_must_use)]

use clap::Parser;
use std::{io, fs, path::Path};

fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}

#[derive(Parser)]
struct Cli {
    name: std::path::PathBuf
}

fn main() {
    let args = Cli::parse();
    copy_dir_all("../template", args.name);
}