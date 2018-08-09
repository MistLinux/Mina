use self::parser::minaparser;

fn get_mina_file() -> minaparser {
    //TODO get rid of this just a test file
    return minaparser::read_mina_file("./bin/mina.yml");
}

fn getName() {
    let mina = get_mina_file();
    println!("{}", mina.name);
}