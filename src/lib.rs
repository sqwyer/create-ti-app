#![allow(unused_results,unused_must_use)]

use std::{process::Command, fs::{create_dir, File}, path::{Path, PathBuf}, io::Write};

mod includes;

fn create_file(static_contents: &'static str, name: &str, filename: &str) {
    let mut p = name.to_string();
    p.push('/');
    p.push_str(filename);
    let mut c = File::create(p).expect(&format!("❌ Failed to create {}", &filename)); // ok to panic because this is for built-ins.
    let _ = c.write(static_contents.as_bytes());
}

pub fn git_is_installed() -> bool {
    let res = Command::new("cmd")
            .args(["git --ver"])
            .output();
    if res.is_err() {
        return false;
    } else {
        return true;
    }
}

// this does not work!
//
// pub fn init_git_repo(path: &PathBuf) {
//     Command::new("cmd")
//         .args([format!("cd {}",&path.display()), "git init".to_string()])
//         .output()
//         .expect("❌ Git repository failed");
// }

pub fn generate_files(path: &Path) {
    create_dir(&path);
    create_dir(&Path::new(&format!("{}/src", &path.display())));
    create_file(&includes::TICONFIG, &path.to_str().unwrap(), "ticonfig.json");
    create_file(&includes::KEYPRESS, &path.to_str().unwrap(), "src/KEYPRESS.8xp");
    create_file(&includes::MAIN, &path.to_str().unwrap(), "src/MAIN.8xp");
    create_file(&includes::UPDATE, &path.to_str().unwrap(), "src/UPDATE.8xp");
    create_file(&includes::_APP, &path.to_str().unwrap(), "src/_APP.8xp");
}