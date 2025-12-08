fn main() {
    //This is how you explicitly make the code panic
    //panic!("Crash and Burn");

    //This code makes rust panic since the array index we're calling is out of bounds of the vector
    //v, in C this would result in returning garbage values from a particular memory location that
    //doesnt belong to this structure, this is called buffer overread and it leads to security
    //vulnerabilities

    let v = vec![1, 2, 3];
    v[99];
}
