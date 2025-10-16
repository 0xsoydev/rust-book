fn main() {
    println!("Hello, world!");
    another_func(44);
    print_labeled_measurement(32,'h');
}

fn another_func(x: i32) {
    println!("The value of x is {x}");
}

fn print_labeled_measurement(value: i32, unit:char) {
    println!("The measurement is: {value}{unit}");
}

//Will throw an error as statements cannot return a result
// fn statement_error_func() {
//     let x = (let y = 7);
// }

//This runs since {} scope resolves to x = 4, NOTE: x + 1 doesnt have a colon (its an expression),
//adding semi-colon makes it a statement which doesn't return results
//writing x + 1 is same as writing "return x + 1" with a return type on the function
fn expression_func() {
    let y = {
        let x = 3;
        x + 1
    };
}


//Functions with return types
fn five() -> i32 {
    5
}
