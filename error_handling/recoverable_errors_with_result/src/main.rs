use std::error::Error;
use std::fs::File;
use std::io::{self, ErrorKind, Read};
use std::{char, fs};

fn main() {
    let greeting_file_result = File::open("src/hello.txt");

    //The File::open returns a Result<T, E> type where the result is either Ok(Type) or an Err()

    //let greeting_file = match greeting_file_result {
    //    Ok(file) => file,
    //    Err(error) => panic!("Problem opening the file: {error:?}"),
    //};

    //We can use ErrorKind enum to perform custom operations like creating a file when
    //ErrorKind::NotFound exists and handling all the other exhaustive cases

    let greeting_file_errorkind = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Error creating file: {e:?}"),
            },
            _ => {
                panic!("Problem opening the file: {error:?}")
            }
        },
    };

    //Another way to do it would be via using closures and if else statements, closures are like
    //anonymous functions, when File::open or File::create return error types, unwrap_or_else()
    //consumes that error and returns a type Result<T, E>, basically returning a value or an error
    //if an action was unable to be performed

    let greeting_file_unwrap = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Failed to create file: {error:?}");
            })
        } else {
            panic!("Problem opening the file: {error:?}");
        }
    });

    //expect is a less notorious version of .unwrap() where it just panics showing custom user
    //defined errors in case of operation failures

    let greeting_file_expect =
        File::open("hello.txt").expect("hello.txt should be a part of this project");

    // Here we're returning the return values either Ok() or Err() to the calling function as it
    // is, this is a better approach since the calling function usually has more context over the
    // following error

    fn read_from_file() -> Result<String, io::Error> {
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

    //This is a much shorter way of propogating error to the calling function by using ? operatior

    fn read_username_from_file() -> Result<String, io::Error> {
        let mut username_file = File::open("hello.txt")?;
        let mut username = String::new();

        username_file.read_to_string(&mut username)?;
        Ok(username)
    }

    //This is a much ergonomic way to write the same function

    fn read_username_from_file_short() -> Result<String, io::Error> {
        let mut username = String::new();
        File::open("hello.txt")?.read_to_string(&mut username)?;

        Ok(username)
    }

    //Reading strings from file is common so rust has a method called fs::read_to_string() to do
    //the same directly

    fn read_file_method() -> Result<String, io::Error> {
        fs::read_to_string("hello.txt")
    }

    //This function will panic because it doesn't have a compatible return type the ? operator
    //supports

    //fn panic_read_file() {
    //    let greeting_file = File::open("hello.txt")?;
    //}

    //The ? operator here works out because it needs either Result<T, E> or Option<T> to propogate
    //a result back to the calling function or else it panics

    fn last_char_of_first_line(text: &str) -> Option<char> {
        text.lines().next()?.chars().last()
    }

    // Another option is to use `dyn` if the type of error is unknown, inside a Box so it's memory
    // allocation for the error is calculated in the heap, realtime during complilation,

    fn dyn_error() -> Result<(), Box<dyn Error>> {
        let greeting_file = File::open("hello.txt")?;
        Ok(())
    }
}
