extern crate serde_yaml;

use std::fs;
use std::fs::File;
use std::io::Write;
use std::io::prelude::*;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MinaFile {
    name:String,
    version:String,
    pkg_rel:String,
    provides:String,
    source:String,
    maintainers:Vec<Maintainers>,
    tags:Vec<String>,
    dependencies: Deps,
    package:Pkg
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Maintainers {
    name:String,
    email:String,
    role:String
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Pkg {
    gpg_sign:bool,
    hash:bool,
    pre_build:Vec<String>,
    build:Vec<String>,
    post_build:Vec<String>

}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Deps {
    required:Vec<String>,
    optional:Vec<String>
}

pub fn print_example() {
    let man1 = Maintainers {name: "test name 1".to_string(), email: "test1@example.com".to_string(),role: "Maintainer".to_string()};
    let man2 = Maintainers {name: "test name 2".to_string(), email: "test2@example.com".to_string(),role: "Contributor".to_string()};


    let pak = Pkg {gpg_sign: true, hash: false, pre_build: vec!["cd foo".to_string(), "make check".to_string()],
        build: vec!["make build".to_string()], post_build: vec!["echo blarge".to_string()]
    };

    let dep = Deps {required: vec!["glibc".to_string(), "clang".to_string()],
        optional: vec!["".to_string()]};

    let point =  MinaFile {name: "foo".to_string(), version: "0.0.1".to_string(), pkg_rel: 1.to_string(),
        maintainers:  vec![man1, man2], tags: vec!["Core".to_string(), "package manager".to_string()],
        source: ".".to_string(), package: pak , dependencies: dep, provides:"".to_string()};
    let s = serde_yaml::to_string(&point).unwrap();

    if !fs::metadata("example.mina.yml").is_ok() {
        println!("creating example mina.yml");

        let mut file = File::create("mina.yml").expect("Unable to create file");
        file.write_all(s.as_bytes()).expect("Unable to write data");
    }
}

pub fn read_mina_file(file:String) -> MinaFile {
    let mut f = File::open(file).expect("File not found");
    let mut content = String::new();
    f.read_to_string(&mut content).expect("Unable to read mina.yml");

    let mina:MinaFile = serde_yaml::from_str(&content).unwrap();

    return mina;

}