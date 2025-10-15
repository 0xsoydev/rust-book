use std::io;

fn main() {
    // datatype specifier :u32 is needed for .parse() to convert datatypes
    let guess: u32 = "69".parse().expect("Not a number!");
    println!("{guess}");

    //mathematical operations

    let x = 2.4; //f64 (default)
    let y: f32 = 9.6; //f32 explicitly specified

    let sum = 5 + 10;
    let difference = 10.2 - 6.9;
    let product = 5 * 99;

    let quotient = 56 / 3;
    let truncated = -5 / 3;

    let modulus = 43 % 5;

    //Booleans
    let t = true;
    let f: bool = false; //explicit annotation

    let c = 'z';
    let z: char = 'Z';
    let some_emoji = '😺';

    //tuples
    let tup: (i32, f64, u32) = (-66, 9.21123, 3333);


    //destructuring tuples
    let tup = (500, 9.2, 22);
    let (x, y, z) = tup;

    println!("The value of y is : {y}");

    //arrays
    let a = [1,2,3,4,5];
    let months = ["Jan", "Feb", "Mar"];

    let a: [f64; 5] = [1.2, 2.3, 3.4, 4.5, 5.6]; //explicitly typed array
    
    //array of fixed length with same value
    let a = [3; 5];

    //accessing array elements
    let first = a[0];
    let second = a[1];


    //Invalid Array Element Access
    {
        let a = [1,2,3,4,5];
        println!("Please enter the index value of an array: ");
        
        let mut index = String::new();

        io::stdin()
            .read_line(&mut index)
            .expect("failed to read line");

        let index: usize = index
            .trim()
            .parse()
            .expect("Index entered was not a number");

        let element = a[index];

        println!("The value of element at index {index} is: {element}");
    }
}
