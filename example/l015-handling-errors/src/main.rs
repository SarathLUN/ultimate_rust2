use core::panic;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::{fs, i32, io, io::Read};

// 1. Error Handling with Result:
fn read_file(filename: &str) -> Result<String, std::io::Error> {
    fs::read_to_string(filename)
}

// 2. Unrecoverable Errors with panic!:
fn something_unexpected() {
    panic!("this is a critical error!");
}

// 3. Handling Errors with if let:
fn get_user_input() -> Result<i32, String> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap(); // this is simplicity, proper error handling needed in real application
    let trimmed = input.trim();
    if let Ok(num) = trimmed.parse::<i32>() {
        Ok(num)
    } else {
        Err(String::from("Invalid input. Please enter a number."))
    }
}

// 4. Default Values with match expressions:
fn get_saved_score() -> Result<i32, std::io::Error> {
    let mut file = fs::File::open("score.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    // parse the score from the string
    let score = contents.trim().parse::<i32>().map_err(|err| {
        std::io::Error::new(std::io::ErrorKind::Other, format!("Parse error: {}", err))
    })?;
    Ok(score)
}

// 5. Propagating Errors:
fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e), // propagate the error to the caller
    };
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// 6. Error Chaining with ? operator:
fn read_file_and_process(filename: &str) -> Result<String, std::io::Error> {
    let file = fs::File::open(filename)?; // Use ? to propagate errors up
    let mut contents = String::new();
    fs::read_to_string(&mut contents)?; // Propagate errors from read_to_string
    Ok(contents)
}

// 7. Handling Multiple Error Types with Traits:
// follow: https://github.com/SarathLUN/ultimate_rust2/tree/main/example/puzzle_game

#[allow(clippy::manual_unwrap_or)]
fn main() {
    // experiment #6
    let result = read_file_and_process("data.txt");
    match result {
        Ok(data) => println!("#6 File content: {}", data),
        Err(err) => println!("#6 Error: {}", err),
    }

    // experiment #5
    match read_username_from_file() {
        Ok(s) => println!("OK from #5: {}", s),
        Err(err) => println!("Error from #5: {}", err),
    };

    // experiment #4
    let score = match get_saved_score() {
        Ok(score) => score,
        Err(_) => 0, // Default score if reading fails
    };
    println!("Score: {}", score);

    // experiment #3
    println!("Enter a number:");
    let result3 = get_user_input();
    if let Err(err) = result3 {
        println!("Error: {}", err);
    } else {
        let num3 = result3.unwrap(); // this is safe here because of the if let checked.
        println!("Entered number: {}", num3);
    }
    // experiment #2
    something_unexpected(); // the code below this line will not run
    println!("Hello, world!");
    // experiment #1
    let result = read_file("data.txt");
    match result {
        Ok(data) => println!("File content: {}", data),
        Err(err) => println!("Error reading file: {}", err),
    }
}
