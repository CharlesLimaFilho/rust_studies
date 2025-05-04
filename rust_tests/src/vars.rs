// Variables are immutable by default
// but "mut" changes it

pub fn run() {
    let name = "Jonas";
    let mut age = 21;

    println!("My bro's name is {} and he is {}", name, age);

    age = 22;
    println!("My bro's name is {} and he is {}", name, age);

    // Define constant
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // Assign multiple vars
    let ( name1, age1 ) = ("Jonas", 21);
    println!("{} is {}", name1, age1);
}