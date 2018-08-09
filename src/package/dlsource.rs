use parser::minaparser::MinaFile;
use std::io::{Write};
use std::fs::File;
use curl::easy::Easy;


pub fn get_source(mina: MinaFile) {
    let name = mina.name;
    let version = mina.version;
    let source = mina.source;

    let newsrc = source.replace("{VERSION}", &version);

    let spltname:Vec<&str> = newsrc.split("/").collect();

    let last = &spltname.as_slice().last();

    let mut buffer = File::create(last.expect("blarg")).expect("Somthing went wrong righting the file");

    let mut easy = Easy::new();
    easy.url(&newsrc).unwrap();
    easy.write_function(move |data| {
        // stdout().write_all(data).unwrap();
        buffer.write(data).expect("somthing went wrong downloading the file");
        Ok(data.len())
    }).unwrap();
    easy.perform().unwrap();

    println!("{}", newsrc);
}