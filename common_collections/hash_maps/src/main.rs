use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 420);
    scores.insert(String::from("Red"), 69);

    let team_name = String::from("Blue");

    let score = scores.get(&team_name);

    //using match to handle the Option<&i32> returning values, Some(n) and None by taking reference
    //of variable "score" instead of copying

    match score {
        Some(n) => println!("{n}"),
        None => println!("0"),
    }

    //We can use "if let" if we want to escape exhuastive type checking

    if let Some(score) = scores.get(&team_name) {
        println!("{score}");
    }

    //Here we took a copy of the results that scores.get() returned by converting Option<&i32> into
    //Option<i32> on which we then call unwrap_or(0) so that it returns a value of 0 if there is no
    //score stored for the "Blue" team

    let score_copy = scores.get(&team_name).copied().unwrap_or(0);
    println!("Copied Score: {}", score_copy);

    for (key, value) in &scores {
        //unnecessary here, just tried it out because i was curious
        //let formatted_output = format!("{key}: {value}");

        println!("{key}: {value}");
    }

    let mut map = HashMap::new();

    let field_name = String::from("Fav Color");
    let field_value = String::from("Blue");

    map.insert(field_name, field_value);

    let mut overwrite_scores = HashMap::new();
    overwrite_scores.insert(String::from("Red"), 500);
    overwrite_scores.insert(String::from("Red"), 467); //We can overwrite the value of the key by
                                                       //simply replacing the (K,V) with a
                                                       //different value (V)

    let mut insert_or_hashmap = HashMap::new();
    insert_or_hashmap.insert(String::from("Orange"), 660);

    insert_or_hashmap
        .entry(String::from("Orange"))
        .or_insert(990); //This will insert a new value 990 into the key "Orange" if the value
                         //doesnt already exist

    insert_or_hashmap
        .entry(String::from("Purple"))
        .or_insert(345); //Here in this case the key "Purple" value will be set to 345 since the
                         //key "Purple" didn't already have a value set for it.

    let iteration_str = "Hello World Hello User";
    let mut iter_map = HashMap::new();

    for word in iteration_str.split_whitespace() {
        let count = iter_map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{iter_map:?}")
}
