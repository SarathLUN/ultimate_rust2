use std::fs;

fn read_file(filename: &str) -> Result<String, std::io::Error> {
   fs::read_to_string(filename) 
}

fn main() {
    println!("Hello, world!");
    let result = read_file("data.txt");
    match result {
        Ok(data)=> println!("File content: {}", data),
        Err(err) => println!("Error reading file: {}", err),
    }
}
