use std::mem;

pub fn run() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4];

    // Re-assign a value
    numbers[2] = 20;

    // Add on to vector
    numbers.push(5);
    numbers.push(6);

    // Pop off last value
    numbers.pop();

    println!("{:?}", numbers);

    // Get single val
    println!("Single value: {}", numbers[0]);

    // Get vector len
    println!("Vector Length: {}", numbers.len());

    // Arrays are stack allocated
    println!("Vector occupies {} bytes", mem::size_of_val(&numbers));

    // Get slice
    let slice: &[i32] = &numbers[1..3];
    println!("Slice: {:?}", slice);
}