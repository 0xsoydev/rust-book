fn main() {
    let mut s = String::from("Hello World!");
    let word = first_word(&s);
    //s.clear();
    println!("{word}");

    //String slice
    let hello = &s[0..5];
    let world = &s[6..11];

    //Alternative string slice
    let slice = &s[..2]; //This is the same as &s[0..2]
    let slice = &s[3..]; //This is the same as &s[3..s.len()]

    let word_slice = first_word_w_slices(&s);
    println!("{}", word_slice);
}

//first word function without using slices
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word_w_slices(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
