use std::fs;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};

use std::env;

fn get_mina_dir() -> PathBuf {

    let mut mina = env::home_dir().unwrap();
    mina.push("mina");
    mina.as_path();

    if !fs::metadata(&mina).is_ok() {
        fs::create_dir(&mina).expect("mina not found")
    }

    mina
}

pub fn create_workspace(str:&str) {
    let mut workspace = get_mina_dir();
    workspace.push(&str);
    workspace.as_path();

    if !workspace.exists() {
        fs::create_dir(&workspace);
    }
}

