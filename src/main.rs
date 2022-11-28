// use std::fs::read_to_string;
// use crate::core;

mod scripts;

fn main() {

    let _contents = scripts::read_external_files::read_file("/Users/molisa/Documents/rust/projects/src/test.txt");

    println!("{:?}", _contents);

    /*
    for entry in contents {
        println!("{:?}", entry);
    }
     */
}
