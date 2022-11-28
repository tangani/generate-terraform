use std::error::Error;
use std::fs;
use std::path::Path;

// pub fn read_file(file_location: &String) -> Result<(), Box<dyn Error>> {

pub fn read_file(file_location: &str) -> Result<Vec<String>, Box<dyn Error>> {

    let _path = Path::new(file_location);
    let _file: Result<String, std::io::Error> = fs::read_to_string(&_path);

    println!("{:?}", _file);

    let binding: String = _file?;
    let _sentences: Vec<String> = binding.split("\n").map(|s| s.to_string()).collect();

    /*
    for line in sentences {
        println!("{:?}", line);
    }
     */
    Ok(_sentences)
    // return sentences;
}