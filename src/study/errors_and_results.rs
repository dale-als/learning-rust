use std::fs::{self, File};
use std::io::{Error, ErrorKind, Read};

fn main() {
    // panic!("crash and burn"); -- excplicilty calling panic
    let v = vec![1, 2, 3];

    //v[99]; -- implicitly causing panic by accesing out of index bounds

    // enum Result<T, E> {
    //     Ok(T),
    //     Err(E),
    // } -- can be matched by Ok where generic T is resulting type or Err where E is generic error

    let greeting_file_result = File::open("hello.txt"); // returns Result

    let file = match greeting_file_result {
        Ok(file) => file, // if file opens - return handle
        Err(error) => match error.kind() {
            // if error - match error by type
            ErrorKind::NotFound => match File::create("hello.txt") {
                // if no file - try to create
                Ok(fc) => fc, // if ok - return handle to created file
                Err(e) => panic!("Problem creating {:?}", e), // if error - panic
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error); // if any other error - panic
            }
        },
    };

    let greeting_file = File::open("hello.txt").unwrap();
    // unwraps result, ether returns contains of Ok or panic with error from E

    let greeting_file = File::open("hello.txt").expect("No such file!");
    // expects result, ether returns contains of Ok or panic with error from param

    let username = read_username_from_file();

    // let greeting_file = File::open("hello.txt")?; -- ? can only be use in function that returns Result, Option etc
}

// propagating error - returning error to the upper scope
// generic T in Result Ok replaced with String - fn returs Result Ok<String> on success
// generic E in Result Err replaced with io::Error - fn returns Result Err<Error> on error
fn read_username_from_file() -> Result<String, Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

// ? symbol after Result works almost as match
// if Result Ok - value inside assigned to let
// if Result Error - errir inside immediately returned from the function
// ? implicitly convert any error type to Error (defined in fn return signature)
fn propagate_with_shortcut() -> Result<String, Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

// shorter verison
fn read_username_from_file_2() -> Result<String, Error> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}

// even shorter version
fn read_username_from_file_3() -> Result<String, Error> {
    fs::read_to_string("hello.txt") // return Ok<String> or Err<Error>
}

// using ? with Option: will return None if Option match None
fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

// Main function returning Result
// use std::error::Error;
// use std::fs::File;

// fn main() -> Result<(), Box<dyn Error>> {
//     let greeting_file = File::open("hello.txt")?;

//     Ok(())
// }
// When a main function returns a Result<(), E>, the executable will exit
// with a value of 0 if main returns Ok(()) and will exit with a nonzero value
// if main returns an Err value.
// this is why code 0 is a success for executables written in C, for example
// The main function may return any types that implement the std::process::Termination
// trait, which contains a function report that returns an ExitCode.
