fn main() {

    //constant declarations
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    // mutability
    let mut x = 5;
    println!("The value of x is : {x}");
    x = 6;

    //shadowing (reuse of x with declaration of 'let')
    let x = x + 1;

    //change of scope
    {
        let x = x * 2;
        println!("The value of x is : {x}");
    }

    println!("The value of x is : {x}");

    let mut spaces = "     ";
    let spaces = spaces.len();
}
