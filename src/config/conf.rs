extern crate toml;

// use std::vec;
use std::fs::File;
use std::fs;
use std::io::Write;


#[derive(Serialize, Deserialize)]
pub struct ConfGlobal {
    enableColor:bool,
    repo:Vec<Repo>

}

#[derive(Serialize, Deserialize)]
pub struct Repo {
    name:String,
    uri:String,
    enabled:bool
}

#[derive(Serialize, Deserialize)]
pub struct LocalConf {
    DevModeEnabled:bool,
    sigkey:String,
    buildDir:String
}


pub fn example_config() {
    let core = Repo { name:"Core".to_string(), uri:"blarg".to_string(), enabled:true};
    let meta = Repo {name:"Meta".to_string(), uri:"blarg".to_string(), enabled:true};
    let comunity = Repo {name:"comunity".to_string(), uri:"blarg".to_string(), enabled:true};
    let evil = Repo {name:"evil".to_string(), uri:"blarg".to_string(), enabled:true};
    let ConfGlobal = ConfGlobal {enableColor: true, repo: vec![core, meta, comunity, evil]};

    let t = toml::to_string(&ConfGlobal).unwrap();

    if !fs::metadata("mina.toml").is_ok() {
        let mut file = File::create("mina.toml").expect("Unable to create file");
        file.write_all(t.as_bytes()).expect("unable to write conf");
    }
}
