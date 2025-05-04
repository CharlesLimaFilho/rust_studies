// Primitive str: Immutable, fixed-length string
// String: Growable, heap-allocated data structure


pub fn run() {
    //let hello = "Hello";
    let mut hello_string = String::from("Hello ");

    // Get length
    println!("Length: {}", hello_string.len());

    // Push char
    hello_string.push('W');

    // Push string
    hello_string.push_str("orld!");

    // Capacity in bytes
    println!("Capacity: {}", hello_string.capacity());

    // Check if empty
    println!("Is empty: {}", hello_string.is_empty());

    // Contains
    println!("Contains 'World': {}", hello_string.contains("World"));

    // Replace
    println!("Replace: {}", hello_string.replace("World", "There"));

    // Loop througth string by whitespace
    for word in hello_string.split_whitespace() {
        println!("{}", word);
    }

    // Create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');


    // Assertion testing
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());

    println!("{}", s);
}