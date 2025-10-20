fn main() {
    let str1 = String::from("This is a string");

    let len_str = check_len(&str1);
    println!("{len_str}");
    println!("{str1}");

    //This will lead to an error since the string is immutable and mod_str is trying to modify the
    //string
    // let immut_str = String::from("This is an immutable string");
    // mod_str(&immut_str);

    let mut mut_str = String::from("This is a mutable string");
    mod_str(&mut mut_str);
    println!("{mut_str}");

    let mut s = String::from("This is a string");
    let r1 = &mut s;
    println!("{r1}");
    let r2 = &mut s;

    let r3 = &s;
    let r4 = &s;
    let r5 = &s;
    let r6 = &s;

    println!("{},{},{},{}", r3, r4, r5, r6);

    // let ref_to_nothing = dangle(); //the &s returned here leads to nothing (dangling pointer)
}

fn check_len(some_str: &String) -> usize {
    some_str.len()
}

fn mod_str(some_str: &mut String) {
    some_str.push_str(" exactly")
}

//This function returns variable s address pointer
// fn dangle() -> &String {
//     let s = String::from("This is a str");
//     &s
// } // s goes out of scope here
