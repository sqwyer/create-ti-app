#![allow(unused_results,unused_must_use)]

use clap::Parser;
use std::path::Path;
use std::fs::{read_dir, copy, create_dir_all};
use std::io::Result;

#[derive(Parser)]
struct Cli {
    name: std::path::PathBuf
}

fn copy_recursively(source: impl AsRef<Path>, destination: impl AsRef<Path>) -> Result<()> {
    create_dir_all(&destination)?;
    for entry in read_dir(source)? {
        let entry = entry?;
        let filetype = entry.file_type()?;
        if filetype.is_dir() {
            copy_recursively(entry.path(), destination.as_ref().join(entry.file_name()))?;
        } else {
            copy(entry.path(), destination.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}

fn main() {
    let args = Cli::parse();
    copy_recursively(&Path::new("template"), &args.name);
    println!("New TI App Initialized!")
}