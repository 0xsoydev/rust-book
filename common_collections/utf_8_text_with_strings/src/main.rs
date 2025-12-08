fn main() {
    let init_data = "Initial Contents";

    let s = init_data.to_string();

    //to_string() works directly on string contents as well

    //let s = "Initial Contents".to_string();

    let s1 = String::from("LO");
    let s2 = String::from("L");

    let concat = s1 + &s2;

    println!("{concat}");

    //This won't compile since the ownership of s1 has now moved to concat and nothing really
    //exists at s1

    //println!("{s1}");
    println!("{s2}");

    let first_str = String::from("tic");
    let second_str = String::from("tac");
    let third_str = String::from("toe");

    // The format!() macro formats makes it possible to format and concatenate strings together in
    // a human readable format

    let format_concat = format!("{first_str}-{second_str}-{third_str}");
    println!("{format_concat}");

    let hello = "Здравствуйте";

    //This wont compile as each cyrillic letter is made up of 2 bytes and since [0..1] only gets
    //the 0th byte, it doesn't have enough information to display the correct letter and hence it
    //panics

    //let cyrillic_s = &hello[0..1];
    let cyrillic_s = &hello[0..2]; //This works since it takes into account 0th and 1st byte
    println!("{cyrillic_s}");

    for c in hello.chars() {
        println!("{c}");
    }

    for c in hello.bytes() {
        println!("{c}");
    }
}
