use parser::minaparser;

use parser::minaparser::MinaFile;

fn get_mina_file() -> MinaFile {
    //TODO get rid of this just a test file
    return minaparser::read_mina_file("./bin/htop.mina".to_string());
}

pub fn get_name() {
    let mina = get_mina_file();

    println!("{}", mina.version);
}