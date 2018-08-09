
use parser::minaparser;
use parser::minaparser::{MinaFile, Pkg};
use std::process::Command;
use package::dlsource;

fn get_mina_file() -> MinaFile {
    //TODO get rid of this just a test file
    return minaparser::read_mina_file("./bin/htop.mina".to_string());
}

pub fn get_name() {
    let mina = get_mina_file();
    println!("{}\n", mina.version);

    pkg_runner(mina);
}

fn pkg_runner(mina:MinaFile) {
    println!("Downloading sources");

    let min =  mina;

    let download = min.clone();
    let prebuild = min.package.pre_build.clone();
    let version  = min.version.clone();
    dlsource::get_source(download);
    pre_build(prebuild, version);
    println!("blarg", )
}

fn pre_build(cmds:Vec<String>, version:String) {

    for cmd in cmds.iter() {
        let mut presplit = cmd.replace("{VERSION}", version.as_str());
        Command::new("sh").arg("-c").arg(presplit).spawn().expect("balrg");
    }
    println!("finished prbild");
}