fn main() {
    let config_max = Some(3u8);
    //match config_max {
    //    Some(max) => println!("The maximum is configured to be : {max}"),
    //    _ => (), //Writing the edge cases handling can be annoying
    //}

    //This is much more convenient to write since we don't need to handle the cases we dont want
    if let Some(max) = config_max {
        println!("The max is configured to be: {max}");
    }
}
