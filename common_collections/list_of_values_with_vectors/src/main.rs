fn main() {
    //let v: Vec<i32> = Vec::new(); //Empty vector with specified type

    //let v = vec![1, 2, 3]; // Convenient vector macro for instant initalization with values

    //let mut v = Vec::new();
    //
    //v.push(1);
    //v.push(2);
    //v.push(3);

    let mut v = vec![11, 22, 33, 44, 55, 66];
    let third: &i32 = &v[2];
    println!("The third number is {}", third);

    let get_third: Option<&i32> = v.get(2);
    match get_third {
        Some(n) => println!("The third number is {}", n),
        None => println!("There is no third element"),
    }

    //This code does not compile since an attempt to modify the vector was made after the value was
    //read, the modification of the vector leads to changing the memory location of the vector,
    //copying all the elements including the new one and hence the variable that reads the vector
    //now points to a deallocated memory location

    //let mut v2 = vec![4, 5, 6, 7, 8];
    //let first = &v2[0];
    //v2.push(9);
    //
    //println!("The first element is {}", first);

    for i in &v {
        println!("{i}")
    }

    //Need to use dereference operator (*) to access the value in the vector to modify it
    for i in &mut v {
        *i += 50;
        println!("{i}")
    }

    #[derive(Debug)]

    //If we want to use multiple data types within a vector, we can use enums within vectors
    enum Spreadsheet {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        Spreadsheet::Int(3),
        Spreadsheet::Float(69.420),
        Spreadsheet::Text(String::from("John Doe")),
    ];

    for i in &row {
        println!("{i:?}");
    }
}
