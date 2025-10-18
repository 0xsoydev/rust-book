fn main() {
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{s}");

    {
        let s = String::from("Valid String"); // s is valid here
    } //after this point on, s is no longer valid


    //Leads to an error since the value of s1 was moved to s2, s1 was dropped
    // let s1 = String::from("Hello World!");
    // let s2 = s1;
    // println!("{s1}");


    //This time s2 copies the data of s1 instead of copying the pointer (deep copying)
    let s1 = String::from("Hello World!");
    let s2 = s1.clone();
    println!("s1: {s1} \ns2: {s2}");


    //This works without calling clone because integers have a fixed size known at compile time
    //unlike strings, so integers are stored on stack unlike strings which are stored on heap
    //(due to unknown size at compile time)
    let x = 2;
    let y = x;
    println!("{y}");

    
    let some_str = String::from("This string is going to be borrowed");
    takes_ownership(some_str);

    let x = 5;
    own_number(x);

    let str1 = gives_ownership(); //str1 becomes the owner of whatever was stored in it as
                                  //give_ownership runs

    let str2 = String::from("Given String");
    let str3 = takes_and_gives_back(str2);

} //Here x and some_str both go out of scope. But since x and some_str ownership was moved, it
  //doesn't matter

fn takes_ownership(s: String) { //function takes_ownership becomes the owner of some_str
    println!("{s}");
}

fn own_number(x: i32) { //function takes_ownership becomes the owner of x
    println!("{x}");
}

fn gives_ownership() -> String {
    let some_str = String::from("String returned");
    some_str //value returned
}

fn takes_and_gives_back(some_str: String) -> String { //recieves value from str2 and str3
    some_str //returns back the ownership to str3
}
