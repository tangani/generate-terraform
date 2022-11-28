// use super::read_external_files::function;

use std::error::Error;
use std::fs;
use std::path::Path;

pub fn read_file() -> Result<(), Box<dyn Error>> {

    let _path = Path::new("/Users/molisa/Documents/rust/projects/src/test.txt");
    let _file: Result<String, std::io::Error> = fs::read_to_string(&_path);

    println!("{:?}", _file);

    let binding: String = file?;
    let sentences = binding.split("\n");

    for line in sentences {
        println!("{:?}", line);
    }
    Ok(())

}