#[macro_use]
extern crate serde_derive;
// extern crate colored;
extern crate curl;
mod config;
mod parser;
mod package;

// use colored::*;

// use config::workspace;
use config::conf;
use package::packager;

fn main() {
//    println!("Hello, world!");

    packager::get_name();
//    println!("{}", "Createing mina file".green());
//    minaparser::writeToFile();
//    workspace::create_mina_dir();
    // pac
    conf::example_config();
    // workspace::create_workspace("Fooo");
}
