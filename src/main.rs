// use core::panic;
// use std::collections::HashMap;
use std::fs;
use std::io::{self, Read};
use std::net::IpAddr;
use std::{fs::File, io::ErrorKind};
fn main() {
    let home: IpAddr = "127.0.0.1"
        .parse()
        .expect("Hardcoded IP address should be valid");
    // ERROR HANDLING

    //Where the operator ? can be used...
    // fn last_char_of_first_line(text: &str) -> Option<char> {
    //     text.lines().next()?.chars().last()
    // }

    // Shortcut for propagation errors, the ? operator
    // fn read_username_from_file() -> Result<String, io::Error> {
    //     fs::read_to_string("hello.txt")
    // }

    // fn read_username_from_file() -> Result<String, io::Error> {
    //     let mut username = String::new();

    //     File::open("hello.txt")?.read_to_string(&mut username)?;

    //     Ok(username)
    // }

    // fn read_username_from_file() -> Result<String, io::Error> {
    //     let mut username_file = File::open("hello.txt")?;
    //     let mut username = String::new();
    //     username_file.read_to_string(&mut username)?;
    //     Ok(username)
    // }

    // Propagation errors...
    // fn read_username_from_file() -> Result<String, io::Error> {
    //     let username_file_result = File::open("hello.txt");

    //     let mut username_file = match username_file_result {
    //         Ok(file) => file,
    //         Err(e) => return Err(e),
    //     };

    //     let mut username = String::new();

    //     match username_file.read_to_string(&mut username) {
    //         Ok(_) => Ok(username),
    //         Err(e) => Err(e),
    //     }
    // }

    // expect
    // let greeting_file =
    //     File::open("hello.txt").expect("hello.txt should be included in this project.");

    // unwrap
    // let greeting_file = File::open("hello.txt").unwrap();

    // unwrap_or_else
    // let greeting_file = File::open("Hello.txt").unwrap_or_else(|error| {
    //     if error.kind() == ErrorKind::NotFound {
    //         File::create("hello.txt")
    //             .unwrap_or_else(|e| panic!("Problem creating the file: {:?}", e))
    //     } else {
    //         panic!("Problem opening the file {:?}", error);
    //     }
    // });

    // match
    // let greeting_file_result = File::open("hello.txt");

    // let greeting_file = match greeting_file_result {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(err) => panic!("Problem creating the file: {:?}", err),
    //         },
    //         other_error => {
    //             panic!("Problem opening the file: {:?}", other_error)
    //         }
    //     },
    // };
    // let v = vec![1, 2, 3];

    // v[99];

    // COMMON COLLECTIONS
    // let text = "This is my string string";

    // let mut map = HashMap::new();

    // for word in text.split_whitespace() {
    //     let count = map.entry(word).or_insert(0);
    //     *count += 1;
    // }

    // println!("{:?}", map);
}
