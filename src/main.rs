#[macro_use]
extern crate serde_derive;
extern crate colored;

mod config;
mod parser;
mod package;

use colored::*;
use std::string::String;

use config::workspace;
use config::conf;

fn main() {
//    println!("Hello, world!");

//    minaparser::print_let();
//    println!("{}", "Createing mina file".green());
//    minaparser::writeToFile();
//    workspace::create_mina_dir();
    // pac
    conf::example_config();
    // workspace::create_workspace("Fooo");
}
