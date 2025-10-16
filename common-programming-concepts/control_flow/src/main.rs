fn main() {
    let number = 3;
    if number < 5 { // if number...  leads to error as there is no bool value, instead an integer
        println!("Condition was true");
    } else {
        println!("Condition was false");
    }

    if number != 0 {
        println!("number was something other than zero");
    }

    if number % 5 == 0 {
        println!("number is divisible by 5");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    }

    //inline way to write it 
    let condition = true;
    let number = if condition {5} else {6};
    println!("the value of number is : {number}");

    //infinite loop
    // loop {
    //     println!("infinite loop");
    // }

    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("the result is {result}");

    nested_loop();
    while_loop();
}

//nested function loop 
fn nested_loop() {
    let mut count = 0;
    'counting_up:loop {
        println!("Count: {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }

    println!("End count: {count}");
}

fn while_loop() {
    let mut number = 3;

    while number != 0 {
        println!("{number}");
        number -= 1;
    }

    println!("LIFT OFF!");
}

//basic for loop iterating through an array
fn for_loop() {
    let a = [1,2,3,4,5,6];

    for element in a {
        println!("{element}");
    }
}

fn short_loop() {
    for number in (1..4).rev() {
        println!("{number}");
    }
    println!("LIFTOFF!!");
}
